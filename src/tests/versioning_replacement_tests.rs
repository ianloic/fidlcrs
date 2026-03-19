#![allow(unused_mut, unused_variables, unused_imports, dead_code)]
use crate::diagnostics::Error;
use crate::experimental_flags::ExperimentalFlag;
use crate::tests::test_library::{LookupHelpers, TestLibrary};

#[test]
#[ignore]
fn good_decl_removed() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(removed=2)
type Foo = struct {};
"#,
    );
    //   library.select_version("example", GetParam());
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_decl_removed() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(removed=2)
type Foo = struct {};

@available(added=2)
type Foo = resource struct {};
"#,
    );
    //   library.select_version("example", GetParam());
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_decl_replaced() {
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
    //   library.select_version("example", GetParam());
    assert!(library.check_compile());

    //             GetParam() == V1 ? Resourceness::kValue : Resourceness::kResource);
}

#[test]
#[ignore]
fn bad_decl_replaced() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(replaced=2)
type Foo = struct {};
"#,
    );
    //   library.select_version("example", GetParam());
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_member_removed() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = table {
    @available(removed=2)
    1: bar string;
};
"#,
    );
    //   library.select_version("example", GetParam());
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_member_removed() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = table {
    @available(removed=2)
    1: bar string;
    @available(added=2)
    1: bar uint32;
};
"#,
    );
    //   library.select_version("example", GetParam());
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_member_replaced() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = table {
    @available(replaced=2)
    1: bar string;
    @available(added=2)
    1: bar uint32;
};
"#,
    );
    //   library.select_version("example", GetParam());
    assert!(library.check_compile());

    //             GetParam() == V1 ? Type::Kind::kString : Type::Kind::kPrimitive);
}

#[test]
#[ignore]
fn bad_member_replaced() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = table {
    @available(replaced=2)
    1: bar string;
};
"#,
    );
    //   library.select_version("example", GetParam());
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_method_removed() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

protocol Foo {
    @available(removed=2)
    Bar();
};
"#,
    );
    //   library.select_version("example", GetParam());
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_method_removed() {
    let mut library = TestLibrary::new();
    //   library.select_version("test", GetParam());
    library.add_errcat_file("bad/fi-0205.test.fidl");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_method_replaced() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

protocol Foo {
    @available(replaced=2)
    strict Bar();
    @available(added=2)
    flexible Bar();
};
"#,
    );
    //   library.select_version("example", GetParam());
    assert!(library.check_compile());

    //             GetParam() == V1 ? Strictness::kStrict : Strictness::kFlexible);
}

#[test]
#[ignore]
fn bad_method_replaced() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0206.test.fidl");
    //   library.select_version("test", GetParam());
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_method_removed_new_compose() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

protocol Protocol {
  @available(removed=2)
  Method();

  @available(added=2)
  compose Base;
};

protocol Base {
  @selector("example/Protocol.method")
  Method();
};
"#,
    );
    //   library.select_version("example", GetParam());
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_method_replaced_new_compose() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

protocol Protocol {
  @available(replaced=2)
  Method();

  @available(added=2)
  compose Base;
};

protocol Base {
  @selector("example/Protocol.method")
  Method();
};
"#,
    );
    //   library.select_version("example", GetParam());
    assert!(library.check_compile());

    //             GetParam() >= V2);
}

#[test]
#[ignore]
fn bad_method_removed_existing_compose() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

protocol Protocol {
  @available(removed=2)
  Method();

  compose Base;
};

protocol Base {
  @available(added=2)
  @selector("example/Protocol.method")
  Method();
};
"#,
    );
    //   library.select_version("example", GetParam());
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_method_replaced_existing_compose() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

protocol Protocol {
  @available(replaced=2)
  Method();

  compose Base;
};

protocol Base {
  @available(added=2)
  @selector("example/Protocol.method")
  Method();
};
"#,
    );
    //   library.select_version("example", GetParam());
    assert!(library.check_compile());

    //             GetParam() >= V2);
}

