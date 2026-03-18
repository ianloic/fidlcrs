#![allow(unused_mut, unused_variables)]
use crate::diagnostics::Error;
use crate::tests::test_library::TestLibrary;

#[test]
fn bad_duplicate_modifier() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type One = strict union { 1: b bool; };
type Two = strict strict union { 1: b bool; };
type Three = strict strict strict union { 1: b bool; };
"#,
    );
    lib.expect_fail(Error::ErrDuplicateModifier, &["\"strict\""]);
    lib.expect_fail(Error::ErrDuplicateModifier, &["\"strict\""]);
    lib.expect_fail(Error::ErrDuplicateModifier, &["\"strict\""]);
    assert!(lib.check_compile());
}

#[test]
fn bad_duplicate_modifier_non_consecutive() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0032.noformat.test.fidl");
    lib.expect_fail(Error::ErrDuplicateModifier, &["\"strict\""]);
    assert!(lib.check_compile());
}

#[test]
fn bad_conflicting_modifiers() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0033.noformat.test.fidl");
    lib.expect_fail(
        Error::ErrConflictingModifier,
        &["\"flexible\"", "\"strict\""],
    );
    lib.expect_fail(
        Error::ErrConflictingModifier,
        &["\"strict\"", "\"flexible\""],
    );
    assert!(lib.check_compile());
}

#[test]
fn good_bits_strictness() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
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
"#,
    );
    lib.compile().unwrap();
}

#[test]
fn good_enum_strictness() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
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
"#,
    );
    lib.compile().unwrap();
}

#[test]
fn good_flexible_enum() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type Foo = flexible enum {
    BAR = 1;
};
"#,
    );
    lib.compile().unwrap();
}

#[test]
fn good_flexible_bits_redundant() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type Foo = flexible bits {
    BAR = 0x1;
};
"#,
    );
    lib.compile().unwrap();
}

#[test]
fn bad_strictness_struct() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0030.noformat.test.fidl");
    lib.expect_fail(
        Error::ErrCannotSpecifyModifier,
        &["\"strict\"", "\"struct\""],
    );
    assert!(lib.check_compile());
}

#[test]
fn bad_strictness_table() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type StrictFoo = strict table {};
"#,
    );
    lib.expect_fail(
        Error::ErrCannotSpecifyModifier,
        &["\"strict\"", "\"table\""],
    );
    assert!(lib.check_compile());
}

#[test]
fn good_union_strictness() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("good/fi-0033.test.fidl");
    lib.compile().unwrap();
}

#[test]
fn good_strict_union_redundant() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type Foo = strict union {
    1: i int32;
};
"#,
    );
    lib.compile().unwrap();
}
