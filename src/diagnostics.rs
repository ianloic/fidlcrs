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

macro_rules! define_diagnostic_constant {
    (ErrorDef $name:ident = $id:literal, $msg:literal) => {
        #[allow(dead_code)]
        pub const $name: ErrorDef = ErrorDef::new($id, $msg);
    };
    (WarningDef $name:ident = $id:literal, $msg:literal) => {
        #[allow(dead_code)]
        pub const $name: ErrorDef = ErrorDef::new($id, $msg);
    };
    (RetiredDef $name:ident = $id:literal) => {};
}

macro_rules! define_diagnostics {
    (
        $(
            $kind:ident $name:ident = $id:literal $(, $msg:literal)? ;
        )*
    ) => {
        #[allow(dead_code, non_camel_case_types)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(usize)]
        pub enum ErrorId {
            $(
                $name = $id,
            )*
        }

        $(
            define_diagnostic_constant!($kind $name = $id $(, $msg)?);
        )*
    }
}

define_diagnostics! {
    ErrorDef ERR_INVALID_CHARACTER = 1, r#"invalid character '{}'"# ;
    ErrorDef ERR_UNEXPECTED_LINE_BREAK = 2, r#"unexpected line-break in string literal"# ;
    ErrorDef ERR_INVALID_ESCAPE_SEQUENCE = 3, r#"invalid escape sequence '{}'"# ;
    ErrorDef ERR_INVALID_HEX_DIGIT = 4, r#"invalid hex digit '{}'"# ;
    RetiredDef ERR_INVALID_OCT_DIGIT = 5 ;
    ErrorDef ERR_EXPECTED_DECLARATION = 6, r#"invalid declaration type {}"# ;
    ErrorDef ERR_UNEXPECTED_TOKEN = 7, r#"found unexpected token"# ;
    ErrorDef ERR_UNEXPECTED_TOKEN_OF_KIND = 8, r#"unexpected token {}, was expecting {}"# ;
    ErrorDef ERR_UNEXPECTED_IDENTIFIER = 9, r#"unexpected identifier {}, was expecting {}"# ;
    ErrorDef ERR_INVALID_IDENTIFIER = 10, r#"invalid identifier '{}'"# ;
    ErrorDef ERR_INVALID_LIBRARY_NAME_COMPONENT = 11, r#"Invalid library name component {}"# ;
    ErrorDef ERR_INVALID_LAYOUT_CLASS = 12, r#"layouts must be of the class: bits, enum, struct, table, or union."# ;
    ErrorDef ERR_INVALID_WRAPPED_TYPE = 13, r#"wrapped type for bits/enum must be an identifier"# ;
    ErrorDef ERR_ATTRIBUTE_WITH_EMPTY_PARENS = 14, r#"attributes without arguments must omit the trailing empty parentheses"# ;
    ErrorDef ERR_ATTRIBUTE_ARGS_MUST_ALL_BE_NAMED = 15, r#"attributes that take multiple arguments must name all of them explicitly"# ;
    ErrorDef ERR_MISSING_ORDINAL_BEFORE_MEMBER = 16, r#"missing ordinal before member"# ;
    ErrorDef ERR_ORDINAL_OUT_OF_BOUND = 17, r#"ordinal out-of-bound"# ;
    ErrorDef ERR_ORDINALS_MUST_START_AT_ONE = 18, r#"ordinals must start at 1"# ;
    ErrorDef ERR_MUST_HAVE_ONE_MEMBER = 19, r#"must have at least one member"# ;
    ErrorDef ERR_INVALID_PROTOCOL_MEMBER = 20, r#"invalid protocol member"# ;
    RetiredDef ERR_EXPECTED_PROTOCOL_MEMBER = 21 ;
    ErrorDef ERR_CANNOT_ATTACH_ATTRIBUTE_TO_IDENTIFIER = 22, r#"cannot attach attributes to identifiers"# ;
    ErrorDef ERR_ATTRIBUTE_INSIDE_TYPE_DECLARATION = 23, r#"attributes are not allowed here; put the attribute before the `type` keyword instead"# ;
    ErrorDef ERR_DOC_COMMENT_ON_PARAMETERS = 24, r#"cannot have doc comment on parameters"# ;
    ErrorDef ERR_LIBRARY_IMPORTS_MUST_BE_GROUPED_AT_TOP_OF_FILE = 25, r#"library imports must be grouped at top-of-file"# ;
    WarningDef WARN_COMMENT_WITHIN_DOC_COMMENT_BLOCK = 26, r#"cannot have comment within doc comment block"# ;
    WarningDef WARN_BLANK_LINES_WITHIN_DOC_COMMENT_BLOCK = 27, r#"cannot have blank lines within doc comment block"# ;
    WarningDef WARN_DOC_COMMENT_MUST_BE_FOLLOWED_BY_DECLARATION = 28, r#"doc comment must be followed by a declaration"# ;
    ErrorDef ERR_MUST_HAVE_ONE_PROPERTY = 29, r#"must have at least one property"# ;
    ErrorDef ERR_CANNOT_SPECIFY_MODIFIER = 30, r#"cannot specify modifier {} for {}"# ;
    ErrorDef ERR_CANNOT_SPECIFY_SUBTYPE = 31, r#"cannot specify subtype for {}"# ;
    ErrorDef ERR_DUPLICATE_MODIFIER = 32, r#"duplicate occurrence of {}"# ;
    ErrorDef ERR_CONFLICTING_MODIFIER = 33, r#"{} conflicts with {}"# ;
    ErrorDef ERR_NAME_COLLISION = 34, r#"{} '{}' has the same name as the {} declared at {}"# ;
    ErrorDef ERR_NAME_COLLISION_CANONICAL = 35, r#"{} '{}' conflicts with {} '{}' declared at {}; both names are represented by the canonical form '{}'"# ;
    ErrorDef ERR_NAME_OVERLAP = 36, r#"{} '{}' has the same name as the {} declared at {}; both are available {} of platform '{}'"# ;
    ErrorDef ERR_NAME_OVERLAP_CANONICAL = 37, r#"{} '{}' conflicts with {} '{}' declared at {}; both names are represented by the canonical form '{}' and are available {} of platform '{}'"# ;
    ErrorDef ERR_DECL_NAME_CONFLICTS_WITH_LIBRARY_IMPORT = 38, r#"Declaration name '{}' conflicts with a library import. Consider using the 'as' keyword to import the library under a different name."# ;
    ErrorDef ERR_DECL_NAME_CONFLICTS_WITH_LIBRARY_IMPORT_CANONICAL = 39, r#"Declaration name '{}' conflicts with a library import due to its canonical form '{}'. Consider using the 'as' keyword to import the library under a different name."# ;
    ErrorDef ERR_FILES_DISAGREE_ON_LIBRARY_NAME = 40, r#"Two files in the library disagree about the name of the library"# ;
    ErrorDef ERR_MULTIPLE_LIBRARIES_WITH_SAME_NAME = 41, r#"There are multiple libraries named '{}'"# ;
    ErrorDef ERR_DUPLICATE_LIBRARY_IMPORT = 42, r#"Library {} already imported. Did you require it twice?"# ;
    ErrorDef ERR_CONFLICTING_LIBRARY_IMPORT = 43, r#"import of library '{}' conflicts with another library import"# ;
    ErrorDef ERR_CONFLICTING_LIBRARY_IMPORT_ALIAS = 44, r#"import of library '{}' under alias '{}' conflicts with another library import"# ;
    ErrorDef ERR_ATTRIBUTES_NOT_ALLOWED_ON_LIBRARY_IMPORT = 45, r#"attributes and doc comments are not allowed on `using` statements"# ;
    ErrorDef ERR_UNKNOWN_LIBRARY = 46, r#"Could not find library named {}. Did you include its sources with --files?"# ;
    RetiredDef ERR_PROTOCOL_COMPOSED_MULTIPLE_TIMES = 47 ;
    ErrorDef ERR_OPTIONAL_TABLE_MEMBER = 48, r#"Table members cannot be optional"# ;
    ErrorDef ERR_OPTIONAL_UNION_MEMBER = 49, r#"Union members cannot be optional"# ;
    ErrorDef ERR_DEPRECATED_STRUCT_DEFAULTS = 50, r#"Struct defaults are deprecated and should not be used (see RFC-0160)"# ;
    ErrorDef ERR_UNKNOWN_DEPENDENT_LIBRARY = 51, r#"Unknown dependent library {} or reference to member of library {}. Did you require it with `using`?"# ;
    ErrorDef ERR_NAME_NOT_FOUND = 52, r#"cannot find '{}' in {}"# ;
    ErrorDef ERR_CANNOT_REFER_TO_MEMBER = 53, r#"cannot refer to member of {}"# ;
    ErrorDef ERR_MEMBER_NOT_FOUND = 54, r#"{} has no member '{}'"# ;
    ErrorDef ERR_INVALID_REFERENCE_TO_DEPRECATED = 55, r#"invalid reference to {}, which is deprecated {} of platform '{}' while {} is not; either remove this reference or mark {} as deprecated"# ;
    ErrorDef ERR_INVALID_REFERENCE_TO_DEPRECATED_OTHER_PLATFORM = 56, r#"invalid reference to {}, which is deprecated {} of platform '{}' while {} is not deprecated {} of platform '{}'; either remove this reference or mark {} as deprecated"# ;
    ErrorDef ERR_ANONYMOUS_NAME_REFERENCE = 58, r#"cannot refer to anonymous name {}"# ;
    ErrorDef ERR_INVALID_CONSTANT_TYPE = 59, r#"invalid constant type {}"# ;
    ErrorDef ERR_CANNOT_RESOLVE_CONSTANT_VALUE = 60, r#"unable to resolve constant value"# ;
    ErrorDef ERR_OR_OPERATOR_ON_NON_PRIMITIVE_VALUE = 61, r#"Or operator can only be applied to primitive-kinded values"# ;
    ErrorDef ERR_NEW_TYPES_NOT_ALLOWED = 62, r#"newtypes not allowed: type declaration {} defines a new type of the existing {} type, which is not yet supported"# ;
    ErrorDef ERR_EXPECTED_VALUE_BUT_GOT_TYPE = 63, r#"{} is a type, but a value was expected"# ;
    ErrorDef ERR_MISMATCHED_NAME_TYPE_ASSIGNMENT = 64, r#"mismatched named type assignment: cannot define a constant or default value of type {} using a value of type {}"# ;
    ErrorDef ERR_TYPE_CANNOT_BE_CONVERTED_TO_TYPE = 65, r#"{} (type {}) cannot be converted to type {}"# ;
    ErrorDef ERR_CONSTANT_OVERFLOWS_TYPE = 66, r#"{} overflows type {}"# ;
    ErrorDef ERR_BITS_MEMBER_MUST_BE_POWER_OF_TWO = 67, r#"bits members must be powers of two"# ;
    ErrorDef ERR_FLEXIBLE_ENUM_MEMBER_WITH_MAX_VALUE = 68, r#"flexible enums must not have a member with a value of {}, which is reserved for the unknown value. either: remove the member, change its value to something else, or explicitly specify the unknown value with the @unknown attribute. see <https://fuchsia.dev/fuchsia-src/reference/fidl/language/attributes#unknown> for more info."# ;
    ErrorDef ERR_BITS_TYPE_MUST_BE_UNSIGNED_INTEGRAL_PRIMITIVE = 69, r#"bits may only be of unsigned integral primitive type, found {}"# ;
    ErrorDef ERR_ENUM_TYPE_MUST_BE_INTEGRAL_PRIMITIVE = 70, r#"enums may only be of integral primitive type, found {}"# ;
    ErrorDef ERR_UNKNOWN_ATTRIBUTE_ON_STRICT_ENUM_MEMBER = 71, r#"the @unknown attribute can be only be used on flexible enum members."# ;
    ErrorDef ERR_UNKNOWN_ATTRIBUTE_ON_MULTIPLE_ENUM_MEMBERS = 72, r#"the @unknown attribute can be only applied to one enum member."# ;
    ErrorDef ERR_COMPOSING_NON_PROTOCOL = 73, r#"This declaration is not a protocol"# ;
    ErrorDef ERR_INVALID_METHOD_PAYLOAD_LAYOUT_CLASS = 74, r#"cannot use {} as a request/response; must use a struct, table, or union"# ;
    ErrorDef ERR_INVALID_METHOD_PAYLOAD_TYPE = 75, r#"invalid request/response type '{}'; must use a struct, table, or union"# ;
    RetiredDef ERR_RESPONSES_WITH_ERRORS_MUST_NOT_BE_EMPTY = 76 ;
    ErrorDef ERR_EMPTY_PAYLOAD_STRUCTS = 77, r#"(struct {}) is not allowed as a request or response, use () instead"# ;
    RetiredDef ERR_DUPLICATE_ELEMENT_NAME = 78 ;
    RetiredDef ERR_DUPLICATE_ELEMENT_NAME_CANONICAL = 79 ;
    ErrorDef ERR_GENERATED_ZERO_VALUE_ORDINAL = 80, r#"Ordinal value 0 disallowed."# ;
    ErrorDef ERR_DUPLICATE_METHOD_ORDINAL = 81, r#"Multiple methods with the same ordinal in a protocol; previous was at {}."# ;
    ErrorDef ERR_INVALID_SELECTOR_VALUE = 82, r#"invalid selector value, must be a method name or a fully qualified method name"# ;
    ErrorDef ERR_FUCHSIA_IO_EXPLICIT_ORDINALS = 83, r#"fuchsia.io must have explicit ordinals (https://fxbug.dev/42157659)"# ;
    ErrorDef ERR_PAYLOAD_STRUCT_HAS_DEFAULT_MEMBERS = 84, r#"default values are not allowed on members of request/response structs"# ;
    RetiredDef ERR_DUPLICATE_SERVICE_MEMBER_NAME = 85 ;
    RetiredDef ERR_STRICT_UNION_MUST_HAVE_NON_RESERVED_MEMBER = 86 ;
    RetiredDef ERR_DUPLICATE_SERVICE_MEMBER_NAME_CANONICAL = 87 ;
    ErrorDef ERR_OPTIONAL_SERVICE_MEMBER = 88, r#"service members cannot be optional"# ;
    RetiredDef ERR_DUPLICATE_STRUCT_MEMBER_NAME = 89 ;
    RetiredDef ERR_DUPLICATE_STRUCT_MEMBER_NAME_CANONICAL = 90 ;
    ErrorDef ERR_INVALID_STRUCT_MEMBER_TYPE = 91, r#"struct field {} has an invalid default type {}"# ;
    ErrorDef ERR_TABLE_ORDINAL_TOO_LARGE = 92, r#"ordinal is too large; table ordinals cannot be greater than 64"# ;
    ErrorDef ERR_MAX_ORDINAL_NOT_TABLE = 93, r#"the 64th ordinal of a table may only contain a table type"# ;
    ErrorDef ERR_DUPLICATE_TABLE_FIELD_ORDINAL = 94, r#"multiple table fields with the same ordinal; previous was at {}"# ;
    RetiredDef ERR_DUPLICATE_TABLE_FIELD_NAME = 95 ;
    RetiredDef ERR_DUPLICATE_TABLE_FIELD_NAME_CANONICAL = 96 ;
    ErrorDef ERR_DUPLICATE_UNION_MEMBER_ORDINAL = 97, r#"multiple union fields with the same ordinal; previous was at {}"# ;
    RetiredDef ERR_DUPLICATE_UNION_MEMBER_NAME = 98 ;
    RetiredDef ERR_DUPLICATE_UNION_MEMBER_NAME_CANONICAL = 99 ;
    RetiredDef ERR_NON_DENSE_ORDINAL = 100 ;
    ErrorDef ERR_COULD_NOT_RESOLVE_SIZE_BOUND = 101, r#"unable to resolve size bound"# ;
    ErrorDef ERR_COULD_NOT_RESOLVE_MEMBER = 102, r#"unable to resolve {} member"# ;
    ErrorDef ERR_COULD_NOT_RESOLVE_MEMBER_DEFAULT = 103, r#"unable to resolve {} default value"# ;
    ErrorDef ERR_COULD_NOT_RESOLVE_ATTRIBUTE_ARG = 104, r#"unable to resolve attribute argument"# ;
    RetiredDef ERR_DUPLICATE_MEMBER_NAME = 105 ;
    RetiredDef ERR_DUPLICATE_MEMBER_NAME_CANONICAL = 106 ;
    ErrorDef ERR_DUPLICATE_MEMBER_VALUE = 107, r#"value of {} member '{}' conflicts with previously declared member '{}' at {}"# ;
    RetiredDef ERR_DUPLICATE_RESOURCE_PROPERTY_NAME = 108 ;
    RetiredDef ERR_DUPLICATE_RESOURCE_PROPERTY_NAME_CANONICAL = 109 ;
    ErrorDef ERR_TYPE_MUST_BE_RESOURCE = 110, r#"{} '{}' may contain handles (due to member '{}' at {}), so it must be declared with the `resource` modifier: `resource {} {}`"# ;
    ErrorDef ERR_INLINE_SIZE_EXCEEDS_LIMIT = 111, r#"'{}' has an inline size of {} bytes, which exceeds the maximum allowed inline size of {} bytes"# ;
    ErrorDef ERR_ONLY_CLIENT_ENDS_IN_SERVICES = 112, r#"service members must be client_end:P"# ;
    ErrorDef ERR_MISMATCHED_TRANSPORT_IN_SERVICES = 113, r#"service member {} is over the {} transport, but member {} is over the {} transport. Multiple transports are not allowed."# ;
    ErrorDef ERR_COMPOSED_PROTOCOL_TOO_OPEN = 114, r#"{} protocol '{}' cannot compose {} protocol '{}'; composed protocol may not be more open than composing protocol"# ;
    ErrorDef ERR_FLEXIBLE_TWO_WAY_METHOD_REQUIRES_OPEN_PROTOCOL = 115, r#"flexible two-way method may only be defined in an open protocol, not {}"# ;
    ErrorDef ERR_FLEXIBLE_ONE_WAY_METHOD_IN_CLOSED_PROTOCOL = 116, r#"flexible {} may only be defined in an open or ajar protocol, not closed"# ;
    ErrorDef ERR_HANDLE_USED_IN_INCOMPATIBLE_TRANSPORT = 117, r#"handle of type {} may not be sent over transport {} used by {}"# ;
    ErrorDef ERR_TRANSPORT_END_USED_IN_INCOMPATIBLE_TRANSPORT = 118, r#"client_end / server_end of transport type {} may not be sent over transport {} used by {}"# ;
    RetiredDef ERR_EVENT_ERROR_SYNTAX = 119 ;
    ErrorDef ERR_INVALID_ATTRIBUTE_PLACEMENT = 120, r#"placement of attribute '{}' disallowed here"# ;
    ErrorDef ERR_DEPRECATED_ATTRIBUTE = 121, r#"attribute '{}' is deprecated"# ;
    ErrorDef ERR_DUPLICATE_ATTRIBUTE = 122, r#"duplicate attribute '{}'; previous was at {}"# ;
    ErrorDef ERR_DUPLICATE_ATTRIBUTE_CANONICAL = 123, r#"attribute '{}' conflicts with attribute '{}' from {}; both are represented by the canonical form '{}'"# ;
    ErrorDef ERR_CAN_ONLY_USE_STRING_OR_BOOL = 124, r#"argument '{}' on user-defined attribute '{}' cannot be a numeric value; use a bool or string instead"# ;
    ErrorDef ERR_ATTRIBUTE_ARG_MUST_NOT_BE_NAMED = 125, r#"attributes that take a single argument must not name that argument"# ;
    ErrorDef ERR_ATTRIBUTE_ARG_NOT_NAMED = 126, r#"attributes that take multiple arguments must name all of them explicitly, but '{}' was not"# ;
    ErrorDef ERR_MISSING_REQUIRED_ATTRIBUTE_ARG = 127, r#"attribute '{}' is missing the required '{}' argument"# ;
    ErrorDef ERR_MISSING_REQUIRED_ANONYMOUS_ATTRIBUTE_ARG = 128, r#"attribute '{}' is missing its required argument"# ;
    ErrorDef ERR_UNKNOWN_ATTRIBUTE_ARG = 129, r#"attribute '{}' does not support the '{}' argument"# ;
    ErrorDef ERR_DUPLICATE_ATTRIBUTE_ARG = 130, r#"attribute '{}' provides the '{}' argument multiple times; previous was at {}"# ;
    ErrorDef ERR_DUPLICATE_ATTRIBUTE_ARG_CANONICAL = 131, r#"attribute '{}' argument '{}' conflicts with argument '{}' from {}; both are represented by the canonical form '{}'"# ;
    ErrorDef ERR_ATTRIBUTE_DISALLOWS_ARGS = 132, r#"attribute '{}' does not support arguments"# ;
    ErrorDef ERR_ATTRIBUTE_ARG_REQUIRES_LITERAL = 133, r#"argument '{}' of attribute '{}' does not support referencing constants; please use a literal instead"# ;
    RetiredDef ERR_ATTRIBUTE_CONSTRAINT_NOT_SATISFIED = 134 ;
    ErrorDef ERR_INVALID_DISCOVERABLE_NAME = 135, r#"invalid @discoverable name '{}'; must follow the format 'the.library.name.TheProtocolName'"# ;
    RetiredDef ERR_TABLE_CANNOT_BE_SIMPLE = 136 ;
    RetiredDef ERR_UNION_CANNOT_BE_SIMPLE = 137 ;
    RetiredDef ERR_ELEMENT_MUST_BE_SIMPLE = 138 ;
    RetiredDef ERR_TOO_MANY_BYTES = 139 ;
    RetiredDef ERR_TOO_MANY_HANDLES = 140 ;
    ErrorDef ERR_INVALID_ERROR_TYPE = 141, r#"invalid error type: must be int32, uint32 or an enum thereof"# ;
    RetiredDef ERR_BOUND_IS_TOO_BIG = 143 ;
    RetiredDef ERR_UNABLE_TO_PARSE_BOUND = 144 ;
    WarningDef WARN_ATTRIBUTE_TYPO = 145, r#"suspect attribute with name '{}'; did you mean '{}'?"# ;
    ErrorDef ERR_INVALID_GENERATED_NAME = 146, r#"generated name must be a valid identifier"# ;
    ErrorDef ERR_AVAILABLE_MISSING_ARGUMENTS = 147, r#"at least one argument is required: 'added', 'deprecated', or 'removed'"# ;
    ErrorDef ERR_NOTE_WITHOUT_DEPRECATION_OR_REMOVAL = 148, r#"the @available argument 'note' cannot be used without 'deprecated', 'removed', or 'replaced'"# ;
    ErrorDef ERR_PLATFORM_NOT_ON_LIBRARY = 149, r#"the @available argument 'platform' can only be used on the library's @available attribute"# ;
    ErrorDef ERR_LIBRARY_AVAILABILITY_MISSING_ADDED = 150, r#"missing 'added' argument on the library's @available attribute"# ;
    ErrorDef ERR_MISSING_LIBRARY_AVAILABILITY = 151, r#"to use the @available attribute here, you must also annotate the `library {};` declaration in one of the library's files"# ;
    ErrorDef ERR_INVALID_PLATFORM = 152, r#"invalid platform '{}'; must match the regex [a-z][a-z0-9]*"# ;
    ErrorDef ERR_INVALID_VERSION = 153, r#"invalid version '{}'; must be an integer from 1 to 2^31-1 inclusive, or one of the special constants `NEXT` or `HEAD`"# ;
    ErrorDef ERR_INVALID_AVAILABILITY_ORDER = 154, r#"invalid @available attribute; must have {}"# ;
    ErrorDef ERR_AVAILABILITY_CONFLICTS_WITH_PARENT = 155, r#"the argument {}={} conflicts with {}={} at {}; a child element cannot be {} {} its parent element is {}"# ;
    ErrorDef ERR_CANNOT_BE_OPTIONAL = 156, r#"{} cannot be optional"# ;
    ErrorDef ERR_MUST_BE_APROTOCOL = 157, r#"{} must be a protocol"# ;
    ErrorDef ERR_CANNOT_BOUND_TWICE = 158, r#"{} cannot bound twice"# ;
    ErrorDef ERR_STRUCT_CANNOT_BE_OPTIONAL = 159, r#"structs can no longer be marked optional; please use the new syntax, `box<{}>`"# ;
    ErrorDef ERR_CANNOT_INDICATE_OPTIONAL_TWICE = 160, r#"{} is already optional, cannot indicate optionality twice"# ;
    ErrorDef ERR_MUST_HAVE_NON_ZERO_SIZE = 161, r#"{} must have non-zero size"# ;
    ErrorDef ERR_WRONG_NUMBER_OF_LAYOUT_PARAMETERS = 162, r#"{} expected {} layout parameter(s), but got {}"# ;
    ErrorDef ERR_MULTIPLE_CONSTRAINT_DEFINITIONS = 163, r#"cannot specify multiple constraint sets on a type"# ;
    ErrorDef ERR_TOO_MANY_CONSTRAINTS = 164, r#"{} expected at most {} constraints, but got {}"# ;
    ErrorDef ERR_EXPECTED_TYPE = 165, r#"expected type but got a literal or constant"# ;
    ErrorDef ERR_UNEXPECTED_CONSTRAINT = 166, r#"{} failed to resolve constraint"# ;
    ErrorDef ERR_CANNOT_CONSTRAIN_TWICE = 167, r#"{} cannot add additional constraint"# ;
    ErrorDef ERR_PROTOCOL_CONSTRAINT_REQUIRED = 168, r#"{} requires a protocol as its first constraint"# ;
    ErrorDef ERR_BOX_CANNOT_BE_OPTIONAL = 169, r#"cannot specify optionality for box, boxes are optional by default"# ;
    RetiredDef ERR_BOXED_TYPE_CANNOT_BE_OPTIONAL = 170 ;
    ErrorDef ERR_CANNOT_BE_BOXED_SHOULD_BE_OPTIONAL = 171, r#"type {} cannot be boxed, try using optional instead"# ;
    ErrorDef ERR_RESOURCE_MUST_BE_UINT32_DERIVED = 172, r#"resource {} must be uint32"# ;
    ErrorDef ERR_RESOURCE_MISSING_SUBTYPE_PROPERTY = 173, r#"resource {} expected to have the subtype property, but it was missing"# ;
    RetiredDef ERR_RESOURCE_MISSING_RIGHTS_PROPERTY = 174 ;
    ErrorDef ERR_RESOURCE_SUBTYPE_PROPERTY_MUST_REFER_TO_ENUM = 175, r#"the subtype property must be an enum, but wasn't in resource {}"# ;
    RetiredDef ERR_HANDLE_SUBTYPE_MUST_REFER_TO_RESOURCE_SUBTYPE = 176 ;
    ErrorDef ERR_RESOURCE_RIGHTS_PROPERTY_MUST_REFER_TO_BITS = 177, r#"the rights property must be a uint32 or a uint32-based bits, but wasn't defined as such in resource {}"# ;
    ErrorDef ERR_UNUSED_IMPORT = 178, r#"{} imports {} but does not use it; either use it or remove the import"# ;
    ErrorDef ERR_NEW_TYPE_CANNOT_HAVE_CONSTRAINT = 179, r#"{} is a newtype, which cannot carry constraints"# ;
    ErrorDef ERR_EXPERIMENTAL_ZX_CTYPES_DISALLOWED = 180, r#"{} is an experimental type that must be enabled by with `--experimental zx_c_types`"# ;
    ErrorDef ERR_REFERENCE_IN_LIBRARY_ATTRIBUTE = 181, r#"attributes on the 'library' declaration do not support referencing constants"# ;
    RetiredDef ERR_LEGACY_WITHOUT_REMOVAL = 182 ;
    RetiredDef ERR_LEGACY_CONFLICTS_WITH_PARENT = 183 ;
    ErrorDef ERR_UNEXPECTED_CONTROL_CHARACTER = 184, r#"unexpected control character in string literal; use the Unicode escape `\\\\u{{}}` instead"# ;
    ErrorDef ERR_UNICODE_ESCAPE_MISSING_BRACES = 185, r#"Unicode escape must use braces, like `\\\\u{a}` for U+000A"# ;
    ErrorDef ERR_UNICODE_ESCAPE_UNTERMINATED = 186, r#"Unicode escape is missing a closing brace '}'"# ;
    ErrorDef ERR_UNICODE_ESCAPE_EMPTY = 187, r#"Unicode escape must have at least 1 hex digit"# ;
    ErrorDef ERR_UNICODE_ESCAPE_TOO_LONG = 188, r#"Unicode escape must have at most 6 hex digits"# ;
    ErrorDef ERR_UNICODE_ESCAPE_TOO_LARGE = 189, r#"invalid Unicode code point '{}'; maximum is 10FFFF"# ;
    RetiredDef ERR_SIMPLE_PROTOCOL_MUST_BE_CLOSED = 190 ;
    ErrorDef ERR_METHOD_MUST_DEFINE_STRICTNESS = 191, r#"Method {} must explicitly specify strict or flexible. (The default is changing from strict to flexible, and explicit modifiers are mandatory during the migration.)"# ;
    ErrorDef ERR_PROTOCOL_MUST_DEFINE_OPENNESS = 192, r#"Protocol {} must explicitly specify open, ajar, or closed. (The default is changing from closed to open, and explicit modifiers are mandatory during the migration.)"# ;
    ErrorDef ERR_CANNOT_BE_BOXED_NOR_OPTIONAL = 193, r#"type {} cannot be boxed"# ;
    RetiredDef ERR_EMPTY_PAYLOAD_STRUCTS_WHEN_RESULT_UNION = 194 ;
    RetiredDef ERR_EXPERIMENTAL_OVERFLOWING_ATTRIBUTE_MISSING_EXPERIMENTAL_FLAG = 195 ;
    RetiredDef ERR_EXPERIMENTAL_OVERFLOWING_INCORRECT_USAGE = 196 ;
    ErrorDef ERR_OVERLAY_MUST_BE_STRICT = 197, r#"overlays must be strict"# ;
    ErrorDef ERR_OVERLAY_MUST_BE_VALUE = 198, r#"overlays must be value (not resource) types"# ;
    ErrorDef ERR_OVERLAY_MEMBER_MUST_BE_VALUE = 199, r#"overlays may not contain resource members"# ;
    RetiredDef ERR_OVERLAY_MUST_NOT_CONTAIN_RESERVED = 200 ;
    ErrorDef ERR_PLATFORM_VERSION_NOT_SELECTED = 201, r#"{} belongs to platform '{}', but no version was selected for it; please choose a version N by passing `--available {}:N`"# ;
    RetiredDef ERR_TRANSITIONAL_NOT_ALLOWED = 202 ;
    ErrorDef ERR_REMOVED_AND_REPLACED = 203, r#"the @available arguments 'removed' and 'replaced' are mutually exclusive"# ;
    ErrorDef ERR_LIBRARY_REPLACED = 204, r#"the @available argument 'replaced' cannot be used on the library declaration; used 'removed' instead"# ;
    ErrorDef ERR_INVALID_REMOVED = 205, r#"{} is marked removed={}, but there is a replacement marked added={} at {}; either change removed={} to replaced={}, or delete the replacement"# ;
    ErrorDef ERR_INVALID_REPLACED = 206, r#"{} is marked replaced={}, but there is no replacement marked added={}; either change replaced={} to removed={}, or define a replacement"# ;
    ErrorDef ERR_TYPE_SHAPE_INTEGER_OVERFLOW = 207, r#"cannot calculate type shape because of integer overflow in {} {} {}"# ;
    ErrorDef ERR_RESERVED_PLATFORM = 208, r#"platform '{}' is reserved; choose a different platform name using @available(platform=\, added=...)"# ;
    ErrorDef ERR_RESERVED_NOT_ALLOWED = 209, r#"FIDL no longer supports reserved table or union fields; use @available instead"# ;
    ErrorDef ERR_INVALID_DISCOVERABLE_LOCATION = 210, r#"invalid @discoverable location '{}'; must be comma separated 'platform' and/or 'external'"# ;
    ErrorDef ERR_CANNOT_BE_RENAMED = 211, r#"the @available argument 'renamed' cannot be used on a {}; it can only be used on members of a declaration"# ;
    ErrorDef ERR_RENAMED_WITHOUT_REPLACED_OR_REMOVED = 212, r#"the @available argument 'renamed' cannot be used without 'replaced' or 'removed'"# ;
    ErrorDef ERR_RENAMED_TO_SAME_NAME = 213, r#"renaming to '{}' has no effect because the element is already named '{}'; either remove the 'renamed' argument or choose a different name"# ;
    ErrorDef ERR_INVALID_REMOVED_AND_RENAMED = 214, r#"{} is marked removed={}, renamed=\ but the name '{}' is already used at {}"# ;
    ErrorDef ERR_INVALID_REPLACED_AND_RENAMED = 215, r#"{} is marked replaced={}, renamed=\ but there is no replacement '{}' marked added={}; please define it"# ;
    ErrorDef ERR_INVALID_REMOVED_ABI = 216, r#"{} is marked removed={}, but its {} ({}) is reused at {}; use replaced={}, renamed=\ instead of removed={} if you intend to replace the ABI, otherwise choose a different {}"# ;
    ErrorDef ERR_INVALID_REPLACED_ABI = 217, r#"{} is marked replaced={}, but its {} ({}) does not match the replacement's {} ({}) at {}; use removed={} if you intend to remove the ABI, otherwise use the same {}"# ;
    ErrorDef ERR_INVALID_MODIFIER_AVAILABLE_ARGUMENT = 218, r#"invalid argument '{}'; only 'added' and 'removed' are allowed on modifier availabilities"# ;
    ErrorDef ERR_CANNOT_CHANGE_METHOD_STRICTNESS = 219, r#"changing the strictness of a two-way method without error syntax is not allowed because it is ABI breaking"# ;
    ErrorDef ERR_RESOURCE_FORBIDDEN_HERE = 221, r#"'resource' appears in declaration annotated '@no_resource'"# ;
    ErrorDef ERR_EXPERIMENTAL_NO_RESOURCE = 222, r#"'@no_resource' is an experimental attribute that must be enabled with --experimental no_resource_attribute"# ;
    ErrorDef ERR_NO_RESOURCE_FORBIDS_COMPOSE = 223, r#"'{}' has the '@no_resource` attribute, and thus cannot compose '{}' unless it is also has the '@no_resource' attribute"# ;
}

