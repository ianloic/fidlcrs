use crate::flat_ast;
use serde::Serialize;
use std::collections::BTreeMap;
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
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
    pub declarations: indexmap::IndexMap<String, DeclarationKind>,
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
    Internal,
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
#[derive(Serialize, Clone, Debug)]
pub struct Constant {
    pub kind: String,
    pub value: Box<serde_json::value::RawValue>,
    pub expression: Box<serde_json::value::RawValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub literal: Option<Literal>,
}
#[derive(Serialize, Clone, Debug)]
pub struct Literal {
    pub kind: String,
    pub value: Box<serde_json::value::RawValue>,
    pub expression: Box<serde_json::value::RawValue>,
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
#[derive(Serialize, Clone, Debug)]
pub struct ExperimentalMaybeFromAlias {
    pub name: String,
    pub args: Vec<String>,
    pub nullable: bool,
}

#[derive(Serialize, Clone, Debug)]
pub struct PartialTypeCtor {
    pub name: String,
    pub args: Vec<PartialTypeCtor>,
    pub nullable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maybe_size: Option<Constant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle_rights: Option<Constant>,
}

#[derive(Serialize, Clone, Debug)]
pub struct Type {
    #[serde(rename = "kind_v2")]
    pub kind: TypeKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obj_type: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_type: Option<Box<Type>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointee_type: Option<Box<Type>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental_maybe_from_alias: Option<ExperimentalMaybeFromAlias>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maybe_element_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_transport: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub maybe_attributes: Vec<Attribute>,
    #[serde(rename = "field_shape_v2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_shape: Option<FieldShape>,
    #[serde(rename = "type_shape_v2")]
    pub type_shape: TypeShape,
}

impl From<&flat_ast::ExperimentalMaybeFromAlias> for ExperimentalMaybeFromAlias {
    fn from(ast: &flat_ast::ExperimentalMaybeFromAlias) -> Self {
        Self {
            name: ast.name.clone(),
            args: ast.args.clone(),
            nullable: ast.nullable,
        }
    }
}

impl From<&flat_ast::PartialTypeCtor> for PartialTypeCtor {
    fn from(ast: &flat_ast::PartialTypeCtor) -> Self {
        Self {
            name: ast.name.clone(),
            args: ast.args.iter().map(Into::into).collect(),
            nullable: ast.nullable,
            maybe_size: ast.maybe_size.as_ref().map(Into::into),
            handle_rights: ast.handle_rights.as_ref().map(Into::into),
        }
    }
}

impl From<&flat_ast::Type> for Type {
    fn from(ast: &flat_ast::Type) -> Self {
        Self {
            kind: (&ast.kind()).into(),
            obj_type: match ast {
                flat_ast::Type::Handle(t) => t.obj_type,
                _ => None,
            },
            subtype: match ast {
                flat_ast::Type::Primitive(t) => Some(t.subtype.to_string()),
                flat_ast::Type::Handle(t) => t.subtype.clone(),
                flat_ast::Type::Request(t) => t.subtype.clone(),
                flat_ast::Type::Internal(t) => Some(t.subtype.clone()),
                _ => None,
            },
            identifier: ast.identifier(),
            element_type: if ast.kind() != flat_ast::TypeKind::ExperimentalPointer {
                ast.element_type().map(|t| Box::new(t.into()))
            } else {
                None
            },
            pointee_type: if ast.kind() == flat_ast::TypeKind::ExperimentalPointer {
                ast.element_type().map(|t| Box::new(t.into()))
            } else {
                None
            },
            experimental_maybe_from_alias: ast
                .experimental_maybe_from_alias
                .as_ref()
                .map(Into::into),
            deprecated: ast.deprecated,
            role: match ast {
                flat_ast::Type::Endpoint(t) => t.role.clone(),
                _ => None,
            },
            protocol: ast.protocol(),
            element_count: ast.element_count(),
            maybe_element_count: ast.maybe_element_count(),
            rights: ast.rights(),
            nullable: match ast {
                flat_ast::Type::String(t) => Some(t.nullable),
                flat_ast::Type::Vector(t) => Some(t.nullable),
                flat_ast::Type::Endpoint(t) => Some(t.nullable),
                flat_ast::Type::Handle(t) => Some(t.nullable),
                flat_ast::Type::Identifier(t) => Some(t.nullable),
                flat_ast::Type::Struct(t) => Some(t.nullable),
                flat_ast::Type::Request(t) => Some(t.nullable),
                _ => None,
            },
            protocol_transport: match ast {
                flat_ast::Type::Endpoint(t) => t.protocol_transport.clone(),
                _ => None,
            },
            resource_identifier: ast.resource_identifier(),
            maybe_attributes: ast.maybe_attributes.iter().map(Into::into).collect(),
            field_shape: ast.field_shape.as_ref().map(Into::into),
            type_shape: (&ast.type_shape).into(),
        }
    }
}

impl From<&flat_ast::JsonRoot> for JsonRoot {
    fn from(ast: &flat_ast::JsonRoot) -> Self {
        Self {
            name: ast.name.clone(),
            platform: ast.platform.clone(),
            available: ast.available.clone(),
            maybe_attributes: ast.maybe_attributes.iter().map(Into::into).collect(),
            experiments: ast.experiments.clone(),
            library_dependencies: ast.library_dependencies.iter().map(Into::into).collect(),
            bits_declarations: ast.bits_declarations.iter().map(Into::into).collect(),
            const_declarations: ast.const_declarations.iter().map(Into::into).collect(),
            enum_declarations: ast.enum_declarations.iter().map(Into::into).collect(),
            experimental_resource_declarations: ast
                .experimental_resource_declarations
                .iter()
                .map(Into::into)
                .collect(),
            protocol_declarations: ast.protocol_declarations.iter().map(Into::into).collect(),
            service_declarations: ast.service_declarations.iter().map(Into::into).collect(),
            struct_declarations: ast.struct_declarations.iter().map(Into::into).collect(),
            external_struct_declarations: ast
                .external_struct_declarations
                .iter()
                .map(Into::into)
                .collect(),
            table_declarations: ast.table_declarations.iter().map(Into::into).collect(),
            union_declarations: ast.union_declarations.iter().map(Into::into).collect(),
            overlay_declarations: ast
                .overlay_declarations
                .as_ref()
                .map(|v| v.iter().map(Into::into).collect()),
            alias_declarations: ast.alias_declarations.iter().map(Into::into).collect(),
            new_type_declarations: ast.new_type_declarations.iter().map(Into::into).collect(),
            declaration_order: ast.declaration_order.clone(),
            declarations: ast
                .declarations
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
        }
    }
}

impl From<&flat_ast::LibraryDependency> for LibraryDependency {
    fn from(ast: &flat_ast::LibraryDependency) -> Self {
        Self {
            name: ast.name.clone(),
            declarations: ast
                .declarations
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        serde_json::from_str(v).unwrap_or(serde_json::Value::Null),
                    )
                })
                .collect(),
        }
    }
}

