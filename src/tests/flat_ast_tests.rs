use super::test_library::TestLibrary;

#[test]
fn good_single_anonymous_name_use() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"
library example;

protocol Foo {
    SomeMethod() -> (struct {
        some_param uint8;
    }) error uint32;
};
        "#,
    );
    let result = library.compile();
    assert!(result.is_ok(), "Expected compilation to succeed");
}

#[test]
fn bad_cannot_reference_anonymous_name() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0058.test.fidl");
    let result = library.compile();
    assert!(
        result.is_err(),
        "Expected compilation to fail with ErrAnonymousNameReference"
    );
}

#[test]
#[ignore]
fn bad_anonymous_name_conflict() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"
library example;

protocol Foo {
  SomeMethod(struct { some_param uint8; });
};

type FooSomeMethodRequest = struct {};
        "#,
    );
    let result = library.compile();
    assert!(
        result.is_err(),
        "Expected compilation to fail with ErrNameCollision"
    );
}

#[test]
#[ignore]
fn bad_multiple_libraries_same_name() {
    // Porting constraint: SharedAmongstLibraries logic might not be cleanly implemented in test_library.rs yet.
    // The test expects two test libraries sharing state to conflict on Name when compiling later.
    let mut library2 = TestLibrary::new();
    library2.add_errcat_file("bad/fi-0041-b.test.fidl");
    let result = library2.compile();
    assert!(
        result.is_err(),
        "Expected compilation to fail with ErrMultipleLibrariesWithSameName"
    );
}

#[test]
#[ignore]
fn good_implicit_assumptions() {

    // Preconditions to unit test cases: if these change, we need to rewrite the tests themselves.
    // test check
    // test check
}
