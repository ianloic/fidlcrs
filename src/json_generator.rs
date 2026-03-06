use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Serialize, Debug)]
pub struct JsonRoot {
    pub name: String,
    pub platform: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<BTreeMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub maybe_attributes: Vec<Attribute>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlay_declarations: Option<Vec<UnionDeclaration>>,
    pub alias_declarations: Vec<AliasDeclaration>,
    pub new_type_declarations: Vec<NewTypeDeclaration>,
    pub declaration_order: Vec<String>,
    pub declarations: indexmap::IndexMap<String, String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct LibraryDependency {
    pub name: String,
    pub declarations: indexmap::IndexMap<String, serde_json::Value>,
}

#[derive(Serialize, Clone, Debug)]
pub struct Location {
    pub filename: String,
    pub line: usize,
    pub column: usize,
    pub length: usize,
}

#[derive(Serialize, Clone, Debug)]
pub struct TypeShape {
    pub inline_size: u32,
    pub alignment: u32,
    pub depth: u32,
    pub max_handles: u32,
    pub max_out_of_line: u32,
    pub has_padding: bool,
    pub has_flexible_envelope: bool,
}

#[derive(Serialize, Clone, Debug)]
pub struct FieldShape {
    pub offset: u32,
    pub padding: u32,
}

#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TypeKind {
    Primitive,
    String,
    StringArray,
    Unknown,
    Vector,
    Array,
    Endpoint,
    Handle,
    Identifier,
    Struct,
    Request,
    ExperimentalPointer,
}

#[derive(Clone, Debug, Serialize)]
pub struct TypeCommon {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental_maybe_from_alias: Option<ExperimentalMaybeFromAlias>,
    #[serde(skip)]
    pub outer_alias: Option<ExperimentalMaybeFromAlias>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub maybe_attributes: Vec<Attribute>,
    #[serde(rename = "field_shape_v2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_shape: Option<FieldShape>,
    #[serde(rename = "type_shape_v2")]
    pub type_shape: TypeShape,
    #[serde(skip)]
    pub maybe_size_constant_name: Option<String>,
    #[serde(skip)]
    pub resource: bool,
}

#[derive(Clone, Debug)]
pub enum Type {
    Primitive(PrimitiveType),
    String(StringType),
    StringArray(StringArrayType),
    Unknown(UnknownType),
    Vector(VectorType),
    Array(ArrayType),
    Endpoint(EndpointType),
    Handle(HandleType),
    Identifier(IdentifierType),
    Struct(StructType),
    Request(RequestType),
    ExperimentalPointer(ExperimentalPointerType),
}

#[derive(Clone, Debug, Serialize)]
pub struct PrimitiveType {
    #[serde(flatten)]
    pub common: TypeCommon,
    pub subtype: PrimitiveSubtype,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PrimitiveSubtype {
    Bool,
    Int8,
    Int16,
    Int32,
    Int64,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Float32,
    Float64,
    Uchar,
    Usize64,
    Uintptr64,
}

impl std::str::FromStr for PrimitiveSubtype {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bool" => Ok(Self::Bool),
            "int8" => Ok(Self::Int8),
            "int16" => Ok(Self::Int16),
            "int32" => Ok(Self::Int32),
            "int64" => Ok(Self::Int64),
            "uint8" => Ok(Self::Uint8),
            "uint16" => Ok(Self::Uint16),
            "uint32" => Ok(Self::Uint32),
            "uint64" => Ok(Self::Uint64),
            "float32" => Ok(Self::Float32),
            "float64" => Ok(Self::Float64),
            "uchar" => Ok(Self::Uchar),
            "usize64" => Ok(Self::Usize64),
            "uintptr64" => Ok(Self::Uintptr64),
            _ => Err(format!("Invalid primitive subtype: {}", s)),
        }
    }
}

