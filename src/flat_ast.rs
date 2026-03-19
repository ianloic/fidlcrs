use std::collections::BTreeMap;

use crate::names::OwnedQualifiedName;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeclarationKind {
    Bits,
    Const,
    Enum,
    ExperimentalResource,
    Protocol,
    Service,
    Struct,
    Table,
    Union,
    Overlay,
    Alias,
    NewType,
}

impl std::fmt::Display for DeclarationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Bits => "bits",
            Self::Const => "const",
            Self::Enum => "enum",
            Self::ExperimentalResource => "resource",
            Self::Protocol => "protocol",
            Self::Service => "service",
            Self::Struct => "struct",
            Self::Table => "table",
            Self::Union => "union",
            Self::Overlay => "overlay",
            Self::Alias => "alias",
            Self::NewType => "type",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub struct Root {
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
}

#[derive(Clone, Debug)]
pub struct DependencyDeclaration {
    pub kind: DeclarationKind,
    pub resource: Option<bool>,
    pub type_shape: Option<TypeShape>,
}

#[derive(Clone, Debug)]
pub struct LibraryDependency {
    pub name: String,
    pub declarations: indexmap::IndexMap<String, DependencyDeclaration>,
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
    Internal,
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
    Internal(InternalType),
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

#[derive(Clone, Debug)]
pub struct InternalType {
    pub common: TypeCommon,
    pub subtype: String,
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
            Type::Internal(t) => &t.common,
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
            Type::Internal(t) => &mut t.common,
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

