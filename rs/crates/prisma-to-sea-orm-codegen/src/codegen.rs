use heck::{ToSnakeCase, ToUpperCamelCase};
use indexmap::{IndexMap, IndexSet};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{punctuated::Punctuated, token::Comma};

use crate::prisma_dmmf::*;

fn prisma_enum(prisma_dmmf_datamodel_enum: &DatamodelEnum) -> TokenStream {
    let enum_name = &prisma_dmmf_datamodel_enum
        .db_name
        .as_ref()
        .unwrap_or(&prisma_dmmf_datamodel_enum.name)
        .to_string();
    let enum_iden = prisma_enum_iden(&prisma_dmmf_datamodel_enum.name);
    let enum_doc = prisma_dmmf_datamodel_enum
        .documentation
        .as_ref()
        .map(|d| quote! { #[doc = #d] })
        .unwrap_or_default();
    let values = prisma_dmmf_datamodel_enum
        .values
        .iter()
        .map(|v| v.db_name.as_ref().unwrap_or(&v.name).to_string())
        .collect::<Vec<_>>();
    let variants = prisma_dmmf_datamodel_enum.values.iter().map(|v| &v.name).map(|v| {
      safe_enum_variant_name(v, format!("Warning: item '{v}' in the enumeration '{enum_name}' cannot be converted into a valid Rust enum member name. It will be converted to its corresponding UTF-8 encoding. You can modify it later as needed."))
  });

    quote! {
      #enum_doc
      #[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
      #[sea_orm(rs_type = "String", db_type = "Enum", enum_name = #enum_name)]
      pub enum #enum_iden {
          #(
              #[sea_orm(string_value = #values)]
              #variants,
          )*
      }
    }
}

fn prisma_enum_iden(enum_name: impl AsRef<str>) -> Ident {
    format_ident!("{}", enum_name.as_ref().to_upper_camel_case())
}

fn safe_enum_variant_name(value: impl AsRef<str>, warning: impl AsRef<str>) -> Ident {
    let v = value.as_ref();
    if v.chars().next().map(char::is_numeric).unwrap_or(false) {
        format_ident!("_{}", v)
    } else {
        let variant_name = v.to_upper_camel_case();
        if variant_name.is_empty() {
            println!("{}", warning.as_ref());
            let mut ss = String::new();
            for c in v.chars() {
                ss = if c.len_utf8() > 1 {
                    format!("{ss}{c}")
                } else {
                    format!("{ss}U{:04X}", c as u32)
                }
            }
            format_ident!("{}", ss)
        } else {
            format_ident!("{}", variant_name)
        }
    }
}

pub struct ModelCodegen {
    pub use_declarations: Vec<TokenStream>,
    pub model: TokenStream,
}

