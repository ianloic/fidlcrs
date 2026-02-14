#[cfg(test)]
mod tests {
    use crate::test_library::{TestLibrary, LookupHelpers};
    use crate::source_file::SourceFile;
    use std::fs;

    fn get_file_content(path: &str) -> String {
        let full_path = format!("fidlc/tests/fidl/{}", path);
        fs::read_to_string(&full_path)
            .unwrap_or_else(|_| panic!("Failed to read file {}", full_path))
    }

    #[test]
    fn good_enum_test_simple() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type Fruit = enum : uint64 {
    ORANGE = 1;
    APPLE = 2;
    BANANA = 3;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let type_decl = root.lookup_enum("example/Fruit").expect("Fruit enum not found");
        assert_eq!(type_decl.members.len(), 3);
        assert_eq!(type_decl.type_, "uint64");
    }

    #[test]
    fn good_enum_default_uint32() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type Fruit = enum {
    ORANGE = 1;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let type_decl = root.lookup_enum("example/Fruit").expect("Fruit enum not found");
        assert_eq!(type_decl.type_, "uint32");
    }

    #[test]
    #[ignore]
    fn bad_enum_test_with_non_unique_values() {
        let file_content = get_file_content("bad/fi-0107.test.fidl");
        let source = SourceFile::new("bad/fi-0107.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]
    #[ignore]
    fn bad_enum_test_with_non_unique_values_out_of_line() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type Fruit = enum {
    ORANGE = FOUR;
    APPLE = TWO_SQUARED;
};

const FOUR uint32 = 4;
const TWO_SQUARED uint32 = 4;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]
    #[ignore]
    fn bad_enum_test_unsigned_with_negative_member() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type Fruit = enum : uint64 {
    ORANGE = 1;
    APPLE = -2;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]
    #[ignore]
    fn bad_enum_test_inferred_unsigned_with_negative_member() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type Fruit = enum {
    ORANGE = 1;
    APPLE = -2;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]
    #[ignore]
    fn bad_enum_test_member_overflow() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type Fruit = enum : uint8 {
    ORANGE = 1;
    APPLE = 256;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]
    #[ignore]
    fn bad_enum_test_float_type() {
        let file_content = get_file_content("bad/fi-0070.test.fidl");
        let source = SourceFile::new("bad/fi-0070.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]
    #[ignore]
    fn bad_enum_test_duplicate_member() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type Fruit = flexible enum {
    ORANGE = 1;
    APPLE = 2;
    ORANGE = 3;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]
    fn good_enum_test_no_members_allowed_when_defaults_to_flexible() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type E = enum {};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_enum_test_no_members_allowed_when_flexible() {
        let file_content = get_file_content("good/fi-0019-a.test.fidl");
        let source = SourceFile::new("good/fi-0019-a.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_enum_test_strict_with_members() {
        let file_content = get_file_content("good/fi-0019-b.test.fidl");
        let source = SourceFile::new("good/fi-0019-b.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_enum_test_no_members_when_strict() {
        let file_content = get_file_content("bad/fi-0019.test.fidl");
        let source = SourceFile::new("bad/fi-0019.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]
    fn good_enum_test_keyword_names() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type Fruit = enum : uint64 {
    library = 1;
    enum = 2;
    uint64 = 3;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_enum_shant_be_nullable() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type NotNullable = enum {
    MEMBER = 1;
};

type Struct = struct {
    not_nullable NotNullable:optional;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]
    #[ignore]
    fn bad_enum_multiple_constraints() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type NotNullable = enum {
    MEMBER = 1;
};

type Struct = struct {
    not_nullable NotNullable:<1, 2, 3>;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]
    fn good_simple_enum() {
        let file_content = get_file_content("good/fi-0008.test.fidl");
        let source = SourceFile::new("good/fi-0008.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }
}
