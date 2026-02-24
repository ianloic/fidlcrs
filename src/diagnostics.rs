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
pub const ERR_UNEXPECTED_LINE_BREAK: ErrorDef =
    ErrorDef::new(2, "unexpected line-break in string literal");
pub const ERR_INVALID_ESCAPE_SEQUENCE: ErrorDef = ErrorDef::new(3, "invalid escape sequence '{}'");
pub const ERR_INVALID_HEX_DIGIT: ErrorDef = ErrorDef::new(4, "invalid hex digit '{}'");
pub const ERR_UNEXPECTED_CONTROL_CHARACTER: ErrorDef = ErrorDef::new(
    184,
    "unexpected control character in string literal; use the Unicode escape `\\u{{{:x}}}` instead",
);
pub const ERR_UNICODE_ESCAPE_MISSING_BRACES: ErrorDef = ErrorDef::new(
    185,
    "Unicode escape must use braces, like `\\u{{a}}` for U+000A",
);
pub const ERR_UNICODE_ESCAPE_UNTERMINATED: ErrorDef =
    ErrorDef::new(186, "Unicode escape is missing a closing brace '}}'");
pub const ERR_UNICODE_ESCAPE_EMPTY: ErrorDef =
    ErrorDef::new(187, "Unicode escape must have at least 1 hex digit");
pub const ERR_UNICODE_ESCAPE_TOO_LONG: ErrorDef =
    ErrorDef::new(188, "Unicode escape must have at most 6 hex digits");
pub const ERR_UNICODE_ESCAPE_TOO_LARGE: ErrorDef =
    ErrorDef::new(189, "invalid Unicode code point '{}'; maximum is 10FFFF");

pub const ERR_NULLABLE_ARRAY: ErrorDef = ErrorDef::new(62, "arrays cannot be nullable");
pub const ERR_ARRAY_SIZE_ZERO: ErrorDef = ErrorDef::new(161, "arrays cannot have size 0");
pub const ERR_ARRAY_CONSTRAINT: ErrorDef = ErrorDef::new(1001, "arrays cannot have constraints");
pub const ERR_EXPECTED_TYPE: ErrorDef = ErrorDef::new(1002, "expected type");
pub const ERR_EXPECTED_VALUE: ErrorDef = ErrorDef::new(1003, "expected value");
pub const ERR_WRONG_NUMBER_OF_LAYOUT_PARAMETERS: ErrorDef = ErrorDef::new(1004, "wrong number of layout parameters");

pub const ERR_BITS_MEMBER_MUST_BE_POWER_OF_TWO: ErrorDef = ErrorDef::new(1005, "bits member must be power of two");
pub const ERR_BITS_MEMBER_DUPLICATE_NAME: ErrorDef = ErrorDef::new(1006, "bits member name duplicated");
pub const ERR_BITS_MEMBER_DUPLICATE_VALUE: ErrorDef = ErrorDef::new(1007, "bits member value duplicated");
pub const ERR_BITS_TYPE_MUST_BE_UNSIGNED: ErrorDef = ErrorDef::new(1008, "bits type must be an unsigned integer");
pub const ERR_CANNOT_BE_NULLABLE: ErrorDef = ErrorDef::new(1009, "value cannot be nullable");
pub const ERR_CANNOT_HAVE_CONSTRAINTS: ErrorDef = ErrorDef::new(1010, "value cannot have constraints");
pub const ERR_STRICT_BITS_MUST_HAVE_MEMBERS: ErrorDef = ErrorDef::new(1011, "strict bits must have at least one member");
pub const ERR_MEMBER_OVERFLOW: ErrorDef = ErrorDef::new(1012, "member value overflows its underlying type");
pub const ERR_INVALID_MEMBER_VALUE: ErrorDef = ErrorDef::new(1013, "invalid or unparseable member value");
