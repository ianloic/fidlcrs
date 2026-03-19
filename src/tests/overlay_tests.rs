#![allow(unused_mut, unused_variables, unused_imports, dead_code)]
use crate::diagnostics::Error;
use crate::experimental_flags::ExperimentalFlag;
use crate::tests::test_library::{LookupHelpers, TestLibrary};

#[test]
#[ignore]
fn good_overlay_in_other_layouts() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library test;

type Overlay = strict overlay {
    1: member string:32;
};

type Struct = struct {
    o Overlay;
};

type Table = table {
    1: o Overlay;
};

type Union = strict union {
    1: o Overlay;
};
"#,
    );
    library.enable_flag("zx_c_types");

    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_other_layouts_in_overlay() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library test;

type Struct = struct {
    member int32;
};

type Table = table {
    1: member int32;
};

type Union = strict union {
    1: member int32;
};

type Overlay = strict overlay {
    1: s Struct;
    2: bs box<Struct>;
    3: t Table;
    4: u Union;
};

"#,
    );
    library.enable_flag("zx_c_types");

    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_overlay_in_overlay() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library test;

type Inner = strict overlay {
    1: i int32;
    2: b bool;
    3: s string:32;
};

type Outer = strict overlay {
    1: i int32;
    2: b bool;
    3: s string:32;
    4: o Inner;
};

"#,
    );
    library.enable_flag("zx_c_types");

    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_keywords_as_field_names() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library test;

type struct = struct {
    field bool;
};

type Foo = strict overlay {
    1: union int64;
    2: library bool;
    3: uint32 uint32;
    4: member struct;
    5: reserved bool;
};
"#,
    );
    library.enable_flag("zx_c_types");

    assert!(library.check_compile());

    // test check
    // test check
}

#[test]
#[ignore]
fn bad_flexible() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library test;

type Foo = flexible overlay {
    1: flippity int64;
    2: floppity bool;
};
"#,
    );
    library.enable_flag("zx_c_types");

    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_resource() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library test;

type Foo = strict resource overlay {
    1: flippity int64;
    2: floppity bool;
};
"#,
    );
    library.enable_flag("zx_c_types");

    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_resource_member() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library test;
protocol Bar{};

type Foo = strict overlay {
    1: flippity int64;
    2: floppity client_end:Bar;
};
"#,
    );
    library.enable_flag("zx_c_types");

    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_no_experimental_flag() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library test;

type Foo = strict overlay {
    1: bar int64;
    2: baz string:42;
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_optional_overlay() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library test;

type Biff = strict overlay {
    1: foo bool;
    2: bar string;
};

type Baff = struct {
    baz Biff:optional;
};
"#,
    );
    library.enable_flag("zx_c_types");

    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_recursive_overlay() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library test;

type Foo = strict union {
    1: bar Bar;
    2: s string:32;
    3: i int32;
};

type Bar = strict overlay {
    1: foo Foo:optional;
    2: s string:32;
    3: i int32;
};
"#,
    );
    library.enable_flag("zx_c_types");

    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_directly_recursive_overlay() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library test;

type Value = strict overlay {
    1: bool_value bool;
    2: recurse Value;
};
"#,
    );
    library.enable_flag("zx_c_types");

    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_inline_recursive_overlay() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library test;

type Product = struct {
    i int32;
    b bool;
    s string:32;
    sum Sum;
};

type Sum = strict overlay {
    1: i int32;
    2: b bool;
    3: s string:32;
    4: product Product;
};
"#,
    );
    library.enable_flag("zx_c_types");

    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_no_selector() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

type Foo = strict overlay {
  @selector("v2") 1: v string;
};
"#,
    );
    library.enable_flag("zx_c_types");

    // expect_fail
    assert!(library.check_compile());
}
