use prisma_to_sea_orm_codegen::{
    prisma_dmmf::{Datamodel, Field, FieldKind, FieldType, Model},
    prisma_to_sea_orm_codegen,
};

fn create_test_schema_with_unique_email() -> Datamodel {
    Datamodel {
        models: vec![Model {
            name: "User".to_string(),
            db_name: Some("users".to_string()),
            schema: None,
            documentation: None,
            fields: vec![
                Field {
                    name: "id".to_string(),
                    r#type: FieldType::String,
                    is_list: false,
                    is_required: true,
                    is_unique: false,
                    is_id: true,
                    is_read_only: false,
                    has_default_value: false,
                    default: None,
                    native_type: Some(("Uuid".to_string(), vec![])),
                    relation_name: None,
                    relation_from_fields: None,
                    relation_to_fields: None,
                    relation_on_delete: None,
                    relation_on_update: None,
                    is_generated: Some(false),
                    is_updated_at: Some(false),
                    kind: FieldKind::Scalar,
                    documentation: None,
                    db_name: None,
                },
                Field {
                    name: "email".to_string(),
                    r#type: FieldType::String,
                    is_list: false,
                    is_required: true,
                    is_unique: true,
                    is_id: false,
                    is_read_only: false,
                    has_default_value: false,
                    default: None,
                    native_type: None,
                    relation_name: None,
                    relation_from_fields: None,
                    relation_to_fields: None,
                    relation_on_delete: None,
                    relation_on_update: None,
                    is_generated: Some(false),
                    is_updated_at: Some(false),
                    kind: FieldKind::Scalar,
                    documentation: None,
                    db_name: None,
                },
                Field {
                    name: "name".to_string(),
                    r#type: FieldType::String,
                    is_list: false,
                    is_required: true,
                    is_unique: false,
                    is_id: false,
                    is_read_only: false,
                    has_default_value: false,
                    default: None,
                    native_type: None,
                    relation_name: None,
                    relation_from_fields: None,
                    relation_to_fields: None,
                    relation_on_delete: None,
                    relation_on_update: None,
                    is_generated: Some(false),
                    is_updated_at: Some(false),
                    kind: FieldKind::Scalar,
                    documentation: None,
                    db_name: None,
                },
            ],
            unique_fields: vec![vec!["email".to_string()]],
            unique_indexes: vec![],
            primary_key: None,
            is_generated: Some(false),
        }],
        enums: vec![],
        types: vec![],
        indexes: vec![],
    }
}

fn create_test_schema_with_compound_unique() -> Datamodel {
    Datamodel {
        models: vec![Model {
            name: "Project".to_string(),
            db_name: Some("projects".to_string()),
            schema: None,
            documentation: None,
            fields: vec![
                Field {
                    name: "id".to_string(),
                    r#type: FieldType::String,
                    is_list: false,
                    is_required: true,
                    is_unique: false,
                    is_id: true,
                    is_read_only: false,
                    has_default_value: false,
                    default: None,
                    native_type: Some(("Uuid".to_string(), vec![])),
                    relation_name: None,
                    relation_from_fields: None,
                    relation_to_fields: None,
                    relation_on_delete: None,
                    relation_on_update: None,
                    is_generated: Some(false),
                    is_updated_at: Some(false),
                    kind: FieldKind::Scalar,
                    documentation: None,
                    db_name: None,
                },
                Field {
                    name: "org_id".to_string(),
                    r#type: FieldType::String,
                    is_list: false,
                    is_required: true,
                    is_unique: false,
                    is_id: false,
                    is_read_only: false,
                    has_default_value: false,
                    default: None,
                    native_type: Some(("Uuid".to_string(), vec![])),
                    relation_name: None,
                    relation_from_fields: None,
                    relation_to_fields: None,
                    relation_on_delete: None,
                    relation_on_update: None,
                    is_generated: Some(false),
                    is_updated_at: Some(false),
                    kind: FieldKind::Scalar,
                    documentation: None,
                    db_name: None,
                },
                Field {
                    name: "external_id".to_string(),
                    r#type: FieldType::String,
                    is_list: false,
                    is_required: true,
                    is_unique: false,
                    is_id: false,
                    is_read_only: false,
                    has_default_value: false,
                    default: None,
                    native_type: None,
                    relation_name: None,
                    relation_from_fields: None,
                    relation_to_fields: None,
                    relation_on_delete: None,
                    relation_on_update: None,
                    is_generated: Some(false),
                    is_updated_at: Some(false),
                    kind: FieldKind::Scalar,
                    documentation: None,
                    db_name: None,
                },
                Field {
                    name: "name".to_string(),
                    r#type: FieldType::String,
                    is_list: false,
                    is_required: true,
                    is_unique: false,
                    is_id: false,
                    is_read_only: false,
                    has_default_value: false,
                    default: None,
                    native_type: None,
                    relation_name: None,
                    relation_from_fields: None,
                    relation_to_fields: None,
                    relation_on_delete: None,
                    relation_on_update: None,
                    is_generated: Some(false),
                    is_updated_at: Some(false),
                    kind: FieldKind::Scalar,
                    documentation: None,
                    db_name: None,
                },
            ],
            unique_fields: vec![vec!["org_id".to_string(), "external_id".to_string()]],
            unique_indexes: vec![],
            primary_key: None,
            is_generated: Some(false),
        }],
        enums: vec![],
        types: vec![],
        indexes: vec![],
    }
}