    pub fn internal(subtype: String) -> Self {
        Type::Internal(InternalType {
            common: TypeCommon {
                experimental_maybe_from_alias: None,
                outer_alias: None,
                maybe_attributes: vec![],
                field_shape: None,
                maybe_size_constant_name: None,
                resource: false,
                deprecated: None,
                type_shape: TypeShape {
                    inline_size: 4,
                    alignment: 4,
                    depth: 0,
                    max_handles: 0,
                    max_out_of_line: 0,
                    has_padding: false,
                    has_flexible_envelope: false,
                },
            },
            subtype,
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
            Type::Internal(_) => TypeKind::Internal,
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
    pub base: DeclBase,
    pub maybe_default_value: Option<Constant>,
    pub field_shape: FieldShape,
}

#[derive(Clone, Debug)]
pub struct DeclBase {
    pub name: OwnedQualifiedName,
    pub location: Location,
    pub deprecated: bool,
    pub maybe_attributes: Vec<Attribute>,
}

#[derive(Clone, Debug)]
pub struct StructDeclaration {
    pub base: DeclBase,
    pub naming_context: Vec<String>,
    pub members: Vec<StructMember>,
    pub resource: bool,
    pub is_empty_success_struct: bool,
    pub type_shape: TypeShape,
}

impl StructDeclaration {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: OwnedQualifiedName,
        location: Location,
        deprecated: bool,
        maybe_attributes: Vec<Attribute>,
        naming_context: Vec<String>,
        members: Vec<StructMember>,
        resource: bool,
        is_empty_success_struct: bool,
        type_shape: TypeShape,
    ) -> Self {
        Self {
            base: DeclBase {
                name,
                location,
                deprecated,
                maybe_attributes,
            },
            naming_context,
            members,
            resource,
            is_empty_success_struct,
            type_shape,
        }
    }
}

// Placeholders for other declarations
#[derive(Clone, Debug)]
pub struct BitField {
    // ...
}

#[derive(Clone, Debug)]
pub struct BitsDeclaration {
    pub base: DeclBase,
    pub naming_context: Vec<String>,
    pub type_: Type,
    pub mask: String,
    pub members: Vec<BitsMember>,
    pub strict: bool,
}

impl BitsDeclaration {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: OwnedQualifiedName,
        location: Location,
        deprecated: bool,
        maybe_attributes: Vec<Attribute>,
        naming_context: Vec<String>,
        type_: Type,
        mask: String,
        members: Vec<BitsMember>,
        strict: bool,
    ) -> Self {
        Self {
            base: DeclBase {
                name,
                location,
                deprecated,
                maybe_attributes,
            },
            naming_context,
            type_,
            mask,
            members,
            strict,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BitsMember {
    pub base: DeclBase,
    pub value: Constant,
}

#[derive(Clone, Debug)]
pub struct ConstDeclaration {
    pub base: DeclBase,
    pub type_: Type,
    pub value: Constant,
}

impl ConstDeclaration {
    pub fn new(
        name: OwnedQualifiedName,
        location: Location,
        deprecated: bool,
        maybe_attributes: Vec<Attribute>,
        type_: Type,
        value: Constant,
    ) -> Self {
        Self {
            base: DeclBase {
                name,
                location,
                deprecated,
                maybe_attributes,
            },
            type_,
            value,
        }
    }
}
#[derive(Clone, Debug)]
pub struct EnumDeclaration {
    pub base: DeclBase,
    pub naming_context: Vec<String>,
    pub type_: String,
    pub members: Vec<EnumMember>,
    pub strict: bool,
    pub maybe_unknown_value: Option<u64>,
}

impl EnumDeclaration {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: OwnedQualifiedName,
        location: Location,
        deprecated: bool,
        maybe_attributes: Vec<Attribute>,
        naming_context: Vec<String>,
        type_: String,
        members: Vec<EnumMember>,
        strict: bool,
        maybe_unknown_value: Option<u64>,
    ) -> Self {
        Self {
            base: DeclBase {
                name,
                location,
                deprecated,
                maybe_attributes,
            },
            naming_context,
            type_,
            members,
            strict,
            maybe_unknown_value,
        }
    }
}

#[derive(Clone, Debug)]
pub struct EnumMember {
    pub base: DeclBase,
    pub value: Constant,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Constant {
    pub kind: String,
    pub value: String,
    pub expression: String,
    pub identifier: Option<String>,
    pub literal: Option<Literal>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Literal {
    pub kind: String,
    pub value: String,
    pub expression: String,
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
    pub base: DeclBase,
    pub type_: Type,
    pub properties: Vec<ResourceProperty>,
}

impl ExperimentalResourceDeclaration {
    pub fn new(
        name: OwnedQualifiedName,
        location: Location,
        deprecated: bool,
        maybe_attributes: Vec<Attribute>,
        type_: Type,
        properties: Vec<ResourceProperty>,
    ) -> Self {
        Self {
            base: DeclBase {
                name,
                location,
                deprecated,
                maybe_attributes,
            },
            type_,
            properties,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Openness {
    Open,
    Ajar,
    Closed,
}

impl std::fmt::Display for Openness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open => write!(f, "open"),
            Self::Ajar => write!(f, "ajar"),
            Self::Closed => write!(f, "closed"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProtocolDeclaration {
    pub base: DeclBase,
    pub openness: Openness,
    pub composed_protocols: Vec<ProtocolCompose>,
    pub methods: Vec<ProtocolMethod>,
    pub implementation_locations: Option<std::collections::BTreeMap<String, Vec<String>>>,
}

impl ProtocolDeclaration {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: OwnedQualifiedName,
        location: Location,
        deprecated: bool,
        maybe_attributes: Vec<Attribute>,
        openness: Openness,
        composed_protocols: Vec<ProtocolCompose>,
        methods: Vec<ProtocolMethod>,
        implementation_locations: Option<std::collections::BTreeMap<String, Vec<String>>>,
    ) -> Self {
        Self {
            base: DeclBase {
                name,
                location,
                deprecated,
                maybe_attributes,
            },
            openness,
            composed_protocols,
            methods,
            implementation_locations,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProtocolCompose {
    pub base: DeclBase,
}

#[derive(Clone, Debug)]
pub struct ProtocolMethod {
    pub base: DeclBase,
    pub kind: String,
    pub ordinal: u64,
    pub strict: bool,
    pub has_request: bool,
    pub maybe_request_payload: Option<Type>,
    pub has_response: bool,
    pub maybe_response_payload: Option<Type>,
    pub is_composed: bool,
    pub has_error: bool,
    pub maybe_response_success_type: Option<Type>,
    pub maybe_response_err_type: Option<Type>,
}
#[derive(Clone, Debug)]
pub struct ServiceDeclaration {
    pub base: DeclBase,
    pub members: Vec<ServiceMember>,
}

impl ServiceDeclaration {
    pub fn new(
        name: OwnedQualifiedName,
        location: Location,
        deprecated: bool,
        maybe_attributes: Vec<Attribute>,
        members: Vec<ServiceMember>,
    ) -> Self {
        Self {
            base: DeclBase {
                name,
                location,
                deprecated,
                maybe_attributes,
            },
            members,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ServiceMember {
    pub type_: Type,
    pub base: DeclBase,
}
#[derive(Clone, Debug)]
pub struct TableDeclaration {
    pub base: DeclBase,
    pub naming_context: Vec<String>,
    pub members: Vec<TableMember>,
    pub strict: bool,
    pub resource: bool,
    pub type_shape: TypeShape,
}

impl TableDeclaration {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: OwnedQualifiedName,
        location: Location,
        deprecated: bool,
        maybe_attributes: Vec<Attribute>,
        naming_context: Vec<String>,
        members: Vec<TableMember>,
        strict: bool,
        resource: bool,
        type_shape: TypeShape,
    ) -> Self {
        Self {
            base: DeclBase {
                name,
                location,
                deprecated,
                maybe_attributes,
            },
            naming_context,
            members,
            strict,
            resource,
            type_shape,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TableMember {
    pub ordinal: u32,
    pub reserved: Option<bool>,

    pub type_: Option<Type>,
    pub experimental_maybe_from_alias: Option<ExperimentalMaybeFromAlias>,
    pub base: DeclBase,
}

#[derive(Clone, Debug)]
pub struct UnionDeclaration {
    pub base: DeclBase,
    pub naming_context: Vec<String>,
    pub members: Vec<UnionMember>,
    pub strict: bool,
    pub resource: bool,
    pub is_result: Option<bool>,
    pub type_shape: TypeShape,
}

impl UnionDeclaration {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: OwnedQualifiedName,
        location: Location,
        deprecated: bool,
        maybe_attributes: Vec<Attribute>,
        naming_context: Vec<String>,
        members: Vec<UnionMember>,
        strict: bool,
        resource: bool,
        is_result: Option<bool>,
        type_shape: TypeShape,
    ) -> Self {
        Self {
            base: DeclBase {
                name,
                location,
                deprecated,
                maybe_attributes,
            },
            naming_context,
            members,
            strict,
            resource,
            is_result,
            type_shape,
        }
    }
}

#[derive(Clone, Debug)]
pub struct UnionMember {
    pub ordinal: u32,
    pub reserved: Option<bool>,

    pub type_: Option<Type>,
    pub experimental_maybe_from_alias: Option<ExperimentalMaybeFromAlias>,
    pub base: DeclBase,
}
#[derive(Clone, Debug)]
pub struct PartialTypeCtor {
    pub name: String,
    pub args: Vec<PartialTypeCtor>,
    pub nullable: bool,
    pub maybe_size: Option<Constant>,
    pub handle_rights: Option<Constant>,
}

#[derive(Clone, Debug)]
pub struct AliasDeclaration {
    pub base: DeclBase,
    pub partial_type_ctor: PartialTypeCtor,
    pub type_: Type,
}

impl AliasDeclaration {
    pub fn new(
        name: OwnedQualifiedName,
        location: Location,
        deprecated: bool,
        maybe_attributes: Vec<Attribute>,
        partial_type_ctor: PartialTypeCtor,
        type_: Type,
    ) -> Self {
        Self {
            base: DeclBase {
                name,
                location,
                deprecated,
                maybe_attributes,
            },
            partial_type_ctor,
            type_,
        }
    }
}
#[derive(Clone, Debug)]
pub struct NewTypeDeclaration {
    pub base: DeclBase,
    pub type_: Type,
    pub experimental_maybe_from_alias: Option<ExperimentalMaybeFromAlias>,
}

impl NewTypeDeclaration {
    pub fn new(
        name: OwnedQualifiedName,
        location: Location,
        deprecated: bool,
        maybe_attributes: Vec<Attribute>,
        type_: Type,
        experimental_maybe_from_alias: Option<ExperimentalMaybeFromAlias>,
    ) -> Self {
        Self {
            base: DeclBase {
                name,
                location,
                deprecated,
                maybe_attributes,
            },
            type_,
            experimental_maybe_from_alias,
        }
    }
}

macro_rules! impl_deref_for_decl {
    ($t:ty) => {
        impl std::ops::Deref for $t {
            type Target = DeclBase;
            fn deref(&self) -> &Self::Target {
                &self.base
            }
        }
        impl std::ops::DerefMut for $t {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.base
            }
        }
    };
}

impl_deref_for_decl!(StructDeclaration);
impl_deref_for_decl!(BitsDeclaration);
impl_deref_for_decl!(ConstDeclaration);
impl_deref_for_decl!(EnumDeclaration);
impl_deref_for_decl!(ExperimentalResourceDeclaration);
impl_deref_for_decl!(ProtocolDeclaration);
impl_deref_for_decl!(ServiceDeclaration);
impl_deref_for_decl!(TableDeclaration);
impl_deref_for_decl!(UnionDeclaration);
impl_deref_for_decl!(AliasDeclaration);
impl_deref_for_decl!(NewTypeDeclaration);
impl_deref_for_decl!(StructMember);
impl_deref_for_decl!(BitsMember);
impl_deref_for_decl!(EnumMember);
impl_deref_for_decl!(ServiceMember);
impl_deref_for_decl!(TableMember);
impl_deref_for_decl!(UnionMember);
impl_deref_for_decl!(ProtocolMethod);
impl_deref_for_decl!(ProtocolCompose);

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug)]
pub enum Decl {
    Alias(AliasDeclaration),
    Bits(BitsDeclaration),
    Const(ConstDeclaration),
    Enum(EnumDeclaration),
    ExperimentalResource(ExperimentalResourceDeclaration),
    NewType(NewTypeDeclaration),
    Protocol(ProtocolDeclaration),
    Service(ServiceDeclaration),
    Struct(StructDeclaration),
    Table(TableDeclaration),
    Union(UnionDeclaration),
    Overlay(UnionDeclaration),
}

impl std::ops::Deref for Decl {
    type Target = DeclBase;
    fn deref(&self) -> &Self::Target {
        match self {
            Decl::Alias(d) => &d.base,
            Decl::Bits(d) => &d.base,
            Decl::Const(d) => &d.base,
            Decl::Enum(d) => &d.base,
            Decl::ExperimentalResource(d) => &d.base,
            Decl::NewType(d) => &d.base,
            Decl::Protocol(d) => &d.base,
            Decl::Service(d) => &d.base,
            Decl::Struct(d) => &d.base,
            Decl::Table(d) => &d.base,
            Decl::Union(d) => &d.base,
            Decl::Overlay(d) => &d.base,
        }
    }
}

impl std::ops::DerefMut for Decl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Decl::Alias(d) => &mut d.base,
            Decl::Bits(d) => &mut d.base,
            Decl::Const(d) => &mut d.base,
            Decl::Enum(d) => &mut d.base,
            Decl::ExperimentalResource(d) => &mut d.base,
            Decl::NewType(d) => &mut d.base,
            Decl::Protocol(d) => &mut d.base,
            Decl::Service(d) => &mut d.base,
            Decl::Struct(d) => &mut d.base,
            Decl::Table(d) => &mut d.base,
            Decl::Union(d) => &mut d.base,
            Decl::Overlay(d) => &mut d.base,
        }
    }
}

macro_rules! impl_from_decl {
    ($variant:ident, $type:ty) => {
        impl From<$type> for Decl {
            fn from(decl: $type) -> Self {
                Decl::$variant(decl)
            }
        }
    };
}

impl_from_decl!(Alias, AliasDeclaration);
impl_from_decl!(Bits, BitsDeclaration);
impl_from_decl!(Const, ConstDeclaration);
impl_from_decl!(Enum, EnumDeclaration);
impl_from_decl!(ExperimentalResource, ExperimentalResourceDeclaration);
impl_from_decl!(NewType, NewTypeDeclaration);
impl_from_decl!(Protocol, ProtocolDeclaration);
impl_from_decl!(Service, ServiceDeclaration);
impl_from_decl!(Struct, StructDeclaration);
impl_from_decl!(Table, TableDeclaration);
impl_from_decl!(Union, UnionDeclaration);

#[derive(Clone, Debug, Default)]
pub struct Declarations {
    decls: Vec<Decl>,
}

macro_rules! decl_iterators {
    ($method:ident, $mut_method:ident, $variant:ident, $type:ty) => {
        pub fn $method(&self) -> impl Iterator<Item = &$type> {
            self.decls.iter().filter_map(|d| {
                if let Decl::$variant(inner) = d {
                    Some(inner)
                } else {
                    None
                }
            })
        }

        pub fn $mut_method(&mut self) -> impl Iterator<Item = &mut $type> {
            self.decls.iter_mut().filter_map(|d| {
                if let Decl::$variant(inner) = d {
                    Some(inner)
                } else {
                    None
                }
            })
        }
    };
}

impl Declarations {
    pub fn new() -> Self {
        Self { decls: Vec::new() }
    }

    pub fn push(&mut self, decl: Decl) {
        self.decls.push(decl);
    }

    pub fn decls(&self) -> std::slice::Iter<'_, Decl> {
        self.decls.iter()
    }

    pub fn decls_mut(&mut self) -> std::slice::IterMut<'_, Decl> {
        self.decls.iter_mut()
    }

    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&Decl, &Decl) -> std::cmp::Ordering,
    {
        self.decls.sort_by(compare);
    }

    decl_iterators!(aliases, aliases_mut, Alias, AliasDeclaration);
    decl_iterators!(bits, bits_mut, Bits, BitsDeclaration);
    decl_iterators!(consts, consts_mut, Const, ConstDeclaration);
    decl_iterators!(enums, enums_mut, Enum, EnumDeclaration);
    decl_iterators!(
        experimental_resources,
        experimental_resources_mut,
        ExperimentalResource,
        ExperimentalResourceDeclaration
    );
    decl_iterators!(new_types, new_types_mut, NewType, NewTypeDeclaration);
    decl_iterators!(protocols, protocols_mut, Protocol, ProtocolDeclaration);
    decl_iterators!(services, services_mut, Service, ServiceDeclaration);
    decl_iterators!(structs, structs_mut, Struct, StructDeclaration);
    decl_iterators!(tables, tables_mut, Table, TableDeclaration);
    decl_iterators!(unions, unions_mut, Union, UnionDeclaration);
    decl_iterators!(overlays, overlays_mut, Overlay, UnionDeclaration);
}
