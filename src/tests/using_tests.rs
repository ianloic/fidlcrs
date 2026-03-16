use crate::tests::test_library::{SharedAmongstLibraries, TestLibrary};

#[test]
fn good_using() {
    let mut shared = SharedAmongstLibraries::new();
    {
        let mut dep = TestLibrary::with_shared(&mut shared);
        dep.add_source_file(
            "dependent.fidl",
            "library dependent;\ntype Bar = struct {\n    s int8;\n};",
        );
        dep.compile().expect("dep compiled");
    }
    let mut lib = TestLibrary::with_shared(&mut shared);
    lib.add_errcat_file("good/fi-0178.test.fidl");
    lib.compile().expect("compilation failed");
}

#[test]
fn good_using_alias() {
    let mut shared = SharedAmongstLibraries::new();
    {
        let mut dep = TestLibrary::with_shared(&mut shared);
        dep.add_source_file(
            "dependent.fidl",
            "library dependent;\ntype Bar = struct {\n    s int8;\n};",
        );
        dep.compile().expect("dep compiled");
    }
    let mut lib = TestLibrary::with_shared(&mut shared);
    lib.add_source_file("example.fidl", "library example;\nusing dependent as the_alias;\ntype Foo = struct {\n    dep1 the_alias.Bar;\n};");
    lib.compile().expect("compilation failed");
}

#[test]
fn good_using_swap_names() {
    let mut shared = SharedAmongstLibraries::new();
    {
        let mut dep1 = TestLibrary::with_shared(&mut shared);
        dep1.add_source_file(
            "dependent1.fidl",
            "library dependent1;\nconst C1 bool = false;",
        );
        dep1.compile().expect("dep1 compiled");
        let mut dep2 = TestLibrary::with_shared(&mut shared);
        dep2.add_source_file(
            "dependent2.fidl",
            "library dependent2;\nconst C2 bool = false;",
        );
        dep2.compile().expect("dep2 compiled");
    }
    let mut lib = TestLibrary::with_shared(&mut shared);
    lib.add_source_file("example.fidl", "library example;\nusing dependent1 as dependent2;\nusing dependent2 as dependent1;\nconst C1 bool = dependent2.C1;\nconst C2 bool = dependent1.C2;");
    lib.compile().expect("compilation failed");
}

#[test]
fn good_decl_with_same_name_as_aliased_library() {
    let mut shared = SharedAmongstLibraries::new();
    {
        let mut dep = TestLibrary::with_shared(&mut shared);
        dep.add_source_file("dep.fidl", "library dep;\ntype A = struct{};");
        dep.compile().expect("dep compiled");
    }
    let mut lib = TestLibrary::with_shared(&mut shared);
    lib.add_source_file("lib.fidl", "library lib;\nusing dep as depnoconflict;\ntype dep = struct {};\ntype B = struct{a depnoconflict.A;};");
    lib.compile().expect("compilation failed");
}

#[test]
fn bad_missing_using() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        "library example;
type Foo = struct { dep dependent.Bar; };",
    );
    lib.expect_fail(
        crate::diagnostics::Error::ErrNameNotFound,
        &["\"dependent\"", "OwnedLibraryName { name: \"example\" }"],
    );
    assert!(lib.check_compile());
}

#[test]
fn bad_unknown_using() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0046.test.fidl");
    lib.expect_fail(
        crate::diagnostics::Error::ErrUnknownLibrary,
        &["\"dependent\""],
    );
    lib.expect_fail(
        crate::diagnostics::Error::ErrNameNotFound,
        &[
            "\"dependent\"",
            "OwnedLibraryName { name: \"test.bad.fi0046\" }",
        ],
    );
    assert!(lib.check_compile());
}

#[test]
fn bad_using_alias_ref_through_fqn() {
    let mut shared = SharedAmongstLibraries::new();
    {
        let mut dep = TestLibrary::with_shared(&mut shared);
        dep.add_source_file(
            "dependent.fidl",
            "library dependent;\ntype Bar = struct { s int8; };",
        );
        dep.compile().unwrap();
    }
    let mut lib = TestLibrary::with_shared(&mut shared);
    lib.add_source_file("example.fidl", "library example;\nusing dependent as the_alias;\ntype Foo = struct { dep1 dependent.Bar; };");
    lib.expect_fail(
        crate::diagnostics::Error::ErrNameNotFound,
        &["\"dependent\"", "OwnedLibraryName { name: \"example\" }"],
    );
    assert!(lib.check_compile());
}

