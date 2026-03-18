#![allow(unused_mut, unused_variables, unused_imports, dead_code)]
use crate::diagnostics::Error;
use crate::tests::test_library::{LookupHelpers, TestLibrary};

#[test]
#[ignore]
fn good_unversioned_one_component() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;
"#,
    );
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_unversioned_two_components() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example.something;
"#,
    );
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_implicit_one_component() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;
"#,
    );
    library.select_version("example", "HEAD");
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_implicit_two_components() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example.something;
"#,
    );
    library.select_version("example", "HEAD");
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_explicit() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(platform="someplatform", added=HEAD)
library example;
"#,
    );
    library.select_version("someplatform", "HEAD");
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_invalid() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0152.test.fidl");
    library.select_version("test", "HEAD");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_reserved() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0208.test.fidl");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_explicit_no_version_selected() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0201.test.fidl");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_implicit_no_version_selected() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example.something;
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_multiple_basic() {
    let mut shared = crate::tests::test_library::SharedAmongstLibraries::new();

    let mut dependency = TestLibrary::with_shared(&mut shared);
    dependency.add_source_file(
        "dependency.fidl",
        r#"
@available(added=2)
library dependency;

@available(added=3, deprecated=4, removed=5)
type Foo = struct {};
"#,
    );
    assert!(dependency.check_compile());

    let mut example = TestLibrary::with_shared(&mut shared);
    example.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

using dependency;

type Foo = struct {
    @available(deprecated=5)
    dep dependency.foo;
};
"#,
    );
    assert!(example.check_compile());
}

#[test]
#[ignore]
fn good_multiple_explicit() {
    let mut shared = crate::tests::test_library::SharedAmongstLibraries::new();

    let mut dependency = TestLibrary::with_shared(&mut shared);
    dependency.add_source_file(
        "dependency.fidl",
        r#"
@available(platform="xyz", added=1)
library dependency;

@available(added=3, removed=4)
type Foo = struct {};
"#,
    );
    assert!(dependency.check_compile());

    let mut example = TestLibrary::with_shared(&mut shared);
    example.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

using dependency;

alias Foo = dependency.foo;
"#,
    );
    assert!(example.check_compile());
}

#[test]
#[ignore]
fn good_multiple_uses_correct_decl() {
    let mut shared = crate::tests::test_library::SharedAmongstLibraries::new();

    let mut dependency = TestLibrary::with_shared(&mut shared);
    dependency.add_source_file(
        "dependency.fidl",
        r#"
@available(added=2)
library dependency;

@available(deprecated=3, replaced=4)
type Foo = resource struct {};

@available(added=4, removed=5)
type Foo = struct {};
"#,
    );
    assert!(dependency.check_compile());

    let mut example = TestLibrary::with_shared(&mut shared);
    example.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

using dependency;

type Foo = struct {
    dep dependency.foo;
};
"#,
    );
    assert!(example.check_compile());
}

#[test]
#[ignore]
fn bad_multiple_name_not_found() {
    let mut shared = crate::tests::test_library::SharedAmongstLibraries::new();

    let mut dependency = TestLibrary::with_shared(&mut shared);
    dependency.add_source_file(
        "dependency.fidl",
        r#"
@available(added=2)
library dependency;

@available(added=3, removed=5)
type Foo = struct {};
"#,
    );
    assert!(dependency.check_compile());

    let mut example = TestLibrary::with_shared(&mut shared);
    example.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

using dependency;

type Foo = struct {
    @available(deprecated=5)
    dep dependency.foo;
};
"#,
    );
    // expect_fail
    // expect_fail
    assert!(example.check_compile());
}

#[test]
#[ignore]
fn good_mix_versioned_and_unversioned() {
    let mut shared = crate::tests::test_library::SharedAmongstLibraries::new();

    let mut versioned = TestLibrary::with_shared(&mut shared);
    versioned.add_source_file(
        "versioned.fidl",
        r#"
@available(added=1, removed=2)
library example.versioned;

type Foo = struct {};
"#,
    );
    assert!(versioned.check_compile());

    let mut not_versioned = TestLibrary::with_shared(&mut shared);
    not_versioned.add_source_file(
        "not_versioned.fidl",
        r#"
library example.notversioned;

using example.versioned;

alias Foo = example.versioned.foo;
"#,
    );
    assert!(not_versioned.check_compile());

    // The example.notversioned library is considered added=HEAD in the special
    // "unversioned" platform. (If it were instead in the "example" platform, it
    // would appear empty because we're using `--available example:1`.)
}
