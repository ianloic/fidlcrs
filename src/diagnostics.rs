use crate::source_span::SourceSpan;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    Error,
    Warning,
    Retired,
}

#[derive(Debug, Clone)]
pub struct Diagnostic<'a> {
    pub def: Error,
    pub message: String,
    pub span: Option<SourceSpan<'a>>,
}

macro_rules! __kind_to_enum {
    (ErrorDef) => {
        ErrorKind::Error
    };
    (WarningDef) => {
        ErrorKind::Warning
    };
    (RetiredDef) => {
        ErrorKind::Retired
    };
}

macro_rules! define_diagnostics {
    (
        $(
            $kind:ident $camel_name:ident = $id:literal $(, $msg:literal)? $(, documented = $doc:literal)? ;
        )*
    ) => {
        #[allow(dead_code, non_camel_case_types)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(usize)]
        pub enum Error {
            $(
                $camel_name = $id,
            )*
        }

        impl Error {
            pub const fn id(&self) -> usize {
                *self as usize
            }

            pub const fn msg(&self) -> &'static str {
                match self {
                    $(
                        $(Error::$camel_name => $msg,)?
                    )*
                    _ => "",
                }
            }

            pub const fn kind(&self) -> ErrorKind {
                match self {
                    $(
                        Error::$camel_name => __kind_to_enum!($kind),
                    )*
                }
            }

            pub const fn documented(&self) -> bool {
                match self {
                    $(
                        $(Error::$camel_name => $doc,)?
                    )*
                    _ => true,
                }
            }

            pub fn format_id(&self) -> String {
                format!("fi-{:04}", self.id())
            }
        }

        pub const ALL_ERRORS: &'static [Error] = &[
            $(
                Error::$camel_name,
            )*
        ];


    }
}

