use std::collections::BTreeMap;

#[derive(Debug)]
pub struct JsonRoot {
    pub name: String,
    pub platform: String,
    pub available: Option<BTreeMap<String, Vec<String>>>,
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
    pub overlay_declarations: Option<Vec<UnionDeclaration>>,
    pub alias_declarations: Vec<AliasDeclaration>,
    pub new_type_declarations: Vec<NewTypeDeclaration>,
    pub declaration_order: Vec<String>,
    pub declarations: indexmap::IndexMap<String, String>,
}

#[derive(Clone, Debug)]
pub struct LibraryDependency {
    pub name: String,
    pub declarations: indexmap::IndexMap<String, serde_json::Value>,
}

#[derive(Clone, Debug)]
pub struct Location {
    pub filename: String,
    pub line: usize,
    pub column: usize,
    pub length: usize,
}

#[derive(Clone, Debug)]
pub struct TypeShape {
    pub inline_size: u32,
    pub alignment: u32,
    pub depth: u32,
    pub max_handles: u32,
    pub max_out_of_line: u32,
    pub has_padding: bool,
    pub has_flexible_envelope: bool,
}

#[derive(Clone, Debug)]
pub struct FieldShape {
    pub offset: u32,
    pub padding: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug)]
pub struct TypeCommon {
    pub experimental_maybe_from_alias: Option<ExperimentalMaybeFromAlias>,
    pub outer_alias: Option<ExperimentalMaybeFromAlias>,
    pub deprecated: Option<bool>,
    pub maybe_attributes: Vec<Attribute>,