fn prisma_model(prisma_dmmf_model: &Model, prisma_dmmf_indexes: &[Index]) -> ModelCodegen {
    let prisma_dmmf_indexes_for_model = prisma_dmmf_indexes
        .iter()
        .filter(|i| i.model == prisma_dmmf_model.name)
        .collect::<Vec<_>>();

    let table_name = prisma_table_name(prisma_dmmf_model);
    let model_doc = prisma_dmmf_model
        .documentation
        .as_ref()
        .map(|d| quote! { #[doc = #d] })
        .unwrap_or_default();

    let model_fields = prisma_dmmf_model
        .fields
        .iter()
        .filter(|f| match f.kind {
            FieldKind::Enum | FieldKind::Scalar => true,
            // NB: `Object` is a relation
            FieldKind::Object | FieldKind::Unsupported => false,
        })
        .collect::<Vec<_>>();

    let enum_use_declarations = model_fields
        .iter()
        .filter_map(|f| {
            if let (FieldKind::Enum, FieldType::ModelName(enum_name)) = (&f.kind, &f.r#type) {
                Some(enum_name)
            } else {
                None
            }
        })
        .collect::<IndexSet<_>>()
        .into_iter()
        .map(|enum_name| {
            let enum_iden = prisma_enum_iden(enum_name);
            quote! {
              use super::sea_orm_active_enums::#enum_iden;
            }
        })
        .collect::<Vec<_>>();

    let fields = model_fields
        .iter()
        .map(|f| format_ident!("{}", escape_rust_keyword(f.name.to_snake_case())))
        .collect::<Vec<_>>();
    let types = model_fields
        .iter()
        .map(|f| prisma_field_type(f))
        .collect::<Vec<_>>();
    let model_fields_have_primary_key = model_fields.iter().any(|f| f.is_id);
    let attributes = model_fields.iter().enumerate().map(|(f_index, f)| {
    let prisma_dmmf_indexes_for_field = prisma_dmmf_indexes_for_model.iter().filter(|i| {
      i.fields.iter().any(|r#if| r#if.name == f.name)
    }).collect::<Vec<_>>();
    let field_doc = f.documentation.as_ref().map(|d| quote! { #[doc = #d] }).unwrap_or_default();
    let mut attrs: Punctuated<_, Comma> = Punctuated::new();

    let column_name = f.db_name.as_ref().unwrap_or(&f.name);
    attrs.push(quote! { column_name = #column_name });

    // See <https://docs.rs/sea-query/latest/sea_query/table/enum.ColumnType.html#variants> for more.
    let column_type = match (&f.r#type, f.native_type.as_ref().map(|nt| (nt.0.as_str(), nt.1.as_slice()))) {
      (_, Some(("Timestamptz", _))) => Some("TimestampWithTimeZone".to_string()),
      (_, Some(("VarChar", [limit]))) => Some(format!("String(StringLen::N({}))", limit)),
      (_, Some(("VarChar", _))) => Some("String(StringLen::None)".to_string()),
      (_, Some(("Char", [limit]))) => Some(format!("Char(Some({}))", limit)),
      (_, Some(("Char", _))) => Some("Char(None)".to_string()),
      (_, Some(("Text", _))) => Some("Text".to_string()),
      (_, Some(("Uuid", _))) => Some("Uuid".to_string()),
      (_, Some((native_db_type, native_db_type_args))) => {
        println!("Warning: column '{column_name}' in the model '{table_name}' has an unknown column type '{native_db_type}({})'. Ignoring this type.", native_db_type_args.join(","));
        None
      },
      (FieldType::Json, _) => Some("JsonBinary".to_string()),
      (FieldType::Float, _) => Some("Double".to_string()),
      // unimplemented
      _ => None
    };
    if let Some(column_type) = column_type {
      attrs.push(quote! { column_type = #column_type });
      if !f.is_required {
        attrs.push(quote! { nullable });
      }
    }

    let mut primary_key = f.is_id;
    let mut indexed = false;
    let mut unique = false;
    let mut unique_keys = IndexSet::new();
    for index in prisma_dmmf_indexes_for_field {
      match index.r#type {
        IndexType::Id => {
          primary_key = true;
        },
        IndexType::Normal => {
          // NB: SeaORM models cannot configure/express compound indexes
          indexed = true;
        }
        IndexType::Unique => {
          if index.is_defined_on_field {
            unique = true;
          } else {
            unique_keys.insert(index.db_name.clone().unwrap_or_else(|| index.fields.iter().map(|r#if| r#if.name.to_string()).collect::<Vec<_>>().join("_")));
          }
        },
        IndexType::FullText => {},
      }
    }

    // NB: SeaORM requires a primary key 🤷 if none exists, just make each column a part of a
    // composite primary key.
    // https://github.com/SeaQL/sea-orm/issues/485
    if (model_fields_have_primary_key && primary_key) || (
      // NB: 12 is the SeaORM imposed max composite primary key size, and primary keys cannot use
      // optional columns
      !model_fields_have_primary_key && f_index < 12 && f.is_required
    ) {
      attrs.push(quote! { primary_key });
      attrs.push(quote! { auto_increment = false });
    }

    if indexed {
      attrs.push(quote! { indexed })
    }
    if unique {
      attrs.push(quote! { unique });
    }
    for unique_key in unique_keys.iter() {
      attrs.push(quote! { unique_key = #unique_key });
    }

    if let Some(default) = &f.default {
      match default {
        FieldDefaultScalarUnion::Default(field_default) => {
          if let Some(default_expr) = match (field_default.name.as_str(), field_default.args.as_slice()) {
            ("now", []) => Some("Expr::current_timestamp()"),
            // unimplemented
            (_,_) => None,
          } {
            attrs.push(quote! { default_expr = #default_expr });
          }
        },
        FieldDefaultScalarUnion::DefaultScalar(scalar) => {
          match scalar {
            FieldDefaultScalar::Bool(default_value) => {
              attrs.push(quote! { default_value = #default_value });
            },
            FieldDefaultScalar::Number(json_number) => {
              match f.r#type {
                FieldType::BigInt => {
                  if let Some(default_value) = json_number.as_i64() {
                    attrs.push(quote! { default_value = #default_value });
                  }
                },
                FieldType::Float => {
                  if let Some(default_value) = json_number.as_f64() {
                    attrs.push(quote! { default_value = #default_value });
                  }
                },
                FieldType::Int => {
                  if let Some(default_value) = json_number.as_i64() {
                    let default_value = default_value as i32;
                    attrs.push(quote! { default_value = #default_value });
                  }
                },
                _ => {
                  // unimplemented
                }
              }
            },
            FieldDefaultScalar::String(default_value) => {
              attrs.push(quote! { default_value = #default_value });
            },
          }
        }
        FieldDefaultScalarUnion::DefaultScalars(_) => {
          // unimplemented
        }
      }
    }

    let sea_orm_attr = if attrs.is_empty() {
      quote! {}
    } else {
      quote! { #[sea_orm(#attrs)] }
    };

    quote! {
      #field_doc
      #sea_orm_attr
    }
  }).collect::<Vec<_>>();

    ModelCodegen {
        use_declarations: enum_use_declarations,
        model: quote! {
          #model_doc
          #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
          #[sea_orm(table_name = #table_name)]
          pub struct Model {
            #(
              #attributes
              pub #fields : #types,
            )*
          }
        },
    }
}

fn prisma_table_name(prisma_dmmf_model: &Model) -> String {
    prisma_dmmf_model
        .db_name
        .as_ref()
        .unwrap_or(&prisma_dmmf_model.name)
        .to_string()
}

fn prisma_model_module_name(model_name: impl AsRef<str>) -> String {
    model_name.as_ref().to_snake_case()
}

fn prisma_model_entity_name(model_name: impl AsRef<str>) -> String {
    model_name.as_ref().to_upper_camel_case()
}

fn prisma_field_type(prisma_dmmf_field: &Field) -> TokenStream {
    let mut rust_type = match (&prisma_dmmf_field.r#type, &prisma_dmmf_field.native_type) {
        (FieldType::BigInt, _) => quote! { i64 },
        (FieldType::Boolean, _) => quote! { bool },
        (FieldType::Bytes, _) => quote! { Vec<u8> },
        (FieldType::DateTime, Some((native_db_type, _))) if native_db_type == "Timestamptz" => {
            quote! { DateTimeWithTimeZone }
        }
        (FieldType::DateTime, _) => quote! { DateTime },
        (FieldType::Decimal, _) => quote! { Decimal },
        (FieldType::Float, _) => quote! { f64 },
        (FieldType::Int, _) => quote! { i32 },
        (FieldType::Json, _) => quote! { Json },
        (FieldType::String, Some((native_db_type, _))) if native_db_type == "Uuid" => {
            quote! { Uuid }
        }
        (FieldType::String, _) => quote! { String },
        (FieldType::ModelName(mn), _) => {
            let model_name = if matches!(prisma_dmmf_field.kind, FieldKind::Enum) {
                prisma_enum_iden(mn)
            } else {
                println!(
                    "Warning: Unsure of what identifier '{mn}' on field '{}' is.",
                    prisma_dmmf_field.name
                );
                format_ident!("{}", mn)
            };
            quote! { #model_name }
        }
    };

    if prisma_dmmf_field.is_list {
        rust_type = quote! { Vec<#rust_type> }
    }

    if !prisma_dmmf_field.is_required {
        rust_type = quote! { Option<#rust_type> }
    }

    rust_type
}

fn escape_rust_keyword<T>(string: T) -> String
where
    T: ToString,
{
    let string = string.to_string();
    if RUST_KEYWORDS.iter().any(|s| s.eq(&string)) {
        format!("r#{string}")
    } else if RUST_SPECIAL_KEYWORDS.iter().any(|s| s.eq(&string)) {
        format!("{string}_")
    } else {
        string
    }
}

const RUST_KEYWORDS: [&str; 49] = [
    "as", "async", "await", "break", "const", "continue", "dyn", "else", "enum", "extern", "false",
    "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
    "return", "static", "struct", "super", "trait", "true", "type", "union", "unsafe", "use",
    "where", "while", "abstract", "become", "box", "do", "final", "macro", "override", "priv",
    "try", "typeof", "unsized", "virtual", "yield",
];

const RUST_SPECIAL_KEYWORDS: [&str; 3] = ["crate", "Self", "self"];

struct ModelEntityRelations {
    pub relation_enum: TokenStream,
    pub related_entity_impls: Vec<TokenStream>,
}

struct Relation<'a> {
    name: &'a String,
    is_list: bool,
    r#type: &'a String,
    relation_from_fields: &'a Option<Vec<String>>,
    relation_to_fields: &'a Option<Vec<String>>,
    relation_on_delete: &'a Option<String>,
    relation_on_update: &'a Option<String>,
}

fn prisma_model_relations(prisma_dmmf_model: &Model) -> ModelEntityRelations {
    let object_fields = prisma_dmmf_model
        .fields
        .iter()
        .filter_map(|f| {
            if let FieldType::ModelName(r#type) = &f.r#type
                && matches!(f.kind, FieldKind::Object)
            {
                Some(Relation {
                    name: &f.name,
                    is_list: f.is_list,
                    r#type,
                    relation_from_fields: &f.relation_from_fields,
                    relation_to_fields: &f.relation_to_fields,
                    relation_on_delete: &f.relation_on_delete,
                    relation_on_update: &f.relation_on_update,
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let model_name = &prisma_dmmf_model.name;
    let relation_variants = object_fields.iter().map(|r| {
    let name = r.name;
    safe_enum_variant_name(name, format!("Warning: relation item '{name}' in the relation enumeration for '{model_name}' cannot be converted into a valid Rust enum member name. It will be converted to its corresponding UTF-8 encoding. You can modify it later as needed."))
  }).collect::<Vec<_>>();
    let relation_variant_attrs = object_fields
        .iter()
        .map(|r| {
            let model_module_name = prisma_model_module_name(r.r#type);
            let model_module = format!("super::{model_module_name}");
            let model_module_entity = format!("{model_module}::Entity");

            if r.is_list {
                return quote! {
                  #[sea_orm(has_many = #model_module_entity)]
                };
            }

            if let (Some(from_fields), Some(to_fields)) =
                (r.relation_from_fields, r.relation_to_fields)
                && !from_fields.is_empty()
                && !to_fields.is_empty()
            {
                let from = format_inverse_relation_fields(from_fields, |f| {
                    format!("Column::{}", f.to_upper_camel_case())
                });
                let to = format_inverse_relation_fields(to_fields, |f| {
                    format!("{model_module}::Column::{}", f.to_upper_camel_case())
                });

                let on_update = if let Some(ou) = r.relation_on_update {
                    ou
                } else {
                    if is_relation_self_ref(r, prisma_dmmf_model) {
                        "Cascade"
                    } else {
                        "NoAction"
                    }
                };
                let on_delete = if let Some(od) = r.relation_on_delete {
                    od
                } else {
                    "NoAction"
                };

                return quote! {
                  #[sea_orm(
                    belongs_to = #model_module_entity,
                    from = #from,
                    to = #to,
                    on_update = #on_update,
                    on_delete = #on_delete
                )]
                };
            }

            quote! {
              #[sea_orm(has_one = #model_module_entity)]
            }
        })
        .collect::<Vec<_>>();

    let relation_enum = quote! {
      #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
      pub enum Relation {
        #(
          #relation_variant_attrs
          #relation_variants,
        )*
      }
    };

    let related_entity_impls = relation_variants
        .iter()
        .zip(object_fields.iter())
        .filter_map(|(rv, r)| {
            if is_relation_self_ref(r, prisma_dmmf_model) {
                return None;
            }

            let model_module_name = format_ident!("{}", prisma_model_module_name(r.r#type));
            Some((
                r.r#type,
                quote! {
                  impl Related<super::#model_module_name::Entity> for Entity {
                    fn to() -> RelationDef {
                        Relation::#rv.def()
                    }
                  }
                },
            ))
        })
        .collect::<IndexMap<_, _>>()
        .into_values()
        .collect::<Vec<_>>();

    ModelEntityRelations {
        relation_enum,
        related_entity_impls,
    }
}

fn is_relation_self_ref(relation: &Relation<'_>, prisma_dmmf_model: &Model) -> bool {
    relation.r#type == &prisma_dmmf_model.name && !relation.is_list
}

fn format_inverse_relation_fields(
    fields: &[impl AsRef<str>],
    fmt: impl Fn(&str) -> String,
) -> String {
    match fields {
        [single_field] => fmt(single_field.as_ref()),
        _ => format!(
            "({})",
            fields
                .iter()
                .map(|f| fmt(f.as_ref()))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

fn prelude(prisma_dmmf_models: &[Model]) -> TokenStream {
    let module_names = prisma_dmmf_models
        .iter()
        .map(|m| format_ident!("{}", prisma_model_module_name(&m.name)))
        .collect::<Vec<_>>();
    let entity_names = prisma_dmmf_models
        .iter()
        .map(|m| format_ident!("{}", prisma_model_entity_name(&m.name)))
        .collect::<Vec<_>>();
    quote! {
      pub use super::{
        #(
          #module_names::Entity as #entity_names,
        )*
      };
    }
}

pub fn module(prisma_dmmf_datamodel: &Datamodel, module_name: impl AsRef<str>) -> TokenStream {
    let module_ident = format_ident!("{}", module_name.as_ref());
    let prelude_ts = prelude(&prisma_dmmf_datamodel.models);
    let enums = prisma_dmmf_datamodel
        .enums
        .iter()
        .map(prisma_enum)
        .collect::<Vec<_>>();
    let module_names = prisma_dmmf_datamodel
        .models
        .iter()
        .map(|m| format_ident!("{}", prisma_model_module_name(&m.name)))
        .collect::<Vec<_>>();
    let (use_declarations, models): (Vec<_>, Vec<_>) = prisma_dmmf_datamodel
        .models
        .iter()
        .map(|m| {
            let model_codegen = prisma_model(m, &prisma_dmmf_datamodel.indexes);
            (model_codegen.use_declarations, model_codegen.model)
        })
        .unzip();

    let (relation_enums, related_entity_impls): (Vec<_>, Vec<_>) = prisma_dmmf_datamodel
        .models
        .iter()
        .map(|m| {
            let model_entity_relations = prisma_model_relations(m);
            (
                model_entity_relations.relation_enum,
                model_entity_relations.related_entity_impls,
            )
        })
        .unzip();

    quote! {
      mod #module_ident {
        #![allow(unused)]

        pub mod prelude {
          #![allow(unused_imports)]

          #prelude_ts
        }

        pub mod sea_orm_active_enums {
          #![allow(unused)]

          use sea_orm::entity::prelude::*;

          #(#enums)*
        }

        #(
          pub mod #module_names {
            #![allow(unused)]

            use sea_orm::entity::prelude::*;

            #(
              #use_declarations
            )*

            #models

            #relation_enums

            #(
              #related_entity_impls
            )*

            impl ActiveModelBehavior for ActiveModel {}
          }
        )*
      }
    }
}