define_diagnostics! {
    ErrorDef ErrInvalidCharacter = 1, r#"invalid character '{}'"# ;
    ErrorDef ErrUnexpectedLineBreak = 2, r#"unexpected line-break in string literal"# ;
    ErrorDef ErrInvalidEscapeSequence = 3, r#"invalid escape sequence '{}'"# ;
    ErrorDef ErrInvalidHexDigit = 4, r#"invalid hex digit '{}'"# ;
    RetiredDef ErrInvalidOctDigit = 5 ;
    ErrorDef ErrExpectedDeclaration = 6, r#"invalid declaration type {}"# ;
    ErrorDef ErrUnexpectedToken = 7, r#"found unexpected token"# ;
    ErrorDef ErrUnexpectedTokenOfKind = 8, r#"unexpected token {}, was expecting {}"# ;
    ErrorDef ErrUnexpectedIdentifier = 9, r#"unexpected identifier {}, was expecting {}"# ;
    ErrorDef ErrInvalidIdentifier = 10, r#"invalid identifier '{}'"# ;
    ErrorDef ErrInvalidLibraryNameComponent = 11, r#"Invalid library name component {}"# ;
    ErrorDef ErrInvalidLayoutClass = 12, r#"layouts must be of the class: bits, enum, struct, table, or union."# ;
    ErrorDef ErrInvalidWrappedType = 13, r#"wrapped type for bits/enum must be an identifier"# ;
    ErrorDef ErrAttributeWithEmptyParens = 14, r#"attributes without arguments must omit the trailing empty parentheses"# ;
    ErrorDef ErrAttributeArgsMustAllBeNamed = 15, r#"attributes that take multiple arguments must name all of them explicitly"# ;
    ErrorDef ErrMissingOrdinalBeforeMember = 16, r#"missing ordinal before member"# ;
    ErrorDef ErrOrdinalOutOfBound = 17, r#"ordinal out-of-bound"# ;
    ErrorDef ErrOrdinalsMustStartAtOne = 18, r#"ordinals must start at 1"# ;
    ErrorDef ErrMustHaveOneMember = 19, r#"must have at least one member"# ;
    ErrorDef ErrInvalidProtocolMember = 20, r#"invalid protocol member"# ;
    RetiredDef ErrExpectedProtocolMember = 21 ;
    ErrorDef ErrCannotAttachAttributeToIdentifier = 22, r#"cannot attach attributes to identifiers"# ;
    ErrorDef ErrAttributeInsideTypeDeclaration = 23, r#"attributes are not allowed here; put the attribute before the `type` keyword instead"# ;
    ErrorDef ErrDocCommentOnParameters = 24, r#"cannot have doc comment on parameters"# ;
    ErrorDef ErrLibraryImportsMustBeGroupedAtTopOfFile = 25, r#"library imports must be grouped at top-of-file"# ;
    WarningDef WarnCommentWithinDocCommentBlock = 26, r#"cannot have comment within doc comment block"# ;
    WarningDef WarnBlankLinesWithinDocCommentBlock = 27, r#"cannot have blank lines within doc comment block"# ;
    WarningDef WarnDocCommentMustBeFollowedByDeclaration = 28, r#"doc comment must be followed by a declaration"# ;
    ErrorDef ErrMustHaveOneProperty = 29, r#"must have at least one property"# ;
    ErrorDef ErrCannotSpecifyModifier = 30, r#"cannot specify modifier {} for {}"# ;
    ErrorDef ErrCannotSpecifySubtype = 31, r#"cannot specify subtype for {}"# ;
    ErrorDef ErrDuplicateModifier = 32, r#"duplicate occurrence of {}"# ;
    ErrorDef ErrConflictingModifier = 33, r#"{} conflicts with {}"# ;
    ErrorDef ErrNameCollision = 34, r#"{} '{}' has the same name as the {} declared at {}"# ;
    ErrorDef ErrNameCollisionCanonical = 35, r#"{} '{}' conflicts with {} '{}' declared at {}; both names are represented by the canonical form '{}'"# ;
    ErrorDef ErrNameOverlap = 36, r#"{} '{}' has the same name as the {} declared at {}; both are available {} of platform '{}'"# ;
    ErrorDef ErrNameOverlapCanonical = 37, r#"{} '{}' conflicts with {} '{}' declared at {}; both names are represented by the canonical form '{}' and are available {} of platform '{}'"# ;
    ErrorDef ErrDeclNameConflictsWithLibraryImport = 38, r#"Declaration name '{}' conflicts with a library import. Consider using the 'as' keyword to import the library under a different name."# ;
    ErrorDef ErrDeclNameConflictsWithLibraryImportCanonical = 39, r#"Declaration name '{}' conflicts with a library import due to its canonical form '{}'. Consider using the 'as' keyword to import the library under a different name."# ;
    ErrorDef ErrFilesDisagreeOnLibraryName = 40, r#"Two files in the library disagree about the name of the library"# ;
    ErrorDef ErrMultipleLibrariesWithSameName = 41, r#"There are multiple libraries named '{}'"# ;
    ErrorDef ErrDuplicateLibraryImport = 42, r#"Library {} already imported. Did you require it twice?"# ;
    ErrorDef ErrConflictingLibraryImport = 43, r#"import of library '{}' conflicts with another library import"# ;
    ErrorDef ErrConflictingLibraryImportAlias = 44, r#"import of library '{}' under alias '{}' conflicts with another library import"# ;
    ErrorDef ErrAttributesNotAllowedOnLibraryImport = 45, r#"attributes and doc comments are not allowed on `using` statements"# ;
    ErrorDef ErrUnknownLibrary = 46, r#"Could not find library named {}. Did you include its sources with --files?"# ;
    RetiredDef ErrProtocolComposedMultipleTimes = 47 ;
    ErrorDef ErrOptionalTableMember = 48, r#"Table members cannot be optional"# ;
    ErrorDef ErrOptionalUnionMember = 49, r#"Union members cannot be optional"# ;
    ErrorDef ErrDeprecatedStructDefaults = 50, r#"Struct defaults are deprecated and should not be used (see RFC-0160)"# ;
    ErrorDef ErrUnknownDependentLibrary = 51, r#"Unknown dependent library {} or reference to member of library {}. Did you require it with `using`?"# ;
    ErrorDef ErrNameNotFound = 52, r#"cannot find '{}' in {}"# ;
    ErrorDef ErrNameNotFoundInVersionRange = 10052, r#"cannot find '{}' in {} within the valid version range"# ;
    ErrorDef ErrCannotReferToMember = 53, r#"cannot refer to member of {}"# ;
    ErrorDef ErrMemberNotFound = 54, r#"{} has no member '{}'"# ;
    ErrorDef ErrInvalidReferenceToDeprecated = 55, r#"invalid reference to {}, which is deprecated {} of platform '{}' while {} is not; either remove this reference or mark {} as deprecated"# ;
    ErrorDef ErrInvalidReferenceToDeprecatedOtherPlatform = 56, r#"invalid reference to {}, which is deprecated {} of platform '{}' while {} is not deprecated {} of platform '{}'; either remove this reference or mark {} as deprecated"# ;
    ErrorDef ErrIncludeCycle = 57, r#"There is an includes-cycle in declarations: {}"# ;
    ErrorDef ErrAnonymousNameReference = 58, r#"cannot refer to anonymous name {}"# ;
    ErrorDef ErrInvalidConstantType = 59, r#"invalid constant type {}"# ;
    ErrorDef ErrCannotResolveConstantValue = 60, r#"unable to resolve constant value"# ;
    ErrorDef ErrOrOperatorOnNonPrimitiveValue = 61, r#"Or operator can only be applied to primitive-kinded values"# ;
    ErrorDef ErrNewTypesNotAllowed = 62, r#"newtypes not allowed: type declaration {} defines a new type of the existing {} type, which is not yet supported"# ;
    ErrorDef ErrExpectedValueButGotType = 63, r#"{} is a type, but a value was expected"# ;
    ErrorDef ErrMismatchedNameTypeAssignment = 64, r#"mismatched named type assignment: cannot define a constant or default value of type {} using a value of type {}"# ;
    ErrorDef ErrTypeCannotBeConvertedToType = 65, r#"{} (type {}) cannot be converted to type {}"# ;
    ErrorDef ErrConstantOverflowsType = 66, r#"{} overflows type {}"# ;
    ErrorDef ErrBitsMemberMustBePowerOfTwo = 67, r#"bits members must be powers of two"# ;
    ErrorDef ErrFlexibleEnumMemberWithMaxValue = 68, r#"flexible enums must not have a member with a value of {}, which is reserved for the unknown value. either: remove the member, change its value to something else, or explicitly specify the unknown value with the @unknown attribute. see <https://fuchsia.dev/fuchsia-src/reference/fidl/language/attributes#unknown> for more info."# ;
    ErrorDef ErrBitsTypeMustBeUnsignedIntegralPrimitive = 69, r#"bits may only be of unsigned integral primitive type, found {}"# ;
    ErrorDef ErrEnumTypeMustBeIntegralPrimitive = 70, r#"enums may only be of integral primitive type, found {}"# ;
    ErrorDef ErrUnknownAttributeOnStrictEnumMember = 71, r#"the @unknown attribute can be only be used on flexible enum members."# ;
    ErrorDef ErrUnknownAttributeOnMultipleEnumMembers = 72, r#"the @unknown attribute can be only applied to one enum member."# ;
    ErrorDef ErrComposingNonProtocol = 73, r#"This declaration is not a protocol"# ;
    ErrorDef ErrInvalidMethodPayloadLayoutClass = 74, r#"cannot use {} as a request/response; must use a struct, table, or union"# ;
    ErrorDef ErrInvalidMethodPayloadType = 75, r#"invalid request/response type '{}'; must use a struct, table, or union"# ;
    RetiredDef ErrResponsesWithErrorsMustNotBeEmpty = 76 ;
    ErrorDef ErrEmptyPayloadStructs = 77, r#"(struct {}) is not allowed as a request or response, use () instead"# ;
    RetiredDef ErrDuplicateElementName = 78 ;
    RetiredDef ErrDuplicateElementNameCanonical = 79 ;
    ErrorDef ErrGeneratedZeroValueOrdinal = 80, r#"Ordinal value 0 disallowed."# ;
    ErrorDef ErrDuplicateMethodOrdinal = 81, r#"Multiple methods with the same ordinal in a protocol; previous was at {}."# ;
    ErrorDef ErrInvalidSelectorValue = 82, r#"invalid selector value, must be a method name or a fully qualified method name"# ;
    ErrorDef ErrFuchsiaIoExplicitOrdinals = 83, r#"fuchsia.io must have explicit ordinals (https://fxbug.dev/42157659)"# ;
    ErrorDef ErrPayloadStructHasDefaultMembers = 84, r#"default values are not allowed on members of request/response structs"# ;
    RetiredDef ErrDuplicateServiceMemberName = 85 ;
    RetiredDef ErrStrictUnionMustHaveNonReservedMember = 86 ;
    RetiredDef ErrDuplicateServiceMemberNameCanonical = 87 ;
    ErrorDef ErrOptionalServiceMember = 88, r#"service members cannot be optional"# ;
    RetiredDef ErrDuplicateStructMemberName = 89 ;
    RetiredDef ErrDuplicateStructMemberNameCanonical = 90 ;
    ErrorDef ErrInvalidStructMemberType = 91, r#"struct field {} has an invalid default type {}"# ;
    ErrorDef ErrTableOrdinalTooLarge = 92, r#"ordinal is too large; table ordinals cannot be greater than 64"# ;
    ErrorDef ErrMaxOrdinalNotTable = 93, r#"the 64th ordinal of a table may only contain a table type"# ;
    ErrorDef ErrDuplicateTableFieldOrdinal = 94, r#"multiple table fields with the same ordinal; previous was at {}"# ;
    RetiredDef ErrDuplicateTableFieldName = 95 ;
    RetiredDef ErrDuplicateTableFieldNameCanonical = 96 ;
    ErrorDef ErrDuplicateUnionMemberOrdinal = 97, r#"multiple union fields with the same ordinal; previous was at {}"# ;
    RetiredDef ErrDuplicateUnionMemberName = 98 ;
    RetiredDef ErrDuplicateUnionMemberNameCanonical = 99 ;
    RetiredDef ErrNonDenseOrdinal = 100 ;
    ErrorDef ErrCouldNotResolveSizeBound = 101, r#"unable to resolve size bound"# ;
    ErrorDef ErrCouldNotResolveMember = 102, r#"unable to resolve {} member"# ;
    ErrorDef ErrCouldNotResolveMemberDefault = 103, r#"unable to resolve {} default value"# ;
    ErrorDef ErrCouldNotResolveAttributeArg = 104, r#"unable to resolve attribute argument"# ;
    RetiredDef ErrDuplicateMemberName = 105 ;
    RetiredDef ErrDuplicateMemberNameCanonical = 106 ;
    ErrorDef ErrDuplicateMemberValue = 107, r#"value of {} member '{}' conflicts with previously declared member '{}' at {}"# ;
    RetiredDef ErrDuplicateResourcePropertyName = 108 ;
    RetiredDef ErrDuplicateResourcePropertyNameCanonical = 109 ;
    ErrorDef ErrTypeMustBeResource = 110, r#"{} '{}' may contain handles (due to member '{}' at {}), so it must be declared with the `resource` modifier: `resource {} {}`"# ;
    ErrorDef ErrInlineSizeExceedsLimit = 111, r#"'{}' has an inline size of {} bytes, which exceeds the maximum allowed inline size of {} bytes"# ;
    ErrorDef ErrOnlyClientEndsInServices = 112, r#"service members must be client_end:P"# ;
    ErrorDef ErrMismatchedTransportInServices = 113, r#"service member {} is over the {} transport, but member {} is over the {} transport. Multiple transports are not allowed."# ;
    ErrorDef ErrComposedProtocolTooOpen = 114, r#"{} protocol '{}' cannot compose {} protocol '{}'; composed protocol may not be more open than composing protocol"# ;
    ErrorDef ErrFlexibleTwoWayMethodRequiresOpenProtocol = 115, r#"flexible two-way method may only be defined in an open protocol, not {}"# ;
    ErrorDef ErrFlexibleOneWayMethodInClosedProtocol = 116, r#"flexible {} may only be defined in an open or ajar protocol, not closed"# ;
    ErrorDef ErrHandleUsedInIncompatibleTransport = 117, r#"handle of type {} may not be sent over transport {} used by {}"# ;
    ErrorDef ErrTransportEndUsedInIncompatibleTransport = 118, r#"client_end / server_end of transport type {} may not be sent over transport {} used by {}"# ;
    RetiredDef ErrEventErrorSyntax = 119 ;
    ErrorDef ErrInvalidAttributePlacement = 120, r#"placement of attribute '{}' disallowed here"# ;
    ErrorDef ErrDeprecatedAttribute = 121, r#"attribute '{}' is deprecated"# ;
    ErrorDef ErrDuplicateAttribute = 122, r#"duplicate attribute '{}'; previous was at {}"# ;
    ErrorDef ErrDuplicateAttributeCanonical = 123, r#"attribute '{}' conflicts with attribute '{}' from {}; both are represented by the canonical form '{}'"# ;
    ErrorDef ErrCanOnlyUseStringOrBool = 124, r#"argument '{}' on user-defined attribute '{}' cannot be a numeric value; use a bool or string instead"# ;
    ErrorDef ErrAttributeArgMustNotBeNamed = 125, r#"attributes that take a single argument must not name that argument"# ;
    ErrorDef ErrAttributeArgNotNamed = 126, r#"attributes that take multiple arguments must name all of them explicitly, but '{}' was not"# ;
    ErrorDef ErrMissingRequiredAttributeArg = 127, r#"attribute '{}' is missing the required '{}' argument"# ;
    ErrorDef ErrMissingRequiredAnonymousAttributeArg = 128, r#"attribute '{}' is missing its required argument"# ;
    ErrorDef ErrUnknownAttributeArg = 129, r#"attribute '{}' does not support the '{}' argument"# ;
    ErrorDef ErrDuplicateAttributeArg = 130, r#"attribute '{}' provides the '{}' argument multiple times; previous was at {}"# ;
    ErrorDef ErrDuplicateAttributeArgCanonical = 131, r#"attribute '{}' argument '{}' conflicts with argument '{}' from {}; both are represented by the canonical form '{}'"# ;
    ErrorDef ErrAttributeDisallowsArgs = 132, r#"attribute '{}' does not support arguments"# ;
    ErrorDef ErrAttributeArgRequiresLiteral = 133, r#"argument '{}' of attribute '{}' does not support referencing constants; please use a literal instead"# ;
    RetiredDef ErrAttributeConstraintNotSatisfied = 134 ;
    ErrorDef ErrInvalidDiscoverableName = 135, r#"invalid @discoverable name '{}'; must follow the format 'the.library.name.TheProtocolName'"# ;
    RetiredDef ErrTableCannotBeSimple = 136 ;
    RetiredDef ErrUnionCannotBeSimple = 137 ;
    RetiredDef ErrElementMustBeSimple = 138 ;
    RetiredDef ErrTooManyBytes = 139 ;
    RetiredDef ErrTooManyHandles = 140 ;
    ErrorDef ErrInvalidErrorType = 141, r#"invalid error type: must be int32, uint32 or an enum thereof"# ;
    ErrorDef ErrInvalidTransportType = 142, r#"invalid transport type: got {} expected one of {}"# ;
    RetiredDef ErrBoundIsTooBig = 143 ;
    RetiredDef ErrUnableToParseBound = 144 ;
    WarningDef WarnAttributeTypo = 145, r#"suspect attribute with name '{}'; did you mean '{}'?"# ;
    ErrorDef ErrInvalidGeneratedName = 146, r#"generated name must be a valid identifier"# ;
    ErrorDef ErrAvailableMissingArguments = 147, r#"at least one argument is required: 'added', 'deprecated', or 'removed'"# ;
    ErrorDef ErrNoteWithoutDeprecationOrRemoval = 148, r#"the @available argument 'note' cannot be used without 'deprecated', 'removed', or 'replaced'"# ;
    ErrorDef ErrPlatformNotOnLibrary = 149, r#"the @available argument 'platform' can only be used on the library's @available attribute"# ;
    ErrorDef ErrLibraryAvailabilityMissingAdded = 150, r#"missing 'added' argument on the library's @available attribute"# ;
    ErrorDef ErrMissingLibraryAvailability = 151, r#"to use the @available attribute here, you must also annotate the `library {};` declaration in one of the library's files"# ;
    ErrorDef ErrInvalidPlatform = 152, r#"invalid platform '{}'; must match the regex [a-z][a-z0-9]*"# ;
    ErrorDef ErrInvalidVersion = 153, r#"invalid version '{}'; must be an integer from 1 to 2^31-1 inclusive, or one of the special constants `NEXT` or `HEAD`"# ;
    ErrorDef ErrInvalidAvailabilityOrder = 154, r#"invalid @available attribute; must have {}"# ;
    ErrorDef ErrAvailabilityConflictsWithParent = 155, r#"the argument {}={} conflicts with {}={} at {}; a child element cannot be {} {} its parent element is {}"# ;
    ErrorDef ErrCannotBeOptional = 156, r#"{} cannot be optional"# ;
    ErrorDef ErrMustBeAProtocol = 157, r#"{} must be a protocol"# ;
    ErrorDef ErrCannotBoundTwice = 158, r#"{} cannot bound twice"# ;
    ErrorDef ErrStructCannotBeOptional = 159, r#"structs can no longer be marked optional; please use the new syntax, `box<{}>`"# ;
    ErrorDef ErrCannotIndicateOptionalTwice = 160, r#"{} is already optional, cannot indicate optionality twice"# ;
    ErrorDef ErrMustHaveNonZeroSize = 161, r#"{} must have non-zero size"# ;
    ErrorDef ErrWrongNumberOfLayoutParameters = 162, r#"{} expected {} layout parameter(s), but got {}"# ;
    ErrorDef ErrMultipleConstraintDefinitions = 163, r#"cannot specify multiple constraint sets on a type"# ;
    ErrorDef ErrTooManyConstraints = 164, r#"{} expected at most {} constraints, but got {}"# ;
    ErrorDef ErrExpectedType = 165, r#"expected type but got a literal or constant"# ;
    ErrorDef ErrUnexpectedConstraint = 166, r#"{} failed to resolve constraint"# ;
    ErrorDef ErrCannotConstrainTwice = 167, r#"{} cannot add additional constraint"# ;
    ErrorDef ErrProtocolConstraintRequired = 168, r#"{} requires a protocol as its first constraint"# ;
    ErrorDef ErrBoxCannotBeOptional = 169, r#"cannot specify optionality for box, boxes are optional by default"# ;
    RetiredDef ErrBoxedTypeCannotBeOptional = 170 ;
    ErrorDef ErrCannotBeBoxedShouldBeOptional = 171, r#"type {} cannot be boxed, try using optional instead"# ;
    ErrorDef ErrResourceMustBeUint32Derived = 172, r#"resource {} must be uint32"# ;
    ErrorDef ErrResourceMissingSubtypeProperty = 173, r#"resource {} expected to have the subtype property, but it was missing"# ;
    RetiredDef ErrResourceMissingRightsProperty = 174 ;
    ErrorDef ErrResourceSubtypePropertyMustReferToEnum = 175, r#"the subtype property must be an enum, but wasn't in resource {}"# ;
    RetiredDef ErrHandleSubtypeMustReferToResourceSubtype = 176 ;
    ErrorDef ErrResourceRightsPropertyMustReferToBits = 177, r#"the rights property must be a uint32 or a uint32-based bits, but wasn't defined as such in resource {}"# ;
    ErrorDef ErrUnusedImport = 178, r#"{} imports {} but does not use it; either use it or remove the import"# ;
    ErrorDef ErrNewTypeCannotHaveConstraint = 179, r#"{} is a newtype, which cannot carry constraints"# ;
    ErrorDef ErrExperimentalZxCTypesDisallowed = 180, r#"{} is an experimental type that must be enabled by with `--experimental zx_c_types`"# ;
    ErrorDef ErrReferenceInLibraryAttribute = 181, r#"attributes on the 'library' declaration do not support referencing constants"# ;
    RetiredDef ErrLegacyWithoutRemoval = 182 ;
    RetiredDef ErrLegacyConflictsWithParent = 183 ;
    ErrorDef ErrUnexpectedControlCharacter = 184, r#"unexpected control character in string literal; use the Unicode escape `\\\\u{{}}` instead"# ;
    ErrorDef ErrUnicodeEscapeMissingBraces = 185, r#"Unicode escape must use braces, like `\\\\u{a}` for U+000A"# ;
    ErrorDef ErrUnicodeEscapeUnterminated = 186, r#"Unicode escape is missing a closing brace '}'"# ;
    ErrorDef ErrUnicodeEscapeEmpty = 187, r#"Unicode escape must have at least 1 hex digit"# ;
    ErrorDef ErrUnicodeEscapeTooLong = 188, r#"Unicode escape must have at most 6 hex digits"# ;
    ErrorDef ErrUnicodeEscapeTooLarge = 189, r#"invalid Unicode code point '{}'; maximum is 10FFFF"# ;
    RetiredDef ErrSimpleProtocolMustBeClosed = 190 ;
    ErrorDef ErrMethodMustDefineStrictness = 191, r#"Method {} must explicitly specify strict or flexible. (The default is changing from strict to flexible, and explicit modifiers are mandatory during the migration.)"# ;
    ErrorDef ErrProtocolMustDefineOpenness = 192, r#"Protocol {} must explicitly specify open, ajar, or closed. (The default is changing from closed to open, and explicit modifiers are mandatory during the migration.)"# ;
    ErrorDef ErrCannotBeBoxedNorOptional = 193, r#"type {} cannot be boxed"# ;
    RetiredDef ErrEmptyPayloadStructsWhenResultUnion = 194 ;
    RetiredDef ErrExperimentalOverflowingAttributeMissingExperimentalFlag = 195 ;
    RetiredDef ErrExperimentalOverflowingIncorrectUsage = 196 ;
    ErrorDef ErrOverlayMustBeStrict = 197, r#"overlays must be strict"#, documented = false ;
    ErrorDef ErrOverlayMustBeValue = 198, r#"overlays must be value (not resource) types"#, documented = false ;
    ErrorDef ErrOverlayMemberMustBeValue = 199, r#"overlays may not contain resource members"#, documented = false ;
    RetiredDef ErrOverlayMustNotContainReserved = 200 ;
    ErrorDef ErrPlatformVersionNotSelected = 201, r#"{} belongs to platform '{}', but no version was selected for it; please choose a version N by passing `--available {}:N`"# ;
    RetiredDef ErrTransitionalNotAllowed = 202 ;
    ErrorDef ErrRemovedAndReplaced = 203, r#"the @available arguments 'removed' and 'replaced' are mutually exclusive"# ;
    ErrorDef ErrLibraryReplaced = 204, r#"the @available argument 'replaced' cannot be used on the library declaration; used 'removed' instead"# ;
    ErrorDef ErrInvalidRemoved = 205, r#"{} is marked removed={}, but there is a replacement marked added={} at {}; either change removed={} to replaced={}, or delete the replacement"# ;
    ErrorDef ErrInvalidReplaced = 206, r#"{} is marked replaced={}, but there is no replacement marked added={}; either change replaced={} to removed={}, or define a replacement"# ;
    ErrorDef ErrTypeShapeIntegerOverflow = 207, r#"cannot calculate type shape because of integer overflow in {} {} {}"# ;
    ErrorDef ErrReservedPlatform = 208, r#"platform '{}' is reserved; choose a different platform name using @available(platform=\, added=...)"# ;
    ErrorDef ErrReservedNotAllowed = 209, r#"FIDL no longer supports reserved table or union fields; use @available instead"# ;
    ErrorDef ErrInvalidDiscoverableLocation = 210, r#"invalid @discoverable location '{}'; must be comma separated 'platform' and/or 'external'"# ;
    ErrorDef ErrCannotBeRenamed = 211, r#"the @available argument 'renamed' cannot be used on a {}; it can only be used on members of a declaration"# ;
    ErrorDef ErrRenamedWithoutReplacedOrRemoved = 212, r#"the @available argument 'renamed' cannot be used without 'replaced' or 'removed'"# ;
    ErrorDef ErrRenamedToSameName = 213, r#"renaming to '{}' has no effect because the element is already named '{}'; either remove the 'renamed' argument or choose a different name"# ;
    ErrorDef ErrInvalidRemovedAndRenamed = 214, r#"{} is marked removed={}, renamed=\ but the name '{}' is already used at {}"# ;
    ErrorDef ErrInvalidReplacedAndRenamed = 215, r#"{} is marked replaced={}, renamed=\ but there is no replacement '{}' marked added={}; please define it"# ;
    ErrorDef ErrInvalidRemovedAbi = 216, r#"{} is marked removed={}, but its {} ({}) is reused at {}; use replaced={}, renamed=\ instead of removed={} if you intend to replace the ABI, otherwise choose a different {}"# ;
    ErrorDef ErrInvalidReplacedAbi = 217, r#"{} is marked replaced={}, but its {} ({}) does not match the replacement's {} ({}) at {}; use removed={} if you intend to remove the ABI, otherwise use the same {}"# ;
    ErrorDef ErrInvalidModifierAvailableArgument = 218, r#"invalid argument '{}'; only 'added' and 'removed' are allowed on modifier availabilities"# ;
    ErrorDef ErrCannotChangeMethodStrictness = 219, r#"changing the strictness of a two-way method without error syntax is not allowed because it is ABI breaking"# ;
    ErrorDef ErrResourceForbiddenHere = 221, r#"'resource' appears in declaration annotated '@no_resource'"# ;
    ErrorDef ErrExperimentalNoResource = 222, r#"'@no_resource' is an experimental attribute that must be enabled with --experimental no_resource_attribute"# ;
    ErrorDef ErrNoResourceForbidsCompose = 223, r#"'{}' has the '@no_resource` attribute, and thus cannot compose '{}' unless it is also has the '@no_resource' attribute"# ;
    ErrorDef ErrNullableArray = 10062, "arrays cannot be nullable", documented = false ;
    ErrorDef ErrArraySizeZero = 10161, "arrays cannot have size 0", documented = false ;
    ErrorDef ErrArrayConstraint = 1001, "arrays cannot have constraints", documented = false ;
    ErrorDef ErrExpectedValue = 1003, "expected value", documented = false ;
    ErrorDef ErrBitsMemberDuplicateName = 1006, "bits member name duplicated", documented = false ;
    ErrorDef ErrBitsMemberDuplicateValue = 1007, "bits member value duplicated", documented = false ;
    ErrorDef ErrBitsTypeMustBeUnsigned = 1008, "bits type must be an unsigned integer", documented = false ;
    ErrorDef ErrCannotBeNullable = 1009, "value cannot be nullable", documented = false ;
    ErrorDef ErrCannotHaveConstraints = 1010, "value cannot have constraints", documented = false ;
    ErrorDef ErrStrictBitsMustHaveMembers = 1011, "strict bits must have at least one member", documented = false ;
    ErrorDef ErrMemberOverflow = 1012, "member value overflows its underlying type", documented = false ;
    ErrorDef ErrInvalidMemberValue = 1013, "invalid or unparseable member value", documented = false ;
    ErrorDef ErrDuplicateMethodName = 1015, "duplicate method name", documented = false ;
    ErrorDef ErrFlexibleProtocolCannotBeEmpty = 1016, "flexible protocol cannot be empty", documented = false ;
    ErrorDef ErrStrictProtocolCannotBeEmpty = 1017, "strict protocol cannot be empty", documented = false ;
    ErrorDef ErrEmptyProtocolMember = 1018, "protocol member cannot be empty", documented = false ;
    ErrorDef ErrInvalidCompose = 1019, "invalid compose", documented = false ;
    ErrorDef ErrMethodEmptyPayload = 1020, "method payload cannot be empty struct", documented = false ;
    ErrorDef ErrNoStrictOnCompose = 1021, "compose cannot be strict", documented = false ;
    ErrorDef ErrOneWayError = 1022, "one-way method cannot have error", documented = false ;
    ErrorDef ErrRequestMustBeProtocol = 1023, "request type must be a protocol", documented = false ;
    ErrorDef ErrRequestMustBeParameterized = 1024, "request type must be parameterized", documented = false ;
    ErrorDef ErrDisallowedRequestType = 1025, "request type must be struct, table, or union", documented = false ;
    ErrorDef ErrDisallowedResponseType = 1026, "response type must be struct, table, or union", documented = false ;
}
