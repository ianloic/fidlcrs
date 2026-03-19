import glob
import os
import re

missing_tests = []

# Find all C++ test files
cpp_files = glob.glob("fidlc/tests/*_tests.cc")

def to_snake_case(name):
    return re.sub(r'(?<!^)(?=[A-Z])', '_', name).lower()

for cpp_file in cpp_files:
    basename = os.path.basename(cpp_file)
    rs_basename = basename.replace(".cc", ".rs")
    rs_file = f"src/tests/{rs_basename}"
    
    if not os.path.exists(rs_file):
        continue
        
    with open(cpp_file, "r") as f:
        cpp_content = f.read()
        
    with open(rs_file, "r") as f:
        rs_content = f.read()
        
    # Extract C++ tests
    # Format: TEST(TestSuite, TestName)
    # Could also be TEST_P
    cpp_tests = []
    
    for match in re.finditer(r'TEST(?:_P)?\(\w+,\s*(\w+)\)', cpp_content):
        test_name = match.group(1)
        cpp_tests.append(test_name)
        
    for cpp_test in cpp_tests:
        rs_test_name = to_snake_case(cpp_test)
        
        # Check if the function exists in the Rust file
        # It could be fn test_name() or something similar
        pattern = r'fn\s+' + re.escape(rs_test_name) + r'\s*\('
        if not re.search(pattern, rs_content):
            missing_tests.append(f"{basename}: {cpp_test} -> {rs_test_name}")

with open("missing-tests.txt", "w") as f:
    for t in missing_tests:
        f.write(t + "\n")

print(f"Found {len(missing_tests)} missing tests.")
