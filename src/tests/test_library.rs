use crate::attribute_schema::AttributeSchema;
use crate::compiler::Compiler;
use crate::experimental_flags::ExperimentalFlags;
use crate::flat_ast::*;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::raw_ast;
use crate::reporter::Reporter;
use crate::source_file::{SourceFile, VirtualSourceFile};

pub struct TestLibrary<'a> {
    reporter: Reporter<'a>,
    source_files: Vec<&'a SourceFile>,
    shared_sources: Vec<Box<SourceFile>>,
    #[allow(dead_code)]
    generated_source_file: VirtualSourceFile,
    pub experimental_flags: Vec<String>,
    pub select_versions: Vec<(String, String)>,
    pub custom_schemas: std::collections::HashMap<String, AttributeSchema>,
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
            shared_sources: Vec::new(),
            generated_source_file: VirtualSourceFile::new("generated".to_string()),
            experimental_flags: Vec::new(),
            select_versions: Vec::new(),
            custom_schemas: std::collections::HashMap::new(),
        }
    }

    pub fn with_source(source_file: &'a SourceFile) -> Self {
        let mut lib = Self::new();
        lib.add_source(source_file);
        lib
    }

    pub fn enable_flag(&mut self, flag: &str) {
        self.experimental_flags.push(flag.to_string());
    }

    pub fn select_version(&mut self, platform: &str, version: &str) {
        self.select_versions
            .push((platform.to_string(), version.to_string()));
    }

    pub fn add_source(&mut self, source_file: &'a SourceFile) {
        self.source_files.push(source_file);
    }

    pub fn add_attribute_schema(&mut self, name: &str, schema: AttributeSchema) {
        self.custom_schemas.insert(name.to_string(), schema);
    }

    pub fn add_errcat_file(&mut self, path: &str) {
        use crate::tests::errcat::Errcat;
        let content = Errcat::get_fidl(path)
            .unwrap_or_else(|| panic!("failed to read errcat FIDL: {}", path));
        let dummy = Box::new(SourceFile::new(path.to_string(), content));
        let ptr: *const SourceFile = &*dummy;
        self.shared_sources.push(dummy);
        // Safety: the Box lives until the TestLibrary is dropped so this reference is valid.
        self.source_files.push(unsafe { &*ptr });
    }

    pub fn use_library_zx(&mut self) {
        let dummy_zx = Box::new(SourceFile::new(
            "zx.fidl".to_string(),
            r#"
library zx;

type ObjType = strict enum : uint32 {
    NONE = 0;
    PROCESS = 1;
    THREAD = 2;
    VMO = 3;
    CHANNEL = 4;
    EVENT = 5;
    PORT = 6;
};

type Rights = strict bits : uint32 {
    DUPLICATE = 0x00000001;
    TRANSFER = 0x00000002;
};

resource_definition Handle : uint32 {
    properties {
        subtype ObjType;
        rights Rights;
    };
};
"#
            .to_string(),
        ));
        let ptr: *const SourceFile = &*dummy_zx;
        self.shared_sources.push(dummy_zx);
        // Safety: the Box lives until the TestLibrary is dropped so this reference is valid.
        self.source_files.insert(0, unsafe { &*ptr });
    }

    pub fn use_library_fdf(&mut self) {
        let dummy_fdf = Box::new(SourceFile::new(
            "fdf.fidl".to_string(),
            r#"
library fdf;

type ObjType = strict enum : uint32 {
  CHANNEL = 1;
};

resource_definition handle : uint32 {
    properties {
        subtype ObjType;
    };
};
"#
            .to_string(),
        ));
        let ptr: *const SourceFile = &*dummy_fdf;
        self.shared_sources.push(dummy_fdf);
        self.source_files.insert(0, unsafe { &*ptr });
    }

    pub fn compile(&'a self) -> Result<JsonRoot, String> {
        let mut compiler = Compiler::new(&self.reporter);
        let mut flags = ExperimentalFlags::new();
        for f in &self.experimental_flags {
            if let Ok(flag) = f.parse() {
                flags.enable_flag(flag);
            }
        }
        compiler.experimental_flags = flags;
        for (platform, version) in &self.select_versions {
            use crate::versioning_types::{Platform, Version};
            if let Some(p) = Platform::parse(platform) {
                if let Some(v) = Version::parse(version) {
                    let mut versions = std::collections::BTreeSet::new();
                    versions.insert(v);
                    compiler.version_selection.insert(p, versions);
                }
            }
        }
        compiler
            .attribute_schemas
            .schemas
            .extend(self.custom_schemas.clone());
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

        let main_library_name = asts
            .last()
            .and_then(|f| f.library_decl.as_ref().map(|l| l.path.to_string()))
            .unwrap_or_else(|| "unknown".to_string());

        let mut main_asts = Vec::new();
        let mut dep_asts = Vec::new();

        for ast in asts {
            let file_lib = ast
                .library_decl
                .as_ref()
                .map(|l| l.path.to_string())
                .unwrap_or_else(|| "unknown".to_string());
            if file_lib == main_library_name {
                main_asts.push(ast);
            } else {
                dep_asts.push(ast);
            }
        }

        let res = compiler.compile(&main_asts, &dep_asts, &self.source_files);
        if !self.reporter.diagnostics().is_empty() {
            for err in self.reporter.diagnostics().iter() {
                println!("{:?}", err);
            }
            return Err("Compilation failed".to_string());
        }
        res
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
    fn lookup_alias(&self, name: &str) -> Option<&AliasDeclaration>;
    fn lookup_new_type(&self, name: &str) -> Option<&NewTypeDeclaration>;
    fn lookup_service(&self, name: &str) -> Option<&ServiceDeclaration>;
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
        self.union_declarations
            .iter()
            .find(|d| d.name == name)
            .or_else(|| {
                self.overlay_declarations
                    .as_ref()
                    .and_then(|overlays| overlays.iter().find(|d| d.name == name))
            })
    }
    fn lookup_bits(&self, name: &str) -> Option<&BitsDeclaration> {
        self.bits_declarations.iter().find(|d| d.name == name)
    }
    fn lookup_table(&self, name: &str) -> Option<&TableDeclaration> {
        self.table_declarations.iter().find(|d| d.name == name)
    }
    fn lookup_alias(&self, name: &str) -> Option<&AliasDeclaration> {
        self.alias_declarations.iter().find(|d| d.name == name)
    }
    fn lookup_new_type(&self, name: &str) -> Option<&NewTypeDeclaration> {
        self.new_type_declarations.iter().find(|d| d.name == name)
    }
    fn lookup_service(&self, name: &str) -> Option<&ServiceDeclaration> {
        self.service_declarations.iter().find(|d| d.name == name)
    }
}

#[test]
fn test_test_library() {
    let source = SourceFile::new(
        "example.fidl".to_string(),
        "library example; struct Foo { x uint32; };".to_string(),
    );
    let mut lib = TestLibrary::new();
    lib.add_source(&source);
    let root = lib.compile().expect("compilation failed");
    assert_eq!(root.name, "example");
    assert!(root.lookup_struct("example/Foo").is_some());
}
