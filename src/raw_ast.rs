use crate::source_span::SourceSpan;
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct SourceElement<'a> {
    pub start_token: Token<'a>,
    pub end_token: Token<'a>,
}

impl<'a> SourceElement<'a> {
    pub fn new(start: Token<'a>, end: Token<'a>) -> Self {
        Self {
            start_token: start,
            end_token: end,
        }
    }

    pub fn span(&self) -> SourceSpan<'a> {
        // Assume start and end are from same file and valid.
        // Logic to combine spans.
        let start_data = self.start_token.span.data;
        let end_data = self.end_token.span.data;
        let start_ptr = start_data.as_ptr();
        let end_ptr = unsafe { end_data.as_ptr().add(end_data.len()) };
        let _len = unsafe { end_ptr.offset_from(start_ptr) } as usize;

        // Safety: assuming tokens are from the same buffer and ordered.
        // We can just query SourceFile for slice?
        // SourceSpan holds &str.
        // We need to construct a new &str from the range.
        // But SourceSpan::new takes &str.
        // We can use the underlying SourceFile to slice.
        // Or if we trust the ptr logic (which SourceSpan relies on string view).
        // Rust strings are standard utf8.
        // We need the full slice.

        // Better: implement `from_tokens` in SourceSpan or similar.
        // For now, let's just store tokens.
        // We can implement `span()` later properly.
        // Actually, we probably want to store the computed span if we use it often.
        // But C++ computes it on demand.
        self.start_token.span // Placeholder
    }
}

// We will use a trait or just struct composition.
// Rust composition:
// struct Identifier<'a> {
//    element: SourceElement<'a>,
//    ...
// }

#[derive(Debug, Clone)]
pub struct Identifier<'a> {
    pub element: SourceElement<'a>,
    // data is in the span
}

impl<'a> Identifier<'a> {
    pub fn data(&self) -> &'a str {
        self.element.start_token.span.data
    }
}

#[derive(Debug, Clone)]
pub struct CompoundIdentifier<'a> {
    pub element: SourceElement<'a>,
    pub components: Vec<Identifier<'a>>,
}

impl<'a> std::fmt::Display for CompoundIdentifier<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts: Vec<&str> = self.components.iter().map(|c| c.data()).collect();
        write!(f, "{}", parts.join("."))
    }
}

#[derive(Debug, Clone)]
pub struct LibraryDeclaration<'a> {
    pub element: SourceElement<'a>,
    pub attributes: Option<Box<AttributeList<'a>>>,
    pub path: CompoundIdentifier<'a>,
}

#[derive(Debug, Clone)]
pub struct AttributeList<'a> {
    pub element: SourceElement<'a>,
    pub attributes: Vec<Attribute<'a>>,
}

#[derive(Debug, Clone)]
pub struct Attribute<'a> {
    pub element: SourceElement<'a>,
    pub name: Identifier<'a>,
    pub args: Vec<AttributeArg<'a>>,
    pub provenance: AttributeProvenance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributeProvenance {
    Default,
    DocComment,
    ModifierAvailability,
}

#[derive(Debug, Clone)]
pub struct AttributeArg<'a> {
    pub element: SourceElement<'a>,
    pub name: Option<Identifier<'a>>, // inferred if null?
    pub value: Constant<'a>,
}

#[derive(Debug, Clone)]
pub enum Constant<'a> {
    Identifier(IdentifierConstant<'a>),
    Literal(LiteralConstant<'a>),
    BinaryOperator(BinaryOperatorConstant<'a>),
}

#[derive(Debug, Clone)]
pub struct IdentifierConstant<'a> {
    pub element: SourceElement<'a>,
    pub identifier: CompoundIdentifier<'a>,
}

#[derive(Debug, Clone)]
pub struct LiteralConstant<'a> {
    pub element: SourceElement<'a>,
    pub literal: Literal<'a>,
}

