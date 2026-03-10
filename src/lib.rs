#![allow(unused_crate_dependencies)]
pub mod cli;
pub mod compiler;
pub mod diagnostics;
pub mod experimental_flags;
pub mod json_generator;
pub mod flat_ast;
pub mod lexer;
pub mod name;
pub mod parser;
pub mod raw_ast;
pub mod reporter;
pub mod source_file;
pub mod source_span;
pub mod token;
pub mod versioning_types;

pub mod attribute_schema;
pub mod availability_step;
pub mod compile_step;
pub mod consume_step;
pub mod replacement_step;
pub mod resolve_step;
pub mod step;
#[cfg(test)]
pub mod tests;
