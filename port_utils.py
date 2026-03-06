import re

cpp_code = open("fidlc/tests/utils_tests.cc").read()

outputs = []

outputs.append("""// Dummy implementations to allow test compilation
fn split_identifier_words(_s: &str) -> Vec<String> { unimplemented!() }
fn is_upper_camel_case(_s: &str) -> bool { unimplemented!() }
fn to_upper_camel_case(_s: &str) -> String { unimplemented!() }
fn is_lower_camel_case(_s: &str) -> bool { unimplemented!() }
fn to_lower_camel_case(_s: &str) -> String { unimplemented!() }
fn is_upper_snake_case(_s: &str) -> bool { unimplemented!() }
fn to_upper_snake_case(_s: &str) -> String { unimplemented!() }
fn is_lower_snake_case(_s: &str) -> bool { unimplemented!() }
fn to_lower_snake_case(_s: &str) -> String { unimplemented!() }
fn is_valid_library_component(_s: &str) -> bool { unimplemented!() }
fn is_valid_identifier_component(_s: &str) -> bool { unimplemented!() }
fn is_valid_fully_qualified_method_identifier(_s: &str) -> bool { unimplemented!() }
fn remove_whitespace(_s: &str) -> String { unimplemented!() }
fn canonicalize(_s: &str) -> String { unimplemented!() }
fn strip_string_literal_quotes(_s: &str) -> String { unimplemented!() }
fn strip_doc_comment_slashes(_s: &str) -> String { unimplemented!() }
fn decode_unicode_hex(_s: &str) -> u32 { unimplemented!() }
fn string_literal_length(_s: &str) -> usize { unimplemented!() }

""")

# Parse IdToWords
def port_id_to_words():
    block = re.search(r'TEST\(UtilsTests, IdToWords.*?{(.*?)}', cpp_code, re.DOTALL)
    out = "#[test]\n#[ignore]\nfn test_id_to_words() {\n"
    if block:
        for m in re.finditer(r'CompareIdToWords\("(.*?)", "(.*?)"\);', block.group(1)):
            out += f'    assert_eq!(split_identifier_words("{m.group(1)}").join(" "), "{m.group(2)}");\n'
    out += "}\n"
    outputs.append(out)

port_id_to_words()

def port_cases():
    for case in ["UpperCamel", "LowerCamel", "UpperSnake", "LowerSnake"]:
        block = re.search(r'TEST\(UtilsTests, ' + case + r'Case.*?{(.*?)}', cpp_code, re.DOTALL)
        snake_case = re.sub(r'(?<!^)(?=[A-Z])', '_', case).lower()
        
        out = f"#[test]\n#[ignore]\nfn test_{snake_case}_case() {{\n"
        if block:
            for m in re.finditer(r'ASSERT(_BAD)?_CASE\([^,]+, "(.*?)", "(.*?)"\);', block.group(1)):
                is_bad = bool(m.group(1))
                val = m.group(2)
                expected = m.group(3)
                out += f'    // From: "{val}", To: "{expected}"\n'
                out += f'    assert_eq!(to_{snake_case}_case("{val}"), "{expected}");\n'
                if not is_bad:
                    out += f'    assert!(is_{snake_case}_case("{expected}"));\n'
                out += f'    assert!(!is_{snake_case}_case("{val}"));\n'
        out += "}\n"
        outputs.append(out)

port_cases()

def port_bool_tests(name, rust_func):
    block = re.search(rf'TEST\(UtilsTests, {name}.*?{{(.*?)}}', cpp_code, re.DOTALL)
    snake_name = re.sub(r'(?<!^)(?=[A-Z])', '_', name).lower()
    out = f"#[test]\n#[ignore]\nfn test_{snake_name}() {{\n"
    if block:
        for m in re.finditer(r'ASSERT_(TRUE|FALSE)\(' + name + r'\("(.*?)"\)\);', block.group(1)):
            val = m.group(1) == "TRUE"
            out += f'    assert_eq!({rust_func}("{m.group(2)}"), {"true" if val else "false"});\n'
    out += "}\n"
    outputs.append(out)

port_bool_tests("IsValidLibraryComponent", "is_valid_library_component")
port_bool_tests("IsValidIdentifierComponent", "is_valid_identifier_component")
port_bool_tests("IsValidFullyQualifiedMethodIdentifier", "is_valid_fully_qualified_method_identifier")

