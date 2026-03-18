#![allow(unused_mut, unused_variables, unused_imports, dead_code)]
use crate::diagnostics::Error;
use crate::tests::test_library::{LookupHelpers, TestLibrary};

#[test]
#[ignore]
fn good_redundant_with_parent() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=2, deprecated=4, removed=6)
library example;

@available(added=2, deprecated=4, removed=6)
type Foo = struct {};
"#,
    );
    library.select_version("example", "2");
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_added_before_parent_added() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0155-a.test.fidl");
    library.select_version("test", "HEAD");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_added_when_parent_deprecated() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=2, deprecated=4, removed=6)
library example;

@available(added=4)
type Foo = struct {};
"#,
    );
    library.select_version("example", "4");
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_added_after_parent_deprecated() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=2, deprecated=4, removed=6)
library example;

@available(added=5)
type Foo = struct {};
"#,
    );
    library.select_version("example", "5");
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_added_when_parent_removed() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0155-b.test.fidl");
    library.select_version("test", "HEAD");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_added_when_parent_replaced() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=2, deprecated=4, replaced=6)
type Foo = struct {
    @available(added=6)
    member bool;
};

@available(added=6)
type Foo = struct {};
"#,
    );
    library.select_version("example", "HEAD");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_added_after_parent_removed() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=2, deprecated=4, removed=6)
library example;

@available(added=7)
type Foo = struct {};
"#,
    );
    library.select_version("example", "HEAD");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_added_after_parent_replaced() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=2, deprecated=4, replaced=6)
type Foo = struct {
    @available(added=7)
    member bool;
};

@available(added=6)
type Foo = struct {};
"#,
    );
    library.select_version("example", "HEAD");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_deprecated_before_parent_added() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=2, deprecated=4, removed=6)
library example;

@available(deprecated=1)
type Foo = struct {};
"#,
    );
    library.select_version("example", "HEAD");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_deprecated_when_parent_added() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=2, removed=6) // never deprecated
library example;

@available(deprecated=2)
type Foo = struct {};
"#,
    );
    library.select_version("example", "2");
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_deprecated_before_parent_deprecated() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=2, deprecated=4, removed=6)
library example;

@available(deprecated=3)
type Foo = struct {};
"#,
    );
    library.select_version("example", "3");
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_deprecated_after_parent_deprecated() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=2, deprecated=4, removed=6)
library example;

@available(deprecated=5)
type Foo = struct {};
"#,
    );
    library.select_version("example", "HEAD");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_deprecated_when_parent_removed() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=2, deprecated=4, removed=6)
library example;

@available(deprecated=6)
type Foo = struct {};
"#,
    );
    library.select_version("example", "HEAD");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_deprecated_when_parent_replaced() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=2, deprecated=4, replaced=6)
type Foo = struct {
    @available(deprecated=6)
    member bool;
};

@available(added=6)
type Foo = struct {};
"#,
    );
    library.select_version("example", "HEAD");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_deprecated_after_parent_removed() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=2, deprecated=4, removed=6)
library example;

@available(deprecated=7)
type Foo = struct {};
"#,
    );
    library.select_version("example", "HEAD");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_deprecated_after_parent_replaced() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=2, deprecated=4, replaced=6)
type Foo = struct {
    @available(deprecated=7)
    member bool;
};

@available(added=6)
type Foo = struct {};
"#,
    );
    library.select_version("example", "HEAD");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_removed_before_parent_added() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=2, deprecated=4, removed=6)
library example;

@available(removed=1)
type Foo = struct {};
"#,
    );
    library.select_version("example", "HEAD");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_replaced_before_parent_added() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=2, deprecated=4, removed=6)
type Foo = struct {
    @available(replaced=1)
    member bool;
};
"#,
    );
    library.select_version("example", "HEAD");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_removed_when_parent_added() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=2, deprecated=4, removed=6)
library example;