fn create_test_schema_with_multiple_unique() -> Datamodel {
    Datamodel {
        models: vec![Model {
            name: "Account".to_string(),
            db_name: Some("accounts".to_string()),
            schema: None,
            documentation: None,
            fields: vec![
                Field {
                    name: "id".to_string(),
                    r#type: FieldType::String,
                    is_list: false,
                    is_required: true,
                    is_unique: false,
                    is_id: true,
                    is_read_only: false,
                    has_default_value: false,
                    default: None,
                    native_type: Some(("Uuid".to_string(), vec![])),
                    relation_name: None,
                    relation_from_fields: None,
                    relation_to_fields: None,
                    relation_on_delete: None,
                    relation_on_update: None,
                    is_generated: Some(false),
                    is_updated_at: Some(false),
                    kind: FieldKind::Scalar,
                    documentation: None,
                    db_name: None,
                },
                Field {
                    name: "email".to_string(),
                    r#type: FieldType::String,
                    is_list: false,
                    is_required: true,
                    is_unique: true,
                    is_id: false,
                    is_read_only: false,
                    has_default_value: false,
                    default: None,
                    native_type: None,
                    relation_name: None,
                    relation_from_fields: None,
                    relation_to_fields: None,
                    relation_on_delete: None,
                    relation_on_update: None,
                    is_generated: Some(false),
                    is_updated_at: Some(false),
                    kind: FieldKind::Scalar,
                    documentation: None,
                    db_name: None,
                },
                Field {
                    name: "provider".to_string(),
                    r#type: FieldType::String,
                    is_list: false,
                    is_required: true,
                    is_unique: false,
                    is_id: false,
                    is_read_only: false,
                    has_default_value: false,
                    default: None,
                    native_type: None,
                    relation_name: None,
                    relation_from_fields: None,
                    relation_to_fields: None,
                    relation_on_delete: None,
                    relation_on_update: None,
                    is_generated: Some(false),
                    is_updated_at: Some(false),
                    kind: FieldKind::Scalar,
                    documentation: None,
                    db_name: None,
                },
                Field {
                    name: "provider_account_id".to_string(),
                    r#type: FieldType::String,
                    is_list: false,
                    is_required: true,
                    is_unique: false,
                    is_id: false,
                    is_read_only: false,
                    has_default_value: false,
                    default: None,
                    native_type: None,
                    relation_name: None,
                    relation_from_fields: None,
                    relation_to_fields: None,
                    relation_on_delete: None,
                    relation_on_update: None,
                    is_generated: Some(false),
                    is_updated_at: Some(false),
                    kind: FieldKind::Scalar,
                    documentation: None,
                    db_name: None,
                },
            ],
            unique_fields: vec![
                vec!["email".to_string()],
                vec!["provider".to_string(), "provider_account_id".to_string()],
            ],
            unique_indexes: vec![],
            primary_key: None,
            is_generated: Some(false),
        }],
        enums: vec![],
        types: vec![],
        indexes: vec![],
    }
}

