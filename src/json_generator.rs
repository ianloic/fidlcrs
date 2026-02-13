use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Clone, Debug)]
pub struct JsonRoot {
    pub name: String,
    pub platform: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<HashMap<String, Vec<String>>>,
    pub experiments: Vec<String>,
    pub library_dependencies: Vec<LibraryDependency>,
    pub bits_declarations: Vec<BitsDeclaration>,
    pub const_declarations: Vec<ConstDeclaration>,
    pub enum_declarations: Vec<EnumDeclaration>,
    pub experimental_resource_declarations: Vec<ExperimentalResourceDeclaration>,
    pub protocol_declarations: Vec<ProtocolDeclaration>,
    pub service_declarations: Vec<ServiceDeclaration>,
    pub struct_declarations: Vec<StructDeclaration>,
    pub external_struct_declarations: Vec<StructDeclaration>,
    pub table_declarations: Vec<TableDeclaration>,
    pub union_declarations: Vec<UnionDeclaration>,
    pub alias_declarations: Vec<AliasDeclaration>,
    pub new_type_declarations: Vec<NewTypeDeclaration>,
    pub declaration_order: Vec<String>,
    pub declarations: HashMap<String, String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct LibraryDependency {
    pub name: String,
    pub declarations: HashMap<String, String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct Location {
    pub filename: String,
    pub line: usize,
    pub column: usize,
    pub length: usize,
}

#[derive(Serialize, Clone, Debug)]
pub struct TypeShapeV2 {
    pub inline_size: u32,
    pub alignment: u32,
    pub depth: u32,
    pub max_handles: u32,
    pub max_out_of_line: u32,
    pub has_padding: bool,
    pub has_flexible_envelope: bool,
}

#[derive(Serialize, Clone, Debug)]
pub struct FieldShapeV2 {
    pub offset: u32,
    pub padding: u32,
}

#[derive(Serialize, Clone, Debug)]
pub struct Type {
    pub kind_v2: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_type: Option<Box<Type>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub maybe_attributes: Vec<Attribute>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_shape_v2: Option<FieldShapeV2>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maybe_element_count: Option<u32>,
    pub type_shape_v2: TypeShapeV2,
}

#[derive(Serialize, Clone, Debug)]
pub struct StructMember {
    #[serde(rename = "type")]
    pub type_: Type,
    pub name: String,
    pub location: Location,
    pub deprecated: bool,
    pub field_shape_v2: FieldShapeV2,
}

#[derive(Serialize, Clone, Debug)]
pub struct StructDeclaration {
    pub name: String,
    pub naming_context: Vec<String>,
    pub location: Location,
    pub deprecated: bool,
    pub members: Vec<StructMember>,
    pub resource: bool,
    pub is_empty_success_struct: bool,
    pub type_shape_v2: TypeShapeV2,
}

// Placeholders for other declarations
#[derive(Serialize, Clone, Debug)]
pub struct BitField {
    // ...
}

#[derive(Serialize, Clone, Debug)]
pub struct BitsDeclaration {
    pub name: String,
    pub naming_context: Vec<String>,
    pub location: Location,
    pub deprecated: bool,
    #[serde(rename = "type")]
    pub type_: Type,
    pub mask: String,
    pub members: Vec<BitsMember>,
    pub strict: bool,
}

#[derive(Serialize, Clone, Debug)]
pub struct BitsMember {
    pub name: String,
    pub location: Location,
    pub deprecated: bool,
    pub value: Constant,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub maybe_attributes: Vec<Attribute>,
}

#[derive(Serialize, Clone, Debug)]
pub struct ConstDeclaration {}
#[derive(Serialize, Clone, Debug)]
pub struct EnumDeclaration {
    pub name: String,
    pub naming_context: Vec<String>,
    pub location: Location,
    pub deprecated: bool,
    #[serde(rename = "type")]
    pub type_: String,
    pub members: Vec<EnumMember>,
    pub strict: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maybe_unknown_value: Option<u32>,
}

#[derive(Serialize, Clone, Debug)]
pub struct EnumMember {
    pub name: String,
    pub location: Location,
    pub deprecated: bool,
    pub value: Constant,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub maybe_attributes: Vec<Attribute>,
}

#[derive(Serialize, Clone, Debug)]
pub struct Constant {
    pub kind: String,
    pub value: String,
    pub expression: String,
    pub literal: Literal,
}

#[derive(Serialize, Clone, Debug)]
pub struct Literal {
    pub kind: String,
    pub value: String,
    pub expression: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct Attribute {
    pub name: String,
    pub arguments: Vec<String>, // simplified
    pub location: Location,
}
#[derive(Serialize, Clone, Debug)]
pub struct ExperimentalResourceDeclaration {}
#[derive(Serialize, Clone, Debug)]
pub struct ProtocolDeclaration {}
#[derive(Serialize, Clone, Debug)]
pub struct ServiceDeclaration {}
#[derive(Serialize, Clone, Debug)]
pub struct TableDeclaration {
    pub name: String,
    pub naming_context: Vec<String>,
    pub location: Location,
    pub deprecated: bool,
    pub members: Vec<TableMember>,
    pub strict: bool,
    pub resource: bool,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub maybe_attributes: Vec<Attribute>,
    pub type_shape_v2: TypeShapeV2,
}

#[derive(Serialize, Clone, Debug)]
pub struct TableMember {
    pub ordinal: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved: Option<bool>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<Type>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub maybe_attributes: Vec<Attribute>,
}

#[derive(Serialize, Clone, Debug)]
pub struct UnionDeclaration {
    pub name: String,
    pub naming_context: Vec<String>,
    pub location: Location,
    pub deprecated: bool,
    pub members: Vec<UnionMember>,
    pub strict: bool,
    pub resource: bool,
    pub is_result: bool,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub maybe_attributes: Vec<Attribute>,
    pub type_shape_v2: TypeShapeV2,
}

#[derive(Serialize, Clone, Debug)]
pub struct UnionMember {
    pub ordinal: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<Type>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub maybe_attributes: Vec<Attribute>,
}
#[derive(Serialize, Clone, Debug)]
pub struct AliasDeclaration {}
#[derive(Serialize, Clone, Debug)]
pub struct NewTypeDeclaration {}
