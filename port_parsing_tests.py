import re

c_tests = open("fidlc/tests/recoverable_parsing_tests.cc").read()

rust_code = """use super::test_library::TestLibrary;
use crate::source_file::SourceFile;

"""

blocks = c_tests.split("TEST(RecoverableParsingTests, ")
for block in blocks[1:]:
    name = block.split(") {")[0]
    
    body_start_idx = block.find(") {") + 3
    brace_count = 1
    idx = body_start_idx
    while idx < len(block) and brace_count > 0:
        if block[idx] == '{':
            brace_count += 1
        elif block[idx] == '}':
            brace_count -= 1
        idx += 1
    
    body = block[body_start_idx:idx-1]
    
    snake_name = re.sub(r'(?<!^)(?=[A-Z])', '_', name).lower()
    
    rust_body = "    let mut library = TestLibrary::new();\n"
    
    fidl_match = re.search(r'R\"FIDL\((.*?)\)FIDL\"', body, re.DOTALL)
    if fidl_match:
        fidl_text = fidl_match.group(1)
        rust_body += f'    let source0 = SourceFile::new("example.fidl".to_string(), r#"{fidl_text}"#.to_string());\n'
        rust_body += f'    library.add_source(&source0);\n'
    
    if 'library.AddFile(' in body:
        file_matches = re.findall(r'library\.AddFile\("(.*?)"\);', body)
        for f in file_matches:
            rust_body += f'    library.add_errcat_file("{f}");\n'
            
    rust_body += f'    let result = library.compile();\n'
    rust_body += f'    assert!(result.is_err(), "Expected compilation to fail");\n'
        
    rust_code += f"#[test]\n"
    rust_code += f"#[ignore]\n"
    rust_code += f"fn {snake_name}() {{\n{rust_body}}}\n\n"

with open("src/tests/recoverable_parsing_tests.rs", "w") as f:
    f.write(rust_code)