def port_remove_whitespace():
    # Hardcode RemoveWhitespace test to avoid parsing nested C++ braces with Regex
    out = "#[test]\n#[ignore]\nfn test_remove_whitespace() {\n"
    unformatted = r'''
/// C1a
/// C1b
library foo.bar;  // C2

/// C3a
/// C3b
using baz.qux;  // C4

/// C5a
/// C5b
resource_definition thing : uint8 {  // C6
    properties {  // C8
/// C9a
/// C9b
        stuff rights;  // C10
    };
};

/// C11a
/// C11b
const MY_CONST string = "abc";  // C12

/// C13a
/// C13b
type MyEnum = enum {  // C14
/// C15a
/// C17b
    MY_VALUE = 1;  // C16
};

/// C17a
/// C17b
type MyTable = resource table {  // C18
/// C19a
/// C19b
    1: field thing;  // C20
};

/// C21a
/// C21b
alias MyAlias = MyStruct;  // C22

/// C23a
/// C23b
protocol MyProtocol {  // C24
/// C25a
/// C25b
    MyMethod(resource struct {  // C26
/// C27a
/// C27b
        data MyTable;  // C28
    }) -> () error MyEnum;  // C29
};  // 30

/// C29a
/// C29b
service MyService {  // C32
/// C31a
/// C31b
    my_protocol client_end:MyProtocol;  // C34
};  // C35
'''
    formatted = r'''
/// C1a
/// C1b
library foo.bar; // C2

/// C3a
/// C3b
using baz.qux; // C4

/// C5a
/// C5b
resource_definition thing : uint8 { // C6
    properties { // C8
        /// C9a
        /// C9b
        stuff rights; // C10
    };
};

/// C11a
/// C11b
const MY_CONST string = "abc"; // C12

/// C13a
/// C13b
type MyEnum = enum { // C14
    /// C15a
    /// C17b
    MY_VALUE = 1; // C16
};

/// C17a
/// C17b
type MyTable = resource table { // C18
    /// C19a
    /// C19b
    1: field thing; // C20
};

/// C21a
/// C21b
alias MyAlias = MyStruct; // C22

/// C23a
/// C23b
protocol MyProtocol { // C24
    /// C25a
    /// C25b
    MyMethod(resource struct { // C26
        /// C27a
        /// C27b
        data MyTable; // C28
    }) -> () error MyEnum; // C29
}; // 30

/// C29a
/// C29b
service MyService { // C32
    /// C31a
    /// C31b
    my_protocol client_end:MyProtocol; // C34
}; // C35
'''
    out += f'    let unformatted = r#"{unformatted}"#;\n'
    out += f'    let formatted = r#"{formatted}"#;\n'
    out += '    assert_eq!(remove_whitespace(unformatted), remove_whitespace(formatted));\n'
    out += "}\n"
    outputs.append(out)
port_remove_whitespace()


def port_canonicalize():
    block = re.search(r'TEST\(UtilsTests, Canonicalize.*?{(.*?)}', cpp_code, re.DOTALL)
    out = "#[test]\n#[ignore]\nfn test_canonicalize() {\n"
    if block:
        for m in re.finditer(r'EXPECT_EQ\(Canonicalize\("(.*?)"\), "(.*?)"\);', block.group(1)):
            out += f'    assert_eq!(canonicalize("{m.group(1)}"), "{m.group(2)}");\n'
    out += "}\n"
    outputs.append(out)
port_canonicalize()

def port_strip_string_literal_quotes():
    block = re.search(r'TEST\(UtilsTests, StripStringLiteralQuotes.*?{(.*?)}', cpp_code, re.DOTALL)
    out = "#[test]\n#[ignore]\nfn test_strip_string_literal_quotes() {\n"
    if block:
        for m in re.finditer(r'EXPECT_EQ\(StripStringLiteralQuotes\("(.*?)"\), "(.*?)"\);', block.group(1)):
            c1 = m.group(1).replace(r'\"', r'"')
            c2 = m.group(2)
            out += f'    assert_eq!(strip_string_literal_quotes(r#"{c1}"#), r#"{c2}"#);\n'
    out += "}\n"
    outputs.append(out)
port_strip_string_literal_quotes()

def port_strip_doc_comment_slashes():
    block = re.search(r'TEST\(UtilsTests, StripDocCommentSlashes.*?{(.*?)}', cpp_code, re.DOTALL)
    out = "#[test]\n#[ignore]\nfn test_strip_doc_comment_slashes() {\n"
    if block:
        for m in re.finditer(r'EXPECT_EQ\(StripDocCommentSlashes\(R"FIDL\((.*?)\)FIDL"\),\s*"(.*?)"\);', block.group(1), re.DOTALL):
            out += f'    assert_eq!(strip_doc_comment_slashes(r#"{m.group(1)}"#), "{m.group(2).replace(chr(10), "")}");\n'
    out += "}\n"
    outputs.append(out)
port_strip_doc_comment_slashes()

def port_decode_unicode_hex():
    block = re.search(r'TEST\(UtilsTests, DecodeUnicodeHex.*?{(.*?)}', cpp_code, re.DOTALL)
    out = "#[test]\n#[ignore]\nfn test_decode_unicode_hex() {\n"
    if block:
        for m in re.finditer(r'EXPECT_EQ\(DecodeUnicodeHex\("(.*?)"\), (.*?)(u)?\);', block.group(1)):
            out += f'    assert_eq!(decode_unicode_hex("{m.group(1)}"), {m.group(2)});\n'
    out += "}\n"
    outputs.append(out)
port_decode_unicode_hex()

def port_string_literal_length():
    block = re.search(r'TEST\(UtilsTests, StringLiteralLength.*?{(.*?)}', cpp_code, re.DOTALL)
    out = "#[test]\n#[ignore]\nfn test_string_literal_length() {\n"
    if block:
        for m in re.finditer(r'EXPECT_EQ\(StringLiteralLength\(R"\((.*?)\)"\), (.*?)(u)?\);', block.group(1)):
            out += f'    assert_eq!(string_literal_length(r#"{m.group(1)}"#), {m.group(2)});\n'
    out += "}\n"
    outputs.append(out)
port_string_literal_length()

with open("src/tests/utils_tests.rs", "w") as f:
    f.write("\n".join(outputs))

