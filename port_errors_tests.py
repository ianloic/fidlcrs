import re

with open("fidlc/tests/errors_tests.cc", "r") as f:
    text = f.read()

tests = []
for match in re.finditer(r"TEST\(ErrorsTests,\s*(\w+)\)\s*\{", text):
    name = match.group(1)
    # find the matching closing brace
    start = match.end()
    brace_count = 1
    i = start
    while i < len(text) and brace_count > 0:
        if text[i] == "{":
            brace_count += 1
        elif text[i] == "}":
            brace_count -= 1
        i += 1
    body = text[start:i-1]
    tests.append((name, body))

def camel_to_snake(name):
    s1 = re.sub("(.)([A-Z][a-z]+)", r"\1_\2", name)
    return re.sub("([a-z0-9])([A-Z])", r"\1_\2", s1).lower()

out = "use crate::source_file::SourceFile;\n"
out += "use crate::tests::test_library::TestLibrary;\n\n"

for name, body in tests:
    snake_name = camel_to_snake(name)
    fidl_blocks = re.findall(r"R\"FIDL\((.*?)\)FIDL\"", body, re.DOTALL)
    
    out += f"#[test]\n"
    if "ASSERT_COMPILED" in body:
        out += f"fn {snake_name}() {{\n"
        out += "    let mut library = TestLibrary::new();\n"
        
        flags = re.findall(r"library\.EnableFlag\(ExperimentalFlags::Flag::(\w+)\);", body)
        for flag in flags:
            rust_flag = camel_to_snake(flag[1:])
            out += f"    library.enable_flag(\"{rust_flag}\");\n"
            
        for i, fidl in enumerate(fidl_blocks):
            out += f"    let source{i} = SourceFile::new(\"example{i}.fidl\".to_string(), r#\"\n{fidl}\"#.to_string());\n"
            out += f"    library.add_source(&source{i});\n"
        out += "    library.compile().expect(\"compilation failed\");\n"
        
        for line in body.splitlines():
            if "ASSERT_COMPILED" in line or "R\"FIDL(" in line or ")FIDL\"" in line or "TestLibrary " in line or "EnableFlag" in line or not line.strip():
                continue
            if "auto " in line or "ASSERT_" in line or "->" in line:
                out += f"    // TODO: port AST checks: {line.strip()}\n"

        out += "}\n\n"
    elif "ASSERT_ERRORED" in body or "ASSERT_COMPILER_DIAGNOSTICS" in body:
        out += f"fn {snake_name}() {{\n"
        out += "    let mut library = TestLibrary::new();\n"
        
        flags = re.findall(r"library\.EnableFlag\(ExperimentalFlags::Flag::(\w+)\);", body)
        for flag in flags:
            rust_flag = camel_to_snake(flag[1:])
            out += f"    library.enable_flag(\"{rust_flag}\");\n"
            
        for i, fidl in enumerate(fidl_blocks):
            out += f"    let source{i} = SourceFile::new(\"example{i}.fidl\".to_string(), r#\"\n{fidl}\"#.to_string());\n"
            out += f"    library.add_source(&source{i});\n"
        out += "    let result = library.compile();\n"
        out += "    assert!(result.is_err(), \"expected compilation to fail\");\n"
        
        for line in body.splitlines():
            if "ASSERT_ERRORED" in line or "ASSERT_COMPILER_DIAGNOSTICS" in line or "R\"FIDL(" in line or ")FIDL\"" in line or "TestLibrary " in line or "EnableFlag" in line or not line.strip():
                continue
            if "auto " in line or "ASSERT_" in line or "->" in line:
                out += f"    // TODO: port error checks: {line.strip()}\n"
        out += "}\n\n"
    else:
        out += f"#[test]\n"
        out += f"#[ignore]\nfn {snake_name}() {{\n    // TODO: couldn\'t infer success or failure\n}}\n\n"

with open("src/tests/errors_tests.rs", "w") as f:
    f.write(out)
