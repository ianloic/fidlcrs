#![allow(unused_crate_dependencies)]
pub mod compiler;
pub mod diagnostics;
pub mod json_generator;
pub mod lexer;
pub mod parser;
pub mod raw_ast;
pub mod reporter;
pub mod source_file;
pub mod source_span;
pub mod token;
pub mod versioning_types;

#[cfg(test)]
pub mod test_library;
#[cfg(test)]
pub mod enums_tests;
#[cfg(test)]
pub mod parsing_tests;
#[cfg(test)]
pub mod structs_tests;
#[cfg(test)]
pub mod array_tests;
#[cfg(test)]
pub mod bits_tests;
#[cfg(test)]
pub mod protocol_tests;
#[cfg(test)]
pub mod union_tests;
#[cfg(test)]
pub mod consts_tests;

pub mod compile_step;
pub mod consume_step;
pub mod resolve_step;
pub mod step;
pub mod availability_step;
pub mod replacement_step;
