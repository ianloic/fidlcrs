#!/bin/bash
set -e

# Ensure fidlcrs is built and up to date
cargo build

mkdir -p goldens

# List of filenames within fidlc/testdata
FILES=(
    "anonymous.test.fidl"    
    "arrays.test.fidl"
    "bits.test.fidl"
    "doc_comments.test.fidl"
    "empty_struct.test.fidl"
    "encapsulated_structs.test.fidl"
    "enum.test.fidl"
    "error.test.fidl"
    "escaping.test.fidl"
    "large_messages.test.fidl"
    "nullable.test.fidl"
    "padding.test.fidl"
    "protocol_request.test.fidl"
    "protocols.test.fidl"
    "request_flexible_envelope.test.fidl"
    "serializable.test.fidl"
    "service.test.fidl"
    "struct.test.fidl"
    "table.test.fidl"
    "union_sandwich.test.fidl"
    "union.test.fidl"  
    "vectors.test.fidl"

    # "bits_constants.test.fidl"
    # "byte_and_bytes.test.fidl"
    # "constants.test.fidl"
    # "consts.test.fidl"
    # "driver_handle.test.fidl contains_drivers=true"
    # "driver_one_way.test.fidl contains_drivers=true"
    # "driver_service.test.fidl contains_drivers=true"
    # "driver_two_way.test.fidl contains_drivers=true"
    # "experimental_maybe_from_alias.test.fidl"
    # "experimental_zx_c_types.test.fidl experimental=zx_c_types"
    # "handles_in_types.test.fidl"
    # "handles.test.fidl public_deps=//sdk/fidl/fdf contains_drivers=true"
    # "inheritance_with_recursive_decl.test.fidl"
    # "inheritance.test.fidl"
    # "new_type.test.fidl experimental=allow_new_types"
    # "overlay.test.fidl experimental=zx_c_types"
    # "string_arrays.test.fidl experimental=zx_c_types"
    # "time.test.fidl"
    # "types_in_protocols.test.fidl"
    # "unknown_interactions.test.fidl contains_drivers=true"
    # "versions.test.fidl versioned=test:HEAD"
)

for entry in "${FILES[@]}"; do
    set -- $entry
    file=$1
    shift
    name="${file%.test.fidl}"
    
    public_deps=()
    experimental_flags=()
    versioned="fuchsia:42,NEXT,HEAD"
    contains_drivers=false

    while [ $# -gt 0 ]; do
        case "$1" in
            experimental=*)
                experimental_flags+=("${1#*=}")
                ;;
            versioned=*)
                versioned="${1#*=}"
                ;;
            public_deps=*)
                public_deps+=("${1#*=}")
                ;;
            contains_drivers=*)
                contains_drivers="${1#*=}"
                ;;
            *)
                echo "Unknown flag: $1"
                ;;
        esac
        shift
    done

    CMD=("./target/debug/fidlcrs" "--json" "goldens/$name.json")

    for flag in "${experimental_flags[@]}"; do
        CMD+=("--experimental" "$flag")
    done

    if [ -n "$versioned" ]; then
        CMD+=("--versioned" "$versioned")
    fi

    # manually include zx dependency - it's weird so we can't just glob *.fidl from a directory.
    CMD+=("--files" "vdso-fidl/rights.fidl" "vdso-fidl/zx_common.fidl" "vdso-fidl/overview.fidl")

    CMD+=("--files" "fidlc/testdata/$file")

    echo ""
    echo "Running: ${CMD[@]}"
    "${CMD[@]}"
    diff -u "fidlc/goldens/$name.json.golden" "goldens/$name.json" || true
done