impl From<&flat_ast::Location> for Location {
    fn from(ast: &flat_ast::Location) -> Self {
        Self {
            filename: ast.filename.clone(),
            line: ast.line,
            column: ast.column,
            length: ast.length,
        }
    }
}

impl From<&flat_ast::TypeShape> for TypeShape {
    fn from(ast: &flat_ast::TypeShape) -> Self {
        Self {
            inline_size: ast.inline_size,
            alignment: ast.alignment,
            depth: ast.depth,
            max_handles: ast.max_handles,
            max_out_of_line: ast.max_out_of_line,
            has_padding: ast.has_padding,
            has_flexible_envelope: ast.has_flexible_envelope,
        }
    }
}

impl From<&flat_ast::FieldShape> for FieldShape {
    fn from(ast: &flat_ast::FieldShape) -> Self {
        Self {
            offset: ast.offset,
            padding: ast.padding,
        }
    }
}

impl From<&flat_ast::TypeKind> for TypeKind {
    fn from(ast: &flat_ast::TypeKind) -> Self {
        match ast {
            flat_ast::TypeKind::Primitive => Self::Primitive,
            flat_ast::TypeKind::String => Self::String,
            flat_ast::TypeKind::StringArray => Self::StringArray,
            flat_ast::TypeKind::Unknown => Self::Unknown,
            flat_ast::TypeKind::Vector => Self::Vector,
            flat_ast::TypeKind::Array => Self::Array,
            flat_ast::TypeKind::Endpoint => Self::Endpoint,
            flat_ast::TypeKind::Handle => Self::Handle,
            flat_ast::TypeKind::Identifier => Self::Identifier,
            flat_ast::TypeKind::Struct => Self::Struct,
            flat_ast::TypeKind::Request => Self::Request,
            flat_ast::TypeKind::ExperimentalPointer => Self::ExperimentalPointer,
            flat_ast::TypeKind::Internal => Self::Internal,
        }
    }
}

