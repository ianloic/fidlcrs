#!/bin/bash

set -e

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)

function bind_mount() {
  FUCHSIA_SUBDIR="$1"
  MOUNT_SUBDIR="$2"
  mkdir -p "$SCRIPT_DIR/$MOUNT_SUBDIR"
  sudo mount --bind -o ro ~/fuchsia/$FUCHSIA_SUBDIR "$SCRIPT_DIR/$MOUNT_SUBDIR"
}

bind_mount "tools/fidl/fidlc" "fidlc"
bind_mount "sdk/fidl" "sdk-fidl"
bind_mount "out/default/fidling/gen" "sdk-fidl-gen"