impl std::fmt::Display for PrimitiveSubtype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Bool => "bool",
            Self::Int8 => "int8",
            Self::Int16 => "int16",
            Self::Int32 => "int32",
            Self::Int64 => "int64",
            Self::Uint8 => "uint8",
            Self::Uint16 => "uint16",
            Self::Uint32 => "uint32",
            Self::Uint64 => "uint64",
            Self::Float32 => "float32",
            Self::Float64 => "float64",
            Self::Uchar => "uchar",
            Self::Usize64 => "usize64",
            Self::Uintptr64 => "uintptr64",
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct StringType {
    #[serde(flatten)]
    pub common: TypeCommon,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maybe_element_count: Option<u32>,
}

#[derive(Clone, Debug, Serialize)]
pub struct StringArrayType {
    #[serde(flatten)]
    pub common: TypeCommon,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_count: Option<u32>,
}

#[derive(Clone, Debug, Serialize)]
pub struct UnknownType {
    #[serde(flatten)]
    pub common: TypeCommon,
}

#[derive(Clone, Debug, Serialize)]
pub struct VectorType {
    #[serde(flatten)]
    pub common: TypeCommon,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_type: Option<Box<Type>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maybe_element_count: Option<u32>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ArrayType {
    #[serde(flatten)]
    pub common: TypeCommon,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_type: Option<Box<Type>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_count: Option<u32>,
}

#[derive(Clone, Debug, Serialize)]
pub struct EndpointType {
    #[serde(flatten)]
    pub common: TypeCommon,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_transport: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct HandleType {
    #[serde(flatten)]
    pub common: TypeCommon,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obj_type: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct IdentifierType {
    #[serde(flatten)]
    pub common: TypeCommon,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,
}

#[derive(Clone, Debug, Serialize)]
pub struct StructType {
    #[serde(flatten)]
    pub common: TypeCommon,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,
}

#[derive(Clone, Debug, Serialize)]
pub struct RequestType {
    #[serde(flatten)]
    pub common: TypeCommon,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ExperimentalPointerType {
    #[serde(flatten)]
    pub common: TypeCommon,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_type: Option<Box<Type>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,
}

impl std::ops::Deref for Type {
    type Target = TypeCommon;
    fn deref(&self) -> &Self::Target {
        match self {
            Type::Primitive(t) => &t.common,
            Type::String(t) => &t.common,
            Type::StringArray(t) => &t.common,
            Type::Unknown(t) => &t.common,
            Type::Vector(t) => &t.common,
            Type::Array(t) => &t.common,
            Type::Endpoint(t) => &t.common,
            Type::Handle(t) => &t.common,
            Type::Identifier(t) => &t.common,
            Type::Struct(t) => &t.common,
            Type::Request(t) => &t.common,
            Type::ExperimentalPointer(t) => &t.common,
        }
    }
}

impl std::ops::DerefMut for Type {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Type::Primitive(t) => &mut t.common,
            Type::String(t) => &mut t.common,
            Type::StringArray(t) => &mut t.common,
            Type::Unknown(t) => &mut t.common,
            Type::Vector(t) => &mut t.common,
            Type::Array(t) => &mut t.common,
            Type::Endpoint(t) => &mut t.common,
            Type::Handle(t) => &mut t.common,
            Type::Identifier(t) => &mut t.common,
            Type::Struct(t) => &mut t.common,
            Type::Request(t) => &mut t.common,
            Type::ExperimentalPointer(t) => &mut t.common,
        }
    }
}

impl Type {
    pub fn kind(&self) -> TypeKind {
        match self {
            Type::Primitive(_) => TypeKind::Primitive,
            Type::String(_) => TypeKind::String,
            Type::StringArray(_) => TypeKind::StringArray,
            Type::Unknown(_) => TypeKind::Unknown,
            Type::Vector(_) => TypeKind::Vector,
            Type::Array(_) => TypeKind::Array,
            Type::Endpoint(_) => TypeKind::Endpoint,
            Type::Handle(_) => TypeKind::Handle,
            Type::Identifier(_) => TypeKind::Identifier,
            Type::Struct(_) => TypeKind::Struct,
            Type::Request(_) => TypeKind::Request,
            Type::ExperimentalPointer(_) => TypeKind::ExperimentalPointer,
        }
    }

    pub fn identifier(&self) -> Option<String> {
        match self {
            Type::Identifier(t) => t.identifier.clone(),
            Type::Struct(t) => t.identifier.clone(),
            Type::Request(t) => t.identifier.clone(),
            _ => None,
        }
    }