#[test]
#[ignore]
fn bad_compose_removed_new_method() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

protocol Protocol {
  @available(removed=2)
  compose Base;

  @available(added=2)
  Method();
};

protocol Base {
  @selector("example/Protocol.method")
  Method();
};
"#,
    );
    //   library.select_version("example", GetParam());
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_compose_replaced_new_method() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

protocol Protocol {
  @available(replaced=2)
  compose Base;

  @available(added=2)
  Method();
};

protocol Base {
  @selector("example/Protocol.method")
  Method();
};
"#,
    );
    //   library.select_version("example", GetParam());
    assert!(library.check_compile());

    //             GetParam() == V1);
}

#[test]
#[ignore]
fn bad_compose_removed_new_method_composee_simultaneously_removed() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

protocol Protocol {
  @available(removed=2)
  compose Base;

  @available(added=2)
  Method();
};

@available(removed=2)
protocol Base {
  @selector("example/Protocol.method")
  Method();
};
"#,
    );
    //   library.select_version("example", GetParam());
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_compose_replaced_new_method_composee_simultaneously_removed() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

protocol Protocol {
  @available(replaced=2)
  compose Base;

  @available(added=2)
  Method();
};

@available(removed=2)
protocol Base {
  @selector("example/Protocol.method")
  Method();
};
"#,
    );
    //   library.select_version("example", GetParam());
    assert!(library.check_compile());

    //             GetParam() == V1);
}

#[test]
#[ignore]
fn bad_compose_removed_new_compose() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

protocol Protocol {
  @available(removed=2)
  compose Foo;

  @available(added=2)
  compose Bar;
};

protocol Foo {
  @selector("selector/for.method")
  Method();
};

protocol Bar {
  @selector("selector/for.method")
  Method();
};
"#,
    );
    //   library.select_version("example", GetParam());
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_compose_replaced_new_compose() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

protocol Protocol {
  @available(replaced=2)
  compose Foo;

  @available(added=2)
  compose Bar;
};

protocol Foo {
  @selector("selector/for.method")
  Method();
};

protocol Bar {
  @selector("selector/for.method")
  Method();
};
"#,
    );
    //   library.select_version("example", GetParam());
    assert!(library.check_compile());

    //             GetParam() == V1 ? "Foo" : "Bar");
}

#[test]
#[ignore]
fn bad_compose_removed_new_compose_composee_simultaneously_removed() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

protocol Protocol {
  @available(removed=2)
  compose Foo;

  @available(added=2)
  compose Bar;
};

@available(removed=2)
protocol Foo {
  @selector("selector/for.method")
  Method();
};

protocol Bar {
  @selector("selector/for.method")
  Method();
};
"#,
    );
    //   library.select_version("example", GetParam());
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_compose_replaced_new_compose_composee_simultaneously_removed() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

protocol Protocol {
  @available(replaced=2)
  compose Foo;

  @available(added=2)
  compose Bar;
};

@available(removed=2)
protocol Foo {
  @selector("selector/for.method")
  Method();
};

protocol Bar {
  @selector("selector/for.method")
  Method();
};
"#,
    );
    //   library.select_version("example", GetParam());
    assert!(library.check_compile());

    //             GetParam() == V1 ? "Foo" : "Bar");
}

#[test]
#[ignore]
fn bad_composed_method_removed_new_method() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

protocol Protocol {
  compose Base;

  @available(added=2)
  Method();
};

protocol Base {
  @available(removed=2)
  @selector("example/Protocol.method")
  Method();
};
"#,
    );
    //   library.select_version("example", GetParam());
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_method_removed_transitive_compose() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

protocol Protocol {
  @available(removed=2)
  Method();

  compose Intermediate;
};

protocol Intermediate {
    @available(added=2)
    compose Base;
};

protocol Base {
  @selector("example/Protocol.method")
  Method();
};
"#,
    );
    //   library.select_version("example", GetParam());
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_method_replaced_transitive_compose() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

protocol Protocol {
  @available(replaced=2)
  Method();

  compose Intermediate;
};