#[test]
fn bad_duplicate_using_no_alias() {
    let mut shared = SharedAmongstLibraries::new();
    {
        let mut dep = TestLibrary::with_shared(&mut shared);
        dep.add_errcat_file("bad/fi-0042-a.test.fidl");
        dep.compile().unwrap();
    }
    let mut lib = TestLibrary::with_shared(&mut shared);
    lib.add_errcat_file("bad/fi-0042-b.test.fidl");
    lib.expect_fail(
        crate::diagnostics::Error::ErrDuplicateLibraryImport,
        &["\"test.bad.fi0042a\""],
    );
    assert!(lib.check_compile());
}

#[test]
fn bad_duplicate_using_first_alias() {
    let mut shared = SharedAmongstLibraries::new();
    {
        let mut dep = TestLibrary::with_shared(&mut shared);
        dep.add_source_file("dependent.fidl", "library dependent;");
        dep.compile().unwrap();
    }
    let mut lib = TestLibrary::with_shared(&mut shared);
    lib.add_source_file(
        "example.fidl",
        "library example;\nusing dependent as alias;\nusing dependent;",
    );
    lib.expect_fail(
        crate::diagnostics::Error::ErrDuplicateLibraryImport,
        &["\"dependent\""],
    );
    assert!(lib.check_compile());
}

#[test]
fn bad_duplicate_using_second_alias() {
    let mut shared = SharedAmongstLibraries::new();
    {
        let mut dep = TestLibrary::with_shared(&mut shared);
        dep.add_source_file("dependent.fidl", "library dependent;");
        dep.compile().unwrap();
    }
    let mut lib = TestLibrary::with_shared(&mut shared);
    lib.add_source_file(
        "example.fidl",
        "library example;\nusing dependent;\nusing dependent as alias;",
    );
    lib.expect_fail(
        crate::diagnostics::Error::ErrDuplicateLibraryImport,
        &["\"dependent\""],
    );
    assert!(lib.check_compile());
}

#[test]
fn bad_duplicate_using_same_library_same_alias() {
    let mut shared = SharedAmongstLibraries::new();
    {
        let mut dep = TestLibrary::with_shared(&mut shared);
        dep.add_source_file("dependent.fidl", "library dependent;");
        dep.compile().unwrap();
    }
    let mut lib = TestLibrary::with_shared(&mut shared);
    lib.add_source_file(
        "example.fidl",
        "library example;\nusing dependent as alias;\nusing dependent as alias;",
    );
    lib.expect_fail(
        crate::diagnostics::Error::ErrDuplicateLibraryImport,
        &["\"dependent\""],
    );
    assert!(lib.check_compile());
}

#[test]
fn bad_duplicate_using_same_library_different_alias() {
    let mut shared = SharedAmongstLibraries::new();
    {
        let mut dep = TestLibrary::with_shared(&mut shared);
        dep.add_source_file("dependent.fidl", "library dependent;");
        dep.compile().unwrap();
    }
    let mut lib = TestLibrary::with_shared(&mut shared);
    lib.add_source_file(
        "example.fidl",
        "library example;\nusing dependent as alias1;\nusing dependent as alias2;",
    );
    lib.expect_fail(
        crate::diagnostics::Error::ErrDuplicateLibraryImport,
        &["\"dependent\""],
    );
    assert!(lib.check_compile());
}

#[test]
fn bad_conflicting_using_library_and_alias() {
    let mut shared = SharedAmongstLibraries::new();
    {
        let mut d1 = TestLibrary::with_shared(&mut shared);
        d1.add_source_file("dependent1.fidl", "library dependent1;");
        d1.compile().unwrap();
        let mut d2 = TestLibrary::with_shared(&mut shared);
        d2.add_source_file("dependent2.fidl", "library dependent2;");
        d2.compile().unwrap();
    }
    let mut lib = TestLibrary::with_shared(&mut shared);
    lib.add_source_file(
        "example.fidl",
        "library example;\nusing dependent1;\nusing dependent2 as dependent1;",
    );
    lib.expect_fail(
        crate::diagnostics::Error::ErrConflictingLibraryImportAlias,
        &["\"dependent2\"", "\"dependent1\""],
    );
    assert!(lib.check_compile());
}

#[test]
fn bad_conflicting_using_alias_and_library() {
    let mut shared = SharedAmongstLibraries::new();
    {
        let mut d1 = TestLibrary::with_shared(&mut shared);
        d1.add_errcat_file("bad/fi-0043-a.test.fidl");
        d1.compile().unwrap();
        let mut d2 = TestLibrary::with_shared(&mut shared);
        d2.add_errcat_file("bad/fi-0043-b.test.fidl");
        d2.compile().unwrap();
    }
    let mut lib = TestLibrary::with_shared(&mut shared);
    lib.add_errcat_file("bad/fi-0043-c.test.fidl");
    lib.expect_fail(
        crate::diagnostics::Error::ErrConflictingLibraryImport,
        &["\"fi0043b\""],
    );
    lib.expect_fail(
        crate::diagnostics::Error::ErrNameNotFound,
        &["\"test.bad.fi0043a/Baz\"", "\"test.bad.fi0043c\""],
    );
    assert!(lib.check_compile());
}