    pub fn element_type(&self) -> Option<&Type> {
        match self {
            Type::Array(t) => t.element_type.as_deref(),
            Type::Vector(t) => t.element_type.as_deref(),
            Type::ExperimentalPointer(t) => t.element_type.as_deref(),
            _ => None,
        }
    }

    pub fn nullable(&self) -> Option<bool> {
        match self {
            Type::String(t) => t.nullable,
            Type::Vector(t) => t.nullable,
            Type::Endpoint(t) => t.nullable,
            Type::Handle(t) => t.nullable,
            Type::Identifier(t) => t.nullable,
            Type::Struct(t) => t.nullable,
            Type::Request(t) => t.nullable,
            Type::ExperimentalPointer(_) => None,
            _ => None,
        }
    }
    pub fn set_nullable(&mut self, val: bool) {
        match self {
            Type::String(t) => t.nullable = Some(val),
            Type::Vector(t) => t.nullable = Some(val),
            Type::Endpoint(t) => t.nullable = Some(val),
            Type::Handle(t) => t.nullable = Some(val),
            Type::Identifier(t) => t.nullable = Some(val),
            Type::Struct(t) => t.nullable = Some(val),
            Type::Request(t) => t.nullable = Some(val),
            Type::ExperimentalPointer(t) => t.nullable = Some(val),
            _ => {}
        }
    }
    pub fn protocol(&self) -> Option<String> {
        match self {
            Type::Endpoint(t) => t.protocol.clone(),
            _ => None,
        }
    }
    pub fn resource_identifier(&self) -> Option<String> {
        match self {
            Type::Handle(t) => t.resource_identifier.clone(),
            _ => None,
        }
    }
    pub fn element_type_mut(&mut self) -> Option<&mut Type> {
        match self {
            Type::Array(t) => t.element_type.as_deref_mut(),
            Type::Vector(t) => t.element_type.as_deref_mut(),
            Type::ExperimentalPointer(t) => t.element_type.as_deref_mut(),
            _ => None,
        }
    }
    pub fn rights(&self) -> Option<u32> {
        match self {
            Type::Handle(t) => t.rights.clone(),
            _ => None,
        }
    }
    pub fn maybe_element_count(&self) -> Option<u32> {
        match self {
            Type::String(t) => t.maybe_element_count,
            Type::Vector(t) => t.maybe_element_count,
            _ => None,
        }
    }
    pub fn element_count(&self) -> Option<u32> {
        match self {
            Type::Array(t) => t.element_count,
            Type::StringArray(t) => t.element_count,
            _ => None,
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct ExperimentalMaybeFromAlias {
    pub name: String,
    pub args: Vec<String>,
    pub nullable: bool,
}

#[derive(Serialize, Clone, Debug)]
pub struct StructMember {
    #[serde(rename = "type")]
    pub type_: Type,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental_maybe_from_alias: Option<ExperimentalMaybeFromAlias>,
    pub name: String,
    pub location: Location,
    pub deprecated: bool,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub maybe_attributes: Vec<Attribute>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maybe_default_value: Option<Constant>,
    #[serde(rename = "field_shape_v2")]
    pub field_shape: FieldShape,
}

#[derive(Serialize, Clone, Debug)]
pub struct StructDeclaration {
    pub name: String,
    pub naming_context: Vec<String>,
    pub location: Location,
    pub deprecated: bool,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub maybe_attributes: Vec<Attribute>,
    pub members: Vec<StructMember>,
    pub resource: bool,
    pub is_empty_success_struct: bool,
    #[serde(rename = "type_shape_v2")]
    pub type_shape: TypeShape,
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
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub maybe_attributes: Vec<Attribute>,
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
pub struct ConstDeclaration {
    pub name: String,
    pub location: Location,
    pub deprecated: bool,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub maybe_attributes: Vec<Attribute>,
    #[serde(rename = "type")]
    pub type_: Type,
    pub value: Constant,
}
#[derive(Serialize, Clone, Debug)]
pub struct EnumDeclaration {
    pub name: String,
    pub naming_context: Vec<String>,
    pub location: Location,
    pub deprecated: bool,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub maybe_attributes: Vec<Attribute>,
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

#[derive(Serialize, Debug)]
pub struct Constant {
    pub kind: String,
    pub value: Box<serde_json::value::RawValue>,
    pub expression: Box<serde_json::value::RawValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub literal: Option<Literal>,
}

impl Clone for Constant {
    fn clone(&self) -> Self {
        Self {
            kind: self.kind.clone(),
            value: serde_json::value::RawValue::from_string(self.value.get().to_string()).unwrap(),
            expression: serde_json::value::RawValue::from_string(self.expression.get().to_string())
                .unwrap(),
            identifier: self.identifier.clone(),
            literal: self.literal.clone(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Literal {
    pub kind: String,
    pub value: Box<serde_json::value::RawValue>,
    pub expression: Box<serde_json::value::RawValue>,
}

impl Clone for Literal {
    fn clone(&self) -> Self {
        Self {
            kind: self.kind.clone(),
            value: serde_json::value::RawValue::from_string(self.value.get().to_string()).unwrap(),
            expression: serde_json::value::RawValue::from_string(self.expression.get().to_string())
                .unwrap(),
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct AttributeArg {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub value: Constant,
    pub location: Location,
}

#[derive(Serialize, Clone, Debug)]
pub struct Attribute {
    pub name: String,
    pub arguments: Vec<AttributeArg>,
    pub location: Location,
}
#[derive(Serialize, Clone, Debug)]
pub struct ResourceProperty {
    pub name: String,
    pub location: Location,
    pub deprecated: bool,
    #[serde(rename = "type")]
    pub type_: Type,
}

#[derive(Serialize, Clone, Debug)]
pub struct ExperimentalResourceDeclaration {
    pub name: String,
    pub location: Location,
    pub deprecated: bool,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub maybe_attributes: Vec<Attribute>,
    #[serde(rename = "type")]
    pub type_: Type,
    pub properties: Vec<ResourceProperty>,
}
#[derive(Serialize, Clone, Debug)]
pub struct ProtocolDeclaration {
    pub name: String,
    pub location: Location,
    pub deprecated: bool,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub maybe_attributes: Vec<Attribute>,
    pub openness: String,
    pub composed_protocols: Vec<ProtocolCompose>,
    pub methods: Vec<ProtocolMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implementation_locations: Option<std::collections::BTreeMap<String, Vec<String>>>,
}

#[derive(Serialize, Clone, Debug)]
pub struct ProtocolCompose {
    pub name: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub maybe_attributes: Vec<Attribute>,
    pub location: Location,
    pub deprecated: bool,
}

#[derive(Serialize, Clone, Debug)]
pub struct ProtocolMethod {
    pub kind: String,
    pub ordinal: u64,
    pub name: String,
    pub strict: bool,
    pub location: Location,
    pub deprecated: bool,
    pub has_request: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maybe_request_payload: Option<Type>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub maybe_attributes: Vec<Attribute>,
    pub has_response: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maybe_response_payload: Option<Type>,
    pub is_composed: bool,
    pub has_error: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maybe_response_success_type: Option<Type>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maybe_response_err_type: Option<Type>,
}
#[derive(Serialize, Clone, Debug)]
pub struct ServiceDeclaration {
    pub name: String,
    pub location: Location,
    pub deprecated: bool,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub maybe_attributes: Vec<Attribute>,
    pub members: Vec<ServiceMember>,
}

#[derive(Serialize, Clone, Debug)]
pub struct ServiceMember {
    #[serde(rename = "type")]
    pub type_: Type,
    pub name: String,
    pub location: Location,
    pub deprecated: bool,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub maybe_attributes: Vec<Attribute>,
}
#[derive(Serialize, Clone, Debug)]
pub struct TableDeclaration {
    pub name: String,
    pub naming_context: Vec<String>,
    pub location: Location,
    pub deprecated: bool,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub maybe_attributes: Vec<Attribute>,
    pub members: Vec<TableMember>,
    pub strict: bool,
    pub resource: bool,
    #[serde(rename = "type_shape_v2")]
    pub type_shape: TypeShape,
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
    pub experimental_maybe_from_alias: Option<ExperimentalMaybeFromAlias>,
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
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub maybe_attributes: Vec<Attribute>,
    pub members: Vec<UnionMember>,
    pub strict: bool,
    pub resource: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_result: Option<bool>,
    #[serde(rename = "type_shape_v2")]
    pub type_shape: TypeShape,
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
    pub experimental_maybe_from_alias: Option<ExperimentalMaybeFromAlias>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub maybe_attributes: Vec<Attribute>,
}
#[derive(Serialize, Clone, Debug)]
pub struct PartialTypeCtor {
    pub name: String,
    pub args: Vec<PartialTypeCtor>,
    pub nullable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maybe_size: Option<Constant>,
}

#[derive(Serialize, Clone, Debug)]
pub struct AliasDeclaration {
    pub name: String,
    pub location: Location,
    pub deprecated: bool,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub maybe_attributes: Vec<Attribute>,
    pub partial_type_ctor: PartialTypeCtor,
    #[serde(rename = "type")]
    pub type_: Type,
}
#[derive(Serialize, Clone, Debug)]
pub struct NewTypeDeclaration {
    pub name: String,
    pub location: Location,
    pub deprecated: bool,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub maybe_attributes: Vec<Attribute>,
    #[serde(rename = "type")]
    pub type_: Type,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental_maybe_from_alias: Option<ExperimentalMaybeFromAlias>,
}

impl serde::Serialize for Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(serde::Serialize)]
        struct TypeOldFormat<'a> {
            #[serde(rename = "kind_v2")]
            kind: TypeKind,
            #[serde(skip_serializing_if = "Option::is_none")]
            obj_type: Option<u32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            subtype: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            identifier: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            element_type: Option<&'a Type>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pointee_type: Option<&'a Type>,
            #[serde(skip_serializing_if = "Option::is_none")]
            experimental_maybe_from_alias: Option<&'a ExperimentalMaybeFromAlias>,
            #[serde(skip_serializing_if = "Option::is_none")]
            deprecated: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            role: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            protocol: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            element_count: Option<u32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            maybe_element_count: Option<u32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            rights: Option<u32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            nullable: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            protocol_transport: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            resource_identifier: Option<String>,
            #[serde(skip_serializing_if = "Vec::is_empty")]
            maybe_attributes: &'a Vec<Attribute>,
            #[serde(rename = "field_shape_v2")]
            #[serde(skip_serializing_if = "Option::is_none")]
            field_shape: Option<&'a FieldShape>,
            #[serde(rename = "type_shape_v2")]
            type_shape: &'a TypeShape,
        }

        let old = TypeOldFormat {
            kind: self.kind(),
            obj_type: match self {
                Type::Handle(t) => t.obj_type,
                _ => None,
            },
            subtype: match self {
                Type::Primitive(t) => Some(t.subtype.to_string()),
                Type::Handle(t) => t.subtype.clone(),
                Type::Request(t) => t.subtype.clone(),
                _ => None,
            },
            identifier: self.identifier(),
            element_type: if self.kind() != TypeKind::ExperimentalPointer {
                self.element_type()
            } else {
                None
            },
            pointee_type: if self.kind() == TypeKind::ExperimentalPointer {
                self.element_type()
            } else {
                None
            },
            experimental_maybe_from_alias: self.experimental_maybe_from_alias.as_ref(),
            deprecated: self.deprecated,
            role: match self {
                Type::Endpoint(t) => t.role.clone(),
                _ => None,
            },
            protocol: self.protocol(),
            element_count: self.element_count(),
            maybe_element_count: self.maybe_element_count(),
            rights: self.rights(),
            nullable: self.nullable(),
            protocol_transport: match self {
                Type::Endpoint(t) => t.protocol_transport.clone(),
                _ => None,
            },
            resource_identifier: self.resource_identifier(),
            maybe_attributes: &self.maybe_attributes,
            field_shape: self.field_shape.as_ref(),
            type_shape: &self.type_shape,
        };

        old.serialize(serializer)
    }
}
