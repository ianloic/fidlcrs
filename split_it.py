import sys

def split_out(ranges, out_path, name):
    lines = open('src/compiler/mod.rs').read().splitlines()
    
    out = []
    # Ranges must be sorted ascending logically but we extract them in order
    for start, end in ranges:
        out.extend(lines[start-1:end])
        out.append("")
        
    with open(out_path, 'w') as f:
        f.write('use super::Compiler;\n')
        f.write('use crate::json_generator::*;\n')
        f.write('use crate::raw_ast;\n')
        f.write('use crate::diagnostics::ErrorKind;\n')
        f.write('use std::collections::HashMap;\n')
        f.write('use crate::name::NamingContext;\n')
        f.write('use crate::experimental_flags::ExperimentalFlag;\n')
        f.write('use crate::diagnostics::Error;\n')
        f.write('\nimpl<\'node, \'src> Compiler<\'node, \'src> {\n')
        f.write('\n'.join(out))
        f.write('}\n')
        
    # Delete backwards
    for start, end in sorted(ranges, reverse=True):
        del lines[start-1:end]
        
    # Add mod declaration at the top after the imports.
    # Let's find the first `#[derive(Clone)]` or `pub enum RawDecl` and insert it before that.
    insert_idx = 0
    for i, line in enumerate(lines):
        if line.startswith('pub enum RawDecl'):
            insert_idx = i - 1
            break
            
    lines.insert(insert_idx, f'mod {name};')
    
    with open('src/compiler/mod.rs', 'w') as f:
        f.write('\n'.join(lines))

split_out([(5051, 5080), (5082, 5304), (7808, 7954)], 'src/compiler/attributes.rs', 'attributes')