fn create_test_schema_with_nullable_unique() -> Datamodel {
    Datamodel {
        models: vec![Model {
            name: "Profile".to_string(),
            db_name: Some("profiles".to_string()),
            schema: None,
            documentation: None,
            fields: vec![
                Field {
                    name: "id".to_string(),
                    r#type: FieldType::String,
                    is_list: false,
                    is_required: true,
                    is_unique: false,
                    is_id: true,
                    is_read_only: false,
                    has_default_value: false,
                    default: None,
                    native_type: Some(("Uuid".to_string(), vec![])),
                    relation_name: None,
                    relation_from_fields: None,
                    relation_to_fields: None,
                    relation_on_delete: None,
                    relation_on_update: None,
                    is_generated: Some(false),
                    is_updated_at: Some(false),
                    kind: FieldKind::Scalar,
                    documentation: None,
                    db_name: None,
                },
                Field {
                    name: "username".to_string(),
                    r#type: FieldType::String,
                    is_list: false,
                    is_required: false,
                    is_unique: true,
                    is_id: false,
                    is_read_only: false,
                    has_default_value: false,
                    default: None,
                    native_type: None,
                    relation_name: None,
                    relation_from_fields: None,
                    relation_to_fields: None,
                    relation_on_delete: None,
                    relation_on_update: None,
                    is_generated: Some(false),
                    is_updated_at: Some(false),
                    kind: FieldKind::Scalar,
                    documentation: None,
                    db_name: None,
                },
                Field {
                    name: "display_name".to_string(),
                    r#type: FieldType::String,
                    is_list: false,
                    is_required: true,
                    is_unique: false,
                    is_id: false,
                    is_read_only: false,
                    has_default_value: false,
                    default: None,
                    native_type: None,
                    relation_name: None,
                    relation_from_fields: None,
                    relation_to_fields: None,
                    relation_on_delete: None,
                    relation_on_update: None,
                    is_generated: Some(false),
                    is_updated_at: Some(false),
                    kind: FieldKind::Scalar,
                    documentation: None,
                    db_name: None,
                },
            ],
            unique_fields: vec![vec!["username".to_string()]],
            unique_indexes: vec![],
            primary_key: None,
            is_generated: Some(false),
        }],
        enums: vec![],
        types: vec![],
        indexes: vec![],
    }
}

#[test]
fn test_single_field_unique_constraint_generation() {
    let schema = create_test_schema_with_unique_email();
    let generated = prisma_to_sea_orm_codegen(schema, "test_db").unwrap();

    println!("Generated code:\n{}", generated);

    assert!(
        generated.contains("pub enum UniqueConstraint"),
        "Should generate UniqueConstraint enum"
    );
    assert!(
        generated.contains("pub trait EntityExt"),
        "Should generate EntityExt trait"
    );
    assert!(
        generated.contains("fn find_unique"),
        "Should generate find_unique function"
    );

    assert!(
        generated.contains("EmailConstraint"),
        "Should generate EmailConstraint variant"
    );
    assert!(
        generated.contains("EmailConstraint { email: String }"),
        "Should generate enum variant with embedded fields"
    );

    assert!(
        generated.contains("Column::Email.eq(email.as_str())"),
        "Should use simple eq filter for single field"
    );
    assert!(
        !generated.contains("Condition::all()"),
        "Should not use Condition::all for single field"
    );
}

#[test]
fn test_multi_field_unique_constraint_generation() {
    let schema = create_test_schema_with_compound_unique();
    let generated = prisma_to_sea_orm_codegen(schema, "test_db").unwrap();

    println!("Generated code:\n{}", generated);

    assert!(
        generated.contains("pub enum UniqueConstraint"),
        "Should generate UniqueConstraint enum"
    );
    assert!(
        generated.contains("pub trait EntityExt"),
        "Should generate EntityExt trait"
    );
    assert!(
        generated.contains("fn find_unique"),
        "Should generate find_unique function"
    );

    assert!(
        generated.contains("OrgIdExternalIdConstraint"),
        "Should generate compound constraint variant"
    );
    assert!(
        generated.contains("OrgIdExternalIdConstraint { org_id: Uuid, external_id: String }"),
        "Should generate enum variant with embedded fields"
    );

    assert!(
        generated.contains("Condition::all()"),
        "Should use Condition::all for multi-field constraints"
    );
    assert!(
        generated.contains("Column::OrgId.eq(org_id)"),
        "Should filter on org_id"
    );
    assert!(
        generated.contains("Column::ExternalId.eq(external_id.as_str())"),
        "Should filter on external_id"
    );
}

