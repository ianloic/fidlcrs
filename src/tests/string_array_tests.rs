#![allow(unused_mut, unused_variables, unused_imports, dead_code)]
use crate::diagnostics::Error;
use crate::experimental_flags::ExperimentalFlag;
use crate::tests::test_library::{LookupHelpers, TestLibrary};

#[test]
#[ignore]
fn good_nonzero_size_array() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("");
    library.enable_flag("zx_c_types");
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_no_experimental_flag() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
fn bad_zero_size_array() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

type S = struct {
    arr string_array<0>;
};
"#,
    );
    library.enable_flag("zx_c_types");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_no_size_array() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

type S = struct {
    arr string_array;
};
"#,
    );
    library.enable_flag("zx_c_types");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
fn bad_optional_array() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

type S = struct {
    arr string_array<10>:optional;
};
"#,
    );
    library.enable_flag("zx_c_types");
    // expect_fail
    assert!(library.check_compile());
}
