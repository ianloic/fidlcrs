#!/bin/bash

set -e

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)

if [[ "$1" == "unmount" || "$1" == "umount" ]]; then
  ACTION="unmount"
else
  ACTION="mount"
fi

function process_dir() {
  FUCHSIA_SUBDIR="$1"
  MOUNT_SUBDIR="$2"
  if [[ "$ACTION" == "unmount" ]]; then
    sudo umount "$SCRIPT_DIR/$MOUNT_SUBDIR"
  else
    mkdir -p "$SCRIPT_DIR/$MOUNT_SUBDIR"
    sudo mount --bind -o ro ~/fuchsia/$FUCHSIA_SUBDIR "$SCRIPT_DIR/$MOUNT_SUBDIR"
  fi
}

process_dir "tools/fidl/fidlc" "fidlc"
process_dir "sdk/fidl" "sdk-fidl"
process_dir "out/default/fidling/gen/sdk/fidl" "sdk-fidl-gen"
process_dir "zircon/vdso" "vdso-fidl"
process_dir "docs/reference/fidl/language/error-catalog" "errcat-docs"