#![allow(unused_mut, unused_variables)]
use crate::diagnostics::Error;
use crate::source_file::SourceFile;
use crate::tests::test_library::TestLibrary;

#[test]
fn bad_duplicate_modifier() {
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

type One = strict union { 1: b bool; };
type Two = strict strict union { 1: b bool; };
type Three = strict strict strict union { 1: b bool; };
"#
        .to_string(),
    );
    let mut lib = TestLibrary::new();
    lib.add_source(&source);
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors.len(), 3);
    assert_eq!(errors[0].def, Error::ErrDuplicateModifier);
    assert_eq!(errors[1].def, Error::ErrDuplicateModifier);
    assert_eq!(errors[2].def, Error::ErrDuplicateModifier);
}

#[test]
fn bad_duplicate_modifier_non_consecutive() {
    let content =
        std::fs::read_to_string("fidlc/tests/fidl/bad/fi-0032.noformat.test.fidl").unwrap();
    let source = SourceFile::new("bad/fi-0032.noformat.test.fidl".to_string(), content);
    let mut lib = TestLibrary::new();
    lib.add_source(&source);
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].def, Error::ErrDuplicateModifier);
}

#[test]
fn bad_conflicting_modifiers() {
    let content =
        std::fs::read_to_string("fidlc/tests/fidl/bad/fi-0033.noformat.test.fidl").unwrap();
    let source = SourceFile::new("bad/fi-0033.noformat.test.fidl".to_string(), content);
    let mut lib = TestLibrary::new();
    lib.add_source(&source);
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors.len(), 2);
    assert_eq!(errors[0].def, Error::ErrConflictingModifier);
    assert_eq!(errors[1].def, Error::ErrConflictingModifier);
}

#[test]
fn good_bits_strictness() {
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

type DefaultStrictFoo = strict bits {
    BAR = 0x1;
};

type StrictFoo = strict bits {
    BAR = 0x1;
};

type FlexibleFoo = flexible bits {
    BAR = 0x1;
};
"#
        .to_string(),
    );
    let mut lib = TestLibrary::new();
    lib.add_source(&source);
    lib.compile().unwrap();
}

#[test]
fn good_enum_strictness() {
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

type DefaultStrictFoo = strict enum {
    BAR = 1;
};

type StrictFoo = strict enum {
    BAR = 1;
};

type FlexibleFoo = flexible enum {
    BAR = 1;
};
"#
        .to_string(),
    );
    let mut lib = TestLibrary::new();
    lib.add_source(&source);
    lib.compile().unwrap();
}

#[test]
fn good_flexible_enum() {
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

type Foo = flexible enum {
    BAR = 1;
};
"#
        .to_string(),
    );
    let mut lib = TestLibrary::new();
    lib.add_source(&source);
    lib.compile().unwrap();
}

#[test]
fn good_flexible_bits_redundant() {
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

type Foo = flexible bits {
    BAR = 0x1;
};
"#
        .to_string(),
    );
    let mut lib = TestLibrary::new();
    lib.add_source(&source);
    lib.compile().unwrap();
}

#[test]
fn bad_strictness_struct() {
    let content =
        std::fs::read_to_string("fidlc/tests/fidl/bad/fi-0030.noformat.test.fidl").unwrap();
    let source = SourceFile::new("bad/fi-0030.noformat.test.fidl".to_string(), content);
    let mut lib = TestLibrary::new();
    lib.add_source(&source);
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].def, Error::ErrCannotSpecifyModifier);
}

#[test]
fn bad_strictness_table() {
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

type StrictFoo = strict table {};
"#
        .to_string(),
    );
    let mut lib = TestLibrary::new();
    lib.add_source(&source);
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].def, Error::ErrCannotSpecifyModifier);
}

#[test]
fn good_union_strictness() {
    let content = std::fs::read_to_string("fidlc/tests/fidl/good/fi-0033.test.fidl").unwrap();
    let source = SourceFile::new("good/fi-0033.test.fidl".to_string(), content);
    let mut lib = TestLibrary::new();
    lib.add_source(&source);
    lib.compile().unwrap();
}

#[test]
fn good_strict_union_redundant() {
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

type Foo = strict union {
    1: i int32;
};
"#
        .to_string(),
    );
    let mut lib = TestLibrary::new();
    lib.add_source(&source);
    lib.compile().unwrap();
}