impl From<&flat_ast::StructMember> for StructMember {
    fn from(ast: &flat_ast::StructMember) -> Self {
        Self {
            type_: (&ast.type_).into(),
            experimental_maybe_from_alias: ast
                .experimental_maybe_from_alias
                .as_ref()
                .map(Into::into),
            name: ast.name.clone(),
            location: (&ast.location).into(),
            deprecated: ast.deprecated,
            maybe_attributes: ast.maybe_attributes.iter().map(Into::into).collect(),
            maybe_default_value: ast.maybe_default_value.as_ref().map(Into::into),
            field_shape: (&ast.field_shape).into(),
        }
    }
}

impl From<&flat_ast::StructDeclaration> for StructDeclaration {
    fn from(ast: &flat_ast::StructDeclaration) -> Self {
        Self {
            name: ast.name.clone(),
            naming_context: ast.naming_context.clone(),
            location: (&ast.location).into(),
            deprecated: ast.deprecated,
            maybe_attributes: ast.maybe_attributes.iter().map(Into::into).collect(),
            members: ast.members.iter().map(Into::into).collect(),
            resource: ast.resource,
            is_empty_success_struct: ast.is_empty_success_struct,
            type_shape: (&ast.type_shape).into(),
        }
    }
}

impl From<&flat_ast::BitField> for BitField {
    fn from(_ast: &flat_ast::BitField) -> Self {
        Self {}
    }
}

impl From<&flat_ast::BitsDeclaration> for BitsDeclaration {
    fn from(ast: &flat_ast::BitsDeclaration) -> Self {
        Self {
            name: ast.name.clone(),
            naming_context: ast.naming_context.clone(),
            location: (&ast.location).into(),
            deprecated: ast.deprecated,
            maybe_attributes: ast.maybe_attributes.iter().map(Into::into).collect(),
            type_: (&ast.type_).into(),
            mask: ast.mask.clone(),
            members: ast.members.iter().map(Into::into).collect(),
            strict: ast.strict,
        }
    }
}

impl From<&flat_ast::BitsMember> for BitsMember {
    fn from(ast: &flat_ast::BitsMember) -> Self {
        Self {
            name: ast.name.clone(),
            location: (&ast.location).into(),
            deprecated: ast.deprecated,
            value: (&ast.value).into(),
            maybe_attributes: ast.maybe_attributes.iter().map(Into::into).collect(),
        }
    }
}

impl From<&flat_ast::ConstDeclaration> for ConstDeclaration {
    fn from(ast: &flat_ast::ConstDeclaration) -> Self {
        Self {
            name: ast.name.clone(),
            location: (&ast.location).into(),
            deprecated: ast.deprecated,
            maybe_attributes: ast.maybe_attributes.iter().map(Into::into).collect(),
            type_: (&ast.type_).into(),
            value: (&ast.value).into(),
        }
    }
}

impl From<&flat_ast::EnumDeclaration> for EnumDeclaration {
    fn from(ast: &flat_ast::EnumDeclaration) -> Self {
        Self {
            name: ast.name.clone(),
            naming_context: ast.naming_context.clone(),
            location: (&ast.location).into(),
            deprecated: ast.deprecated,
            maybe_attributes: ast.maybe_attributes.iter().map(Into::into).collect(),
            type_: ast.type_.clone(),
            members: ast.members.iter().map(Into::into).collect(),
            strict: ast.strict,
            maybe_unknown_value: ast.maybe_unknown_value,
        }
    }
}

impl From<&flat_ast::EnumMember> for EnumMember {
    fn from(ast: &flat_ast::EnumMember) -> Self {
        Self {
            name: ast.name.clone(),
            location: (&ast.location).into(),
            deprecated: ast.deprecated,
            value: (&ast.value).into(),
            maybe_attributes: ast.maybe_attributes.iter().map(Into::into).collect(),
        }
    }
}

impl From<&flat_ast::Constant> for Constant {
    fn from(ast: &flat_ast::Constant) -> Self {
        Self {
            kind: ast.kind.clone(),
            value: serde_json::value::RawValue::from_string(ast.value.clone()).unwrap(),
            expression: serde_json::value::RawValue::from_string(ast.expression.clone()).unwrap(),
            identifier: ast.identifier.clone(),
            literal: ast.literal.as_ref().map(Into::into),
        }
    }
}

impl From<&flat_ast::Literal> for Literal {
    fn from(ast: &flat_ast::Literal) -> Self {
        Self {
            kind: ast.kind.clone(),
            value: serde_json::value::RawValue::from_string(ast.value.clone()).unwrap(),
            expression: serde_json::value::RawValue::from_string(ast.expression.clone()).unwrap(),
        }
    }
}

