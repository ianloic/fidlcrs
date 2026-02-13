use crate::source_span::SourceSpan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    NotAToken,
    EndOfFile,
    StartOfFile,
    Comment,
    DocComment,
    Identifier,
    NumericLiteral,
    StringLiteral,
    LeftParen,
    RightParen,
    LeftSquare,
    RightSquare,
    LeftCurly,
    RightCurly,
    LeftAngle,
    RightAngle,
    At,
    Dot,
    Comma,
    Semicolon,
    Colon,
    Question,
    Equal,
    Ampersand,
    Arrow,
    Pipe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenSubkind {
    None,
    As,
    Alias,
    Library,
    Using,
    Array,
    Request,
    String,
    StringArray,
    Vector,
    Ajar,
    Bits,
    Closed,
    Const,
    Enum,
    Open,
    Protocol,
    Resource,
    ResourceDefinition,
    Service,
    Strict,
    Struct,
    Table,
    Flexible,
    Type,
    Union,
    Overlay,
    Error,
    True,
    False,
    Compose,
    Reserved,
    Properties,
}

#[derive(Clone, Debug)]
pub struct Token<'a> {
    pub span: SourceSpan<'a>,
    pub kind: TokenKind,
    pub subkind: TokenSubkind,
    pub leading_newlines: u16,
}

impl<'a> Token<'a> {
    pub fn new(span: SourceSpan<'a>, kind: TokenKind, subkind: TokenSubkind, leading_newlines: u16) -> Self {
        Self {
            span,
            kind,
            subkind,
            leading_newlines,
        }
    }
}