// Legacy placeholder error constants
pub const ERR_NULLABLE_ARRAY: ErrorDef = ErrorDef::new(62, "arrays cannot be nullable");
pub const ERR_ARRAY_SIZE_ZERO: ErrorDef = ErrorDef::new(161, "arrays cannot have size 0");
pub const ERR_ARRAY_CONSTRAINT: ErrorDef = ErrorDef::new(1001, "arrays cannot have constraints");
pub const ERR_EXPECTED_VALUE: ErrorDef = ErrorDef::new(1003, "expected value");

pub const ERR_BITS_MEMBER_DUPLICATE_NAME: ErrorDef =
    ErrorDef::new(1006, "bits member name duplicated");
pub const ERR_BITS_MEMBER_DUPLICATE_VALUE: ErrorDef =
    ErrorDef::new(1007, "bits member value duplicated");
pub const ERR_BITS_TYPE_MUST_BE_UNSIGNED: ErrorDef =
    ErrorDef::new(1008, "bits type must be an unsigned integer");
pub const ERR_CANNOT_BE_NULLABLE: ErrorDef = ErrorDef::new(1009, "value cannot be nullable");
pub const ERR_CANNOT_HAVE_CONSTRAINTS: ErrorDef =
    ErrorDef::new(1010, "value cannot have constraints");
