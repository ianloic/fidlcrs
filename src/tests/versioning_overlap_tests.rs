#![allow(unused_mut, unused_variables, unused_imports, dead_code)]
use crate::diagnostics::Error;
use crate::tests::test_library::{LookupHelpers, TestLibrary};

#[test]
fn good_no_gap() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(replaced=2)
type Foo = struct {};

@available(added=2)
type Foo = resource struct {};
"#,
    );

    assert!(library.check_compile());
}

#[test]
fn good_with_gap() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(removed=2)
type Foo = struct {};

@available(added=3)
type Foo = table {};
"#,
    );

    if true {
        // expect_fail
        assert!(library.check_compile());
    } else {
        assert!(library.check_compile());
    }
}

#[test]
fn good_no_gap_canonical() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(removed=2)
type foo = struct {};

@available(added=2)
type FOO = table {};
"#,
    );

    if true {
        // expect_fail
        assert!(library.check_compile());
    } else {
        assert!(library.check_compile());
    }
}

#[test]
fn good_with_gap_canonical() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(removed=2)
type foo = struct {};

@available(added=3)
type FOO = table {};

"#,
    );

    if true {
        // expect_fail
        assert!(library.check_compile());
    } else {
        assert!(library.check_compile());
    }
}

#[test]
#[ignore]
fn bad_equal() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = struct {};
type Foo = table {};
"#,
    );

    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_equal_canonical() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type foo = struct {};
type FOO = table {};
"#,
    );

    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_example() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0036.test.fidl");

    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_example_canonical() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0037.test.fidl");

    // expect_fail
    assert!(library.check_compile());
}

#[test]
fn bad_subset() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = struct {};
@available(removed=2)
type Foo = table {};
"#,
    );

    // expect_fail
    assert!(library.check_compile());
}

#[test]
fn bad_subset_canonical() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type foo = struct {};
@available(removed=2)
type FOO = table {};
"#,
    );

    // expect_fail
    assert!(library.check_compile());
}

#[test]
fn bad_intersect() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(removed=5)
type Foo = struct {};
@available(added=3)
type Foo = table {};
"#,
    );

    // expect_fail
    assert!(library.check_compile());
}

#[test]
fn bad_intersect_canonical() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(removed=5)
type foo = struct {};
@available(added=3)
type FOO = table {};
"#,
    );

    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_multiple() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = struct {};
@available(added=3)
type Foo = table {};
@available(added=HEAD)
const Foo uint32 = 0;
"#,
    );

    // expect_fail
    // expect_fail
    assert!(library.check_compile());
}

#[test]
fn bad_recursive() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=1, removed=5)
type Foo = struct { member box<Foo>; };

@available(added=3, removed=7)
type Foo = struct { member box<Foo>; };
"#,
    );

    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_member_equal() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = struct {
    member bool;
    member bool;
};
"#,
    );

    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_member_equal_canonical() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = struct {
    member bool;
    MEMBER bool;
};
"#,
    );

    // expect_fail
    assert!(library.check_compile());
}

#[test]
fn bad_member_subset() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = struct {
    member bool;
    @available(removed=2)
    member bool;
};
"#,
    );

    // expect_fail
    assert!(library.check_compile());
}

#[test]
fn bad_member_subset_canonical() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = struct {
    member bool;
    @available(removed=2)
    MEMBER bool;
};
"#,
    );

    // expect_fail
    assert!(library.check_compile());
}

#[test]
fn bad_member_intersect() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = struct {
    @available(removed=5)
    member bool;
    @available(added=3)
    member bool;
};
"#,
    );

    // expect_fail
    assert!(library.check_compile());
}

#[test]
fn bad_member_intersect_canonical() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = struct {
    @available(removed=5)
    member bool;
    @available(added=3)
    MEMBER bool;
};
"#,
    );

    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_member_multiple() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = struct {
    member bool;
    @available(added=3)
    member bool;
    @available(added=HEAD)
    member bool;
};
"#,
    );

    // expect_fail
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_member_multiple_canonical() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = struct {
    member bool;
    @available(added=3)
    Member bool;
    @available(added=HEAD)
    MEMBER bool;
};
"#,
    );

    // expect_fail
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_modifier_equal() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = strict strict enum {
    VALUE = 1;
};
"#,
    );

    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_modifier_equal_conflict() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = strict flexible enum {
    VALUE = 1;
};
"#,
    );

    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_modifier_subset() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = strict strict(removed=2) enum {
    VALUE = 1;
};
"#,
    );

    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_modifier_subset_conflict() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = strict flexible(removed=2) enum {
    VALUE = 1;
};
"#,
    );

    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_modifier_intersect() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = strict(removed=5) strict(added=3) enum {
    VALUE = 1;
};
"#,
    );

    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_modifier_intersect_conflict() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = strict(removed=5) flexible(added=3) enum {
    VALUE = 1;
};
"#,
    );

    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_modifier_multiple() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = strict strict(added=3) strict(added=HEAD) enum {
    VALUE = 1;
};
"#,
    );

    // expect_fail
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_modifier_multiple_conflict() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = strict flexible(added=3) strict(added=HEAD) enum {
    VALUE = 1;
};
"#,
    );

    // expect_fail
    // expect_fail
    assert!(library.check_compile());
}
