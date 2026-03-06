use super::test_library::TestLibrary;

#[test]
fn good_library_multiple_files() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0040-a.test.fidl");
    library.add_errcat_file("good/fi-0040-b.test.fidl");

    let result = library.compile();
    assert!(result.is_ok(), "compilation failed");
}

#[test]
#[ignore]
fn bad_files_disagree_on_library_name() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0040-a.test.fidl");
    library.add_errcat_file("bad/fi-0040-b.test.fidl");

    let result = library.compile();
    assert!(
        result.is_err(),
        "Expected compilation to fail with ErrFilesDisagreeOnLibraryName"
    );
}
