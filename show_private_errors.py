import json
import sys
from collections import defaultdict

try:
    with open('errors.json', 'r') as f:
        errors = [json.loads(line) for line in f if line.strip()]
except Exception as e:
    print(f"Error reading errors.json: {e}")
    sys.exit(1)

file_errors = defaultdict(list)
for msg in errors:
    if msg.get('reason') == 'compiler-message' and msg.get('message', {}).get('level') == 'error':
        error = msg['message']
        if error.get('spans') and len(error['spans']) > 0:
            span = error['spans'][0]
            file_name = span['file_name']
            line_num = span['line_start']
            col_num = span['column_start']
            message = error['message']
            if "private" in message.lower():
                file_errors[file_name].append(f"  {line_num}:{col_num} - {message}")

for filename, msgs in file_errors.items():
    print(f"--- {filename} ({len(msgs)} errors) ---")
    for m in msgs:
        print(m)
