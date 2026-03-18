use super::test_library::{LookupHelpers, SharedAmongstLibraries, TestLibrary};
use crate::diagnostics::Error;
use crate::flat_ast;
use test_case::test_case;

use crate::flat_ast::Openness;
macro_rules! version_test_impl {
    (
        [$($ver:expr),+],
        $(#[$meta:meta])*
        fn $name:ident($version_arg:ident: &str) {
            $($body:tt)*
        }
    ) => {
        $(#[$meta])*
        $(#[test_case($ver)])+
        fn $name($version_arg: &str) {
            $($body)*
        }
    };
}

macro_rules! version_test {
    ($($tokens:tt)*) => {
        version_test_impl!(
            ["1", "2", "HEAD"],
            $($tokens)*
        );
    };
}

const V1: u32 = 1;
const V2: u32 = 2;
const HEAD: u32 = u32::MAX;

struct TargetVersions {
    versions: Vec<u32>,
}

impl TargetVersions {
    fn new(s: &str) -> Self {
        let mut versions = Vec::new();
        for p in s.split(',') {
            let v = match p {
                "1" => 1,
                "2" => 2,
                "HEAD" => HEAD,
                _ => panic!(),
            };
            versions.push(v);
        }
        TargetVersions { versions }
    }

    fn any_eq(&self, other: u32) -> bool {
        self.versions.contains(&other)
    }
    fn all_eq(&self, other: u32) -> bool {
        self.versions.iter().all(|&v| v == other)
    }
    fn any_ge(&self, other: u32) -> bool {
        self.versions.iter().any(|&v| v >= other)
    }
    fn all_ge(&self, other: u32) -> bool {
        self.versions.iter().all(|&v| v >= other)
    }
    fn any_le(&self, other: u32) -> bool {
        self.versions.iter().any(|&v| v <= other)
    }
    fn all_le(&self, other: u32) -> bool {
        self.versions.iter().all(|&v| v <= other)
    }
    fn any_lt(&self, other: u32) -> bool {
        self.versions.iter().any(|&v| v < other)
    }
}

version_test! {
fn good_library_default(version: &str) {
    let _tv = TargetVersions::new(version);
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;
"#,
    );
    library.select_version("example", version);
    let _ast = library.compile().unwrap();
}
}

version_test! {
fn good_library_added_at_head(version: &str) {
    let _tv = TargetVersions::new(version);
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=HEAD)
library example;
"#,
    );
    library.select_version("example", version);
    let _ast = library.compile().unwrap();
}
}

version_test! {
fn good_library_added_at_one(version: &str) {
    let _tv = TargetVersions::new(version);
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;
"#,
    );
    library.select_version("example", version);
    let _ast = library.compile().unwrap();
}
}

version_test! {
fn good_library_added_and_removed(version: &str) {
    let _tv = TargetVersions::new(version);
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1, removed=2)
library example;
"#,
    );
    library.select_version("example", version);
    let _ast = library.compile().unwrap();
}
}

version_test! {
fn good_library_added_and_deprecated_and_removed(version: &str) {
    let _tv = TargetVersions::new(version);
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1, deprecated=2, removed=HEAD)
library example;
"#,
    );
    library.select_version("example", version);
    let _ast = library.compile().unwrap();
}
}

version_test! {
fn good_decl_added_at_head(version: &str) {
    let tv = TargetVersions::new(version);
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=HEAD)
type Foo = struct {};
"#,
    );
    library.select_version("example", version);
    let ast = library.compile().unwrap();
    assert_eq!(ast.lookup_struct("example/Foo").is_some(), tv.any_ge(HEAD));
}
}

version_test! {
fn good_decl_added_at_one(version: &str) {
    let _tv = TargetVersions::new(version);
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=1)
type Foo = struct {};
"#,
    );
    library.select_version("example", version);
    let ast = library.compile().unwrap();
    assert!(ast.lookup_struct("example/Foo").is_some());
}
}

version_test! {
fn good_decl_added_and_removed(version: &str) {
    let tv = TargetVersions::new(version);
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=1, removed=2)
type Foo = struct {};
"#,
    );
    library.select_version("example", version);
    let ast = library.compile().unwrap();
    assert_eq!(ast.lookup_struct("example/Foo").is_some(), tv.any_eq(V1));
}
}