protocol Intermediate {
    @available(added=2)
    compose Base;
};

protocol Base {
  @selector("example/Protocol.method")
  Method();
};
"#,
    );
    //   library.select_version("example", GetParam());
    assert!(library.check_compile());

    //             GetParam() == V1 ? "Protocol" : "Base");
}

#[test]
#[ignore]
fn bad_method_and_compose_removed_hybrid() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

protocol Protocol {
  @available(removed=2)
  compose AB;
  @available(removed=2)
  @selector("selector/for.c")
  C();

  @available(added=2)
  @selector("selector/for.a")
  A();
  @available(added=2)
  compose BC;
};

protocol AB {
  @selector("selector/for.a")
  A();
  @selector("selector/for.b")
  B();
};

protocol BC {
  @selector("selector/for.b")
  B();
  @selector("selector/for.c")
  C();
};
"#,
    );
    //   library.select_version("example", GetParam());
    // expect_fail
    // expect_fail
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_method_and_compose_replaced_hybrid() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

protocol Protocol {
  @available(replaced=2)
  compose AB;
  @available(replaced=2)
  @selector("selector/for.c")
  C();

  @available(added=2)
  @selector("selector/for.a")
  A();
  @available(added=2)
  compose BC;
};

protocol AB {
  @selector("selector/for.a")
  A();
  @selector("selector/for.b")
  B();
};

protocol BC {
  @selector("selector/for.b")
  B();
  @selector("selector/for.c")
  C();
};
"#,
    );
    //   library.select_version("example", GetParam());
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_replaced_twice() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(replaced=2)
type Foo = struct {};

@available(added=2, replaced=3)
type Foo = struct {};

@available(added=3)
type Foo = struct {};
"#,
    );
    //   library.select_version("example", GetParam());
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_all_decls_replaced_and_removed() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(replaced=2)
const CONST uint32 = 1;
@available(added=2, removed=3)
const CONST uint32 = 1;

@available(replaced=2)
alias Alias = string;
@available(added=2, removed=3)
alias Alias = string;

// TODO(https://fxbug.dev/42158155): Uncomment.
// @available(replaced=2)
// type Type = string;
// @available(added=2, removed=2)
// type Type = string;

@available(replaced=2)
type Bits = bits {};
@available(added=2, removed=3)
type Bits = bits {};

@available(replaced=2)
type Enum = enum {};
@available(added=2, removed=3)
type Enum = enum {};

@available(replaced=2)
type Struct = struct {};
@available(added=2, removed=3)
type Struct = struct {};

@available(replaced=2)
type Table = table {};
@available(added=2, removed=3)
type Table = table {};

@available(replaced=2)
type Union = union {};
@available(added=2, removed=3)
type Union = union {};

@available(replaced=2)
protocol Protocol {};
@available(added=2, removed=3)
protocol Protocol {};

@available(replaced=2)
service Service {};
@available(added=2, removed=3)
service Service {};

@available(replaced=2)
resource_definition Resource : uint32 { properties { subtype flexible enum : uint32 {}; }; };
@available(added=2, removed=3)
resource_definition Resource : uint32 { properties { subtype flexible enum : uint32 {}; }; };
"#,
    );
    //   library.select_version("example", GetParam());
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_all_members_replaced_and_removed() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Bits = bits {
    @available(replaced=2)
    MEMBER = 1;
    @available(added=2, removed=3)
    MEMBER = 1;
};

type Enum = enum {
    @available(replaced=2)
    MEMBER = 1;
    @available(added=2, removed=3)
    MEMBER = 1;
};

type Struct = struct {
    @available(replaced=2)
    member uint32;
    @available(added=2, removed=3)
    member uint32;
};

type Table = table {
    @available(replaced=2)
    1: member uint32;
    @available(added=2, removed=3)
    1: member uint32;
};

type Union = union {
    @available(replaced=2)
    1: member uint32;
    @available(added=2, removed=3)
    1: member uint32;
};

protocol Protocol {
    @available(replaced=2)
    Member();
    @available(added=2, removed=3)
    Member();
};

