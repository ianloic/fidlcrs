#![allow(unused_crate_dependencies)]
pub mod compiler;
pub mod diagnostics;
pub mod json_generator;
pub mod lexer;
pub mod name;
pub mod parser;
pub mod raw_ast;
pub mod reporter;
pub mod source_file;
pub mod source_span;
pub mod token;
pub mod versioning_types;

#[cfg(test)]
pub mod alias_tests;
#[cfg(test)]
pub mod array_tests;
#[cfg(test)]
pub mod attributes_tests;
#[cfg(test)]
pub mod bits_tests;
#[cfg(test)]
pub mod canonical_names_tests;
#[cfg(test)]
pub mod compare_generation_tests;
#[cfg(test)]
pub mod consts_tests;
#[cfg(test)]
pub mod declaration_order_tests;
#[cfg(test)]
pub mod direct_dependencies_tests;
#[cfg(test)]
pub mod enums_tests;
#[cfg(test)]
pub mod handle_tests;
#[cfg(test)]
pub mod method_tests;
#[cfg(test)]
pub mod parsing_tests;
#[cfg(test)]
pub mod protocol_tests;
#[cfg(test)]
pub mod resource_tests;
#[cfg(test)]
pub mod resourceness_tests;
#[cfg(test)]
pub mod strictness_tests;
#[cfg(test)]
pub mod structs_tests;
#[cfg(test)]
pub mod test_library;
#[cfg(test)]
pub mod types_tests;
#[cfg(test)]
pub mod typeshape_tests;
#[cfg(test)]
pub mod union_tests;
#[cfg(test)]
pub mod using_tests;
#[cfg(test)]
pub mod virtual_source_tests;

pub mod availability_step;
pub mod compile_step;
pub mod consume_step;
pub mod replacement_step;
pub mod resolve_step;
pub mod service_tests;
pub mod step;
