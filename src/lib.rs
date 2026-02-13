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

#[cfg(test)]
mod golden_test;

pub fn run() {
    println!("Hello from fidlcrs lib!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
