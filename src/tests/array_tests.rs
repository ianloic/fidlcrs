use crate::diagnostics::Error;
use crate::tests::test_library::TestLibrary;

#[test]
fn good_nonzero_size_array() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type S = struct {
    arr array<uint8, 1>;
};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn bad_zero_size_array() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0161.test.fidl");
    lib.expect_fail(Error::ErrMustHaveNonZeroSize, &["\"array\""]);

    assert!(lib.check_compile());
}

#[test]
fn bad_no_size_array() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type S = struct {
    arr array<uint8>;
};
"#,
    );
    lib.expect_fail(Error::ErrWrongNumberOfLayoutParameters, &["{}", "{}", "{}"]);

    assert!(lib.check_compile());
}

#[test]
fn bad_non_parameterized_array() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type S = struct {
    arr array;
};
"#,
    );
    lib.expect_fail(Error::ErrWrongNumberOfLayoutParameters, &["{}", "{}", "{}"]);

    assert!(lib.check_compile());
}

#[test]
fn bad_optional_array() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type S = struct {
    arr array<uint8, 10>:optional;
};
"#,
    );
    lib.expect_fail(Error::ErrCannotBeOptional, &["\"array\""]);

    lib.expect_fail(Error::ErrTooManyConstraints, &["\"array\"", "0", "1"]);

    assert!(lib.check_compile());
}

#[test]
fn bad_multiple_constraints_on_array() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type S = struct {
    arr array<uint8, 10>:<1, 2, 3>;
};
"#,
    );
    lib.expect_fail(Error::ErrTooManyConstraints, &["\"array\"", "0", "3"]);

    assert!(lib.check_compile());
}