version_test! {
fn good_decl_added_and_replaced(version: &str) {
    let tv = TargetVersions::new(version);
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=1, replaced=2)
type Foo = struct {};

@available(added=2)
type Foo = resource struct {};
"#,
    );
    library.select_version("example", version);
    let ast = library.compile().unwrap();
    assert_eq!(
        ast.lookup_struct("example/Foo").unwrap().resource,
        !tv.all_eq(V1)
    );
}
}

version_test! {
fn good_decl_added_and_deprecated_and_removed(version: &str) {
    let tv = TargetVersions::new(version);
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=1, deprecated=2, removed=HEAD)
type Foo = struct {};
"#,
    );
    library.select_version("example", version);
    let ast = library.compile().unwrap();
    let present = tv.any_lt(HEAD);
    assert_eq!(ast.lookup_struct("example/Foo").is_some(), present);
    if present {
        assert_eq!(
            ast.lookup_struct("example/Foo").unwrap().deprecated,
            tv.any_ge(V2)
        );
    }
}
}

version_test! {
fn good_member_added_at_head(version: &str) {
    let tv = TargetVersions::new(version);
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = struct {
    @available(added=HEAD)
    member string;
};
"#,
    );
    library.select_version("example", version);
    let ast = library.compile().unwrap();
    assert_eq!(
        ast.lookup_struct("example/Foo").unwrap().members.len(),
        if tv.any_ge(HEAD) { 1 } else { 0 }
    );
}
}

version_test! {
fn good_member_added_at_one(version: &str) {
    let _tv = TargetVersions::new(version);
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = struct {
    @available(added=1)
    member string;
};
"#,
    );
    library.select_version("example", version);
    let ast = library.compile().unwrap();
    assert_eq!(ast.lookup_struct("example/Foo").unwrap().members.len(), 1);
}
}

version_test! {
fn good_member_added_and_removed(version: &str) {
    let tv = TargetVersions::new(version);
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = struct {
    @available(added=1, removed=2)
    member string;
};
"#,
    );
    library.select_version("example", version);
    let ast = library.compile().unwrap();
    assert_eq!(
        ast.lookup_struct("example/Foo").unwrap().members.len(),
        if tv.any_eq(V1) { 1 } else { 0 }
    );
}
}

version_test! {
fn good_member_added_and_replaced(version: &str) {
    let tv = TargetVersions::new(version);
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = struct {
    @available(added=1, replaced=2)
    member string;
    @available(added=2)
    member uint32;
};
"#,
    );
    library.select_version("example", version);
    let ast = library.compile().unwrap();
    assert_eq!(ast.lookup_struct("example/Foo").unwrap().members.len(), 1);
    assert_eq!(
        ast.lookup_struct("example/Foo").unwrap().members[0]
            .type_
            .kind(),
        if tv.all_eq(V1) {
            flat_ast::TypeKind::String
        } else {
            flat_ast::TypeKind::Primitive
        }
    );
}
}

version_test! {
fn good_member_added_and_deprecated_and_removed(version: &str) {
    let tv = TargetVersions::new(version);
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = struct {
    @available(added=1, deprecated=2, removed=HEAD)
    member string;
};
"#,
    );
    library.select_version("example", version);
    let ast = library.compile().unwrap();
    let present = tv.any_lt(HEAD);
    assert_eq!(
        ast.lookup_struct("example/Foo").unwrap().members.len(),
        if present { 1 } else { 0 }
    );
    if present {
        assert_eq!(
            ast.lookup_struct("example/Foo").unwrap().members[0].deprecated,
            tv.any_ge(V2)
        );
    }
}
}

version_test! {
fn good_decl_strictness_added_and_removed(version: &str) {
    let tv = TargetVersions::new(version);
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = strict(added=2, removed=3) flexible(added=3) enum {
    VALUE = 1;
};
"#,
    );
    library.select_version("example", version);
    let ast = library.compile().unwrap();
    // Foo is flexible by default at V1, strict at V2, and explicitly flexible from V3 onwards.
    assert_eq!(
        ast.lookup_enum("example/Foo").unwrap().strict,
        tv.any_eq(V2) && tv.all_le(V2)
    );
}
}

