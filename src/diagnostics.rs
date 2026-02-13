use crate::source_span::SourceSpan;

#[derive(Debug, Clone)]
pub struct Diagnostic<'a> {
    pub id: usize, // ErrorId
    pub message: String,
    pub span: Option<SourceSpan<'a>>,
}

// Error definitions
// We can use an enum or constants. Structs like C++ ErrorDef are nice.

pub struct ErrorDef {
    pub id: usize,
    pub msg: &'static str,
}

impl ErrorDef {
    pub const fn new(id: usize, msg: &'static str) -> Self {
        Self { id, msg }
    }
}

pub const ERR_INVALID_CHARACTER: ErrorDef = ErrorDef::new(1, "invalid character '{}'");
pub const ERR_UNEXPECTED_LINE_BREAK: ErrorDef = ErrorDef::new(2, "unexpected line-break in string literal");
pub const ERR_INVALID_ESCAPE_SEQUENCE: ErrorDef = ErrorDef::new(3, "invalid escape sequence '{}'");
pub const ERR_INVALID_HEX_DIGIT: ErrorDef = ErrorDef::new(4, "invalid hex digit '{}'");
pub const ERR_UNEXPECTED_CONTROL_CHARACTER: ErrorDef = ErrorDef::new(184, "unexpected control character in string literal; use the Unicode escape `\\u{{{:x}}}` instead");
pub const ERR_UNICODE_ESCAPE_MISSING_BRACES: ErrorDef = ErrorDef::new(185, "Unicode escape must use braces, like `\\u{{a}}` for U+000A");
pub const ERR_UNICODE_ESCAPE_UNTERMINATED: ErrorDef = ErrorDef::new(186, "Unicode escape is missing a closing brace '}}'");
pub const ERR_UNICODE_ESCAPE_EMPTY: ErrorDef = ErrorDef::new(187, "Unicode escape must have at least 1 hex digit");
pub const ERR_UNICODE_ESCAPE_TOO_LONG: ErrorDef = ErrorDef::new(188, "Unicode escape must have at most 6 hex digits");
pub const ERR_UNICODE_ESCAPE_TOO_LARGE: ErrorDef = ErrorDef::new(189, "invalid Unicode code point '{}'; maximum is 10FFFF");