    pub field_shape: Option<FieldShape>,
    pub type_shape: TypeShape,
    pub maybe_size_constant_name: Option<String>,
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

#[derive(Clone, Debug)]
pub struct PrimitiveType {
    pub common: TypeCommon,
    pub subtype: PrimitiveSubtype,
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

impl PrimitiveType {
    pub fn new(subtype: PrimitiveSubtype) -> Self {
        let (inline_size, alignment) = match subtype {
            PrimitiveSubtype::Bool
            | PrimitiveSubtype::Int8
            | PrimitiveSubtype::Uint8
            | PrimitiveSubtype::Uchar => (1, 1),
            PrimitiveSubtype::Int16 | PrimitiveSubtype::Uint16 => (2, 2),
            PrimitiveSubtype::Int32 | PrimitiveSubtype::Uint32 | PrimitiveSubtype::Float32 => {
                (4, 4)
            }
            PrimitiveSubtype::Int64
            | PrimitiveSubtype::Uint64
            | PrimitiveSubtype::Float64
            | PrimitiveSubtype::Usize64
            | PrimitiveSubtype::Uintptr64 => (8, 8),
        };

        Self {
            subtype,
            common: TypeCommon {
                experimental_maybe_from_alias: None,
                outer_alias: None,
                deprecated: None,
                maybe_attributes: vec![],
                field_shape: None,
                type_shape: TypeShape {
                    inline_size,
                    alignment,
                    depth: 0,
                    max_handles: 0,
                    max_out_of_line: 0,
                    has_padding: false,
                    has_flexible_envelope: false,
                },
                maybe_size_constant_name: None,
                resource: false,
            },
        }
    }
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

#[derive(Clone, Debug)]
pub struct StringType {
    pub common: TypeCommon,
    pub nullable: bool,
    pub maybe_element_count: Option<u32>,
}

#[derive(Clone, Debug)]
pub struct StringArrayType {
    pub common: TypeCommon,
    pub element_count: Option<u32>,
}

#[derive(Clone, Debug)]
pub struct UnknownType {
    pub common: TypeCommon,
}

#[derive(Clone, Debug)]
pub struct VectorType {
    pub common: TypeCommon,
    pub element_type: Box<Type>,
    pub nullable: bool,
    pub maybe_element_count: Option<u32>,
}

#[derive(Clone, Debug)]
pub struct ArrayType {
    pub common: TypeCommon,
    pub element_type: Box<Type>,
    pub element_count: u32,
}

#[derive(Clone, Debug)]
pub struct EndpointType {
    pub common: TypeCommon,
    pub nullable: bool,
    pub protocol: Option<String>,
    pub role: Option<String>,
    pub protocol_transport: Option<String>,
}

#[derive(Clone, Debug)]
pub struct HandleType {
    pub common: TypeCommon,
    pub subtype: Option<String>,
    pub rights: Option<u32>,
    pub obj_type: Option<u32>,
    pub nullable: bool,
    pub resource_identifier: Option<String>,
}

#[derive(Clone, Debug)]
pub struct IdentifierType {
    pub common: TypeCommon,
    pub identifier: Option<String>,
    pub nullable: bool,
}

#[derive(Clone, Debug)]
pub struct StructType {
    pub common: TypeCommon,
    pub identifier: Option<String>,
    pub nullable: bool,
}

#[derive(Clone, Debug)]
pub struct RequestType {
    pub common: TypeCommon,
    pub subtype: Option<String>,
    pub identifier: Option<String>,
    pub nullable: bool,
}

#[derive(Clone, Debug)]
pub struct ExperimentalPointerType {
    pub common: TypeCommon,
    pub element_type: Option<Box<Type>>,
    pub nullable: bool,
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
    pub fn primitive(subtype: PrimitiveSubtype) -> Self {
        Type::Primitive(PrimitiveType::new(subtype))
    }
    pub fn string(
        maybe_element_count: Option<u32>,
        nullable: bool,
        maybe_size_constant_name: Option<String>,
    ) -> Self {
        Type::String(StringType {
            common: TypeCommon {
                experimental_maybe_from_alias: None,
                outer_alias: None,
                maybe_attributes: vec![],
                field_shape: None,
                maybe_size_constant_name,
                resource: false,
                deprecated: None,
                type_shape: TypeShape {
                    inline_size: 16,
                    alignment: 8,
                    depth: 1,
                    max_handles: 0,
                    max_out_of_line: match maybe_element_count {
                        None => u32::MAX,
                        Some(max_len) => (max_len + 7) & !7,
                    },
                    has_padding: true,
                    has_flexible_envelope: false,
                },
            },
            maybe_element_count,
            nullable,
        })
    }
    pub fn string_array(element_count: Option<u32>) -> Self {
        Type::StringArray(StringArrayType {
            common: TypeCommon {
                experimental_maybe_from_alias: None,
                outer_alias: None,
                maybe_attributes: vec![],
                field_shape: None,
                maybe_size_constant_name: None,
                resource: false,
                deprecated: None,
                type_shape: TypeShape {
                    inline_size: element_count.unwrap_or(u32::MAX),
                    alignment: 1,
                    depth: 0,
                    max_handles: 0,
                    max_out_of_line: 0,
                    has_padding: false,
                    has_flexible_envelope: false,
                },
            },
            element_count,
        })
    }
    pub fn unknown() -> Self {
        Type::Unknown(UnknownType {
            common: TypeCommon {
                experimental_maybe_from_alias: None,
                outer_alias: None,
                maybe_attributes: vec![],
                field_shape: None,
                maybe_size_constant_name: None,
                resource: false,
                deprecated: None,
                type_shape: TypeShape {
                    inline_size: 0,
                    alignment: 1,
                    depth: 0,
                    max_handles: 0,
                    max_out_of_line: 0,
                    has_padding: false,
                    has_flexible_envelope: false,
                },
            },
        })
    }
    pub fn vector(
        mut element_type: Box<Type>,
        maybe_element_count: Option<u32>,
        nullable: bool,
        maybe_size_constant_name: Option<String>,
    ) -> Self {
        let mut inner_alias = element_type.outer_alias.take();
        if inner_alias.is_none()
            && element_type.kind() != TypeKind::Array
            && element_type.kind() != TypeKind::Vector
            && element_type.kind() != TypeKind::String
            && element_type.kind() != TypeKind::Request
        {
            inner_alias = element_type.experimental_maybe_from_alias.take();
        }
        let new_depth = element_type.type_shape.depth.saturating_add(1);
        let max_count = maybe_element_count.unwrap_or(u32::MAX);
        let elem_size = element_type.type_shape.inline_size;
        let elem_ool = element_type.type_shape.max_out_of_line;
        let content_size = max_count.saturating_mul(elem_size.saturating_add(elem_ool));
        let max_ool = if content_size.is_multiple_of(8) {
            content_size
        } else {
            content_size.saturating_add(8 - (content_size % 8))
        };
        let max_handles = max_count.saturating_mul(element_type.type_shape.max_handles);

        Type::Vector(VectorType {
            common: TypeCommon {
                experimental_maybe_from_alias: inner_alias,
                outer_alias: None,
                maybe_attributes: vec![],
                field_shape: None,
                maybe_size_constant_name,
                resource: element_type.resource,
                deprecated: None,
                type_shape: TypeShape {
                    inline_size: 16,
                    alignment: 8,
                    depth: new_depth,
                    max_handles,
                    max_out_of_line: max_ool,
                    has_padding: element_type.type_shape.has_padding
                        || !elem_size.is_multiple_of(8),
                    has_flexible_envelope: element_type.type_shape.has_flexible_envelope,
                },
            },
            element_type,
            nullable,
            maybe_element_count,
        })
    }
    pub fn array(
        mut element_type: Box<Type>,
        element_count: u32,
        maybe_size_constant_name: Option<String>,
    ) -> Self {
        let mut inner_alias = element_type.outer_alias.take();
        if inner_alias.is_none()
            && element_type.kind() != TypeKind::Array
            && element_type.kind() != TypeKind::Vector
            && element_type.kind() != TypeKind::String
            && element_type.kind() != TypeKind::Request
        {
            inner_alias = element_type.experimental_maybe_from_alias.take();
        }
        let elem_size = element_type.type_shape.inline_size;
        let total_size = elem_size.saturating_mul(element_count);
        let elem_ool = element_type.type_shape.max_out_of_line;
        let max_ool = elem_ool.saturating_mul(element_count);
        let max_handles = element_type
            .type_shape
            .max_handles
            .saturating_mul(element_count);
        Type::Array(ArrayType {
            common: TypeCommon {
                experimental_maybe_from_alias: inner_alias,
                outer_alias: None,
                maybe_attributes: vec![],
                field_shape: None,
                maybe_size_constant_name,
                resource: element_type.resource,
                deprecated: None,
                type_shape: TypeShape {
                    inline_size: total_size,
                    alignment: element_type.type_shape.alignment,
                    depth: element_type.type_shape.depth,
                    max_handles,
                    max_out_of_line: max_ool,
                    has_padding: element_type.type_shape.has_padding,
                    has_flexible_envelope: element_type.type_shape.has_flexible_envelope,
                },
            },
            element_type,
            element_count,
        })
    }
    pub fn endpoint(
        protocol: Option<String>,
        role: Option<String>,
        nullable: bool,
        protocol_transport: Option<String>,
    ) -> Self {
        Type::Endpoint(EndpointType {
            common: TypeCommon {
                experimental_maybe_from_alias: None,
                outer_alias: None,
                maybe_attributes: vec![],
                field_shape: None,
                maybe_size_constant_name: None,
                resource: true,
                deprecated: None,
                type_shape: TypeShape {
                    inline_size: 4,
                    alignment: 4,
                    depth: 0,
                    max_handles: 1,
                    max_out_of_line: 0,
                    has_padding: false,
                    has_flexible_envelope: false,
                },
            },
            nullable,
            protocol,
            role,
            protocol_transport: if protocol_transport.is_some() {
                protocol_transport
            } else {
                Some("Channel".to_string())
            },
        })
    }
    pub fn handle(
        subtype: Option<String>,
        rights: Option<u32>,
        obj_type: Option<u32>,
        nullable: bool,
        resource_identifier: Option<String>,
    ) -> Self {
        Type::Handle(HandleType {
            common: TypeCommon {
                experimental_maybe_from_alias: None,
                outer_alias: None,
                maybe_attributes: vec![],
                field_shape: None,
                maybe_size_constant_name: None,
                resource: true,
                deprecated: None,
                type_shape: TypeShape {
                    inline_size: 4,
                    alignment: 4,
                    depth: 0,
                    max_handles: 1,
                    max_out_of_line: 0,
                    has_padding: false,
                    has_flexible_envelope: false,
                },
            },
            subtype,
            rights,
            obj_type,
            nullable,
            resource_identifier,
        })
    }
    pub fn identifier_type(
        identifier: Option<String>,
        nullable: bool,
        type_shape: TypeShape,
        resource: bool,
    ) -> Self {
        Type::Identifier(IdentifierType {
            common: TypeCommon {
                experimental_maybe_from_alias: None,
                outer_alias: None,
                maybe_attributes: vec![],
                field_shape: None,
                maybe_size_constant_name: None,
                resource,
                deprecated: None,
                type_shape,
            },
            identifier,
            nullable,
        })
    }
    pub fn struct_type(
        identifier: Option<String>,
        nullable: bool,
        type_shape: TypeShape,
        resource: bool,
    ) -> Self {
        Type::Struct(StructType {
            common: TypeCommon {
                experimental_maybe_from_alias: None,
                outer_alias: None,
                maybe_attributes: vec![],
                field_shape: None,
                maybe_size_constant_name: None,
                resource,
                deprecated: None,
                type_shape,
            },
            identifier,
            nullable,
        })
    }
    pub fn request(subtype: Option<String>, identifier: Option<String>, nullable: bool) -> Self {
        Type::Request(RequestType {
            common: TypeCommon {
                experimental_maybe_from_alias: None,
                outer_alias: None,
                maybe_attributes: vec![],
                field_shape: None,
                maybe_size_constant_name: None,
                resource: true,
                deprecated: None,
                type_shape: TypeShape {
                    inline_size: 4,
                    alignment: 4,
                    depth: 0,
                    max_handles: 1,
                    max_out_of_line: 0,
                    has_padding: false,
                    has_flexible_envelope: false,
                },
            },
            subtype,
            identifier,
            nullable,
        })
    }
    pub fn experimental_pointer(element_type: Option<Box<Type>>, nullable: bool) -> Self {
        Type::ExperimentalPointer(ExperimentalPointerType {
            common: TypeCommon {
                experimental_maybe_from_alias: None,
                outer_alias: None,
                maybe_attributes: vec![],
                field_shape: None,
                maybe_size_constant_name: None,
                resource: false,
                deprecated: None,
                type_shape: TypeShape {
                    inline_size: 8,
                    alignment: 8,
                    depth: 0,
                    max_handles: 0,
                    max_out_of_line: 0,
                    has_padding: false,
                    has_flexible_envelope: false,
                },
            },
            element_type,
            nullable,
        })
    }

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
            Type::Array(t) => Some(t.element_type.as_ref()),
            Type::Vector(t) => Some(t.element_type.as_ref()),
            Type::ExperimentalPointer(t) => t.element_type.as_deref(),
            _ => None,
        }
    }