version_test! {
fn good_decl_resourceness_added_and_removed(version: &str) {
    let tv = TargetVersions::new(version);
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = resource(added=2, removed=3) struct {};
"#,
    );
    library.select_version("example", version);
    let ast = library.compile().unwrap();
    // Foo is a resource type at V2 only.
    assert_eq!(
        ast.lookup_struct("example/Foo").unwrap().resource,
        tv.any_eq(V2) && tv.all_le(V2)
    );
}
}

version_test! {
fn good_protocol_openness_added_and_removed(version: &str) {
    let tv = TargetVersions::new(version);
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

closed(added=2, removed=3) open(added=3) protocol Foo {};
"#,
    );
    library.select_version("example", version);
    let ast = library.compile().unwrap();
    // Foo is open by default at V1, explicitly closed at V2, and explicitly open from V3 onwards.
    assert_eq!(
        ast.lookup_protocol("example/Foo").unwrap().openness,
        if tv.any_eq(V2) && tv.all_le(V2) {
            Openness::Closed
        } else {
            Openness::Open
        }
    );
}
}

version_test! {
fn good_method_strictness_added_and_removed(version: &str) {
    let tv = TargetVersions::new(version);
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

open protocol Protocol {
    strict(added=2, removed=3) flexible(added=3) Foo();
};
"#,
    );
    library.select_version("example", version);
    let ast = library.compile().unwrap();
    // Foo is flexible by default at V1, strict at V2, and explicitly flexible from V3 onwards.
    assert_eq!(
        ast.lookup_protocol("example/Protocol").unwrap().methods[0].strict,
        tv.any_eq(V2) && tv.all_le(V2)
    );
}
}

version_test! {
fn bad_change_interacting_modifiers(version: &str) {
    let tv = TargetVersions::new(version);
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

closed(removed=2) ajar(added=2, removed=3) open(added=3) protocol Protocol {
    flexible OneWay();
    flexible -> Event();
    flexible TwoWay() -> () error uint32;
};
"#,
    );
    library.select_version("example", version);
    if tv.all_eq(V1) {
        library.expect_fail(
            Error::ErrFlexibleOneWayMethodInClosedProtocol,
            &["\"one-way method\""],
        );
        library.expect_fail(
            Error::ErrFlexibleOneWayMethodInClosedProtocol,
            &["\"event\""],
        );
        library.expect_fail(
            Error::ErrFlexibleTwoWayMethodRequiresOpenProtocol,
            &["\"closed\""],
        );
        assert!(library.check_compile());
    } else if tv.all_eq(V2) {
        library.expect_fail(
            Error::ErrFlexibleTwoWayMethodRequiresOpenProtocol,
            &["\"ajar\""],
        );
        assert!(library.check_compile());
    } else {
        assert!(library.compile().is_ok());
    }
}
}

version_test! {
fn good_change_interacting_modifiers(version: &str) {
    let _tv = TargetVersions::new(version);
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

closed(removed=2) ajar(added=2, removed=3) open(added=3) protocol Protocol {
    strict(removed=2) flexible(added=2) OneWay();
    strict(removed=2) flexible(added=2) -> Event();
    strict(removed=3) flexible(added=3) TwoWay() -> () error uint32;
};
"#,
    );
    library.select_version("example", version);
    let _ast = library.compile().unwrap();
}
}

version_test! {
fn good_add_resource_modifier_and_handle(version: &str) {
    let _tv = TargetVersions::new(version);
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

using zx;

type Foo = resource(added=2) table {
    @available(added=2)
    1: handle zx.Handle;
};
"#,
    );
    library.select_version("example", version);
    library.use_library_zx();
    let _ast = library.compile().unwrap();
}
}

version_test! {
fn remove_resource_modifier_and_handle(version: &str) {
    let tv = TargetVersions::new(version);
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

using zx;

type Foo = resource(removed=2) table {
    @available(removed=2)
    1: handle zx.Handle;
};
"#,
    );
    library.select_version("example", version);
    library.use_library_zx();
    let has_handle = tv.any_eq(V1);
    let is_resource = tv.all_eq(V1);
    if has_handle && !is_resource {
        library.expect_fail(
            Error::ErrTypeMustBeResource,
            &["table", "Foo", "handle", "example.fidl:9:8"],
        );
        assert!(library.check_compile());
    } else {
        let _ast = library.compile().unwrap();
    }
}
}

