use crate::flat_ast::Type;
use crate::flat_ast::TypeKind;
use crate::source_file::SourceFile;
use crate::tests::test_library::{LookupHelpers, TestLibrary};
use std::fs;

fn get_file_content(path: &str) -> String {
    let full_path = format!("fidlc/tests/fidl/{}", path);
    fs::read_to_string(&full_path).unwrap_or_else(|_| panic!("Failed to read file {}", full_path))
}

#[test]
fn good_simple() {
    let file_content = get_file_content("good/fi-0067-a.test.fidl");

    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "good/fi-0067-a.test.fidl".to_string(),
        file_content,
    ));
    let root = lib.compile().expect("compilation failed");

    let type_decl = root
        .lookup_bits("test.good.fi0067a/Fruit")
        .expect("Fruit bits not found");
    assert_eq!(type_decl.members.len(), 3);
    assert_eq!(type_decl.type_.kind(), TypeKind::Primitive);
    assert_eq!(
        match &type_decl.type_ {
            Type::Primitive(t) => Some(t.subtype.to_string()),
            _ => None,
        }
        .as_deref(),
        Some("uint64")
    );
}

#[test]
fn good_default_uint32() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"library example;

type Fruit = bits {
    ORANGE = 1;
};
"#
        .to_string(),
    ));
    let root = lib.compile().expect("compilation failed");

    let type_decl = root
        .lookup_bits("example/Fruit")
        .expect("Fruit bits not found");
    assert_eq!(type_decl.type_.kind(), TypeKind::Primitive);
    assert_eq!(
        match &type_decl.type_ {
            Type::Primitive(t) => Some(t.subtype.to_string()),
            _ => None,
        }
        .as_deref(),
        Some("uint32")
    );
}

#[test]
fn bad_signed() {
    let file_content = get_file_content("bad/fi-0069.test.fidl");

    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "bad/fi-0069.test.fidl".to_string(),
        file_content,
    ));
    assert!(lib.compile().is_err(), "expected compilation to fail");
}

#[test]
fn bad_non_unique_values() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"library example;

type Fruit = bits : uint64 {
    ORANGE = 1;
    APPLE = 1;
};
"#
        .to_string(),
    ));
    assert!(lib.compile().is_err(), "expected compilation to fail");
}

#[test]
fn bad_non_unique_values_out_of_line() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"library example;

type Fruit = bits {
    ORANGE = FOUR;
    APPLE = TWO_SQUARED;
};

const FOUR uint32 = 4;
const TWO_SQUARED uint32 = 4;
"#
        .to_string(),
    ));
    assert!(lib.compile().is_err(), "expected compilation to fail");
}

#[test]
fn bad_unsigned_with_negative_member() {
    let file_content = get_file_content("bad/fi-0102.test.fidl");

    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "bad/fi-0102.test.fidl".to_string(),
        file_content,
    ));
    assert!(lib.compile().is_err(), "expected compilation to fail");
}

#[test]
fn bad_member_overflow() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"library example;

type Fruit = bits : uint8 {
    ORANGE = 1;
    APPLE = 256;
};
"#
        .to_string(),
    ));
    assert!(lib.compile().is_err(), "expected compilation to fail");
}

#[test]
fn bad_duplicate_member() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"library example;

type Fruit = bits : uint64 {
    ORANGE = 1;
    APPLE = 2;
    ORANGE = 4;
};
"#
        .to_string(),
    ));
    assert!(lib.compile().is_err(), "expected compilation to fail");
}

#[test]
fn bad_no_members_when_strict() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"library example;

type B = strict bits {};
"#
        .to_string(),
    ));
    assert!(lib.compile().is_err(), "expected compilation to fail");
}

#[test]
fn good_no_members_allowed_when_flexible() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"library example;

type B = flexible bits {};
"#
        .to_string(),
    ));
    lib.compile().expect("compilation failed");
}

#[test]
fn good_no_members_allowed_when_defaults_to_flexible() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"library example;

type B = bits {};
"#
        .to_string(),
    ));
    lib.compile().expect("compilation failed");
}

#[test]
fn good_keyword_names() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"library example;

type Fruit = bits : uint64 {
    library = 1;
    bits = 2;
    uint64 = 4;
};
"#
        .to_string(),
    ));
    lib.compile().expect("compilation failed");
}

#[test]
fn bad_non_power_of_two() {
    let file_content = get_file_content("bad/fi-0067.test.fidl");

    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "bad/fi-0067.test.fidl".to_string(),
        file_content,
    ));
    assert!(lib.compile().is_err(), "expected compilation to fail");
}

#[test]
fn good_with_mask() {
    let file_content = get_file_content("good/fi-0067-b.test.fidl");

    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "good/fi-0067-b.test.fidl".to_string(),
        file_content,
    ));
    let root = lib.compile().expect("compilation failed");

    let type_decl = root
        .lookup_bits("test.good.fi0067b/Life")
        .expect("Life bits not found");
    assert_eq!(type_decl.mask, "42");
}

#[test]
fn bad_shant_be_nullable() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"library example;

type NotNullable = bits {
    MEMBER = 1;
};

type Struct = struct {
    not_nullable NotNullable:optional;
};
"#
        .to_string(),
    ));
    assert!(lib.compile().is_err(), "expected compilation to fail");
}

#[test]
fn bad_multiple_constraints() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"library example;

type NotNullable = bits {
    MEMBER = 1;
};

type Struct = struct {
    not_nullable NotNullable:<1, 2, 3>;
};
"#
        .to_string(),
    ));
    assert!(lib.compile().is_err(), "expected compilation to fail");
}