impl From<&flat_ast::AttributeArg> for AttributeArg {
    fn from(ast: &flat_ast::AttributeArg) -> Self {
        Self {
            name: ast.name.clone(),
            type_: ast.type_.clone(),
            value: (&ast.value).into(),
            location: (&ast.location).into(),
        }
    }
}

impl From<&flat_ast::Attribute> for Attribute {
    fn from(ast: &flat_ast::Attribute) -> Self {
        Self {
            name: ast.name.clone(),
            arguments: ast.arguments.iter().map(Into::into).collect(),
            location: (&ast.location).into(),
        }
    }
}

impl From<&flat_ast::ResourceProperty> for ResourceProperty {
    fn from(ast: &flat_ast::ResourceProperty) -> Self {
        Self {
            name: ast.name.clone(),
            location: (&ast.location).into(),
            deprecated: ast.deprecated,
            type_: (&ast.type_).into(),
        }
    }
}

impl From<&flat_ast::ExperimentalResourceDeclaration> for ExperimentalResourceDeclaration {
    fn from(ast: &flat_ast::ExperimentalResourceDeclaration) -> Self {
        Self {
            name: ast.name.clone(),
            location: (&ast.location).into(),
            deprecated: ast.deprecated,
            maybe_attributes: ast.maybe_attributes.iter().map(Into::into).collect(),
            type_: (&ast.type_).into(),
            properties: ast.properties.iter().map(Into::into).collect(),
        }
    }
}

impl From<&flat_ast::ProtocolDeclaration> for ProtocolDeclaration {
    fn from(ast: &flat_ast::ProtocolDeclaration) -> Self {
        Self {
            name: ast.name.clone(),
            location: (&ast.location).into(),
            deprecated: ast.deprecated,
            maybe_attributes: ast.maybe_attributes.iter().map(Into::into).collect(),
            openness: ast.openness.clone(),
            composed_protocols: ast.composed_protocols.iter().map(Into::into).collect(),
            methods: ast.methods.iter().map(Into::into).collect(),
            implementation_locations: ast.implementation_locations.clone(),
        }
    }
}

impl From<&flat_ast::ProtocolCompose> for ProtocolCompose {
    fn from(ast: &flat_ast::ProtocolCompose) -> Self {
        Self {
            name: ast.name.clone(),
            maybe_attributes: ast.maybe_attributes.iter().map(Into::into).collect(),
            location: (&ast.location).into(),
            deprecated: ast.deprecated,
        }
    }
}

impl From<&flat_ast::ProtocolMethod> for ProtocolMethod {
    fn from(ast: &flat_ast::ProtocolMethod) -> Self {
        Self {
            kind: ast.kind.clone(),
            ordinal: ast.ordinal,
            name: ast.name.clone(),
            strict: ast.strict,
            location: (&ast.location).into(),
            deprecated: ast.deprecated,
            has_request: ast.has_request,
            maybe_request_payload: ast.maybe_request_payload.as_ref().map(Into::into),
            maybe_attributes: ast.maybe_attributes.iter().map(Into::into).collect(),
            has_response: ast.has_response,
            maybe_response_payload: ast.maybe_response_payload.as_ref().map(Into::into),
            is_composed: ast.is_composed,
            has_error: ast.has_error,
            maybe_response_success_type: ast.maybe_response_success_type.as_ref().map(Into::into),
            maybe_response_err_type: ast.maybe_response_err_type.as_ref().map(Into::into),
        }
    }
}

impl From<&flat_ast::ServiceDeclaration> for ServiceDeclaration {
    fn from(ast: &flat_ast::ServiceDeclaration) -> Self {
        Self {
            name: ast.name.clone(),
            location: (&ast.location).into(),
            deprecated: ast.deprecated,
            maybe_attributes: ast.maybe_attributes.iter().map(Into::into).collect(),
            members: ast.members.iter().map(Into::into).collect(),
        }
    }
}

impl From<&flat_ast::ServiceMember> for ServiceMember {
    fn from(ast: &flat_ast::ServiceMember) -> Self {
        Self {
            type_: (&ast.type_).into(),
            name: ast.name.clone(),
            location: (&ast.location).into(),
            deprecated: ast.deprecated,
            maybe_attributes: ast.maybe_attributes.iter().map(Into::into).collect(),
        }
    }
}

