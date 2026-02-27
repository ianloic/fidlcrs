use crate::source_span::SourceSpan;

#[derive(Debug, Clone)]
pub struct Diagnostic<'a> {
    pub id: usize, // ErrorId
    pub message: String,
    pub span: Option<SourceSpan<'a>>,
}

pub struct ErrorDef {
    pub id: usize,
    pub msg: &'static str,
}

impl ErrorDef {
    pub const fn new(id: usize, msg: &'static str) -> Self {
        Self { id, msg }
    }
}

#[allow(dead_code)]
pub const ERR_INVALID_CHARACTER: ErrorDef = ErrorDef::new(1, r#"invalid character '{}'"#);
#[allow(dead_code)]
pub const ERR_UNEXPECTED_LINE_BREAK: ErrorDef =
    ErrorDef::new(2, r#"unexpected line-break in string literal"#);
#[allow(dead_code)]
pub const ERR_INVALID_ESCAPE_SEQUENCE: ErrorDef =
    ErrorDef::new(3, r#"invalid escape sequence '{}'"#);
#[allow(dead_code)]
pub const ERR_INVALID_HEX_DIGIT: ErrorDef = ErrorDef::new(4, r#"invalid hex digit '{}'"#);
// ERR_INVALID_OCT_DIGIT is retired (id: 5)
#[allow(dead_code)]
pub const ERR_EXPECTED_DECLARATION: ErrorDef = ErrorDef::new(6, r#"invalid declaration type {}"#);
#[allow(dead_code)]
pub const ERR_UNEXPECTED_TOKEN: ErrorDef = ErrorDef::new(7, r#"found unexpected token"#);
#[allow(dead_code)]
pub const ERR_UNEXPECTED_TOKEN_OF_KIND: ErrorDef =
    ErrorDef::new(8, r#"unexpected token {}, was expecting {}"#);
#[allow(dead_code)]
pub const ERR_UNEXPECTED_IDENTIFIER: ErrorDef =
    ErrorDef::new(9, r#"unexpected identifier {}, was expecting {}"#);
#[allow(dead_code)]
pub const ERR_INVALID_IDENTIFIER: ErrorDef = ErrorDef::new(10, r#"invalid identifier '{}'"#);
#[allow(dead_code)]
pub const ERR_INVALID_LIBRARY_NAME_COMPONENT: ErrorDef =
    ErrorDef::new(11, r#"Invalid library name component {}"#);
#[allow(dead_code)]
pub const ERR_INVALID_LAYOUT_CLASS: ErrorDef = ErrorDef::new(
    12,
    r#"layouts must be of the class: bits, enum, struct, table, or union."#,
);
#[allow(dead_code)]
pub const ERR_INVALID_WRAPPED_TYPE: ErrorDef =
    ErrorDef::new(13, r#"wrapped type for bits/enum must be an identifier"#);
#[allow(dead_code)]
pub const ERR_ATTRIBUTE_WITH_EMPTY_PARENS: ErrorDef = ErrorDef::new(
    14,
    r#"attributes without arguments must omit the trailing empty parentheses"#,
);
#[allow(dead_code)]
pub const ERR_ATTRIBUTE_ARGS_MUST_ALL_BE_NAMED: ErrorDef = ErrorDef::new(
    15,
    r#"attributes that take multiple arguments must name all of them explicitly"#,
);
#[allow(dead_code)]
pub const ERR_MISSING_ORDINAL_BEFORE_MEMBER: ErrorDef =
    ErrorDef::new(16, r#"missing ordinal before member"#);
#[allow(dead_code)]
pub const ERR_ORDINAL_OUT_OF_BOUND: ErrorDef = ErrorDef::new(17, r#"ordinal out-of-bound"#);
#[allow(dead_code)]
pub const ERR_ORDINALS_MUST_START_AT_ONE: ErrorDef =
    ErrorDef::new(18, r#"ordinals must start at 1"#);
#[allow(dead_code)]
pub const ERR_MUST_HAVE_ONE_MEMBER: ErrorDef =
    ErrorDef::new(19, r#"must have at least one member"#);
#[allow(dead_code)]
pub const ERR_INVALID_PROTOCOL_MEMBER: ErrorDef = ErrorDef::new(20, r#"invalid protocol member"#);
// ERR_EXPECTED_PROTOCOL_MEMBER is retired (id: 21)
#[allow(dead_code)]
pub const ERR_CANNOT_ATTACH_ATTRIBUTE_TO_IDENTIFIER: ErrorDef =
    ErrorDef::new(22, r#"cannot attach attributes to identifiers"#);
#[allow(dead_code)]
pub const ERR_ATTRIBUTE_INSIDE_TYPE_DECLARATION: ErrorDef = ErrorDef::new(
    23,
    r#"attributes are not allowed here; put the attribute before the `type` keyword instead"#,
);
#[allow(dead_code)]
pub const ERR_DOC_COMMENT_ON_PARAMETERS: ErrorDef =
    ErrorDef::new(24, r#"cannot have doc comment on parameters"#);
#[allow(dead_code)]
pub const ERR_LIBRARY_IMPORTS_MUST_BE_GROUPED_AT_TOP_OF_FILE: ErrorDef =
    ErrorDef::new(25, r#"library imports must be grouped at top-of-file"#);
#[allow(dead_code)]
pub const WARN_COMMENT_WITHIN_DOC_COMMENT_BLOCK: ErrorDef =
    ErrorDef::new(26, r#"cannot have comment within doc comment block"#);
#[allow(dead_code)]
pub const WARN_BLANK_LINES_WITHIN_DOC_COMMENT_BLOCK: ErrorDef =
    ErrorDef::new(27, r#"cannot have blank lines within doc comment block"#);
#[allow(dead_code)]
pub const WARN_DOC_COMMENT_MUST_BE_FOLLOWED_BY_DECLARATION: ErrorDef =
    ErrorDef::new(28, r#"doc comment must be followed by a declaration"#);
#[allow(dead_code)]
pub const ERR_MUST_HAVE_ONE_PROPERTY: ErrorDef =
    ErrorDef::new(29, r#"must have at least one property"#);
#[allow(dead_code)]
pub const ERR_CANNOT_SPECIFY_MODIFIER: ErrorDef =
    ErrorDef::new(30, r#"cannot specify modifier {} for {}"#);
#[allow(dead_code)]
pub const ERR_CANNOT_SPECIFY_SUBTYPE: ErrorDef =
    ErrorDef::new(31, r#"cannot specify subtype for {}"#);
#[allow(dead_code)]
pub const ERR_DUPLICATE_MODIFIER: ErrorDef = ErrorDef::new(32, r#"duplicate occurrence of {}"#);
#[allow(dead_code)]
pub const ERR_CONFLICTING_MODIFIER: ErrorDef = ErrorDef::new(33, r#"{} conflicts with {}"#);
#[allow(dead_code)]
pub const ERR_NAME_COLLISION: ErrorDef =
    ErrorDef::new(34, r#"{} '{}' has the same name as the {} declared at {}"#);
#[allow(dead_code)]
pub const ERR_NAME_COLLISION_CANONICAL: ErrorDef = ErrorDef::new(
    35,
    r#"{} '{}' conflicts with {} '{}' declared at {}; both names are represented by the canonical form '{}'"#,
);
#[allow(dead_code)]
pub const ERR_NAME_OVERLAP: ErrorDef = ErrorDef::new(
    36,
    r#"{} '{}' has the same name as the {} declared at {}; both are available {} of platform '{}'"#,
);
#[allow(dead_code)]
pub const ERR_NAME_OVERLAP_CANONICAL: ErrorDef = ErrorDef::new(
    37,
    r#"{} '{}' conflicts with {} '{}' declared at {}; both names are represented by the canonical form '{}' and are available {} of platform '{}'"#,
);
#[allow(dead_code)]
pub const ERR_DECL_NAME_CONFLICTS_WITH_LIBRARY_IMPORT: ErrorDef = ErrorDef::new(
    38,
    r#"Declaration name '{}' conflicts with a library import. Consider using the 'as' keyword to import the library under a different name."#,
);
#[allow(dead_code)]
pub const ERR_DECL_NAME_CONFLICTS_WITH_LIBRARY_IMPORT_CANONICAL: ErrorDef = ErrorDef::new(
    39,
    r#"Declaration name '{}' conflicts with a library import due to its canonical form '{}'. Consider using the 'as' keyword to import the library under a different name."#,
);
#[allow(dead_code)]
pub const ERR_FILES_DISAGREE_ON_LIBRARY_NAME: ErrorDef = ErrorDef::new(
    40,
    r#"Two files in the library disagree about the name of the library"#,
);
#[allow(dead_code)]
pub const ERR_MULTIPLE_LIBRARIES_WITH_SAME_NAME: ErrorDef =
    ErrorDef::new(41, r#"There are multiple libraries named '{}'"#);
#[allow(dead_code)]
pub const ERR_DUPLICATE_LIBRARY_IMPORT: ErrorDef = ErrorDef::new(
    42,
    r#"Library {} already imported. Did you require it twice?"#,
);
#[allow(dead_code)]
pub const ERR_CONFLICTING_LIBRARY_IMPORT: ErrorDef = ErrorDef::new(
    43,
    r#"import of library '{}' conflicts with another library import"#,
);
#[allow(dead_code)]
pub const ERR_CONFLICTING_LIBRARY_IMPORT_ALIAS: ErrorDef = ErrorDef::new(
    44,
    r#"import of library '{}' under alias '{}' conflicts with another library import"#,
);
#[allow(dead_code)]
pub const ERR_ATTRIBUTES_NOT_ALLOWED_ON_LIBRARY_IMPORT: ErrorDef = ErrorDef::new(
    45,
    r#"attributes and doc comments are not allowed on `using` statements"#,
);
#[allow(dead_code)]
pub const ERR_UNKNOWN_LIBRARY: ErrorDef = ErrorDef::new(
    46,
    r#"Could not find library named {}. Did you include its sources with --files?"#,
);
// ERR_PROTOCOL_COMPOSED_MULTIPLE_TIMES is retired (id: 47)
#[allow(dead_code)]
pub const ERR_OPTIONAL_TABLE_MEMBER: ErrorDef =
    ErrorDef::new(48, r#"Table members cannot be optional"#);
#[allow(dead_code)]
pub const ERR_OPTIONAL_UNION_MEMBER: ErrorDef =
    ErrorDef::new(49, r#"Union members cannot be optional"#);
#[allow(dead_code)]
pub const ERR_DEPRECATED_STRUCT_DEFAULTS: ErrorDef = ErrorDef::new(
    50,
    r#"Struct defaults are deprecated and should not be used (see RFC-0160)"#,
);
#[allow(dead_code)]
pub const ERR_UNKNOWN_DEPENDENT_LIBRARY: ErrorDef = ErrorDef::new(
    51,
    r#"Unknown dependent library {} or reference to member of library {}. Did you require it with `using`?"#,
);
#[allow(dead_code)]
pub const ERR_NAME_NOT_FOUND: ErrorDef = ErrorDef::new(52, r#"cannot find '{}' in {}"#);
#[allow(dead_code)]
pub const ERR_CANNOT_REFER_TO_MEMBER: ErrorDef =
    ErrorDef::new(53, r#"cannot refer to member of {}"#);
#[allow(dead_code)]
pub const ERR_MEMBER_NOT_FOUND: ErrorDef = ErrorDef::new(54, r#"{} has no member '{}'"#);
#[allow(dead_code)]
pub const ERR_INVALID_REFERENCE_TO_DEPRECATED: ErrorDef = ErrorDef::new(
    55,
    r#"invalid reference to {}, which is deprecated {} of platform '{}' while {} is not; either remove this reference or mark {} as deprecated"#,
);
#[allow(dead_code)]
pub const ERR_INVALID_REFERENCE_TO_DEPRECATED_OTHER_PLATFORM: ErrorDef = ErrorDef::new(
    56,
    r#"invalid reference to {}, which is deprecated {} of platform '{}' while {} is not deprecated {} of platform '{}'; either remove this reference or mark {} as deprecated"#,
);
#[allow(dead_code)]
pub const ERR_ANONYMOUS_NAME_REFERENCE: ErrorDef =
    ErrorDef::new(58, r#"cannot refer to anonymous name {}"#);
#[allow(dead_code)]
pub const ERR_INVALID_CONSTANT_TYPE: ErrorDef = ErrorDef::new(59, r#"invalid constant type {}"#);
#[allow(dead_code)]
pub const ERR_CANNOT_RESOLVE_CONSTANT_VALUE: ErrorDef =
    ErrorDef::new(60, r#"unable to resolve constant value"#);
#[allow(dead_code)]
pub const ERR_OR_OPERATOR_ON_NON_PRIMITIVE_VALUE: ErrorDef = ErrorDef::new(
    61,
    r#"Or operator can only be applied to primitive-kinded values"#,
);
#[allow(dead_code)]
pub const ERR_NEW_TYPES_NOT_ALLOWED: ErrorDef = ErrorDef::new(
    62,
    r#"newtypes not allowed: type declaration {} defines a new type of the existing {} type, which is not yet supported"#,
);
#[allow(dead_code)]
pub const ERR_EXPECTED_VALUE_BUT_GOT_TYPE: ErrorDef =
    ErrorDef::new(63, r#"{} is a type, but a value was expected"#);
#[allow(dead_code)]
pub const ERR_MISMATCHED_NAME_TYPE_ASSIGNMENT: ErrorDef = ErrorDef::new(
    64,
    r#"mismatched named type assignment: cannot define a constant or default value of type {} using a value of type {}"#,
);
#[allow(dead_code)]
pub const ERR_TYPE_CANNOT_BE_CONVERTED_TO_TYPE: ErrorDef =
    ErrorDef::new(65, r#"{} (type {}) cannot be converted to type {}"#);
#[allow(dead_code)]
pub const ERR_CONSTANT_OVERFLOWS_TYPE: ErrorDef = ErrorDef::new(66, r#"{} overflows type {}"#);
#[allow(dead_code)]
pub const ERR_BITS_MEMBER_MUST_BE_POWER_OF_TWO: ErrorDef =
    ErrorDef::new(67, r#"bits members must be powers of two"#);
#[allow(dead_code)]
pub const ERR_FLEXIBLE_ENUM_MEMBER_WITH_MAX_VALUE: ErrorDef = ErrorDef::new(
    68,
    r#"flexible enums must not have a member with a value of {}, which is reserved for the unknown value. either: remove the member, change its value to something else, or explicitly specify the unknown value with the @unknown attribute. see <https://fuchsia.dev/fuchsia-src/reference/fidl/language/attributes#unknown> for more info."#,
);
#[allow(dead_code)]
pub const ERR_BITS_TYPE_MUST_BE_UNSIGNED_INTEGRAL_PRIMITIVE: ErrorDef = ErrorDef::new(
    69,
    r#"bits may only be of unsigned integral primitive type, found {}"#,
);
#[allow(dead_code)]
pub const ERR_ENUM_TYPE_MUST_BE_INTEGRAL_PRIMITIVE: ErrorDef = ErrorDef::new(
    70,
    r#"enums may only be of integral primitive type, found {}"#,
);
#[allow(dead_code)]
pub const ERR_UNKNOWN_ATTRIBUTE_ON_STRICT_ENUM_MEMBER: ErrorDef = ErrorDef::new(
    71,
    r#"the @unknown attribute can be only be used on flexible enum members."#,
);
#[allow(dead_code)]
pub const ERR_UNKNOWN_ATTRIBUTE_ON_MULTIPLE_ENUM_MEMBERS: ErrorDef = ErrorDef::new(
    72,
    r#"the @unknown attribute can be only applied to one enum member."#,
);
#[allow(dead_code)]
pub const ERR_COMPOSING_NON_PROTOCOL: ErrorDef =
    ErrorDef::new(73, r#"This declaration is not a protocol"#);
#[allow(dead_code)]
pub const ERR_INVALID_METHOD_PAYLOAD_LAYOUT_CLASS: ErrorDef = ErrorDef::new(
    74,
    r#"cannot use {} as a request/response; must use a struct, table, or union"#,
);
#[allow(dead_code)]
pub const ERR_INVALID_METHOD_PAYLOAD_TYPE: ErrorDef = ErrorDef::new(
    75,
    r#"invalid request/response type '{}'; must use a struct, table, or union"#,
);
// ERR_RESPONSES_WITH_ERRORS_MUST_NOT_BE_EMPTY is retired (id: 76)
#[allow(dead_code)]
pub const ERR_EMPTY_PAYLOAD_STRUCTS: ErrorDef = ErrorDef::new(
    77,
    r#"(struct {}) is not allowed as a request or response, use () instead"#,
);
// ERR_DUPLICATE_ELEMENT_NAME is retired (id: 78)
// ERR_DUPLICATE_ELEMENT_NAME_CANONICAL is retired (id: 79)
#[allow(dead_code)]
pub const ERR_GENERATED_ZERO_VALUE_ORDINAL: ErrorDef =
    ErrorDef::new(80, r#"Ordinal value 0 disallowed."#);
#[allow(dead_code)]
pub const ERR_DUPLICATE_METHOD_ORDINAL: ErrorDef = ErrorDef::new(
    81,
    r#"Multiple methods with the same ordinal in a protocol; previous was at {}."#,
);
#[allow(dead_code)]
pub const ERR_INVALID_SELECTOR_VALUE: ErrorDef = ErrorDef::new(
    82,
    r#"invalid selector value, must be a method name or a fully qualified method name"#,
);
#[allow(dead_code)]
pub const ERR_FUCHSIA_IO_EXPLICIT_ORDINALS: ErrorDef = ErrorDef::new(
    83,
    r#"fuchsia.io must have explicit ordinals (https://fxbug.dev/42157659)"#,
);
#[allow(dead_code)]
pub const ERR_PAYLOAD_STRUCT_HAS_DEFAULT_MEMBERS: ErrorDef = ErrorDef::new(
    84,
    r#"default values are not allowed on members of request/response structs"#,
);
// ERR_DUPLICATE_SERVICE_MEMBER_NAME is retired (id: 85)
// ERR_STRICT_UNION_MUST_HAVE_NON_RESERVED_MEMBER is retired (id: 86)
// ERR_DUPLICATE_SERVICE_MEMBER_NAME_CANONICAL is retired (id: 87)
#[allow(dead_code)]
pub const ERR_OPTIONAL_SERVICE_MEMBER: ErrorDef =
    ErrorDef::new(88, r#"service members cannot be optional"#);
// ERR_DUPLICATE_STRUCT_MEMBER_NAME is retired (id: 89)
// ERR_DUPLICATE_STRUCT_MEMBER_NAME_CANONICAL is retired (id: 90)
#[allow(dead_code)]
pub const ERR_INVALID_STRUCT_MEMBER_TYPE: ErrorDef =
    ErrorDef::new(91, r#"struct field {} has an invalid default type {}"#);
#[allow(dead_code)]
pub const ERR_TABLE_ORDINAL_TOO_LARGE: ErrorDef = ErrorDef::new(
    92,
    r#"ordinal is too large; table ordinals cannot be greater than 64"#,
);
#[allow(dead_code)]
pub const ERR_MAX_ORDINAL_NOT_TABLE: ErrorDef = ErrorDef::new(
    93,
    r#"the 64th ordinal of a table may only contain a table type"#,
);
#[allow(dead_code)]
pub const ERR_DUPLICATE_TABLE_FIELD_ORDINAL: ErrorDef = ErrorDef::new(
    94,
    r#"multiple table fields with the same ordinal; previous was at {}"#,
);
// ERR_DUPLICATE_TABLE_FIELD_NAME is retired (id: 95)
// ERR_DUPLICATE_TABLE_FIELD_NAME_CANONICAL is retired (id: 96)
#[allow(dead_code)]
pub const ERR_DUPLICATE_UNION_MEMBER_ORDINAL: ErrorDef = ErrorDef::new(
    97,
    r#"multiple union fields with the same ordinal; previous was at {}"#,
);
// ERR_DUPLICATE_UNION_MEMBER_NAME is retired (id: 98)
// ERR_DUPLICATE_UNION_MEMBER_NAME_CANONICAL is retired (id: 99)
// ERR_NON_DENSE_ORDINAL is retired (id: 100)
#[allow(dead_code)]
pub const ERR_COULD_NOT_RESOLVE_SIZE_BOUND: ErrorDef =
    ErrorDef::new(101, r#"unable to resolve size bound"#);
#[allow(dead_code)]
pub const ERR_COULD_NOT_RESOLVE_MEMBER: ErrorDef =
    ErrorDef::new(102, r#"unable to resolve {} member"#);
#[allow(dead_code)]
pub const ERR_COULD_NOT_RESOLVE_MEMBER_DEFAULT: ErrorDef =
    ErrorDef::new(103, r#"unable to resolve {} default value"#);
#[allow(dead_code)]
pub const ERR_COULD_NOT_RESOLVE_ATTRIBUTE_ARG: ErrorDef =
    ErrorDef::new(104, r#"unable to resolve attribute argument"#);
// ERR_DUPLICATE_MEMBER_NAME is retired (id: 105)
// ERR_DUPLICATE_MEMBER_NAME_CANONICAL is retired (id: 106)
#[allow(dead_code)]
pub const ERR_DUPLICATE_MEMBER_VALUE: ErrorDef = ErrorDef::new(
    107,
    r#"value of {} member '{}' conflicts with previously declared member '{}' at {}"#,
);
// ERR_DUPLICATE_RESOURCE_PROPERTY_NAME is retired (id: 108)
// ERR_DUPLICATE_RESOURCE_PROPERTY_NAME_CANONICAL is retired (id: 109)
#[allow(dead_code)]
pub const ERR_TYPE_MUST_BE_RESOURCE: ErrorDef = ErrorDef::new(
    110,
    r#"{} '{}' may contain handles (due to member '{}' at {}), so it must be declared with the `resource` modifier: `resource {} {}`"#,
);
#[allow(dead_code)]
pub const ERR_INLINE_SIZE_EXCEEDS_LIMIT: ErrorDef = ErrorDef::new(
    111,
    r#"'{}' has an inline size of {} bytes, which exceeds the maximum allowed inline size of {} bytes"#,
);
#[allow(dead_code)]
pub const ERR_ONLY_CLIENT_ENDS_IN_SERVICES: ErrorDef =
    ErrorDef::new(112, r#"service members must be client_end:P"#);
#[allow(dead_code)]
pub const ERR_MISMATCHED_TRANSPORT_IN_SERVICES: ErrorDef = ErrorDef::new(
    113,
    r#"service member {} is over the {} transport, but member {} is over the {} transport. Multiple transports are not allowed."#,
);
#[allow(dead_code)]
pub const ERR_COMPOSED_PROTOCOL_TOO_OPEN: ErrorDef = ErrorDef::new(
    114,
    r#"{} protocol '{}' cannot compose {} protocol '{}'; composed protocol may not be more open than composing protocol"#,
);
#[allow(dead_code)]
pub const ERR_FLEXIBLE_TWO_WAY_METHOD_REQUIRES_OPEN_PROTOCOL: ErrorDef = ErrorDef::new(
    115,
    r#"flexible two-way method may only be defined in an open protocol, not {}"#,
);
#[allow(dead_code)]
pub const ERR_FLEXIBLE_ONE_WAY_METHOD_IN_CLOSED_PROTOCOL: ErrorDef = ErrorDef::new(
    116,
    r#"flexible {} may only be defined in an open or ajar protocol, not closed"#,
);
#[allow(dead_code)]
pub const ERR_HANDLE_USED_IN_INCOMPATIBLE_TRANSPORT: ErrorDef = ErrorDef::new(
    117,
    r#"handle of type {} may not be sent over transport {} used by {}"#,
);
#[allow(dead_code)]
pub const ERR_TRANSPORT_END_USED_IN_INCOMPATIBLE_TRANSPORT: ErrorDef = ErrorDef::new(
    118,
    r#"client_end / server_end of transport type {} may not be sent over transport {} used by {}"#,
);
// ERR_EVENT_ERROR_SYNTAX is retired (id: 119)
#[allow(dead_code)]
pub const ERR_INVALID_ATTRIBUTE_PLACEMENT: ErrorDef =
    ErrorDef::new(120, r#"placement of attribute '{}' disallowed here"#);
#[allow(dead_code)]
pub const ERR_DEPRECATED_ATTRIBUTE: ErrorDef =
    ErrorDef::new(121, r#"attribute '{}' is deprecated"#);
#[allow(dead_code)]
pub const ERR_DUPLICATE_ATTRIBUTE: ErrorDef =
    ErrorDef::new(122, r#"duplicate attribute '{}'; previous was at {}"#);
#[allow(dead_code)]
pub const ERR_DUPLICATE_ATTRIBUTE_CANONICAL: ErrorDef = ErrorDef::new(
    123,
    r#"attribute '{}' conflicts with attribute '{}' from {}; both are represented by the canonical form '{}'"#,
);
#[allow(dead_code)]
pub const ERR_CAN_ONLY_USE_STRING_OR_BOOL: ErrorDef = ErrorDef::new(
    124,
    r#"argument '{}' on user-defined attribute '{}' cannot be a numeric value; use a bool or string instead"#,
);
#[allow(dead_code)]
pub const ERR_ATTRIBUTE_ARG_MUST_NOT_BE_NAMED: ErrorDef = ErrorDef::new(
    125,
    r#"attributes that take a single argument must not name that argument"#,
);
#[allow(dead_code)]
pub const ERR_ATTRIBUTE_ARG_NOT_NAMED: ErrorDef = ErrorDef::new(
    126,
    r#"attributes that take multiple arguments must name all of them explicitly, but '{}' was not"#,
);
#[allow(dead_code)]
pub const ERR_MISSING_REQUIRED_ATTRIBUTE_ARG: ErrorDef = ErrorDef::new(
    127,
    r#"attribute '{}' is missing the required '{}' argument"#,
);
#[allow(dead_code)]
pub const ERR_MISSING_REQUIRED_ANONYMOUS_ATTRIBUTE_ARG: ErrorDef =
    ErrorDef::new(128, r#"attribute '{}' is missing its required argument"#);
#[allow(dead_code)]
pub const ERR_UNKNOWN_ATTRIBUTE_ARG: ErrorDef =
    ErrorDef::new(129, r#"attribute '{}' does not support the '{}' argument"#);
#[allow(dead_code)]
pub const ERR_DUPLICATE_ATTRIBUTE_ARG: ErrorDef = ErrorDef::new(
    130,
    r#"attribute '{}' provides the '{}' argument multiple times; previous was at {}"#,
);
#[allow(dead_code)]
pub const ERR_DUPLICATE_ATTRIBUTE_ARG_CANONICAL: ErrorDef = ErrorDef::new(
    131,
    r#"attribute '{}' argument '{}' conflicts with argument '{}' from {}; both are represented by the canonical form '{}'"#,
);
#[allow(dead_code)]
pub const ERR_ATTRIBUTE_DISALLOWS_ARGS: ErrorDef =
    ErrorDef::new(132, r#"attribute '{}' does not support arguments"#);
#[allow(dead_code)]
pub const ERR_ATTRIBUTE_ARG_REQUIRES_LITERAL: ErrorDef = ErrorDef::new(
    133,
    r#"argument '{}' of attribute '{}' does not support referencing constants; please use a literal instead"#,
);
// ERR_ATTRIBUTE_CONSTRAINT_NOT_SATISFIED is retired (id: 134)
#[allow(dead_code)]
pub const ERR_INVALID_DISCOVERABLE_NAME: ErrorDef = ErrorDef::new(
    135,
    r#"invalid @discoverable name '{}'; must follow the format 'the.library.name.TheProtocolName'"#,
);
// ERR_TABLE_CANNOT_BE_SIMPLE is retired (id: 136)
// ERR_UNION_CANNOT_BE_SIMPLE is retired (id: 137)
// ERR_ELEMENT_MUST_BE_SIMPLE is retired (id: 138)
// ERR_TOO_MANY_BYTES is retired (id: 139)
// ERR_TOO_MANY_HANDLES is retired (id: 140)
#[allow(dead_code)]
pub const ERR_INVALID_ERROR_TYPE: ErrorDef = ErrorDef::new(
    141,
    r#"invalid error type: must be int32, uint32 or an enum thereof"#,
);
// ERR_BOUND_IS_TOO_BIG is retired (id: 143)
// ERR_UNABLE_TO_PARSE_BOUND is retired (id: 144)
#[allow(dead_code)]
pub const WARN_ATTRIBUTE_TYPO: ErrorDef = ErrorDef::new(
    145,
    r#"suspect attribute with name '{}'; did you mean '{}'?"#,
);
#[allow(dead_code)]
pub const ERR_INVALID_GENERATED_NAME: ErrorDef =
    ErrorDef::new(146, r#"generated name must be a valid identifier"#);
#[allow(dead_code)]
pub const ERR_AVAILABLE_MISSING_ARGUMENTS: ErrorDef = ErrorDef::new(
    147,
    r#"at least one argument is required: 'added', 'deprecated', or 'removed'"#,
);
#[allow(dead_code)]
pub const ERR_NOTE_WITHOUT_DEPRECATION_OR_REMOVAL: ErrorDef = ErrorDef::new(
    148,
    r#"the @available argument 'note' cannot be used without 'deprecated', 'removed', or 'replaced'"#,
);
#[allow(dead_code)]
pub const ERR_PLATFORM_NOT_ON_LIBRARY: ErrorDef = ErrorDef::new(
    149,
    r#"the @available argument 'platform' can only be used on the library's @available attribute"#,
);
#[allow(dead_code)]
pub const ERR_LIBRARY_AVAILABILITY_MISSING_ADDED: ErrorDef = ErrorDef::new(
    150,
    r#"missing 'added' argument on the library's @available attribute"#,
);
#[allow(dead_code)]
pub const ERR_MISSING_LIBRARY_AVAILABILITY: ErrorDef = ErrorDef::new(
    151,
    r#"to use the @available attribute here, you must also annotate the `library {};` declaration in one of the library's files"#,
);
#[allow(dead_code)]
pub const ERR_INVALID_PLATFORM: ErrorDef = ErrorDef::new(
    152,
    r#"invalid platform '{}'; must match the regex [a-z][a-z0-9]*"#,
);
#[allow(dead_code)]
pub const ERR_INVALID_VERSION: ErrorDef = ErrorDef::new(
    153,
    r#"invalid version '{}'; must be an integer from 1 to 2^31-1 inclusive, or one of the special constants `NEXT` or `HEAD`"#,
);
#[allow(dead_code)]
pub const ERR_INVALID_AVAILABILITY_ORDER: ErrorDef =
    ErrorDef::new(154, r#"invalid @available attribute; must have {}"#);
#[allow(dead_code)]
pub const ERR_AVAILABILITY_CONFLICTS_WITH_PARENT: ErrorDef = ErrorDef::new(
    155,
    r#"the argument {}={} conflicts with {}={} at {}; a child element cannot be {} {} its parent element is {}"#,
);
#[allow(dead_code)]
pub const ERR_CANNOT_BE_OPTIONAL: ErrorDef = ErrorDef::new(156, r#"{} cannot be optional"#);
#[allow(dead_code)]
pub const ERR_MUST_BE_APROTOCOL: ErrorDef = ErrorDef::new(157, r#"{} must be a protocol"#);
#[allow(dead_code)]
pub const ERR_CANNOT_BOUND_TWICE: ErrorDef = ErrorDef::new(158, r#"{} cannot bound twice"#);
#[allow(dead_code)]
pub const ERR_STRUCT_CANNOT_BE_OPTIONAL: ErrorDef = ErrorDef::new(
    159,
    r#"structs can no longer be marked optional; please use the new syntax, `box<{}>`"#,
);
#[allow(dead_code)]
pub const ERR_CANNOT_INDICATE_OPTIONAL_TWICE: ErrorDef = ErrorDef::new(
    160,
    r#"{} is already optional, cannot indicate optionality twice"#,
);
#[allow(dead_code)]
pub const ERR_MUST_HAVE_NON_ZERO_SIZE: ErrorDef =
    ErrorDef::new(161, r#"{} must have non-zero size"#);
#[allow(dead_code)]
pub const ERR_WRONG_NUMBER_OF_LAYOUT_PARAMETERS: ErrorDef =
    ErrorDef::new(162, r#"{} expected {} layout parameter(s), but got {}"#);
#[allow(dead_code)]
pub const ERR_MULTIPLE_CONSTRAINT_DEFINITIONS: ErrorDef =
    ErrorDef::new(163, r#"cannot specify multiple constraint sets on a type"#);
#[allow(dead_code)]
pub const ERR_TOO_MANY_CONSTRAINTS: ErrorDef =
    ErrorDef::new(164, r#"{} expected at most {} constraints, but got {}"#);
#[allow(dead_code)]
pub const ERR_EXPECTED_TYPE: ErrorDef =
    ErrorDef::new(165, r#"expected type but got a literal or constant"#);
#[allow(dead_code)]
pub const ERR_UNEXPECTED_CONSTRAINT: ErrorDef =
    ErrorDef::new(166, r#"{} failed to resolve constraint"#);
#[allow(dead_code)]
pub const ERR_CANNOT_CONSTRAIN_TWICE: ErrorDef =
    ErrorDef::new(167, r#"{} cannot add additional constraint"#);
#[allow(dead_code)]
pub const ERR_PROTOCOL_CONSTRAINT_REQUIRED: ErrorDef =
    ErrorDef::new(168, r#"{} requires a protocol as its first constraint"#);
#[allow(dead_code)]
pub const ERR_BOX_CANNOT_BE_OPTIONAL: ErrorDef = ErrorDef::new(
    169,
    r#"cannot specify optionality for box, boxes are optional by default"#,
);
// ERR_BOXED_TYPE_CANNOT_BE_OPTIONAL is retired (id: 170)
#[allow(dead_code)]
pub const ERR_CANNOT_BE_BOXED_SHOULD_BE_OPTIONAL: ErrorDef = ErrorDef::new(
    171,
    r#"type {} cannot be boxed, try using optional instead"#,
);
#[allow(dead_code)]
pub const ERR_RESOURCE_MUST_BE_UINT32_DERIVED: ErrorDef =
    ErrorDef::new(172, r#"resource {} must be uint32"#);
#[allow(dead_code)]
pub const ERR_RESOURCE_MISSING_SUBTYPE_PROPERTY: ErrorDef = ErrorDef::new(
    173,
    r#"resource {} expected to have the subtype property, but it was missing"#,
);
// ERR_RESOURCE_MISSING_RIGHTS_PROPERTY is retired (id: 174)
#[allow(dead_code)]
pub const ERR_RESOURCE_SUBTYPE_PROPERTY_MUST_REFER_TO_ENUM: ErrorDef = ErrorDef::new(
    175,
    r#"the subtype property must be an enum, but wasn't in resource {}"#,
);
// ERR_HANDLE_SUBTYPE_MUST_REFER_TO_RESOURCE_SUBTYPE is retired (id: 176)
#[allow(dead_code)]
pub const ERR_RESOURCE_RIGHTS_PROPERTY_MUST_REFER_TO_BITS: ErrorDef = ErrorDef::new(
    177,
    r#"the rights property must be a uint32 or a uint32-based bits, but wasn't defined as such in resource {}"#,
);
#[allow(dead_code)]
pub const ERR_UNUSED_IMPORT: ErrorDef = ErrorDef::new(
    178,
    r#"{} imports {} but does not use it; either use it or remove the import"#,
);
#[allow(dead_code)]
pub const ERR_NEW_TYPE_CANNOT_HAVE_CONSTRAINT: ErrorDef =
    ErrorDef::new(179, r#"{} is a newtype, which cannot carry constraints"#);
#[allow(dead_code)]
pub const ERR_EXPERIMENTAL_ZX_CTYPES_DISALLOWED: ErrorDef = ErrorDef::new(
    180,
    r#"{} is an experimental type that must be enabled by with `--experimental zx_c_types`"#,
);
#[allow(dead_code)]
pub const ERR_REFERENCE_IN_LIBRARY_ATTRIBUTE: ErrorDef = ErrorDef::new(
    181,
    r#"attributes on the 'library' declaration do not support referencing constants"#,
);
// ERR_LEGACY_WITHOUT_REMOVAL is retired (id: 182)
// ERR_LEGACY_CONFLICTS_WITH_PARENT is retired (id: 183)
#[allow(dead_code)]
pub const ERR_UNEXPECTED_CONTROL_CHARACTER: ErrorDef = ErrorDef::new(
    184,
    r#"unexpected control character in string literal; use the Unicode escape `\\\\u{{}}` instead"#,
);
#[allow(dead_code)]
pub const ERR_UNICODE_ESCAPE_MISSING_BRACES: ErrorDef = ErrorDef::new(
    185,
    r#"Unicode escape must use braces, like `\\\\u{a}` for U+000A"#,
);
#[allow(dead_code)]
pub const ERR_UNICODE_ESCAPE_UNTERMINATED: ErrorDef =
    ErrorDef::new(186, r#"Unicode escape is missing a closing brace '}'"#);
#[allow(dead_code)]
pub const ERR_UNICODE_ESCAPE_EMPTY: ErrorDef =
    ErrorDef::new(187, r#"Unicode escape must have at least 1 hex digit"#);
#[allow(dead_code)]
pub const ERR_UNICODE_ESCAPE_TOO_LONG: ErrorDef =
    ErrorDef::new(188, r#"Unicode escape must have at most 6 hex digits"#);
#[allow(dead_code)]
pub const ERR_UNICODE_ESCAPE_TOO_LARGE: ErrorDef =
    ErrorDef::new(189, r#"invalid Unicode code point '{}'; maximum is 10FFFF"#);
// ERR_SIMPLE_PROTOCOL_MUST_BE_CLOSED is retired (id: 190)
#[allow(dead_code)]
pub const ERR_METHOD_MUST_DEFINE_STRICTNESS: ErrorDef = ErrorDef::new(
    191,
    r#"Method {} must explicitly specify strict or flexible. (The default is changing from strict to flexible, and explicit modifiers are mandatory during the migration.)"#,
);
#[allow(dead_code)]
pub const ERR_PROTOCOL_MUST_DEFINE_OPENNESS: ErrorDef = ErrorDef::new(
    192,
    r#"Protocol {} must explicitly specify open, ajar, or closed. (The default is changing from closed to open, and explicit modifiers are mandatory during the migration.)"#,
);
#[allow(dead_code)]
pub const ERR_CANNOT_BE_BOXED_NOR_OPTIONAL: ErrorDef =
    ErrorDef::new(193, r#"type {} cannot be boxed"#);
// ERR_EMPTY_PAYLOAD_STRUCTS_WHEN_RESULT_UNION is retired (id: 194)
// ERR_EXPERIMENTAL_OVERFLOWING_ATTRIBUTE_MISSING_EXPERIMENTAL_FLAG is retired (id: 195)
// ERR_EXPERIMENTAL_OVERFLOWING_INCORRECT_USAGE is retired (id: 196)
#[allow(dead_code)]
pub const ERR_OVERLAY_MUST_BE_STRICT: ErrorDef = ErrorDef::new(197, r#"overlays must be strict"#);
#[allow(dead_code)]
pub const ERR_OVERLAY_MUST_BE_VALUE: ErrorDef =
    ErrorDef::new(198, r#"overlays must be value (not resource) types"#);
#[allow(dead_code)]
pub const ERR_OVERLAY_MEMBER_MUST_BE_VALUE: ErrorDef =
    ErrorDef::new(199, r#"overlays may not contain resource members"#);
// ERR_OVERLAY_MUST_NOT_CONTAIN_RESERVED is retired (id: 200)
#[allow(dead_code)]
pub const ERR_PLATFORM_VERSION_NOT_SELECTED: ErrorDef = ErrorDef::new(
    201,
    r#"{} belongs to platform '{}', but no version was selected for it; please choose a version N by passing `--available {}:N`"#,
);
// ERR_TRANSITIONAL_NOT_ALLOWED is retired (id: 202)
#[allow(dead_code)]
pub const ERR_REMOVED_AND_REPLACED: ErrorDef = ErrorDef::new(
    203,
    r#"the @available arguments 'removed' and 'replaced' are mutually exclusive"#,
);
#[allow(dead_code)]
pub const ERR_LIBRARY_REPLACED: ErrorDef = ErrorDef::new(
    204,
    r#"the @available argument 'replaced' cannot be used on the library declaration; used 'removed' instead"#,
);
#[allow(dead_code)]
pub const ERR_INVALID_REMOVED: ErrorDef = ErrorDef::new(
    205,
    r#"{} is marked removed={}, but there is a replacement marked added={} at {}; either change removed={} to replaced={}, or delete the replacement"#,
);
#[allow(dead_code)]
pub const ERR_INVALID_REPLACED: ErrorDef = ErrorDef::new(
    206,
    r#"{} is marked replaced={}, but there is no replacement marked added={}; either change replaced={} to removed={}, or define a replacement"#,
);
#[allow(dead_code)]
pub const ERR_TYPE_SHAPE_INTEGER_OVERFLOW: ErrorDef = ErrorDef::new(
    207,
    r#"cannot calculate type shape because of integer overflow in {} {} {}"#,
);
#[allow(dead_code)]
pub const ERR_RESERVED_PLATFORM: ErrorDef = ErrorDef::new(
    208,
    r#"platform '{}' is reserved; choose a different platform name using @available(platform="...", added=...)"#,
);
#[allow(dead_code)]
pub const ERR_RESERVED_NOT_ALLOWED: ErrorDef = ErrorDef::new(
    209,
    r#"FIDL no longer supports reserved table or union fields; use @available instead"#,
);
#[allow(dead_code)]
pub const ERR_INVALID_DISCOVERABLE_LOCATION: ErrorDef = ErrorDef::new(
    210,
    r#"invalid @discoverable location '{}'; must be comma separated 'platform' and/or 'external'"#,
);
#[allow(dead_code)]
pub const ERR_CANNOT_BE_RENAMED: ErrorDef = ErrorDef::new(
    211,
    r#"the @available argument 'renamed' cannot be used on a {}; it can only be used on members of a declaration"#,
);
#[allow(dead_code)]
pub const ERR_RENAMED_WITHOUT_REPLACED_OR_REMOVED: ErrorDef = ErrorDef::new(
    212,
    r#"the @available argument 'renamed' cannot be used without 'replaced' or 'removed'"#,
);
#[allow(dead_code)]
pub const ERR_RENAMED_TO_SAME_NAME: ErrorDef = ErrorDef::new(
    213,
    r#"renaming to '{}' has no effect because the element is already named '{}'; either remove the 'renamed' argument or choose a different name"#,
);
#[allow(dead_code)]
pub const ERR_INVALID_REMOVED_AND_RENAMED: ErrorDef = ErrorDef::new(
    214,
    r#"{} is marked removed={}, renamed="{}" but the name '{}' is already used at {}"#,
);
#[allow(dead_code)]
pub const ERR_INVALID_REPLACED_AND_RENAMED: ErrorDef = ErrorDef::new(
    215,
    r#"{} is marked replaced={}, renamed="{}" but there is no replacement '{}' marked added={}; please define it"#,
);
#[allow(dead_code)]
pub const ERR_INVALID_REMOVED_ABI: ErrorDef = ErrorDef::new(
    216,
    r#"{} is marked removed={}, but its {} ({}) is reused at {}; use replaced={}, renamed="{}" instead of removed={} if you intend to replace the ABI, otherwise choose a different {}"#,
);
#[allow(dead_code)]
pub const ERR_INVALID_REPLACED_ABI: ErrorDef = ErrorDef::new(
    217,
    r#"{} is marked replaced={}, but its {} ({}) does not match the replacement's {} ({}) at {}; use removed={} if you intend to remove the ABI, otherwise use the same {}"#,
);
#[allow(dead_code)]
pub const ERR_INVALID_MODIFIER_AVAILABLE_ARGUMENT: ErrorDef = ErrorDef::new(
    218,
    r#"invalid argument '{}'; only 'added' and 'removed' are allowed on modifier availabilities"#,
);
#[allow(dead_code)]
pub const ERR_CANNOT_CHANGE_METHOD_STRICTNESS: ErrorDef = ErrorDef::new(
    219,
    r#"changing the strictness of a two-way method without error syntax is not allowed because it is ABI breaking"#,
);
#[allow(dead_code)]
pub const ERR_RESOURCE_FORBIDDEN_HERE: ErrorDef = ErrorDef::new(
    221,
    r#"'resource' appears in declaration annotated '@no_resource'"#,
);
#[allow(dead_code)]
pub const ERR_EXPERIMENTAL_NO_RESOURCE: ErrorDef = ErrorDef::new(
    222,
    r#"'@no_resource' is an experimental attribute that must be enabled with --experimental no_resource_attribute"#,
);
#[allow(dead_code)]
pub const ERR_NO_RESOURCE_FORBIDS_COMPOSE: ErrorDef = ErrorDef::new(
    223,
    r#"'{}' has the '@no_resource` attribute, and thus cannot compose '{}' unless it is also has the '@no_resource' attribute"#,
);

pub const ERR_NULLABLE_ARRAY: ErrorDef = ErrorDef::new(62, "arrays cannot be nullable");
pub const ERR_ARRAY_SIZE_ZERO: ErrorDef = ErrorDef::new(161, "arrays cannot have size 0");
pub const ERR_ARRAY_CONSTRAINT: ErrorDef = ErrorDef::new(1001, "arrays cannot have constraints");
pub const ERR_EXPECTED_VALUE: ErrorDef = ErrorDef::new(1003, "expected value");

pub const ERR_BITS_MEMBER_DUPLICATE_NAME: ErrorDef = ErrorDef::new(1006, "bits member name duplicated");
pub const ERR_BITS_MEMBER_DUPLICATE_VALUE: ErrorDef = ErrorDef::new(1007, "bits member value duplicated");
pub const ERR_BITS_TYPE_MUST_BE_UNSIGNED: ErrorDef = ErrorDef::new(1008, "bits type must be an unsigned integer");
pub const ERR_CANNOT_BE_NULLABLE: ErrorDef = ErrorDef::new(1009, "value cannot be nullable");
pub const ERR_CANNOT_HAVE_CONSTRAINTS: ErrorDef = ErrorDef::new(1010, "value cannot have constraints");
pub const ERR_STRICT_BITS_MUST_HAVE_MEMBERS: ErrorDef = ErrorDef::new(1011, "strict bits must have at least one member");
pub const ERR_MEMBER_OVERFLOW: ErrorDef = ErrorDef::new(1012, "member value overflows its underlying type");
pub const ERR_INVALID_MEMBER_VALUE: ErrorDef = ErrorDef::new(1013, "invalid or unparseable member value");
pub const ERR_DUPLICATE_METHOD_NAME: ErrorDef = ErrorDef::new(1015, "duplicate method name");
pub const ERR_FLEXIBLE_PROTOCOL_CANNOT_BE_EMPTY: ErrorDef = ErrorDef::new(1016, "flexible protocol cannot be empty");
pub const ERR_STRICT_PROTOCOL_CANNOT_BE_EMPTY: ErrorDef = ErrorDef::new(1017, "strict protocol cannot be empty");
pub const ERR_EMPTY_PROTOCOL_MEMBER: ErrorDef = ErrorDef::new(1018, "protocol member cannot be empty");
pub const ERR_INVALID_COMPOSE: ErrorDef = ErrorDef::new(1019, "invalid compose");
pub const ERR_METHOD_EMPTY_PAYLOAD: ErrorDef = ErrorDef::new(1020, "method payload cannot be empty struct");
pub const ERR_NO_STRICT_ON_COMPOSE: ErrorDef = ErrorDef::new(1021, "compose cannot be strict");
pub const ERR_ONE_WAY_ERROR: ErrorDef = ErrorDef::new(1022, "one-way method cannot have error");
pub const ERR_REQUEST_MUST_BE_PROTOCOL: ErrorDef = ErrorDef::new(1023, "request type must be a protocol");
pub const ERR_REQUEST_MUST_BE_PARAMETERIZED: ErrorDef = ErrorDef::new(1024, "request type must be parameterized");
pub const ERR_DISALLOWED_REQUEST_TYPE: ErrorDef = ErrorDef::new(1025, "request type must be struct, table, or union");
pub const ERR_DISALLOWED_RESPONSE_TYPE: ErrorDef = ErrorDef::new(1026, "response type must be struct, table, or union");
