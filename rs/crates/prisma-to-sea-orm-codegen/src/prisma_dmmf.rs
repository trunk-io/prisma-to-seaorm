use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Datamodel {
    pub models: Vec<Model>,
    pub enums: Vec<DatamodelEnum>,
    pub types: Vec<Model>,
    pub indexes: Vec<Index>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub name: String,
    pub db_name: Option<String>,
    pub schema: Option<String>,
    pub fields: Vec<Field>,
    pub unique_fields: Vec<Vec<String>>,
    pub unique_indexes: Vec<UniqueIndex>,
    pub documentation: Option<String>,
    pub primary_key: Option<PrimaryKey>,
    pub is_generated: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatamodelEnum {
    pub name: String,
    pub values: Vec<EnumValue>,
    pub db_name: Option<String>,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Index {
    pub model: String,
    pub r#type: IndexType,
    pub is_defined_on_field: bool,
    pub name: Option<String>,
    pub db_name: Option<String>,
    pub algorithm: Option<String>,
    pub clustered: Option<bool>,
    pub fields: Vec<IndexField>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IndexType {
    Id,
    Normal,
    Unique,
    FullText,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexField {
    pub name: String,
    pub sort_order: Option<SortOrder>,
    pub length: Option<usize>,
    pub operator_class: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub kind: FieldKind,
    pub name: String,
    pub is_required: bool,
    pub is_list: bool,
    pub is_unique: bool,
    pub is_id: bool,
    pub is_read_only: bool,
    pub is_generated: Option<bool>,
    pub is_updated_at: Option<bool>,
    pub r#type: FieldType,
    /// Native database type, if specified.
    /// For example, `@db.VarChar(191)` is encoded as `['VarChar', ['191']]`,
    /// `@db.Text` is encoded as `['Text', []]`.
    pub native_type: Option<(String, Vec<String>)>,
    pub db_name: Option<String>,
    pub has_default_value: bool,
    pub default: Option<FieldDefaultScalarUnion>,
    pub relation_from_fields: Option<Vec<String>>,
    pub relation_to_fields: Option<Vec<String>>,
    pub relation_on_delete: Option<String>,
    pub relation_on_update: Option<String>,
    pub relation_name: Option<String>,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum FieldType {
    BigInt,
    Boolean,
    Bytes,
    DateTime,
    Decimal,
    Float,
    Int,
    Json,
    String,
    #[serde(untagged)]
    ModelName(String),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FieldKind {
    Scalar,
    Object,
    Enum,
    Unsupported,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum FieldDefaultScalarUnion {
    Default(FieldDefault),
    DefaultScalar(FieldDefaultScalar),
    DefaultScalars(Vec<FieldDefaultScalar>),
}

#[derive(Debug, Clone, Deserialize)]
pub struct FieldDefault {
    pub name: String,
    pub args: Vec<FieldDefaultArg>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum FieldDefaultArg {
    String(String),
    Number(serde_json::Number),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum FieldDefaultScalar {
    String(String),
    Bool(bool),
    Number(serde_json::Number),
}

#[derive(Debug, Clone, Deserialize)]
pub struct UniqueIndex {
    // NB: Prisma's TS type says this field is required, but it's not actually 🤷
    pub name: Option<String>,
    pub fields: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PrimaryKey {
    pub name: Option<String>,
    pub fields: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnumValue {
    pub name: String,
    pub db_name: Option<String>,
}