pub const ERR_STRICT_BITS_MUST_HAVE_MEMBERS: ErrorDef =
    ErrorDef::new(1011, "strict bits must have at least one member");
pub const ERR_MEMBER_OVERFLOW: ErrorDef =
    ErrorDef::new(1012, "member value overflows its underlying type");
pub const ERR_INVALID_MEMBER_VALUE: ErrorDef =
    ErrorDef::new(1013, "invalid or unparseable member value");
pub const ERR_DUPLICATE_METHOD_NAME: ErrorDef = ErrorDef::new(1015, "duplicate method name");
pub const ERR_FLEXIBLE_PROTOCOL_CANNOT_BE_EMPTY: ErrorDef =
    ErrorDef::new(1016, "flexible protocol cannot be empty");
pub const ERR_STRICT_PROTOCOL_CANNOT_BE_EMPTY: ErrorDef =
    ErrorDef::new(1017, "strict protocol cannot be empty");
pub const ERR_EMPTY_PROTOCOL_MEMBER: ErrorDef =
    ErrorDef::new(1018, "protocol member cannot be empty");
pub const ERR_INVALID_COMPOSE: ErrorDef = ErrorDef::new(1019, "invalid compose");
pub const ERR_METHOD_EMPTY_PAYLOAD: ErrorDef =
    ErrorDef::new(1020, "method payload cannot be empty struct");
pub const ERR_NO_STRICT_ON_COMPOSE: ErrorDef = ErrorDef::new(1021, "compose cannot be strict");
pub const ERR_ONE_WAY_ERROR: ErrorDef = ErrorDef::new(1022, "one-way method cannot have error");
pub const ERR_REQUEST_MUST_BE_PROTOCOL: ErrorDef =
    ErrorDef::new(1023, "request type must be a protocol");
pub const ERR_REQUEST_MUST_BE_PARAMETERIZED: ErrorDef =
    ErrorDef::new(1024, "request type must be parameterized");
pub const ERR_DISALLOWED_REQUEST_TYPE: ErrorDef =
    ErrorDef::new(1025, "request type must be struct, table, or union");
pub const ERR_DISALLOWED_RESPONSE_TYPE: ErrorDef =
    ErrorDef::new(1026, "response type must be struct, table, or union");