@available(removed=2)
type Foo = struct {};
"#,
    );
    library.select_version("example", "HEAD");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_replaced_when_parent_added() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=2, deprecated=4, removed=6)
type Foo = struct {
    @available(replaced=2)
    member bool;
};
"#,
    );
    library.select_version("example", "HEAD");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_removed_before_parent_deprecated() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=2, deprecated=4, removed=6)
library example;

@available(removed=3)
type Foo = struct {};
"#,
    );
    library.select_version("example", "2");
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_replaced_before_parent_deprecated() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=2, deprecated=4, removed=6)
type Foo = struct {
    @available(replaced=3)
    member bool;
    @available(added=3)
    member uint32;
};
"#,
    );
    library.select_version("example", "2");
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_removed_when_parent_deprecated() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=2, deprecated=4, removed=6)
library example;

@available(removed=4)
type Foo = struct {};
"#,
    );
    library.select_version("example", "3");
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_replaced_when_parent_deprecated() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=2, deprecated=4, removed=6)
type Foo = struct {
    @available(replaced=4)
    member bool;
    @available(added=4)
    member uint32;
};
"#,
    );
    library.select_version("example", "3");
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_removed_after_parent_deprecated() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=2, deprecated=4, removed=6)
library example;

@available(removed=5)
type Foo = struct {};
"#,
    );
    library.select_version("example", "4");
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_replaced_after_parent_deprecated() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=2, deprecated=4, removed=6)
type Foo = struct {
    @available(replaced=5)
    member bool;
    @available(added=5)
    member uint32;
};
"#,
    );
    library.select_version("example", "4");
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_removed_after_parent_removed() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=2, deprecated=4, removed=6)
library example;

@available(removed=7)
type Foo = struct {};
"#,
    );
    library.select_version("example", "HEAD");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_removed_after_parent_replaced() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=2, deprecated=4, replaced=6)
type Foo = struct {
    @available(removed=7)
    member bool;
};

@available(added=6)
type Foo = struct {};
"#,
    );
    library.select_version("example", "HEAD");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_replaced_after_parent_removed() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=2, deprecated=4, removed=6)
type Foo = struct {
    @available(replaced=7)
    member bool;
};
"#,
    );
    library.select_version("example", "HEAD");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_replaced_after_parent_replaced() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=2, deprecated=4, replaced=6)
type Foo = struct {
    @available(replaced=7)
    member bool;
};

@available(added=6)
type Foo = struct {};
"#,
    );
    library.select_version("example", "HEAD");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_removed_when_parent_replaced() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=2, deprecated=4, replaced=6)
type Foo = struct {
    @available(added=2, deprecated=4, removed=6)
    member bool;
};

@available(added=6)
type Foo = struct {};
"#,
    );
    library.select_version("example", "6");
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_replaced_when_parent_removed() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=2, deprecated=4, removed=6)
type Foo = struct {
    @available(added=2, deprecated=4, replaced=6)
    member bool;
};
"#,
    );
    library.select_version("example", "HEAD");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_replaced_when_parent_replaced() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=2, deprecated=4, replaced=6)
type Foo = struct {
    @available(added=2, deprecated=4, replaced=6)
    member bool;
};

@available(added=6)
type Foo = struct {};
"#,
    );
    library.select_version("example", "HEAD");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_member_inherits_from_parent() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=2)
