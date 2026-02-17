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

    # "protocols.test.fidl"
    # "versions.test.fidl versioned=test:HEAD"


    "nullable.test.fidl"


    # "bits_constants.test.fidl"
    # "byte_and_bytes.test.fidl"
    # "constants.test.fidl"
    # "consts.test.fidl"
    # "driver_handle.test.fidl contains_drivers=true"
    # "driver_one_way.test.fidl contains_drivers=true"
    # "driver_service.test.fidl contains_drivers=true"
    # "driver_two_way.test.fidl contains_drivers=true"
    # "empty_struct.test.fidl"
    # "encapsulated_structs.test.fidl"
    # "error.test.fidl"
    # "experimental_maybe_from_alias.test.fidl"
    # "experimental_zx_c_types.test.fidl experimental=zx_c_types"
    # "handles.test.fidl public_deps=//sdk/fidl/fdf contains_drivers=true"
    # "handles_in_types.test.fidl"
    # "inheritance.test.fidl"
    # "inheritance_with_recursive_decl.test.fidl"
    # "large_messages.test.fidl"
    # "new_type.test.fidl experimental=allow_new_types"
    # "overlay.test.fidl experimental=zx_c_types"
    # "padding.test.fidl"
    # "protocol_request.test.fidl"
    # "request_flexible_envelope.test.fidl"
    # "serializable.test.fidl"
    # "string_arrays.test.fidl experimental=zx_c_types"
    # "time.test.fidl"
    # "types_in_protocols.test.fidl"
    # "union_sandwich.test.fidl"
    # "unknown_interactions.test.fidl contains_drivers=true"
)

for entry in "${FILES[@]}"; do
    set -- $entry
    file=$1
    shift
    name="${file%.test.fidl}"
    
    public_deps=( "//zircon/vdso/zx" )
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

    CMD+=("--files" "fidlc/testdata/$file")

    echo "Generating JSON IR for $file -> $name.json"
    "${CMD[@]}"
    diff -u "fidlc/goldens/$name.json.golden" "goldens/$name.json" || true
done

echo "Done generating goldens."
