import re
import os

for root, _, files in os.walk("fidlc/tests"):
    for file in files:
        if file.endswith(".cc"):
            path = os.path.join(root, file)
            with open(path) as f:
                content = f.read()
            for match in re.finditer(r'TEST\s*\(\s*([a-zA-Z0-9_]+)\s*,\s*([a-zA-Z0-9_]+)\s*\)\s*\{', content):
                start = match.end()
                # Find the matching closing brace
                brace_count = 1
                end = start
                while brace_count > 0 and end < len(content):
                    if content[end] == '{':
                        brace_count += 1
                    elif content[end] == '}':
                        brace_count -= 1
                    end += 1
                body = content[start:end]
                if "SharedAmongstLibraries" in body:
                    print(f"{file}::{match.group(2)}")