impl From<&flat_ast::TableDeclaration> for TableDeclaration {
    fn from(ast: &flat_ast::TableDeclaration) -> Self {
        Self {
            name: ast.name.clone(),
            naming_context: ast.naming_context.clone(),
            location: (&ast.location).into(),
            deprecated: ast.deprecated,
            maybe_attributes: ast.maybe_attributes.iter().map(Into::into).collect(),
            members: ast.members.iter().map(Into::into).collect(),
            strict: ast.strict,
            resource: ast.resource,
            type_shape: (&ast.type_shape).into(),
        }
    }
}

impl From<&flat_ast::TableMember> for TableMember {
    fn from(ast: &flat_ast::TableMember) -> Self {
        Self {
            ordinal: ast.ordinal,
            reserved: ast.reserved,
            type_: ast.type_.as_ref().map(Into::into),
            experimental_maybe_from_alias: ast
                .experimental_maybe_from_alias
                .as_ref()
                .map(Into::into),
            name: ast.name.clone(),
            location: ast.location.as_ref().map(Into::into),
            deprecated: ast.deprecated,
            maybe_attributes: ast.maybe_attributes.iter().map(Into::into).collect(),
        }
    }
}

impl From<&flat_ast::UnionDeclaration> for UnionDeclaration {
    fn from(ast: &flat_ast::UnionDeclaration) -> Self {
        Self {
            name: ast.name.clone(),
            naming_context: ast.naming_context.clone(),
            location: (&ast.location).into(),
            deprecated: ast.deprecated,
            maybe_attributes: ast.maybe_attributes.iter().map(Into::into).collect(),
            members: ast.members.iter().map(Into::into).collect(),
            strict: ast.strict,
            resource: ast.resource,
            is_result: ast.is_result,
            type_shape: (&ast.type_shape).into(),
        }
    }
}

impl From<&flat_ast::UnionMember> for UnionMember {
    fn from(ast: &flat_ast::UnionMember) -> Self {
        Self {
            ordinal: ast.ordinal,
            reserved: ast.reserved,
            name: ast.name.clone(),
            type_: ast.type_.as_ref().map(Into::into),
            experimental_maybe_from_alias: ast
                .experimental_maybe_from_alias
                .as_ref()
                .map(Into::into),
            location: ast.location.as_ref().map(Into::into),
            deprecated: ast.deprecated,
            maybe_attributes: ast.maybe_attributes.iter().map(Into::into).collect(),
        }
    }
}

impl From<&flat_ast::AliasDeclaration> for AliasDeclaration {
    fn from(ast: &flat_ast::AliasDeclaration) -> Self {
        Self {
            name: ast.name.clone(),
            location: (&ast.location).into(),
            deprecated: ast.deprecated,
            maybe_attributes: ast.maybe_attributes.iter().map(Into::into).collect(),
            partial_type_ctor: (&ast.partial_type_ctor).into(),
            type_: (&ast.type_).into(),
        }
    }
}

impl From<&flat_ast::NewTypeDeclaration> for NewTypeDeclaration {
    fn from(ast: &flat_ast::NewTypeDeclaration) -> Self {
        Self {
            name: ast.name.clone(),
            location: (&ast.location).into(),
            deprecated: ast.deprecated,
            maybe_attributes: ast.maybe_attributes.iter().map(Into::into).collect(),
            type_: (&ast.type_).into(),
            experimental_maybe_from_alias: ast
                .experimental_maybe_from_alias
                .as_ref()
                .map(Into::into),
        }
    }
}

impl From<&flat_ast::DeclarationKind> for DeclarationKind {
    fn from(ast: &flat_ast::DeclarationKind) -> Self {
        match ast {
            flat_ast::DeclarationKind::Bits => Self::Bits,
            flat_ast::DeclarationKind::Const => Self::Const,
            flat_ast::DeclarationKind::Enum => Self::Enum,
            flat_ast::DeclarationKind::ExperimentalResource => Self::ExperimentalResource,
            flat_ast::DeclarationKind::Protocol => Self::Protocol,
            flat_ast::DeclarationKind::Service => Self::Service,
            flat_ast::DeclarationKind::Struct => Self::Struct,
            flat_ast::DeclarationKind::Table => Self::Table,
            flat_ast::DeclarationKind::Union => Self::Union,
            flat_ast::DeclarationKind::Overlay => Self::Overlay,
            flat_ast::DeclarationKind::Alias => Self::Alias,
            flat_ast::DeclarationKind::NewType => Self::NewType,
        }
    }
}