#[derive(Debug, Clone)]
pub struct BinaryOperatorConstant<'a> {
    pub element: SourceElement<'a>,
    pub left: Box<Constant<'a>>,
    pub right: Box<Constant<'a>>,
    pub op: BinaryOperator,
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOperator {
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralKind {
    Bool(bool),
    DocComment,
    Numeric,
    String,
}

#[derive(Debug, Clone)]
pub struct Literal<'a> {
    pub element: SourceElement<'a>,
    pub kind: LiteralKind,
    pub value: String, // Parsed value or just string repr?
                       // C++ extracts value for String/DocComment.
}

#[derive(Debug, Clone)]
pub struct ConstDeclaration<'a> {
    pub element: SourceElement<'a>,
    pub attributes: Option<Box<AttributeList<'a>>>,
    pub name: Identifier<'a>,
    pub type_ctor: TypeConstructor<'a>,
    pub value: Constant<'a>,
}

#[derive(Debug, Clone)]
pub struct TypeConstructor<'a> {
    pub element: SourceElement<'a>,
    pub layout: LayoutParameter<'a>,
    pub parameters: Vec<TypeConstructor<'a>>,
    pub constraints: Vec<Constant<'a>>,
    pub nullable: bool,
}

#[derive(Debug, Clone)]
pub enum LayoutParameter<'a> {
    Identifier(CompoundIdentifier<'a>),
    Literal(LiteralConstant<'a>),
    Type(Box<TypeConstructor<'a>>),
}

#[derive(Debug, Clone)]
pub struct File<'a> {
    pub element: SourceElement<'a>,
    pub library_decl: Option<Box<LibraryDeclaration<'a>>>,
    pub const_decls: Vec<ConstDeclaration<'a>>,
    pub type_decls: Vec<TypeDeclaration<'a>>, // New
    pub struct_decls: Vec<StructDeclaration<'a>>,
    pub enum_decls: Vec<EnumDeclaration<'a>>,
    pub bits_decls: Vec<BitsDeclaration<'a>>,
    pub union_decls: Vec<UnionDeclaration<'a>>,
    pub table_decls: Vec<TableDeclaration<'a>>,
    pub protocol_decls: Vec<ProtocolDeclaration<'a>>,
    pub service_decls: Vec<ServiceDeclaration<'a>>,
    pub tokens: Vec<Token<'a>>,
}

#[derive(Debug, Clone)]
pub struct TypeDeclaration<'a> {
    pub element: SourceElement<'a>,
    pub attributes: Option<Box<AttributeList<'a>>>,
    pub name: Identifier<'a>,
    pub layout: Layout<'a>,
}

#[derive(Debug, Clone)]
pub enum Layout<'a> {
    Struct(StructDeclaration<'a>),
    Enum(EnumDeclaration<'a>),
    Bits(BitsDeclaration<'a>),
    Union(UnionDeclaration<'a>),
    Table(TableDeclaration<'a>),
    TypeConstructor(TypeConstructor<'a>),
}

#[derive(Debug, Clone)]
pub struct StructDeclaration<'a> {
    pub element: SourceElement<'a>,
    pub attributes: Option<Box<AttributeList<'a>>>,
    pub is_resource: bool,
    pub name: Option<Identifier<'a>>, // Changed to Option
    pub members: Vec<StructMember<'a>>,
}

#[derive(Debug, Clone)]
pub struct StructMember<'a> {
    pub element: SourceElement<'a>,
    pub attributes: Option<Box<AttributeList<'a>>>,
    pub type_ctor: TypeConstructor<'a>,
    pub name: Identifier<'a>,
    pub default_value: Option<Constant<'a>>,
}

#[derive(Debug, Clone)]
pub struct EnumDeclaration<'a> {
    pub element: SourceElement<'a>,
    pub attributes: Option<Box<AttributeList<'a>>>,
    pub name: Option<Identifier<'a>>, // Changed to Option
    pub subtype: Option<TypeConstructor<'a>>,
    pub strictness: Option<Strictness>,
    pub members: Vec<EnumMember<'a>>,
}

#[derive(Debug, Clone)]
pub struct EnumMember<'a> {
    pub element: SourceElement<'a>,
    pub attributes: Option<Box<AttributeList<'a>>>,
    pub name: Identifier<'a>,
    pub value: Constant<'a>,
}

#[derive(Debug, Clone)]
pub struct BitsDeclaration<'a> {
    pub element: SourceElement<'a>,
    pub attributes: Option<Box<AttributeList<'a>>>,
    pub name: Option<Identifier<'a>>, // Changed to Option
    pub subtype: Option<TypeConstructor<'a>>,
    pub strictness: Option<Strictness>,
    pub members: Vec<BitsMember<'a>>,
}

