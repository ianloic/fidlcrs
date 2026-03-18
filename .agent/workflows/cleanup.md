---
description: Clean up coding style
---

We're going to clean up the coding style of the Rust source code in the src/ directory and its subdirectories.

1. If there are any crate-relative references (ie: `crate::...`) to types in code (ie: outside of `use` statements):
  - do a top-level `use crate::..` of the module containing the code
  - drop all of `crate::` prefixes to that module name in file
2. If there are types from another module that can be imported directly without introducing any name conflicts then do that.
3. Move all `use` statments to the top of their containing block.
4. `cargo fmt` to ensure that code is formatted correctly.
5. `cargo test` to ensure that we haven't introduced any regressions.