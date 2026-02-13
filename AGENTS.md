# fidlcrs - Rust Port of the FIDL Compiler

## Project Overview
This project involves porting the FIDL compiler (`fidlc`) from C++ to Rust.
The goal is to produce a compiler that generates **identical JSON IR** to the existing C++ implementation.
The original C++ implementation is in the `fidlc` directory. There are some tests in `fidlc/tests` to be ported too.

The example FIDL libraries in `fidlc/testdata` have been compiled to JSON IR and are stored in `fidlc/goldens/*.json.golden`.

## Codebase
- **Compiler Logic**: `compiler.rs` (AST traversal and IR generation)
- **JSON Generation**: `json_generator.rs` (Serialization to FIDL JSON IR)
- **AST Definition**: `raw_ast.rs` (Matches `fidl_c_ast`)

## Testing
This is a cargo based project that can be tested with `cargo test`.

### Golden Tests
The primary verification mechanism is comparing generated JSON against the official `fidlc` golden files found in `tools/fidl/fidlc/goldens`.

Test runner: `src/golden_test.rs`.

**Workflow:**
1. Open `src/golden_test.rs`.
2. Uncomment the test case you are working on (e.g., `test_struct_golden`).
3. Comment out other tests to reduce noise.
4. Run `cargo test`.
5. If the test fails, debug `compiler.rs` or `json_generator.rs` to match the expected JSON structure.

## Debugging
- Use `eprintln!` for debugging as `println!` might be captured or messy.
- Compare output with `fidlc/goldens/<name>.json.golden`.

## Rules
 - NEVER modify any files in the `fidlc` directory. These are for reference ONLY.
 - there's a JSON Schema for FIDL JSON IR files at `fidlc/schema.json`.
 - If you need to understand how to implement a feature, look at the C++ implementation in `fidlc/src` and try to replicate it in Rust.
 - If you need to create temporary files as part of your work, ONLY create them in the `tmp` directory, and NEVER commit files in `tmp`.

## Discoveries & Gotchas
- **Serde Attributes:** Use `#[serde(skip_serializing_if = "Vec::is_empty", default)]` for optional list fields like `maybe_attributes` in `json_generator.rs` to match the exact JSON emission from the canonical `fidlc` compiler, which completely omits them instead of outputting `[]`.
- **Doc Comments:** In `compiler.rs`, doc comments (`/// ...`) are compiled into a unified `"doc"` attribute. Ensure you only strip `///` (via `&text[3..]`) without explicitly trimming the single following space, as canonical FIDL preserves that space exactly in their generated string values.
- **Parser Traps (Attributes):** In `parser.rs`, modifiers like `closed`, `open`, `ajar`, `strict`, `flexible`, and `resource` can cause token iteration where attributes are parsed prior to the modifier. Ensure `attributes.take()` is correctly passed down into functions like `parse_protocol_declaration` to avoid dropping attributes.
- **Inherited Attributes:** Types like Enums, Structs, and Bits can have outer attributes that must be compounded down to the internal layout nodes. Passing an `inherited_attributes` parameter to methods like `compile_struct` and `.extend()`ing them ensures outer and inner attributes merge correctly in the final output.
