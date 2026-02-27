import re

with open('fidlc/src/diagnostics.h', 'r') as f:
    text = f.read()

out = []
out.append("use crate::source_span::SourceSpan;")
out.append("")
out.append("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]")
out.append("pub enum ErrorKind {")
out.append("    Error,")
out.append("    Warning,")
out.append("    Retired,")
out.append("}")
out.append("")
out.append("#[derive(Debug, Clone)]")
out.append("pub struct Diagnostic<'a> {")
out.append("    pub def: Error,")
out.append("    pub message: String,")
out.append("    pub span: Option<SourceSpan<'a>>,")
out.append("}")
out.append("")
out.append("macro_rules! __kind_to_enum {")
out.append("    (ErrorDef) => { ErrorKind::Error };")
out.append("    (WarningDef) => { ErrorKind::Warning };")
out.append("    (RetiredDef) => { ErrorKind::Retired };")
out.append("}")
out.append("")
out.append("macro_rules! define_diagnostics {")
out.append("    (")
out.append("        $(")
out.append("            $kind:ident $camel_name:ident = $id:literal $(, $msg:literal)? ;")
out.append("        )*")
out.append("    ) => {")
out.append("        #[allow(dead_code, non_camel_case_types)]")
out.append("        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]")
out.append("        #[repr(usize)]")
out.append("        pub enum Error {")
out.append("            $(")
out.append("                $camel_name = $id,")
out.append("            )*")
out.append("        }")
out.append("")
out.append("        impl Error {")
out.append("            pub const fn id(&self) -> usize {")
out.append("                *self as usize")
out.append("            }")
out.append("")
out.append("            pub const fn msg(&self) -> &'static str {")
out.append("                match self {")
out.append("                    $(")
out.append("                        $(Error::$camel_name => $msg,)?")
out.append("                    )*")
out.append("                    _ => \"\",")
out.append("                }")
out.append("            }")
out.append("")
out.append("            pub const fn kind(&self) -> ErrorKind {")
out.append("                match self {")
out.append("                    $(")
out.append("                        Error::$camel_name => __kind_to_enum!($kind),")
out.append("                    )*")
out.append("                }")
out.append("            }")
out.append("        }")
out.append("")
out.append("")
out.append("    }")
out.append("}")
out.append("")
out.append("define_diagnostics! {")

matches = re.finditer(r'constexpr\s+(ErrorDef|WarningDef|RetiredDef)<([^>]+)>\s+(\w+)(?:\((.*?)\))?\s*;', text, flags=re.DOTALL)

for m in matches:
    def_type = m.group(1)
    args = m.group(2)
    name = m.group(3)
    val = m.group(4)
    
    # parse numeric id
    id_str = args.split(',')[0].strip()
    
    if def_type == 'RetiredDef':
        out.append(f"    RetiredDef {name} = {id_str} ;")
    else:
        if val:
            # handle multi string literals by stripping quotes and whitespace
            strings = re.findall(r'"(.*?)(?<!\\\\)"', val, flags=re.DOTALL)
            string_literal = "".join(strings)
            string_literal = string_literal.replace('\\r\\n', ' ').replace('\\n', ' ').replace('  ', ' ')
            string_literal = string_literal.replace('\\\\"', '"').replace('\\\\', '\\\\\\\\')
            
            # replace C++ `{0}` with `{}`
            rust_string = string_literal
            for i in range(10):
                rust_string = rust_string.replace(f'{{{i}}}', '{}')
            
            out.append(f"    {def_type} {name} = {id_str}, r#\"{rust_string}\"# ;")
out.append("""    ErrorDef ErrNullableArray = 10062, "arrays cannot be nullable" ;
    ErrorDef ErrArraySizeZero = 10161, "arrays cannot have size 0" ;
    ErrorDef ErrArrayConstraint = 1001, "arrays cannot have constraints" ;
    ErrorDef ErrExpectedValue = 1003, "expected value" ;
    ErrorDef ErrBitsMemberDuplicateName = 1006, "bits member name duplicated" ;
    ErrorDef ErrBitsMemberDuplicateValue = 1007, "bits member value duplicated" ;
    ErrorDef ErrBitsTypeMustBeUnsigned = 1008, "bits type must be an unsigned integer" ;
    ErrorDef ErrCannotBeNullable = 1009, "value cannot be nullable" ;
    ErrorDef ErrCannotHaveConstraints = 1010, "value cannot have constraints" ;
    ErrorDef ErrStrictBitsMustHaveMembers = 1011, "strict bits must have at least one member" ;
    ErrorDef ErrMemberOverflow = 1012, "member value overflows its underlying type" ;
    ErrorDef ErrInvalidMemberValue = 1013, "invalid or unparseable member value" ;
    ErrorDef ErrDuplicateMethodName = 1015, "duplicate method name" ;
    ErrorDef ErrFlexibleProtocolCannotBeEmpty = 1016, "flexible protocol cannot be empty" ;
    ErrorDef ErrStrictProtocolCannotBeEmpty = 1017, "strict protocol cannot be empty" ;
    ErrorDef ErrEmptyProtocolMember = 1018, "protocol member cannot be empty" ;
    ErrorDef ErrInvalidCompose = 1019, "invalid compose" ;
    ErrorDef ErrMethodEmptyPayload = 1020, "method payload cannot be empty struct" ;
    ErrorDef ErrNoStrictOnCompose = 1021, "compose cannot be strict" ;
    ErrorDef ErrOneWayError = 1022, "one-way method cannot have error" ;
    ErrorDef ErrRequestMustBeProtocol = 1023, "request type must be a protocol" ;
    ErrorDef ErrRequestMustBeParameterized = 1024, "request type must be parameterized" ;
    ErrorDef ErrDisallowedRequestType = 1025, "request type must be struct, table, or union" ;
    ErrorDef ErrDisallowedResponseType = 1026, "response type must be struct, table, or union" ;""")

out.append("}")
out.append("")



with open('src/diagnostics.rs', 'w') as f:
    f.write("\n".join(out) + "\n")
