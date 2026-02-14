use crate::compiler::Compiler;
use crate::json_generator::*;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::raw_ast;
use crate::reporter::Reporter;
use crate::source_file::SourceFile;

pub struct TestLibrary<'a> {
    reporter: Reporter<'a>,
    source_files: Vec<&'a SourceFile>,
}

impl<'a> Default for TestLibrary<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> TestLibrary<'a> {
    pub fn new() -> Self {
        Self {
            reporter: Reporter::new(),
            source_files: Vec::new(),
        }
    }

    pub fn with_source(source_file: &'a SourceFile) -> Self {
        let mut lib = Self::new();
        lib.add_source(source_file);
        lib
    }

    pub fn add_source(&mut self, source_file: &'a SourceFile) {
        self.source_files.push(source_file);
    }

    pub fn compile(&'a self) -> Result<JsonRoot, String> {
        let mut compiler = Compiler::new();
        let mut asts = Vec::new();

        for file in &self.source_files {
            let mut lexer = Lexer::new(file, &self.reporter);
            let mut parser = Parser::new(&mut lexer, &self.reporter);
            if let Some(ast) = parser.parse_file() {
                asts.push(ast);
            } else {
                return Err("Parsing failed".to_string());
            }
        }

        let root = compiler.compile(&asts, &self.source_files);
        Ok(root)
    }

    pub fn parse(&'a self) -> Result<Vec<raw_ast::File<'a>>, String> {
        let mut asts = Vec::new();
        for file in &self.source_files {
            let mut lexer = Lexer::new(file, &self.reporter);
            let mut parser = Parser::new(&mut lexer, &self.reporter);
            if let Some(ast) = parser.parse_file() {
                asts.push(ast);
            } else {
                return Err("Parsing failed".to_string());
            }
        }
        Ok(asts)
    }

    pub fn reporter(&self) -> &Reporter<'a> {
        &self.reporter
    }
}

pub trait LookupHelpers {
    fn lookup_struct(&self, name: &str) -> Option<&StructDeclaration>;
    fn lookup_protocol(&self, name: &str) -> Option<&ProtocolDeclaration>;
    fn lookup_enum(&self, name: &str) -> Option<&EnumDeclaration>;
    fn lookup_union(&self, name: &str) -> Option<&UnionDeclaration>;
    fn lookup_bits(&self, name: &str) -> Option<&BitsDeclaration>;
    fn lookup_table(&self, name: &str) -> Option<&TableDeclaration>;
}

impl LookupHelpers for JsonRoot {
    fn lookup_struct(&self, name: &str) -> Option<&StructDeclaration> {
        self.struct_declarations.iter().find(|d| d.name == name)
    }
    fn lookup_protocol(&self, name: &str) -> Option<&ProtocolDeclaration> {
        self.protocol_declarations.iter().find(|d| d.name == name)
    }
    fn lookup_enum(&self, name: &str) -> Option<&EnumDeclaration> {
        self.enum_declarations.iter().find(|d| d.name == name)
    }
    fn lookup_union(&self, name: &str) -> Option<&UnionDeclaration> {
        self.union_declarations.iter().find(|d| d.name == name)
    }
    fn lookup_bits(&self, name: &str) -> Option<&BitsDeclaration> {
        self.bits_declarations.iter().find(|d| d.name == name)
    }
    fn lookup_table(&self, name: &str) -> Option<&TableDeclaration> {
        self.table_declarations.iter().find(|d| d.name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_library() {
        let source = SourceFile::new("example.fidl".to_string(), "library example; struct Foo { x uint32; };".to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        assert_eq!(root.name, "example");
        assert!(root.lookup_struct("example/Foo").is_some());
    }
}
