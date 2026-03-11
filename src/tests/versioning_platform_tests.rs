use crate::tests::test_library::{SharedAmongstLibraries, TestLibrary};

#[test]
fn good_multiple_basic() {
    let mut shared = SharedAmongstLibraries::new();
    shared
        .select_versions
        .push(("some_platform".to_string(), "1".to_string()));
    let mut dep = TestLibrary::with_shared(&mut shared);
    dep.add_source_file(
        "dep.fidl",
        r#"@available(platform="some_platform", added=1)
library dependent;
type Bar = struct {};"#,
    );
    dep.compile().expect("dep compiled");

    let mut lib = TestLibrary::with_shared(&mut shared);
    lib.add_source_file(
        "example.fidl",
        r#"@available(platform="some_platform", added=2)
library example;
using dependent;
type Foo = struct { b dependent.Bar; };"#,
    );
    lib.compile().expect("lib compiled");
}

#[test]
fn good_multiple_explicit() {
    let mut shared = SharedAmongstLibraries::new();
    shared
        .select_versions
        .push(("a".to_string(), "1".to_string()));
    shared
        .select_versions
        .push(("b".to_string(), "2".to_string()));
    let mut dep = TestLibrary::with_shared(&mut shared);
    dep.add_source_file(
        "dep.fidl",
        r#"@available(platform="a", added=1)
library dependent;
type Bar = struct {};"#,
    );
    dep.compile().expect("dep compiled");

    let mut lib = TestLibrary::with_shared(&mut shared);
    lib.add_source_file(
        "example.fidl",
        r#"@available(platform="b", added=2)
library example;
using dependent;
type Foo = struct { b dependent.Bar; };"#,
    );
    lib.compile().expect("lib compiled");
}

#[test]
fn good_multiple_uses_correct_decl() {
    let mut shared = SharedAmongstLibraries::new();
    shared
        .select_versions
        .push(("other_platform".to_string(), "1".to_string()));
    let mut dep = TestLibrary::with_shared(&mut shared);
    dep.add_source_file(
        "dep.fidl",
        r#"@available(platform="some_platform", added=1)
library dependent;
type Bar = struct {};"#,
    );
    dep.compile().expect("dep compiled");

    let mut lib = TestLibrary::with_shared(&mut shared);
    lib.add_source_file(
        "example.fidl",
        r#"@available(platform="other_platform", added=1)
library example;
using dependent;
type Foo = struct { b dependent.Bar; };"#,
    );
    lib.compile().expect("lib compiled");
}

#[test]
fn bad_multiple_name_not_found() {
    let mut shared = SharedAmongstLibraries::new();
    shared
        .select_versions
        .push(("dependency".to_string(), "HEAD".to_string()));
    shared
        .select_versions
        .push(("example".to_string(), "HEAD".to_string()));

    let mut dep = TestLibrary::with_shared(&mut shared);
    dep.add_source_file(
        "dep.fidl",
        r#"@available(added=2)
library dependency;
@available(added=3, removed=5)
type Foo = struct {};"#,
    );
    dep.compile().expect("dep compiled");

    let mut lib = TestLibrary::with_shared(&mut shared);
    lib.add_source_file(
        "example.fidl",
        r#"@available(added=1)
library example;
using dependency;
type Foo = struct {
    @available(deprecated=5)
    dep dependency.Foo;
};"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn good_mix_versioned_and_unversioned() {
    let mut shared = SharedAmongstLibraries::new();
    let mut dep = TestLibrary::with_shared(&mut shared);
    dep.add_source_file(
        "dep.fidl",
        "library dependent;
type Bar = struct {};",
    );
    dep.compile().expect("dep compiled");

    let mut lib = TestLibrary::with_shared(&mut shared);
    lib.add_source_file(
        "example.fidl",
        "@available(added=1)
library example;
using dependent;
type Foo = struct { b dependent.Bar; };",
    );
    lib.compile().expect("lib compiled");
}