    pub fn nullable(&self) -> bool {
        match self {
            Type::String(t) => t.nullable,
            Type::Vector(t) => t.nullable,
            Type::Endpoint(t) => t.nullable,
            Type::Handle(t) => t.nullable,
            Type::Identifier(t) => t.nullable,
            Type::Struct(t) => t.nullable,
            Type::Request(t) => t.nullable,
            Type::ExperimentalPointer(t) => t.nullable,
            _ => false,
        }
    }
    pub fn set_nullable(&mut self, val: bool) {
        match self {
            Type::String(t) => t.nullable = val,
            Type::Vector(t) => t.nullable = val,
            Type::Endpoint(t) => t.nullable = val,
            Type::Handle(t) => t.nullable = val,
            Type::Identifier(t) => t.nullable = val,
            Type::Struct(t) => t.nullable = val,
            Type::Request(t) => t.nullable = val,
            Type::ExperimentalPointer(t) => t.nullable = val,
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
            Type::Array(t) => Some(t.element_type.as_mut()),
            Type::Vector(t) => Some(t.element_type.as_mut()),
            Type::ExperimentalPointer(t) => t.element_type.as_deref_mut(),
            _ => None,
        }
    }
    pub fn rights(&self) -> Option<u32> {
        match self {
            Type::Handle(t) => t.rights,
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
            Type::Array(t) => Some(t.element_count),
            Type::StringArray(t) => t.element_count,
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExperimentalMaybeFromAlias {
    pub name: String,
    pub args: Vec<String>,
    pub nullable: bool,
}

#[derive(Clone, Debug)]
pub struct StructMember {
    pub type_: Type,
    pub experimental_maybe_from_alias: Option<ExperimentalMaybeFromAlias>,
    pub name: String,
    pub location: Location,
    pub deprecated: bool,
    pub maybe_attributes: Vec<Attribute>,
    pub maybe_default_value: Option<Constant>,
    pub field_shape: FieldShape,
}

#[derive(Clone, Debug)]
pub struct StructDeclaration {
    pub name: String,
    pub naming_context: Vec<String>,
    pub location: Location,
    pub deprecated: bool,
    pub maybe_attributes: Vec<Attribute>,
    pub members: Vec<StructMember>,
    pub resource: bool,
    pub is_empty_success_struct: bool,
    pub type_shape: TypeShape,
}

// Placeholders for other declarations
#[derive(Clone, Debug)]
pub struct BitField {
    // ...
}

#[derive(Clone, Debug)]
pub struct BitsDeclaration {
    pub name: String,
    pub naming_context: Vec<String>,
    pub location: Location,
    pub deprecated: bool,
    pub maybe_attributes: Vec<Attribute>,
    pub type_: Type,
    pub mask: String,
    pub members: Vec<BitsMember>,
    pub strict: bool,
}

#[derive(Clone, Debug)]
pub struct BitsMember {
    pub name: String,
    pub location: Location,
    pub deprecated: bool,
    pub value: Constant,
    pub maybe_attributes: Vec<Attribute>,
}

#[derive(Clone, Debug)]
pub struct ConstDeclaration {
    pub name: String,
    pub location: Location,
    pub deprecated: bool,
    pub maybe_attributes: Vec<Attribute>,
    pub type_: Type,
    pub value: Constant,
}
#[derive(Clone, Debug)]
pub struct EnumDeclaration {
    pub name: String,
    pub naming_context: Vec<String>,
    pub location: Location,
    pub deprecated: bool,
    pub maybe_attributes: Vec<Attribute>,
    pub type_: String,
    pub members: Vec<EnumMember>,
    pub strict: bool,
    pub maybe_unknown_value: Option<u32>,
}

#[derive(Clone, Debug)]
pub struct EnumMember {
    pub name: String,
    pub location: Location,
    pub deprecated: bool,
    pub value: Constant,
    pub maybe_attributes: Vec<Attribute>,
}

#[derive(Debug)]
pub struct Constant {
    pub kind: String,
    pub value: Box<serde_json::value::RawValue>,
    pub expression: Box<serde_json::value::RawValue>,
    pub identifier: Option<String>,
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

#[derive(Debug)]
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

#[derive(Clone, Debug)]
pub struct AttributeArg {
    pub name: String,
    pub type_: String,
    pub value: Constant,
    pub location: Location,
}

#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: String,
    pub arguments: Vec<AttributeArg>,
    pub location: Location,
}
#[derive(Clone, Debug)]
pub struct ResourceProperty {
    pub name: String,
    pub location: Location,
    pub deprecated: bool,
    pub type_: Type,
}

#[derive(Clone, Debug)]
pub struct ExperimentalResourceDeclaration {
    pub name: String,
    pub location: Location,
    pub deprecated: bool,
    pub maybe_attributes: Vec<Attribute>,
    pub type_: Type,
    pub properties: Vec<ResourceProperty>,
}
#[derive(Clone, Debug)]
pub struct ProtocolDeclaration {
    pub name: String,
    pub location: Location,
    pub deprecated: bool,
    pub maybe_attributes: Vec<Attribute>,
    pub openness: String,
    pub composed_protocols: Vec<ProtocolCompose>,
    pub methods: Vec<ProtocolMethod>,
    pub implementation_locations: Option<std::collections::BTreeMap<String, Vec<String>>>,
}

#[derive(Clone, Debug)]
pub struct ProtocolCompose {
    pub name: String,
    pub maybe_attributes: Vec<Attribute>,
    pub location: Location,
    pub deprecated: bool,
}

#[derive(Clone, Debug)]
pub struct ProtocolMethod {
    pub kind: String,
    pub ordinal: u64,
    pub name: String,
    pub strict: bool,
    pub location: Location,
    pub deprecated: bool,
    pub has_request: bool,
    pub maybe_request_payload: Option<Type>,
    pub maybe_attributes: Vec<Attribute>,
    pub has_response: bool,
    pub maybe_response_payload: Option<Type>,
    pub is_composed: bool,
    pub has_error: bool,
    pub maybe_response_success_type: Option<Type>,
    pub maybe_response_err_type: Option<Type>,
}
#[derive(Clone, Debug)]
pub struct ServiceDeclaration {
    pub name: String,
    pub location: Location,
    pub deprecated: bool,
    pub maybe_attributes: Vec<Attribute>,
    pub members: Vec<ServiceMember>,
}

#[derive(Clone, Debug)]
pub struct ServiceMember {
    pub type_: Type,
    pub name: String,
    pub location: Location,
    pub deprecated: bool,
    pub maybe_attributes: Vec<Attribute>,
}
#[derive(Clone, Debug)]
pub struct TableDeclaration {
    pub name: String,
    pub naming_context: Vec<String>,
    pub location: Location,
    pub deprecated: bool,
    pub maybe_attributes: Vec<Attribute>,
    pub members: Vec<TableMember>,
    pub strict: bool,
    pub resource: bool,
    pub type_shape: TypeShape,
}

#[derive(Clone, Debug)]
pub struct TableMember {
    pub ordinal: u32,
    pub reserved: Option<bool>,

    pub type_: Option<Type>,
    pub experimental_maybe_from_alias: Option<ExperimentalMaybeFromAlias>,
    pub name: Option<String>,
    pub location: Option<Location>,
    pub deprecated: Option<bool>,
    pub maybe_attributes: Vec<Attribute>,
}

#[derive(Clone, Debug)]
pub struct UnionDeclaration {
    pub name: String,
    pub naming_context: Vec<String>,
    pub location: Location,
    pub deprecated: bool,
    pub maybe_attributes: Vec<Attribute>,
    pub members: Vec<UnionMember>,
    pub strict: bool,
    pub resource: bool,
    pub is_result: Option<bool>,
    pub type_shape: TypeShape,
}

#[derive(Clone, Debug)]
pub struct UnionMember {
    pub ordinal: u32,
    pub reserved: Option<bool>,
    pub name: Option<String>,

    pub type_: Option<Type>,
    pub experimental_maybe_from_alias: Option<ExperimentalMaybeFromAlias>,
    pub location: Option<Location>,
    pub deprecated: Option<bool>,
    pub maybe_attributes: Vec<Attribute>,
}
#[derive(Clone, Debug)]
pub struct PartialTypeCtor {
    pub name: String,
    pub args: Vec<PartialTypeCtor>,
    pub nullable: bool,
    pub maybe_size: Option<Constant>,
}

#[derive(Clone, Debug)]
pub struct AliasDeclaration {
    pub name: String,
    pub location: Location,
    pub deprecated: bool,
    pub maybe_attributes: Vec<Attribute>,
    pub partial_type_ctor: PartialTypeCtor,
    pub type_: Type,
}
#[derive(Clone, Debug)]
pub struct NewTypeDeclaration {
    pub name: String,
    pub location: Location,
    pub deprecated: bool,
    pub maybe_attributes: Vec<Attribute>,
    pub type_: Type,
    pub experimental_maybe_from_alias: Option<ExperimentalMaybeFromAlias>,
}
