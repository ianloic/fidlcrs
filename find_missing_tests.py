import os
import re

cpp_tests_dir = "fidlc/tests"
rs_tests_dir = "src/tests"

missing_tests = []

for root, _, files in os.walk(cpp_tests_dir):
    for file in files:
        if file.endswith(".cc"):
            path = os.path.join(root, file)
            with open(path, 'r') as f:
                content = f.read()
                
            # Find all TEST(...) blocks
            # This regex looks for TEST(Suite, Name) { ... }
            # and checks if the body has SharedAmongstLibraries
            tests = re.finditer(r'TEST\s*\(\s*([A-Za-z0-9_]+)\s*,\s*([A-Za-z0-9_]+)\s*\)\s*\{([^}]*)\}', content)
            for match in tests:
                suite = match.group(1)
                name = match.group(2)
                body = match.group(3)
                if "SharedAmongstLibraries" in body:
                    # Convert CamelCase test name to snake_case
                    snake_name = re.sub(r'(?<!^)(?=[A-Z])', '_', name).lower()
                    
                    # Search for snake_name in rust tests
                    found = False
                    for rs_root, _, rs_files in os.walk(rs_tests_dir):
                        for rs_file in rs_files:
                            if rs_file.endswith(".rs"):
                                with open(os.path.join(rs_root, rs_file), 'r') as rs_f:
                                    if snake_name in rs_f.read().lower():
                                        found = True
                                        break
                        if found:
                            break
                    if not found:
                        missing_tests.append(f"{file}: {name} -> {snake_name}")

for t in missing_tests:
    print(t)
