use super::test_library::{LookupHelpers, TestLibrary};
use crate::source_file::SourceFile;

#[test]
fn bad_enum_multiple_unknown() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0072.test.fidl");
    let result = library.compile();
    assert!(
        result.is_err(),
        "Expected ErrUnknownAttributeOnMultipleEnumMembers"
    );
}

#[test]
fn bad_enum_max_value_without_unknown_unsigned() {
    {
        let mut library = TestLibrary::new();
        library.add_errcat_file("bad/fi-0068.test.fidl");
        let result = library.compile();
        assert!(
            result.is_err(),
            "Expected ErrFlexibleEnumMemberWithMaxValue"
        );
    }
    {
        let mut library = TestLibrary::new();
        library.add_errcat_file("good/fi-0068-a.test.fidl");
        let result = library.compile();
        assert!(result.is_ok(), "Expected compilation to succeed");
    }
    {
        let mut library = TestLibrary::new();
        library.add_errcat_file("good/fi-0068-b.test.fidl");
        let result = library.compile();
        assert!(result.is_ok(), "Expected compilation to succeed");
    }
}

#[test]
fn bad_enum_max_value_without_unknown_signed() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example0.fidl".to_string(),
        r#"
library example;

type Foo = flexible enum : int8 {
  ZERO = 0;
  ONE = 1;
  MAX = 127;
};
    "#
        .to_string(),
    );
    library.add_source(&source);
    let result = library.compile();
    assert!(
        result.is_err(),
        "Expected ErrFlexibleEnumMemberWithMaxValue"
    );
}

#[test]
fn good_enum_can_use_max_value_if_other_is_unknown_unsigned() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example0.fidl".to_string(),
        r#"
library example;

type Foo = flexible enum : uint8 {
    ZERO = 0;
    @unknown
    ONE = 1;
    MAX = 255;
};
    "#
        .to_string(),
    );
    library.add_source(&source);
    let root = library.compile().expect("Expected compilation to succeed");

    let foo_enum = root
        .lookup_enum("example/Foo")
        .expect("Expected to find Foo enum");
    assert!(foo_enum.maybe_unknown_value.is_some());
    assert_eq!(foo_enum.maybe_unknown_value.unwrap(), 1);
}

#[test]
fn good_enum_can_use_max_value_if_other_is_unknown_signed() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example0.fidl".to_string(),
        r#"
library example;

type Foo = flexible enum : int8 {
    ZERO = 0;
    @unknown
    ONE = 1;
    MAX = 127;
};
    "#
        .to_string(),
    );
    library.add_source(&source);
    let root = library.compile().expect("Expected compilation to succeed");

    let foo_enum = root
        .lookup_enum("example/Foo")
        .expect("Expected to find Foo enum");
    assert!(foo_enum.maybe_unknown_value.is_some());
    assert_eq!(foo_enum.maybe_unknown_value.unwrap(), 1);
}

#[test]
fn good_enum_can_use_zero_as_unknown_value() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example0.fidl".to_string(),
        r#"
library example;

type Foo = flexible enum : int8 {
    @unknown
    ZERO = 0;
    ONE = 1;
    MAX = 127;
};
    "#
        .to_string(),
    );
    library.add_source(&source);
    let root = library.compile().expect("Expected compilation to succeed");

    let foo_enum = root
        .lookup_enum("example/Foo")
        .expect("Expected to find Foo enum");
    assert!(foo_enum.maybe_unknown_value.is_some());
    assert_eq!(foo_enum.maybe_unknown_value.unwrap(), 0);
}