type Foo = struct {
    @available(deprecated=3)
    member1 bool;
};
"#,
    );
    library.select_version("example", "2");
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_complex_inheritance() {
    // The following libraries all define a struct Bar with effective availability
    // @available(added=2, deprecated=3, removed=4) in different ways.

    let mut sources: Vec<&str> = Vec::new();

    // Direct annotation.
    sources.push(
        r#"
@available(added=1)
library example;

@available(added=2, deprecated=3, removed=4)
type Bar = struct {};
"#,
    );

    // Fully inherit from library declaration.
    sources.push(
        r#"
@available(added=2, deprecated=3, removed=4)
library example;

type Bar = struct {};
"#,
    );

    // Partially inherit from library declaration.
    sources.push(
        r#"
@available(added=1, deprecated=3)
library example;

@available(added=2, removed=4)
type Bar = struct {};
"#,
    );

    // Inherit from parent.
    sources.push(
        r#"
@available(added=1)
library example;

@available(added=2, deprecated=3, removed=4)
type Foo = struct {
    member @generated_name("Bar") struct {};
};
"#,
    );

    // Inherit from member.
    sources.push(
        r#"
@available(added=1)
library example;

type Foo = struct {
    @available(added=2, deprecated=3, removed=4)
    member @generated_name("Bar") struct {};
};
"#,
    );

    // Inherit from multiple, forward.
    sources.push(
        r#"
@available(added=2)
library example;

@available(deprecated=3)
type Foo = struct {
    @available(removed=4)
    member @generated_name("Bar") struct {};
};
"#,
    );

    // Inherit from multiple, backward.
    sources.push(
        r#"
@available(added=1, removed=4)
library example;

@available(deprecated=3)
type Foo = struct {
    @available(added=2)
    member @generated_name("Bar") struct {};
};
"#,
    );

    // Inherit from multiple, mixed.
    sources.push(
        r#"
@available(added=1)
library example;

@available(added=2)
type Foo = struct {
    @available(deprecated=3, removed=4)
    member @generated_name("Bar") struct {};
};
"#,
    );

    // Inherit via nested layouts.
    sources.push(
        r#"
@available(added=1)
library example;

@available(added=2)
type Foo = struct {
    @available(deprecated=3)
    member1 struct {
        @available(removed=4)
        member2 struct {
            member3 @generated_name("Bar") struct {};
        };
    };
};
"#,
    );

    // Inherit via nested type constructors.
    sources.push(
        r#"
@available(added=1)
library example;

@available(added=2)
type Foo = struct {
    @available(deprecated=3, removed=4)
    member1 vector<vector<vector<@generated_name("Bar") struct{}>>>;
};
"#,
    );

    for version in ["1", "2", "3", "4"] {
        // SCOPED_TRACE(version);
        for source in &sources {
            let mut library = TestLibrary::new();
            library.add_errcat_file(source);
            library.select_version("example", version);
            assert!(library.check_compile());

            if true {}
        }
    }
}

#[test]
#[ignore]
fn bad_decl_conflicts_with_parent() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#" // L1
@available(added=2)           // L2
library example;              // L3
                              // L4
@available(added=1)           // L5
type Foo = struct {};
"#,
    );
    library.select_version("example", "HEAD");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_member_conflicts_with_parent() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#" // L1
@available(added=1)           // L2
library example;              // L3
                              // L4
@available(added=2)           // L5
type Foo = struct {           // L6
    @available(added=1)       // L7
    member1 bool;
};
"#,
    );
    library.select_version("example", "HEAD");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_member_conflicts_with_grand_parent() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#" // L1
@available(added=2)           // L2
library example;              // L3
                              // L4
@available(removed=3)         // L5
type Foo = struct {           // L6
    @available(added=1)       // L7
    member1 bool;
};
"#,
    );
    library.select_version("example", "HEAD");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_member_conflicts_with_grand_parent_through_anonymous() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#" // L1
@available(added=1)           // L2
library example;              // L3
                              // L4
@available(added=2)           // L5
type Foo = struct {           // L6
    member1 struct {          // L7
        @available(removed=1) // L8
        member2 bool;
    };
};
"#,
    );
    library.select_version("example", "HEAD");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_modifier_added_before_parent_added() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=2)
library example;

type Foo = resource(added=1) struct {};
"#,
    );
    library.select_version("example", "HEAD");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_modifier_removed_after_parent_removed() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1, removed=2)
library example;

type Foo = resource(removed=3) struct {};
"#,
    );
    library.select_version("example", "HEAD");
    // expect_fail
    assert!(library.check_compile());
}
