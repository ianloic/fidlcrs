import re

rust_code = open('src/structs_tests.rs').read()
cpp_code = open('fidlc/tests/structs_tests.cc').read()

def to_snake(name):
    return re.sub(r'(?<!^)(?=[A-Z])', '_', name).lower()

tests = re.findall(r'TEST\(StructsTests,\s*(\w+)\)\s*\{([^}]+)\}', cpp_code)

for tname, tbody in tests:
    snake = to_snake(tname)
    
    # Check if there are ExpectFail in tbody
    errors = re.findall(r'ExpectFail\s*\(\s*(fidlc::)?(Err\w+)', tbody)
    
    if errors:
        asserts = []
        asserts.append("        let errors = lib.reporter.diagnostics();")
        asserts.append(f"        assert_eq!(errors.len(), {len(errors)});")
        for i, err in enumerate(errors):
            asserts.append(f"        assert_eq!(errors[{i}].def, crate::diagnostics::Error::{err[1]});")
        
        rust_pattern = re.compile(r'(fn ' + snake + r'\(\)\s*\{[\s\S]*?lib\.add_source\(&source\);)\s*assert!\(lib\.compile\(\)\.is_err\(\)\);\s*(\})')
        replacement = r'\1\n        assert!(lib.compile().is_err());\n' + '\n'.join(asserts) + r'\n    \2'
        rust_code = rust_pattern.sub(replacement, rust_code)

open('src/structs_tests.rs', 'w').write(rust_code)