service Service {
    @available(replaced=2)
    member client_end:Protocol;
    @available(added=2, removed=3)
    member client_end:Protocol;
};
"#,
    );
    //   library.select_version("example", GetParam());
    assert!(library.check_compile());
    //   let num_members = GetParam() <= V2 ? 1u : 0u;
}

#[test]
#[ignore]
fn good_all_members_replaced_and_renamed() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Bits = bits {
    @available(replaced=2, renamed="NEW")
    OLD = 1;
    @available(added=2)
    NEW = 1;
};

type Enum = enum {
    @available(replaced=2, renamed="NEW")
    OLD = 1;
    @available(added=2)
    NEW = 1;
};

type Struct = struct {
    @available(replaced=2, renamed="new")
    old uint32;
    @available(added=2)
    new uint32;
};

type Table = table {
    @available(replaced=2, renamed="new")
    1: old uint32;
    @available(added=2)
    1: new uint32;
};

type Union = union {
    @available(replaced=2, renamed="new")
    1: old uint32;
    @available(added=2)
    1: new uint32;
};

protocol Protocol {
    @available(replaced=2, renamed="New")
    Old();
    @available(added=2)
    @selector("Old")
    New();
};

service Service {
    @available(replaced=2, renamed="new")
    old client_end:Protocol;
    @available(added=2)
    new client_end:Protocol;
};
"#,
    );
    //   library.select_version("example", GetParam());
    assert!(library.check_compile());

    //   bool old = GetParam() == V1;
}

#[test]
#[ignore]
fn bad_removed_named_to_anonymous() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(removed=2)
type Foo = struct {};

type Bar = struct {
    @available(added=2)
    foo struct {};
};
"#,
    );
    //   library.select_version("example", GetParam());
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_removed_anonymous_to_named() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Bar = struct {
    // The anonymous type "Foo" inherits removed=2, but removed/replaced
    // does not apply to inherited availabilities.
    @available(removed=2)
    foo struct {};
};

@available(added=2)
type Foo = table {};
"#,
    );
    //   library.select_version("example", GetParam());
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_removed_anonymous_to_anonymous() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Bar1 = struct {
    // The anonymous type "Foo" inherits removed=2, but removed/replaced
    // does not apply to inherited availabilities.
    @available(removed=2)
    foo struct {};
};

type Bar2 = struct {
    @available(added=2)
    foo table {};
};
"#,
    );
    //   library.select_version("example", GetParam());
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_replaced_named_to_anonymous() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(replaced=2)
type Foo = struct {};

type Bar = struct {
    @available(added=2)
    foo table {};
};
"#,
    );
    //   library.select_version("example", GetParam());
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_replaced_anonymous_to_named() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Bar = struct {
    @available(replaced=2)
    foo struct {};
    @available(added=2)
    foo string;
};

@available(added=2)
type Foo = table {};
"#,
    );
    //   library.select_version("example", GetParam());
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_replaced_anonymous_to_anonymous() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Bar1 = struct {
    @available(replaced=2)
    foo struct {};
    @available(added=2)
    foo string;
};

type Bar2 = struct {
    @available(added=2)
    foo table {};
};
"#,
    );
    //   library.select_version("example", GetParam());
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_replaced_anonymous_to_nothing() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Bar = struct {
    // The anonymous type "Foo" inherits replaced=2, but removed/replaced
    // validation does not apply to inherited availabilities.
    @available(replaced=2)
    foo struct {};
    @available(added=2)
    foo string;
};
"#,
    );
    //   library.select_version("example", GetParam());
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_member_removed_and_renamed() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = table {
    @available(removed=2, renamed="old_bar")
    1: bar string;
};
"#,
    );
    //   library.select_version("example", GetParam());
    assert!(library.check_compile());

    // TODO(https://fxbug.dev/42085274): Assert "old_bar" exists when targeting both 1 and >1.
    if true {}
}

