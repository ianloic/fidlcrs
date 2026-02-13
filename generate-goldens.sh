#!/bin/bash
set -e

# Ensure fidlcrs is built and up to date
cargo build

mkdir -p goldens

# List of filenames within fidlc/testdata
FILES=(
    "bits.test.fidl"
    "enum.test.fidl"
    "struct.test.fidl"
    "table.test.fidl"
    "union.test.fidl"
  

    # "anonymous.test.fidl"
    # "arrays.test.fidl"
    # "bits_constants.test.fidl"
    # "byte_and_bytes.test.fidl"
    # "constants.test.fidl"
    # "consts.test.fidl"
    # "doc_comments.test.fidl"
    # "driver_handle.test.fidl"
    # "driver_one_way.test.fidl"
    # "driver_service.test.fidl"
    # "driver_two_way.test.fidl"
    # "empty_struct.test.fidl"
    # "encapsulated_structs.test.fidl"
    # "error.test.fidl"
    # "escaping.test.fidl"
    # "experimental_maybe_from_alias.test.fidl"
    # "experimental_zx_c_types.test.fidl"
    # "handles.test.fidl"
    # "handles_in_types.test.fidl"
    # "inheritance.test.fidl"
    # "inheritance_with_recursive_decl.test.fidl"
    # "large_messages.test.fidl"
    # "new_type.test.fidl"
    # "nullable.test.fidl"
    # "overlay.test.fidl"
    # "padding.test.fidl"
    # "protocol_request.test.fidl"
    # "protocols.test.fidl"
    # "request_flexible_envelope.test.fidl"
    # "serializable.test.fidl"
    # "service.test.fidl"
    # "string_arrays.test.fidl"
    # "time.test.fidl"
    # "types_in_protocols.test.fidl"
    # "union_sandwich.test.fidl"
    # "unknown_interactions.test.fidl"
    # "vectors.test.fidl"
    # "versions.test.fidl"
)

for file in "${FILES[@]}"; do
    name="${file%.test.fidl}"
    echo "Generating JSON IR for $file -> $name.json"
    ./target/debug/fidlcrs "fidlc/testdata/$file" --json "goldens/$name.json"
    diff -u "fidlc/goldens/$name.json.golden" "goldens/$name.json"
done

echo "Done generating goldens."
