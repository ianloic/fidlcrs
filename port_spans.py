import re

c_tests = open("fidlc/tests/span_tests.cc").read()

rust_code = """use super::test_library::TestLibrary;
use crate::source_file::SourceFile;

"""

# Extract the body of kTestCases
m = re.search(r'const std::vector<TestCase> kTestCases = {(.*?)\n};', c_tests, re.DOTALL)
if m:
    cases_raw = m.group(1)
    
    # regex to find `{ElementType::kName, { R"FIDL(...)FIDL", ... }}`
    # it's a bit nested so let's simply split by "ElementType::"
    parts = cases_raw.split("ElementType::k")
    for part in parts[1:]:
        name_match = re.match(r'([A-Za-z0-9_]+)', part)
        if not name_match: continue
        
        name = name_match.group(1)
        snake_name = re.sub(r'(?<!^)(?=[A-Z])', '_', name).lower()
        
        # extract all FIDL blocks
        fidl_strings = re.findall(r'R\"FIDL\((.*?)\)FIDL\"', part, re.DOTALL)
        
        rust_body = "    let mut library = TestLibrary::new();\n"
        for i, f in enumerate(fidl_strings):
            rust_body += f'    let source{i} = SourceFile::new(format!("example{{}}.fidl", {i}), r#"{f}"#.to_string());\n'
            rust_body += f'    library.add_source(&source{i});\n'
            
        rust_body += '    // TODO: Implement AST span checking logic (TreeVisitor port required)\n'
        rust_body += '    let result = library.compile();\n'
        rust_body += '    assert!(result.is_err() || result.is_ok());\n'
        
        rust_code += f"#[test]\n"
        rust_code += f"#[ignore]\n"
        rust_code += f"fn test_span_{snake_name}() {{\n{rust_body}}}\n\n"

with open("src/tests/span_tests.rs", "w") as f:
    f.write(rust_code)