#[test]
#[ignore]
fn good_member_removed_and_renamed_name_reused() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = table {
    @available(removed=2, renamed="old_bar")
    1: bar string;
    @available(added=2)
    2: bar uint32;
};
"#,
    );
    //   library.select_version("example", GetParam());
    assert!(library.check_compile());

    // TODO(https://fxbug.dev/42085274): Assert "old_bar" exists when targeting both 1 and >1.

    //             GetParam() == V1 ? Type::Kind::kString : Type::Kind::kPrimitive);
}

#[test]
#[ignore]
fn bad_member_removed_and_renamed_name_already_used_existing() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = table {
    @available(removed=2, renamed="old_bar")
    1: bar string;
    2: old_bar uint32;
};
"#,
    );
    //   library.select_version("example", GetParam());
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_member_removed_and_renamed_name_already_used_new() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = table {
    @available(removed=2, renamed="old_bar")
    1: bar string;
    @available(added=2)
    2: old_bar uint32;
};
"#,
    );
    //   library.select_version("example", GetParam());
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_method_removed_and_renamed() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0214.test.fidl");
    //   library.select_version("test", GetParam());
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_method_replaced_and_renamed() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0215.test.fidl");
    //   library.select_version("test", GetParam());
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_bits_member_removed_abi() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = bits {
    @available(removed=2)
    A = 1;
    @available(added=2)
    B = 1;
};
"#,
    );
    //   library.select_version("example", GetParam());
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_bits_member_replaced_abi() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = bits {
    @available(replaced=2)
    A = 1;
    @available(added=2)
    A = 2;
};
"#,
    );
    //   library.select_version("example", GetParam());
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_enum_member_removed_abi() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = enum {
    @available(removed=2)
    A = 1;
    @available(added=2)
    B = 1;
};
"#,
    );
    //   library.select_version("example", GetParam());
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_enum_member_replaced_abi() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = enum {
    @available(replaced=2)
    A = 1;
    @available(added=2)
    A = 2;
};
"#,
    );
    //   library.select_version("example", GetParam());
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_struct_member_removed_abi() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = struct {
    @available(removed=2)
    bar uint32;
    @available(added=2)
    baz uint32;
};
"#,
    );
    //   library.select_version("example", GetParam());
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_struct_member_replaced_abi() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = struct {
    @available(replaced=2)
    bar uint32;
    gap uint32;
    @available(added=2)
    bar uint32;
};
"#,
    );
    //   library.select_version("example", GetParam());
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_table_member_removed_abi() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = table {
    @available(removed=2)
    1: bar uint32;
    @available(added=2)
    1: baz uint32;
};
"#,
    );
    //   library.select_version("example", GetParam());
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_table_member_replaced_abi() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = table {
    @available(replaced=2)
    1: bar uint32;
    @available(added=2)
    2: bar uint32;
};
"#,
    );
    //   library.select_version("example", GetParam());
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_union_member_removed_abi() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = union {
    @available(removed=2)
    1: bar uint32;
    @available(added=2)
    1: baz uint32;
};
"#,
    );
    //   library.select_version("example", GetParam());
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_union_member_replaced_abi() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = union {
    @available(replaced=2)
    1: bar uint32;
    @available(added=2)
    2: bar uint32;
};
"#,
    );
    //   library.select_version("example", GetParam());
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_overlay_member_removed_abi() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = strict overlay {
    @available(removed=2)
    1: bar uint32;
    @available(added=2)
    1: baz uint32;
};
"#,
    );
    library.enable_flag("zx_c_types");
    //   library.select_version("example", GetParam());
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_overlay_member_replaced_abi() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = strict overlay {
    @available(replaced=2)
    1: bar uint32;
    @available(added=2)
    2: bar uint32;
};
"#,
    );
    library.enable_flag("zx_c_types");
    //   library.select_version("example", GetParam());
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_method_removed_abi() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0216.test.fidl");
    //   library.select_version("test", GetParam());
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_method_replaced_abi() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0217.test.fidl");
    //   library.select_version("test", GetParam());
    // expect_fail
    assert!(library.check_compile());
}