#[derive(Debug, Clone)]
pub struct BitsMember<'a> {
    pub element: SourceElement<'a>,
    pub attributes: Option<Box<AttributeList<'a>>>,
    pub name: Identifier<'a>,
    pub value: Constant<'a>,
}

#[derive(Debug, Clone)]
pub struct UnionDeclaration<'a> {
    pub element: SourceElement<'a>,
    pub attributes: Option<Box<AttributeList<'a>>>,
    pub name: Option<Identifier<'a>>, // Changed to Option
    pub strictness: Strictness,
    pub is_resource: bool,
    pub members: Vec<UnionMember<'a>>,
}

#[derive(Debug, Clone)]
pub struct UnionMember<'a> {
    pub element: SourceElement<'a>,
    pub attributes: Option<Box<AttributeList<'a>>>,
    pub ordinal: Option<Literal<'a>>,
    pub name: Option<Identifier<'a>>, // Reserved members have no name
    pub type_ctor: Option<TypeConstructor<'a>>,
}

#[derive(Debug, Clone)]
pub struct TableDeclaration<'a> {
    pub element: SourceElement<'a>,
    pub attributes: Option<Box<AttributeList<'a>>>,
    pub name: Option<Identifier<'a>>, // Changed to Option
    pub is_resource: bool,
    pub members: Vec<TableMember<'a>>,
}

#[derive(Debug, Clone)]
pub struct TableMember<'a> {
    pub element: SourceElement<'a>,
    pub attributes: Option<Box<AttributeList<'a>>>,
    pub ordinal: Literal<'a>,
    pub name: Option<Identifier<'a>>,
    pub type_ctor: Option<TypeConstructor<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strictness {
    Strict,
    Flexible,
}

// Add to File

#[derive(Debug, Clone)]
pub struct ProtocolDeclaration<'a> {
    pub element: SourceElement<'a>,
    pub attributes: Option<Box<AttributeList<'a>>>,
    pub name: Identifier<'a>,
    pub methods: Vec<ProtocolMethod<'a>>,
}

#[derive(Debug, Clone)]
pub struct ProtocolMethod<'a> {
    pub element: SourceElement<'a>,
    pub attributes: Option<Box<AttributeList<'a>>>,
    pub name: Identifier<'a>,
    pub has_request: bool,
    pub request_payload: Option<Layout<'a>>,
    pub has_response: bool,
    pub response_payload: Option<Layout<'a>>,
    pub has_error: bool,
    pub error_payload: Option<Layout<'a>>,
}

#[derive(Debug, Clone)]
pub struct ServiceDeclaration<'a> {
    pub element: SourceElement<'a>,
    pub attributes: Option<Box<AttributeList<'a>>>,
    pub name: Identifier<'a>,
    pub members: Vec<ServiceMember<'a>>,
}

#[derive(Debug, Clone)]
pub struct ServiceMember<'a> {
    pub element: SourceElement<'a>,
    pub attributes: Option<Box<AttributeList<'a>>>,
    pub type_ctor: TypeConstructor<'a>,
    pub name: Identifier<'a>,
}
