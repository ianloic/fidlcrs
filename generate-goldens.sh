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
    "arrays.test.fidl"
    "vectors.test.fidl"
    "service.test.fidl"
    "escaping.test.fidl"
    "doc_comments.test.fidl"



    # "anonymous.test.fidl"
    # "nullable.test.fidl"


    # "bits_constants.test.fidl"
    # "byte_and_bytes.test.fidl"
    # "constants.test.fidl"
    # "consts.test.fidl"
    # "driver_handle.test.fidl"
    # "driver_one_way.test.fidl"
    # "driver_service.test.fidl"
    # "driver_two_way.test.fidl"
    # "empty_struct.test.fidl"
    # "encapsulated_structs.test.fidl"
    # "error.test.fidl"
    # "experimental_maybe_from_alias.test.fidl"
    # "experimental_zx_c_types.test.fidl"
    # "handles.test.fidl"
    # "handles_in_types.test.fidl"
    # "inheritance.test.fidl"
    # "inheritance_with_recursive_decl.test.fidl"
    # "large_messages.test.fidl"
    # "new_type.test.fidl"
    # "overlay.test.fidl"
    # "padding.test.fidl"
    # "protocol_request.test.fidl"
    # "protocols.test.fidl"
    # "request_flexible_envelope.test.fidl"
    # "serializable.test.fidl"
    # "string_arrays.test.fidl"
    # "time.test.fidl"
    # "types_in_protocols.test.fidl"
    # "union_sandwich.test.fidl"
    # "unknown_interactions.test.fidl"
    # "versions.test.fidl"
)

for file in "${FILES[@]}"; do
    name="${file%.test.fidl}"
    
    public_deps=( "//zircon/vdso/zx" )
    experimental_flags=()
    versioned=""
    contains_drivers=false

    if [ "$file" == "handles.test.fidl" ]; then
        public_deps+=( "//sdk/fidl/fdf" )
    fi

    if [ "$file" == "new_type.test.fidl" ]; then
        experimental_flags+=( "allow_new_types" )
    elif [ "$file" == "experimental_zx_c_types.test.fidl" ] || [ "$file" == "string_arrays.test.fidl" ] || [ "$file" == "overlay.test.fidl" ]; then
        experimental_flags+=( "zx_c_types" )
    fi

    if [ "$file" == "versions.test.fidl" ]; then
        versioned="test:1"
    fi

    if [[ "$file" == "driver_handle.test.fidl" || "$file" == "driver_one_way.test.fidl" || "$file" == "driver_service.test.fidl" || "$file" == "driver_two_way.test.fidl" || "$file" == "handles.test.fidl" || "$file" == "unknown_interactions.test.fidl" ]]; then
        contains_drivers=true
    fi

    CMD=("./target/debug/fidlcrs" "--json" "goldens/$name.json")

    for flag in "${experimental_flags[@]}"; do
        CMD+=("--experimental" "$flag")
    done

    if [ -n "$versioned" ]; then
        CMD+=("--versioned" "$versioned")
    fi

    CMD+=("--files" "fidlc/testdata/$file")

    echo "Generating JSON IR for $file -> $name.json"
    "${CMD[@]}"
    diff -u "fidlc/goldens/$name.json.golden" "goldens/$name.json" || true
done

echo "Done generating goldens."