version_test! {
fn bad_reference_outside_availability(version: &str) {
    let tv = TargetVersions::new(version);
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0220.test.fidl");
    library.select_version("test", version);
    if tv.all_eq(V1) {
        library.expect_fail(Error::ErrNameNotFound, &["\"Bar\"", "\"test.bad.fi0220\""]);
        assert!(library.check_compile());
    } else {
        assert!(library.compile().is_ok());
    }
}
}

version_test! {
fn good_regular_deprecated_references_versioned_deprecated(version: &str) {
    let _tv = TargetVersions::new(version);
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@deprecated
const FOO uint32 = BAR;
@available(deprecated=1)
const BAR uint32 = 1;
"#,
    );
    library.select_version("example", version);
    let _ast = library.compile().unwrap();
}
}

version_test! {
fn good_deprecation_logic_regression1(version: &str) {
    let tv = TargetVersions::new(version);
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(deprecated=1, removed=3)
type Foo = struct {};

@available(deprecated=1, removed=3)
type Bar = struct {
    foo Foo;
    @available(added=2)
    ensure_split_at_v2 string;
};
"#,
    );
    library.select_version("example", version);
    let ast = library.compile().unwrap();
    let present = tv.any_le(V2);
    assert_eq!(ast.lookup_struct("example/Bar").is_some(), present);
    if present {
        assert_eq!(
            ast.lookup_struct("example/Bar").unwrap().members.len(),
            if tv.all_eq(V1) { 1 } else { 2 }
        );
    }
}
}

version_test! {
fn good_deprecation_logic_regression2(version: &str) {
    let tv = TargetVersions::new(version);
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(deprecated=1)
type Foo = struct {};

@available(deprecated=1, removed=3)
type Bar = struct {
    foo Foo;
    @available(added=2)
    ensure_split_at_v2 string;
};
"#,
    );
    library.select_version("example", version);
    let ast = library.compile().unwrap();
    let present = tv.any_le(V2);
    assert_eq!(ast.lookup_struct("example/Bar").is_some(), present);
    if present {
        assert_eq!(
            ast.lookup_struct("example/Bar").unwrap().members.len(),
            if tv.all_eq(V1) { 1 } else { 2 }
        );
    }
}
}

version_test! {
fn good_multiple_files(version: &str) {
    let tv = TargetVersions::new(version);
    let mut library = TestLibrary::new();
    library.add_source_file(
        "overview.fidl",
        r#"
/// Some doc comment.
@available(added=1)
library example;
"#,
    );
    library.add_source_file(
        "first.fidl",
        r#"
library example;

@available(added=2)
type Foo = struct {
    bar box<Bar>;
};
"#,
    );
    library.add_source_file(
        "second.fidl",
        r#"
library example;

@available(added=2)
type Bar = struct {
    foo box<Foo>;
};
"#,
    );
    library.select_version("example", version);
    let ast = library.compile().unwrap();
    assert_eq!(ast.lookup_struct("example/Foo").is_some(), tv.any_ge(V2));
    assert_eq!(ast.lookup_struct("example/Bar").is_some(), tv.any_ge(V2));
}
}

version_test! {
fn good_multiple_libraries(version: &str) {
    let tv = TargetVersions::new(version);
    let mut shared = SharedAmongstLibraries::new();
    shared
        .select_versions
        .push(("platform".to_string(), version.to_string()));
    let mut dependency = TestLibrary::with_shared(&mut shared);
    dependency.add_source_file(
        "dependency.fidl",
        r#"
@available(added=1)
library platform.dependency;

type Foo = struct {
    @available(added=2)
    member string;
};
"#,
    );
    let _ast = dependency.compile().unwrap();
    let mut example = TestLibrary::with_shared(&mut shared);
    example.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library platform.example;

using platform.dependency;

type ShouldBeSplit = struct {
    foo platform.dependency.Foo;
};
"#,
    );
    let ast = example.compile().unwrap();
    assert_eq!(
        ast.lookup_struct("platform.example/ShouldBeSplit")
            .unwrap()
            .type_shape
            .inline_size,
        if tv.all_eq(V1) { 1 } else { 16 }
    );
}
}
