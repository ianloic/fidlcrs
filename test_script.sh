#!/bin/bash
cargo test bad_no_resource_uses_resource -- --ignored
cargo test bad_no_resource_composition -- --ignored
