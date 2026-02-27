import re

with open('fidlc/src/diagnostics.h', 'r') as f:
    text = f.read()

out = []
out.append("use crate::source_span::SourceSpan;")
out.append("")
out.append("#[derive(Debug, Clone)]")
out.append("pub struct Diagnostic<'a> {")
out.append("    pub id: usize, // ErrorId")
out.append("    pub message: String,")
out.append("    pub span: Option<SourceSpan<'a>>,")
out.append("}")
out.append("")
out.append("pub struct ErrorDef {")
out.append("    pub id: usize,")
out.append("    pub msg: &'static str,")
out.append("}")
out.append("")
out.append("impl ErrorDef {")
out.append("    pub const fn new(id: usize, msg: &'static str) -> Self {")
out.append("        Self { id, msg }")
out.append("    }")
out.append("}")
out.append("")
out.append("macro_rules! define_diagnostic_constant {")
out.append("    (ErrorDef $name:ident = $id:literal, $msg:literal) => {")
out.append("        #[allow(dead_code)]")
out.append("        pub const $name: ErrorDef = ErrorDef::new($id, $msg);")
out.append("    };")
out.append("    (WarningDef $name:ident = $id:literal, $msg:literal) => {")
out.append("        #[allow(dead_code)]")
out.append("        pub const $name: ErrorDef = ErrorDef::new($id, $msg);")
out.append("    };")
out.append("    (RetiredDef $name:ident = $id:literal) => {")
out.append("    };")
out.append("}")
out.append("")
out.append("macro_rules! define_diagnostics {")
out.append("    (")
out.append("        $(")
out.append("            $kind:ident $name:ident = $id:literal $(, $msg:literal)? ;")
out.append("        )*")
out.append("    ) => {")
out.append("        #[allow(dead_code, non_camel_case_types)]")
out.append("        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]")
out.append("        #[repr(usize)]")
out.append("        pub enum ErrorId {")
out.append("            $(")
out.append("                $name = $id,")
out.append("            )*")
out.append("        }")
out.append("")
out.append("        $(")
out.append("            define_diagnostic_constant!($kind $name = $id $(, $msg)?);")
out.append("        )*")
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
    
    # format name from CamelCase to UPPER_SNAKE_CASE
    snake_name = re.sub('([a-z0-9])([A-Z])', r'\1_\2', name).upper()
    
    if def_type == 'RetiredDef':
        out.append(f"    RetiredDef {snake_name} = {id_str} ;")
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
            
            out.append(f"    {def_type} {snake_name} = {id_str}, r#\"{rust_string}\"# ;")

out.append("}")
out.append("")

out.append("""// Legacy placeholder error constants
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
""")

with open('src/diagnostics.rs', 'w') as f:
    f.write("\n".join(out) + "\n")