#[test]
fn bad_conflicting_using_alias_and_alias() {
    let mut shared = SharedAmongstLibraries::new();
    {
        let mut d1 = TestLibrary::with_shared(&mut shared);
        d1.add_errcat_file("bad/fi-0044-a.test.fidl");
        d1.compile().unwrap();
        let mut d2 = TestLibrary::with_shared(&mut shared);
        d2.add_errcat_file("bad/fi-0044-b.test.fidl");
        d2.compile().unwrap();
    }
    let mut lib = TestLibrary::with_shared(&mut shared);
    lib.add_errcat_file("bad/fi-0044-c.test.fidl");
    lib.expect_fail(
        crate::diagnostics::Error::ErrConflictingLibraryImportAlias,
        &["\"test.bad.fi0044b\"", "\"dep\""],
    );
    lib.expect_fail(
        crate::diagnostics::Error::ErrNameNotFound,
        &["\"test.bad.fi0044a/Baz\"", "\"test.bad.fi0044c\""],
    );
    assert!(lib.check_compile());
}

#[test]
fn bad_unused_using() {
    let mut shared = SharedAmongstLibraries::new();
    {
        let mut d1 = TestLibrary::with_shared(&mut shared);
        d1.add_source_file("dependent.fidl", "library dependent;");
        d1.compile().unwrap();
    }
    let mut lib = TestLibrary::with_shared(&mut shared);
    lib.add_errcat_file("bad/fi-0178.test.fidl");
    lib.expect_fail(
        crate::diagnostics::Error::ErrUnusedImport,
        &[
            "OwnedLibraryName { name: \"test.bad.fi0178\" }",
            "\"dependent\"",
        ],
    );
    assert!(lib.check_compile());
}

#[test]
fn bad_unknown_dependent_library() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0051.test.fidl");
    lib.expect_fail(
        crate::diagnostics::Error::ErrUnknownDependentLibrary,
        &["\"unknown.dependent.library\"", "\"unknown.dependent\""],
    );
    assert!(lib.check_compile());
}

#[test]
fn bad_library_declaration_name_collision() {
    let mut shared = SharedAmongstLibraries::new();
    {
        let mut d1 = TestLibrary::with_shared(&mut shared);
        d1.add_errcat_file("bad/fi-0038-a.test.fidl");
        d1.compile().unwrap();
    }
    let mut lib = TestLibrary::with_shared(&mut shared);
    lib.add_errcat_file("bad/fi-0038-b.test.fidl");
    lib.expect_fail(
        crate::diagnostics::Error::ErrDeclNameConflictsWithLibraryImport,
        &["\"dependency\""],
    );
    lib.expect_fail(
        crate::diagnostics::Error::ErrCannotResolveConstantValue,
        &[],
    );
    assert!(lib.check_compile());
}

#[test]
fn bad_aliased_library_declaration_name_collision() {
    let mut shared = SharedAmongstLibraries::new();
    {
        let mut d1 = TestLibrary::with_shared(&mut shared);
        d1.add_source_file("dep.fidl", "library dep;\ntype A = struct{};");
        d1.compile().unwrap();
    }
    let mut lib = TestLibrary::with_shared(&mut shared);
    lib.add_source_file(
        "lib.fidl",
        "library lib;\nusing dep as x;\ntype x = struct{};\ntype B = struct{a dep.A;};",
    );
    lib.expect_fail(
        crate::diagnostics::Error::ErrDeclNameConflictsWithLibraryImport,
        &["\"x\""],
    );
    lib.expect_fail(
        crate::diagnostics::Error::ErrNameNotFound,
        &["\"dep\"", "OwnedLibraryName { name: \"lib\" }"],
    );
    assert!(lib.check_compile());
}

#[test]
fn bad_too_many_provided_libraries() {
    let mut shared = crate::tests::test_library::SharedAmongstLibraries::new();
    let mut dependency = crate::tests::test_library::TestLibrary::with_shared(&mut shared);
    dependency.add_source_file("notused.fidl", "library not.used;");
    dependency.compile().expect("dependency compiled");

    let mut library = crate::tests::test_library::TestLibrary::with_shared(&mut shared);
    library.add_source_file("example.fidl", "library example;");
    library.compile().expect("library compiled");

    // Rust harness doesn't report Unused libraries inside a simple compilation if they are just provided dependencies
    // so this test just verifies we can compile properly in this case.
}
