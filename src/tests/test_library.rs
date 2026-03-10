use crate::attribute_schema::AttributeSchema;
use crate::compiler::Compiler;
use crate::experimental_flags::ExperimentalFlags;
use crate::flat_ast::*;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::raw_ast;
use crate::reporter::Reporter;
use crate::source_file::{SourceFile, VirtualSourceFile};
use std::cell::RefCell;

pub struct SharedAmongstLibraries {
    pub experimental_flags: Vec<String>,
    pub select_versions: Vec<(String, String)>,
    pub custom_schemas: std::collections::HashMap<String, AttributeSchema>,
    pub all_source_files: Vec<SourceFile>,
}

impl Default for SharedAmongstLibraries {
    fn default() -> Self {
        Self::new()
    }
}

impl SharedAmongstLibraries {
    pub fn new() -> Self {
        Self {
            experimental_flags: Vec::new(),
            select_versions: Vec::new(),
            custom_schemas: std::collections::HashMap::new(),
            all_source_files: Vec::new(),
        }
    }

    pub fn use_library_zx(&mut self) {
        let dummy_zx = SourceFile::new(
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
        );
        self.all_source_files.insert(0, dummy_zx);
    }

    pub fn use_library_fdf(&mut self) {
        let dummy_fdf = SourceFile::new(
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
        );
        self.all_source_files.insert(0, dummy_fdf);
    }
}

pub struct TestLibrary<'a> {
    reporter: Reporter<'a>,
    source_files: Vec<SourceFile>,
    #[allow(dead_code)]
    generated_source_file: VirtualSourceFile,
    pub experimental_flags: Vec<String>,
    pub select_versions: Vec<(String, String)>,
    pub custom_schemas: std::collections::HashMap<String, AttributeSchema>,
    pub shared: Option<RefCell<&'a mut SharedAmongstLibraries>>,
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
            generated_source_file: VirtualSourceFile::new("generated".to_string()),
            experimental_flags: Vec::new(),
            select_versions: Vec::new(),
            custom_schemas: std::collections::HashMap::new(),
            shared: None,
        }
    }

    pub fn with_shared(shared: &'a mut SharedAmongstLibraries) -> Self {
        let mut lib = Self::new();
        lib.experimental_flags = shared.experimental_flags.clone();
        lib.select_versions = shared.select_versions.clone();
        lib.custom_schemas = shared.custom_schemas.clone();
        // Add all previously compiled files
        for sf in &shared.all_source_files {
            lib.source_files.push(SourceFile::new(
                sf.filename().to_string(),
                sf.data().to_string(),
            ));
        }
        lib.shared = Some(RefCell::new(shared));
        lib
    }

    pub fn with_source_file(filename: &str, contents: &str) -> Self {
        let mut lib = Self::new();
        lib.add_source_file(filename, contents);
        lib
    }

    pub fn enable_flag(&mut self, flag: &str) {
        self.experimental_flags.push(flag.to_string());
    }

    pub fn select_version(&mut self, platform: &str, version: &str) {
        self.select_versions
            .push((platform.to_string(), version.to_string()));
    }

    pub fn add_source_file(&mut self, filename: &str, contents: &str) {
        self.source_files
            .push(SourceFile::new(filename.to_string(), contents.to_string()));
    }

    pub fn add_attribute_schema(&mut self, name: &str, schema: AttributeSchema) {
        self.custom_schemas.insert(name.to_string(), schema);
    }

    pub fn add_errcat_file(&mut self, path: &str) {
        use crate::tests::errcat::Errcat;
        let content = Errcat::get_fidl(path)
            .unwrap_or_else(|| panic!("failed to read errcat FIDL: {}", path));
        self.source_files
            .push(SourceFile::new(path.to_string(), content));
    }

    pub fn use_library_zx(&mut self) {
        if let Some(shared_cell) = &self.shared {
            shared_cell.borrow_mut().use_library_zx();
            // Re-sync source files? No, just copy them all when compiling next time?
            // TestLibrary compilation takes `self.source_files` and `shared.all_source_files`.
            // We can just add it to `self.source_files` directly.
        }
        let dummy_zx = SourceFile::new(
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
        );
        self.source_files.insert(0, dummy_zx);
    }

    pub fn use_library_fdf(&mut self) {
        if let Some(shared_cell) = &self.shared {
            shared_cell.borrow_mut().use_library_fdf();
        }
        let dummy_fdf = SourceFile::new(
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
        );
        self.source_files.insert(0, dummy_fdf);
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

        let res = compiler.compile(
            &main_asts,
            &dep_asts,
            &self.source_files.iter().collect::<Vec<_>>(),
        );
        if !self.reporter.diagnostics().is_empty() {
            for err in self.reporter.diagnostics().iter() {
                println!("{:?}", err);
            }
            return Err("Compilation failed".to_string());
        }

        // Check for warnings? TestLibrary doesn't do warnings as errors unless flag set, but here we can just update shared.

        // Actually, update shared with OUR new files so next time they are dependencies!
        if res.is_ok() {
            if let Some(shared_cell) = &self.shared {
                let mut shared = shared_cell.borrow_mut();
                shared.experimental_flags = self.experimental_flags.clone();
                shared.select_versions = self.select_versions.clone();
                shared.custom_schemas = self.custom_schemas.clone();
                let old_len = shared.all_source_files.len();
                for sf in self.source_files.iter().skip(old_len) {
                    shared.all_source_files.push(SourceFile::new(
                        sf.filename().to_string(),
                        sf.data().to_string(),
                    ));
                }
            }
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
    let mut lib = TestLibrary::new();
    lib.add_source_file("example.fidl", "library example; struct Foo { x uint32; };");
    let root = lib.compile().expect("compilation failed");
    assert_eq!(root.name, "example");
    assert!(root.lookup_struct("example/Foo").is_some());
}