#[test]
fn test_multiple_unique_constraints_generation() {
    let schema = create_test_schema_with_multiple_unique();
    let generated = prisma_to_sea_orm_codegen(schema, "test_db").unwrap();

    println!("Generated code:\n{}", generated);

    assert!(
        generated.contains("EmailConstraint"),
        "Should generate single field constraint"
    );
    assert!(
        generated.contains("ProviderProviderAccountIdConstraint"),
        "Should generate compound constraint"
    );

    assert!(
        generated.contains("EmailConstraint { email: String }"),
        "Should generate single field constraint with embedded fields"
    );
    assert!(
        generated.contains("ProviderProviderAccountIdConstraint {"),
        "Should generate compound constraint with embedded fields"
    );

    assert!(
        generated.contains("Column::Email.eq(email.as_str())"),
        "Should use simple eq for single field"
    );
    assert!(
        generated.contains("Condition::all()"),
        "Should use Condition::all for multi-field"
    );
    assert!(
        generated.contains("Column::Provider.eq(provider.as_str())"),
        "Should filter on provider"
    );
    assert!(
        generated.contains("Column::ProviderAccountId.eq(provider_account_id.as_str())"),
        "Should filter on provider_account_id"
    );
}

#[test]
fn test_nullable_field_unique_constraint_generation() {
    let schema = create_test_schema_with_nullable_unique();
    let generated = prisma_to_sea_orm_codegen(schema, "test_db").unwrap();

    println!("Generated code:\n{}", generated);

    assert!(
        generated.contains("UsernameConstraint"),
        "Should generate constraint for nullable field"
    );
    assert!(
        generated.contains("UsernameConstraint { username: Option<String> }"),
        "Should generate constraint variant with embedded fields"
    );

    assert!(
        generated.contains("pub username: Option<String>"),
        "Nullable field should be Option<T>"
    );

    assert!(
        generated.contains("Column::Username.eq(username.as_deref())"),
        "Should generate filter for nullable field"
    );
}

#[test]
fn test_no_unique_constraints_no_generation() {
    // Create a model with no unique constraints
    let schema = Datamodel {
        models: vec![Model {
            name: "SimpleModel".to_string(),
            db_name: Some("simple_models".to_string()),
            schema: None,
            documentation: None,
            fields: vec![
                Field {
                    name: "id".to_string(),
                    r#type: FieldType::String,
                    is_list: false,
                    is_required: true,
                    is_unique: false,
                    is_id: true,
                    is_read_only: false,
                    has_default_value: false,
                    default: None,
                    native_type: Some(("Uuid".to_string(), vec![])),
                    relation_name: None,
                    relation_from_fields: None,
                    relation_to_fields: None,
                    relation_on_delete: None,
                    relation_on_update: None,
                    is_generated: Some(false),
                    is_updated_at: Some(false),
                    kind: FieldKind::Scalar,
                    documentation: None,
                    db_name: None,
                },
                Field {
                    name: "name".to_string(),
                    r#type: FieldType::String,
                    is_list: false,
                    is_required: true,
                    is_unique: false,
                    is_id: false,
                    is_read_only: false,
                    has_default_value: false,
                    default: None,
                    native_type: None,
                    relation_name: None,
                    relation_from_fields: None,
                    relation_to_fields: None,
                    relation_on_delete: None,
                    relation_on_update: None,
                    is_generated: Some(false),
                    is_updated_at: Some(false),
                    kind: FieldKind::Scalar,
                    documentation: None,
                    db_name: None,
                },
            ],
            unique_fields: vec![],
            unique_indexes: vec![],
            primary_key: None,
            is_generated: Some(false),
        }],
        enums: vec![],
        types: vec![],
        indexes: vec![],
    };

    let generated = prisma_to_sea_orm_codegen(schema, "test_db").unwrap();

    println!("Generated code:\n{}", generated);

    assert!(
        !generated.contains("pub enum UniqueConstraint"),
        "Should not generate UniqueConstraint enum when no unique constraints"
    );
    assert!(
        !generated.contains("pub trait EntityExt"),
        "Should not generate EntityExt trait when no unique constraints"
    );
    assert!(
        !generated.contains("fn find_unique"),
        "Should not generate find_unique function when no unique constraints"
    );
}

#[test]
fn test_generated_code_compiles() {
    let schema = create_test_schema_with_multiple_unique();
    let generated = prisma_to_sea_orm_codegen(schema, "test_db").unwrap();

    // Test that the generated code can be parsed as valid Rust
    let parsed = syn::parse_file(&generated);
    assert!(
        parsed.is_ok(),
        "Generated code should be valid Rust syntax: {:#?}",
        parsed.err()
    );
}
