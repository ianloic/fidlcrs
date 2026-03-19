#![allow(unused_mut, unused_variables, unused_imports, dead_code)]
use crate::diagnostics::Error;
use crate::experimental_flags::ExperimentalFlag;
use crate::tests::test_library::{LookupHelpers, TestLibrary};

#[test]
fn good_new_types() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

type Foo = struct {
  bytes vector<uint8>;
};

type OpaqueFoo = Foo;

type Bar = enum {
  PARALLEL = 0;
  PERPENDICULAR = 1;
};

type OpaqueBar = Bar;
"#,
    );

    library.enable_flag("allow_new_types");
    assert!(library.check_compile());
}

#[test]
fn good_new_types_resourceness() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

type A = resource struct {};
type B = A;
type C = resource struct { b B; };
"#,
    );

    library.enable_flag("allow_new_types");
    assert!(library.check_compile());
}

#[test]
fn bad_new_types_resourceness() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

type A = resource struct {};
type B = A;
type C = struct { b B; };
"#,
    );

    library.enable_flag("allow_new_types");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
fn good_new_types_simple() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

type Bits = bits { A = 1; };
type Enum = enum {
  A = 1;
  B = 15;
};
type Struct = struct { foo string; };
type Table = table {};
type Union = union { 1: b bool; };
alias Alias = Struct;

// Now for the new-types
type NewBits = Bits;
type NewEnum = Enum;
type NewStruct = Struct;
type NewTable = Table;
type NewUnion = Union;
type NewAlias = Alias;
"#,
    );

    library.enable_flag("allow_new_types");
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_new_types_builtin() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;
using zx;

type Struct = struct {};
protocol Protocol {};

type NewBool = bool;
type NewInt = int16;
type NewString = string;
type NewArray = array<int8, 3>;
type NewVector = vector<bool>;
type NewBox = box<Struct>;
type NewHandle = zx.handle;
type NewClientEnd = client_end:Protocol;
type NewServerEnd = server_end:Protocol;
"#,
    );

    library.enable_flag("allow_new_types");
    library.use_library_zx();
    assert!(library.check_compile());
}

#[test]
fn good_new_types_complex() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

type Struct = struct {};
type NewStruct = Struct;
type DoubleNewStruct = NewStruct;
"#,
    );

    library.enable_flag("allow_new_types");
    assert!(library.check_compile());
}

#[test]
fn good_new_types_constrained() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

type ConstrainedVec = vector<int32>:<5, optional>;
type ConstrainedString = string:108;
"#,
    );

    library.enable_flag("allow_new_types");
    assert!(library.check_compile());
}

#[test]
fn bad_new_types_constraints() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0179.test.fidl");
    library.enable_flag("allow_new_types");
    // expect_fail
    assert!(library.check_compile());
}
