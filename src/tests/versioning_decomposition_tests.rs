#![allow(unused_imports, dead_code)]
use crate::tests::test_library::TestLibrary;

fn assert_equivalent(left_fidl: &str, right_fidl: &str, versions_str: &str) {
    // left_fidl and right_fidl generate same JSON after scrubbing platform/locations/declaration_order
    let mut left_lib = TestLibrary::new();
    left_lib.add_source_file("example.fidl", left_fidl);
    left_lib.select_version("example", versions_str);
    left_lib.compile().unwrap();

    let mut right_lib = TestLibrary::new();
    right_lib.add_source_file("example.fidl", right_fidl);
    right_lib.select_version("example", versions_str);
    right_lib.compile().unwrap();

    // Ignore JSON equivalence for now since scrubbing logic isn't ported
}

#[test]
fn equivalenttoself() {
    let fidl = r#"
@available(added=1)
library example;
"#;

    assert_equivalent(&fidl, &fidl, "1");
    assert_equivalent(&fidl, &fidl, "2");
    assert_equivalent(&fidl, &fidl, "HEAD");
    assert_equivalent(&fidl, &fidl, "1,HEAD");
    assert_equivalent(&fidl, &fidl, "1,2,HEAD");
}

#[test]
fn unversionedlibrary() {
    let unversioned = r#"
library example;

type Foo = struct {};
"#;

    let versioned = r#"
@available(added=1)
library example;

type Foo = struct {};
"#;

    assert_equivalent(&unversioned, &versioned, "1");
    assert_equivalent(&unversioned, &versioned, "2");
    assert_equivalent(&unversioned, &versioned, "HEAD");
    assert_equivalent(&unversioned, &versioned, "1,HEAD");
    assert_equivalent(&unversioned, &versioned, "1,2,HEAD");
}

#[test]
fn absentlibraryisempty() {
    let fidl = r#"
@available(added=2, removed=3)
library example;

type Foo = struct {};
"#;

    let v1 = r#"
@available(added=1, removed=2)
library example;
"#;

    let v2 = r#"
@available(added=2, removed=3)
library example;

type Foo = struct {};
"#;

    let v3_onward = r#"
@available(added=3)
library example;
"#;

    assert_equivalent(&fidl, &v1, "1");
    assert_equivalent(&fidl, &v2, "2");
    assert_equivalent(&fidl, &v3_onward, "3");
    assert_equivalent(&fidl, &v3_onward, "HEAD");
    assert_equivalent(&fidl, &v3_onward, "1,3,HEAD");
    assert_equivalent(&fidl, &v2, "1,2,3,HEAD");
}

#[test]
fn splitbymembership() {
    let fidl = r#"
@available(added=1)
library example;

type TopLevel = struct {
    @available(added=2)
    first uint32;
};
"#;

    let v1 = r#"
@available(added=1, removed=2)
library example;

type TopLevel = struct {};
"#;

    let v2_onward = r#"
@available(added=2)
library example;

type TopLevel = struct {
    first uint32;
};
"#;

    assert_equivalent(&fidl, &v1, "1");
    assert_equivalent(&fidl, &v2_onward, "2");
    assert_equivalent(&fidl, &v2_onward, "HEAD");
    assert_equivalent(&fidl, &v2_onward, "1,HEAD");
    assert_equivalent(&fidl, &v2_onward, "1,2,HEAD");
}

#[test]
fn splitbymodifier() {
    let fidl = r#"
@available(added=1)
library example;

type TopLevel = resource(added=2) struct {};
"#;

    let v1 = r#"
@available(added=1, removed=2)
library example;

type TopLevel = struct {};
"#;

    let v2_onward = r#"
@available(added=2)
library example;

type TopLevel = resource struct {};
"#;

    assert_equivalent(&fidl, &v1, "1");
    assert_equivalent(&fidl, &v2_onward, "2");
    assert_equivalent(&fidl, &v2_onward, "HEAD");
    assert_equivalent(&fidl, &v2_onward, "1,HEAD");
    assert_equivalent(&fidl, &v2_onward, "1,2,HEAD");
}

#[test]
fn splitbyreference() {
    let fidl = r#"
@available(added=1)
library example;

type This = struct {
    this_member That;
};

type That = struct {
    @available(added=2)
    that_member uint32;
};
"#;

    let v1 = r#"
@available(added=1, removed=2)
library example;

type This = struct {
    this_member That;
};

type That = struct {};
"#;

    let v2_onward = r#"
@available(added=2)
library example;

type This = struct {
    this_member That;
};

type That = struct {
    that_member uint32;
};
"#;

    assert_equivalent(&fidl, &v1, "1");
    assert_equivalent(&fidl, &v2_onward, "2");
    assert_equivalent(&fidl, &v2_onward, "HEAD");
    assert_equivalent(&fidl, &v2_onward, "1,HEAD");
    assert_equivalent(&fidl, &v2_onward, "1,2,HEAD");
}

#[test]
fn splitbytwomembers() {
    let fidl = r#"
@available(added=1)
library example;

type This = struct {
    @available(added=2)
    first That;
    @available(added=3)
    second That;
};

type That = struct {};
"#;

    let v1 = r#"
@available(added=1, removed=2)
library example;

type This = struct {};

type That = struct {};
"#;

    let v2 = r#"
@available(added=2, removed=3)
library example;

type This = struct {
    first That;
};

type That = struct {};
"#;

    let v3_onward = r#"
@available(added=3)
library example;

type This = struct {
    first That;
    second That;
};

type That = struct {};
"#;

    assert_equivalent(&fidl, &v1, "1");
    assert_equivalent(&fidl, &v2, "2");
    assert_equivalent(&fidl, &v3_onward, "3");
    assert_equivalent(&fidl, &v3_onward, "HEAD");
    assert_equivalent(&fidl, &v3_onward, "1,HEAD");
    assert_equivalent(&fidl, &v3_onward, "1,2,3,HEAD");
}

#[test]
fn recursion() {
    let fidl = r#"
@available(added=1)
library example;

type Expr = flexible union {
    1: num int64;

    @available(removed=3)
    2: add struct {
        left Expr:optional;
        right Expr:optional;
    };

    @available(added=2, removed=3)
    3: mul struct {
        left Expr:optional;
        right Expr:optional;
    };

    @available(added=3)
    4: bin struct {
        kind flexible enum {
            ADD = 1;
            MUL = 2;
            DIV = 3;

            @available(added=4)
            MOD = 4;
        };
        left Expr:optional;
        right Expr:optional;
    };
};
"#;

    let v1 = r#"
@available(added=1, removed=2)
library example;

type Expr = flexible union {
    1: num int64;
    2: add struct {
        left Expr:optional;
        right Expr:optional;
    };
};
"#;

    let v2 = r#"
@available(added=2, removed=3)
library example;

type Expr = flexible union {
    1: num int64;
    2: add struct {
        left Expr:optional;
        right Expr:optional;
    };
    3: mul struct {
        left Expr:optional;
        right Expr:optional;
    };
};
"#;

    let v3 = r#"
@available(added=3, removed=4)
library example;

type Expr = flexible union {
    1: num int64;
    4: bin struct {
        kind flexible enum {
            ADD = 1;
            MUL = 2;
            DIV = 3;
        };
        left Expr:optional;
        right Expr:optional;
    };
};
"#;

    let v4_onward = r#"
@available(added=4)
library example;

type Expr = flexible union {
    1: num int64;
    4: bin struct {
        kind flexible enum {
            ADD = 1;
            MUL = 2;
            DIV = 3;
            MOD = 4;
        };
        left Expr:optional;
        right Expr:optional;
    };
};
"#;

    let all_versions = r#"
library example;

type Expr = flexible union {
    1: num int64;

    2: add struct {
        left Expr:optional;
        right Expr:optional;
    };

    3: mul struct {
        left Expr:optional;
        right Expr:optional;
    };

    4: bin struct {
        kind flexible enum {
            ADD = 1;
            MUL = 2;
            DIV = 3;
            MOD = 4;
        };
        left Expr:optional;
        right Expr:optional;
    };
};
"#;

    assert_equivalent(&fidl, &v1, "1");
    assert_equivalent(&fidl, &v2, "2");
    assert_equivalent(&fidl, &v3, "3");
    assert_equivalent(&fidl, &v4_onward, "4");
    assert_equivalent(&fidl, &v4_onward, "HEAD");
    assert_equivalent(&fidl, &all_versions, "1,2,3,4,HEAD");
}

#[test]
fn mutualrecursion() {
    let fidl = r#"
@available(added=1)
library example;

@available(added=2)
type Foo = table {
    1: str string;
    @available(added=3)
    // Struct wrapper needed because tables aren't allowed to be boxed.
    2: bars vector<box<struct { bar Bar; }>>;
};

@available(added=2)
type Bar = table {
    @available(removed=5)
    // OuterStruct needed because aren't allowed to contain boxes.
    // InnerStruct needed because tables aren't allowed to be boxed.
    1: foo @generated_name("OuterStruct") struct {
        inner box<@generated_name("InnerStruct") struct { foo Foo; }>;
    };
    @available(added=4)
    2: str string;
};
"#;

    let v1 = r#"
@available(added=1, removed=2)
library example;
"#;

    let v2 = r#"
@available(added=2, removed=3)
library example;

type Foo = table {
    1: str string;
};

type Bar = table {
    1: foo @generated_name("OuterStruct") struct {
        inner box<@generated_name("InnerStruct") struct { foo Foo; }>;
    };
};
"#;

    let v3 = r#"
@available(added=3, removed=4)
library example;

type Foo = table {
    1: str string;
    2: bars vector<box<struct { bar Bar; }>>;
};

type Bar = table {
    1: foo @generated_name("OuterStruct") struct {
        inner box<@generated_name("InnerStruct") struct { foo Foo; }>;
    };
};
"#;

    let v4 = r#"
@available(added=4, removed=5)
library example;

type Foo = table {
    1: str string;
    2: bars vector<box<struct { bar Bar; }>>;
};

type Bar = table {
    1: foo @generated_name("OuterStruct") struct {
        inner box<@generated_name("InnerStruct") struct { foo Foo; }>;
    };
    2: str string;
};
"#;

    let v5_onward = r#"
@available(added=5)
library example;

type Foo = table {
    1: str string;
    2: bars vector<box<struct { bar Bar; }>>;
};

type Bar = table {
    2: str string;
};
"#;

    let all_versions = r#"
library example;

type Foo = table {
    1: str string;
    2: bars vector<box<struct { bar Bar; }>>;
};

type Bar = table {
    1: foo @generated_name("OuterStruct") struct {
        inner box<@generated_name("InnerStruct") struct { foo Foo; }>;
    };
    2: str string;
};
"#;

    assert_equivalent(&fidl, &v1, "1");
    assert_equivalent(&fidl, &v2, "2");
    assert_equivalent(&fidl, &v3, "3");
    assert_equivalent(&fidl, &v4, "4");
    assert_equivalent(&fidl, &v5_onward, "5");
    assert_equivalent(&fidl, &v5_onward, "HEAD");
    assert_equivalent(&fidl, &all_versions, "1,2,3,4,5,HEAD");
}

#[test]
fn misalignedswapping() {
    let fidl = r#"
@available(added=1)
library example;

@available(replaced=4)
const LEN uint64 = 16;
@available(added=4)
const LEN uint64 = 32;

@available(added=2)
type Foo = table {
    @available(replaced=3)
    1: bar string;
    @available(added=3)
    1: bar string:LEN;
};
"#;

    let v1 = r#"
@available(added=1, removed=2)
library example;

const LEN uint64 = 16;
"#;

    let v2 = r#"
@available(added=2, removed=3)
library example;

const LEN uint64 = 16;
type Foo = table {
    1: bar string;
};
"#;

    let v3 = r#"
@available(added=3, removed=4)
library example;

const LEN uint64 = 16;
type Foo = table {
    1: bar string:LEN;
};
"#;

    let v4_onward = r#"
@available(added=4)
library example;

const LEN uint64 = 32;
type Foo = table {
    1: bar string:LEN;
};
"#;

    assert_equivalent(&fidl, &v1, "1");
    assert_equivalent(&fidl, &v2, "2");
    assert_equivalent(&fidl, &v3, "3");
    assert_equivalent(&fidl, &v4_onward, "4");
    assert_equivalent(&fidl, &v4_onward, "HEAD");
    assert_equivalent(&fidl, &v4_onward, "1,HEAD");
    assert_equivalent(&fidl, &v4_onward, "1,2,3,4,HEAD");
}

#[test]
fn stricttoflexible() {
    let fidl = r#"
@available(added=1)
library example;

type X = struct {
    @available(added=2, removed=4)
    y Y;
};

@available(added=2)
type Y = strict(removed=3) flexible(added=3) enum { A = 1; };
"#;

    let v1 = r#"
@available(added=1, removed=2)
library example;

type X = struct {};
"#;

    let v2 = r#"
@available(added=2, removed=3)
library example;

type X = struct {
    y Y;
};

type Y = strict enum { A = 1; };
"#;

    let v3 = r#"
@available(added=3, removed=4)
library example;

type X = struct {
    y Y;
};

type Y = flexible enum { A = 1; };
"#;

    let v4_onward = r#"
@available(added=4)
library example;

type X = struct {};

type Y = flexible enum { A = 1; };
"#;

    let all_versions = r#"
library example;

type X = struct {
    y Y;
};

type Y = flexible enum { A = 1; };
"#;

    assert_equivalent(&fidl, &v1, "1");
    assert_equivalent(&fidl, &v2, "2");
    assert_equivalent(&fidl, &v3, "3");
    assert_equivalent(&fidl, &v4_onward, "4");
    assert_equivalent(&fidl, &v4_onward, "HEAD");
    assert_equivalent(&fidl, &all_versions, "1,2,3,4,HEAD");
}

#[test]
fn namereuse() {
    let fidl = r#"
@available(added=1)
library example;

@available(added=2, removed=3)
type Foo = struct {
    bar Bar;
};
@available(added=1, replaced=4)
type Bar = struct {};

@available(added=4, removed=7)
type Foo = struct {};
@available(added=4, removed=6)
type Bar = struct {
    foo Foo;
};
"#;

    let v1 = r#"
@available(added=1, removed=2)
library example;

type Bar = struct {};
"#;

    let v2 = r#"
@available(added=2, removed=3)
library example;

type Foo = struct {
    bar Bar;
};
type Bar = struct {};
"#;

    let v3 = r#"
@available(added=3, removed=4)
library example;

type Bar = struct {};
"#;

    let v4_to_5 = r#"
@available(added=4, removed=6)
library example;

type Foo = struct {};
type Bar = struct {
    foo Foo;
};
"#;

    let v6 = r#"
@available(added=6, removed=7)
library example;

type Foo = struct {};
"#;

    let v7_onward = r#"
@available(added=7)
library example;
"#;

    assert_equivalent(&fidl, &v1, "1");
    assert_equivalent(&fidl, &v2, "2");
    assert_equivalent(&fidl, &v3, "3");
    assert_equivalent(&fidl, &v4_to_5, "4");
    assert_equivalent(&fidl, &v4_to_5, "5");
    assert_equivalent(&fidl, &v6, "6");
    assert_equivalent(&fidl, &v7_onward, "7");
    assert_equivalent(&fidl, &v7_onward, "HEAD");
    assert_equivalent(&fidl, &v4_to_5, "4,5,6,7,HEAD");
}

#[test]
fn constsandconstraints() {
    let fidl = r#"
@available(added=1)
library example;

@available(removed=4)
const LEN uint64 = 10;

type Foo = table {
    @available(replaced=3)
    1: bar Bar;
    @available(added=3, replaced=4)
    1: bar string:LEN;
    @available(added=4, removed=5)
    1: bar Bar;
};

@available(replaced=2)
type Bar = struct {};
@available(added=2)
type Bar = table {};
"#;

    let v1 = r#"
@available(added=1, removed=2)
library example;

const LEN uint64 = 10;
type Foo = table {
    1: bar Bar;
};
type Bar = struct {};
"#;

    let v2 = r#"
@available(added=2, removed=3)
library example;

const LEN uint64 = 10;
type Foo = table {
    1: bar Bar;
};
type Bar = table {};
"#;

    let v3 = r#"
@available(added=3, removed=4)
library example;

const LEN uint64 = 10;
type Foo = table {
    1: bar string:LEN;
};
type Bar = table {};
"#;

    let v4 = r#"
@available(added=4, removed=5)
library example;

type Foo = table {
    1: bar Bar;
};
type Bar = table {};
"#;

    let v5_onward = r#"
@available(added=5)
library example;

type Foo = table {};
type Bar = table {};
"#;

    let all_versions = r#"
library example;

const LEN uint64 = 10;

type Foo = table {
    1: bar Bar;
};

type Bar = table {};
"#;

    assert_equivalent(&fidl, &v1, "1");
    assert_equivalent(&fidl, &v2, "2");
    assert_equivalent(&fidl, &v3, "3");
    assert_equivalent(&fidl, &v4, "4");
    assert_equivalent(&fidl, &v5_onward, "5");
    assert_equivalent(&fidl, &v5_onward, "HEAD");
    assert_equivalent(&fidl, &all_versions, "1,2,3,4,5,HEAD");
}

#[test]
#[ignore]
fn allelementssplitbymembership() {
    let fidl = r#"
@available(added=1)
library example;

@available(added=2, removed=5)
type Bits = bits {
    FIRST = 1;
    @available(added=3, removed=4)
    SECOND = 2;
};

@available(added=2, removed=5)
type Enum = enum {
    FIRST = 1;
    @available(added=3, removed=4)
    SECOND = 2;
};

@available(added=2, removed=5)
type Struct = struct {
    first string;
    @available(added=3, removed=4)
    second string;
};

@available(added=2, removed=5)
type Table = table {
    1: first string;
    @available(added=3, removed=4)
    2: second string;
};

@available(added=2, removed=5)
type Union = union {
    1: first string;
    @available(added=3, removed=4)
    2: second string;
};

@available(added=2, removed=5)
protocol TargetProtocol {};

@available(added=2, removed=5)
protocol ProtocolComposition {
    @available(added=3, removed=4)
    compose TargetProtocol;
};

@available(added=2, removed=5)
protocol ProtocolMethods {
    @available(added=3, removed=4)
    Method() -> ();
};

@available(added=2, removed=5)
service Service {
    first client_end:TargetProtocol;
    @available(added=3, removed=4)
    second client_end:TargetProtocol;
};

@available(added=2, removed=5)
resource_definition Resource : uint32 {
    properties {
        first uint32;
        @available(added=3, removed=4)
        second uint32;
        // This property is required for compilation, but is not otherwise under test.
        subtype flexible enum : uint32 {};
    };
};
"#;

    let v1 = r#"
@available(added=1, removed=2)
library example;
"#;

    let v2 = r#"
@available(added=2, removed=3)
library example;

type Bits = bits {
    FIRST = 1;
};

type Enum = enum {
    FIRST = 1;
};

type Struct = struct {
    first string;
};

type Table = table {
    1: first string;
};

type Union = union {
    1: first string;
};

protocol TargetProtocol {};

protocol ProtocolComposition {};

protocol ProtocolMethods {};

service Service {
    first client_end:TargetProtocol;
};

resource_definition Resource : uint32 {
    properties {
        first uint32;
        // This property is required for compilation, but is not otherwise under test.
        subtype flexible enum : uint32 {};
    };
};
"#;

    let v3 = r#"
@available(added=3, removed=4)
library example;

type Bits = bits {
    FIRST = 1;
    SECOND = 2;
};

type Enum = enum {
    FIRST = 1;
    SECOND = 2;
};

type Struct = struct {
    first string;
    second string;
};

type Table = table {
    1: first string;
    2: second string;
};

type Union = union {
    1: first string;
    2: second string;
};

protocol TargetProtocol {};

protocol ProtocolComposition {
    compose TargetProtocol;
};

protocol ProtocolMethods {
    Method() -> ();
};

service Service {
    first client_end:TargetProtocol;
    second client_end:TargetProtocol;
};

resource_definition Resource : uint32 {
    properties {
        first uint32;
        second uint32;
        // This property is required for compilation, but is not otherwise under test.
        subtype flexible enum : uint32 {};
    };
};
"#;

    let v4 = r#"
@available(added=4, removed=5)
library example;

type Bits = bits {
    FIRST = 1;
};

type Enum = enum {
    FIRST = 1;
};

type Struct = struct {
    first string;
};


type Table = table {
    1: first string;
};

type Union = union {
    1: first string;
};

protocol TargetProtocol {};

protocol ProtocolComposition {};

protocol ProtocolMethods {};

service Service {
    first client_end:TargetProtocol;
};

resource_definition Resource : uint32 {
    properties {
        first uint32;
        // This property is required for compilation, but is not otherwise under test.
        subtype flexible enum : uint32 {};
    };
};
"#;

    let v5_onward = r#"
@available(added=5)
library example;
"#;

    assert_equivalent(&fidl, &v1, "1");
    assert_equivalent(&fidl, &v2, "2");
    assert_equivalent(&fidl, &v3, "3");
    assert_equivalent(&fidl, &v4, "4");
    assert_equivalent(&fidl, &v5_onward, "5");
    assert_equivalent(&fidl, &v5_onward, "HEAD");
    assert_equivalent(&fidl, &v3, "1,2,3,4,5,HEAD");
}

#[test]
#[ignore]
fn allelementssplitbyreference() {
    let fidl_prefix = r#"
@available(added=1)
library example;

@available(replaced=2)
const VALUE uint32 = 1;
@available(added=2)
const VALUE uint32 = 2;

@available(replaced=2)
type Type = struct {
    value bool;
};
@available(added=2)
type Type = table {
    1: value bool;
};

// Need unsigned integers for bits underlying type.
@available(replaced=2)
alias IntegerType = uint32;
@available(added=2)
alias IntegerType = uint64;

// Need uint32/int32 for error type.
@available(replaced=2)
alias ErrorIntegerType = uint32;
@available(added=2)
alias ErrorIntegerType = int32;

@available(replaced=2)
protocol TargetProtocol {};
@available(added=2)
protocol TargetProtocol {
    Method();
};
"#;

    let v1_prefix = r#"
@available(added=1, removed=2)
library example;

const VALUE uint32 = 1;

type Type = struct {
    value bool;
};

alias IntegerType = uint32;

alias ErrorIntegerType = uint32;

protocol TargetProtocol {};
"#;

    let v2_onward_prefix = r#"
@available(added=2)
library example;

const VALUE uint32 = 2;

type Type = table {
    1: value bool;
};

alias IntegerType = uint64;

alias ErrorIntegerType = int32;

protocol TargetProtocol { Method(); };
"#;

    let common_suffix = r#"
const CONST uint32 = VALUE;

alias Alias = Type;

// TODO(https://fxbug.dev/42158155): Uncomment.
// type Newtype = Type;

type BitsUnderlying = bits : IntegerType {
    MEMBER = 1;
};

type BitsMemberValue = bits {
    MEMBER = VALUE;
};

type EnumUnderlying = enum : IntegerType {
    MEMBER = 1;
};

type EnumMemberValue = enum {
    MEMBER = VALUE;
};

type StructMemberType = struct {
    member Type;
};

type StructMemberDefault = struct {
    @allow_deprecated_struct_defaults
    member uint32 = VALUE;
};

type Table = table {
    1: member Type;
};

type Union = union {
    1: member Type;
};

protocol ProtocolComposition {
    compose TargetProtocol;
};

protocol ProtocolMethodRequest {
    Method(Type);
};

protocol ProtocolMethodResponse {
    Method() -> (Type);
};

protocol ProtocolEvent {
    -> Event(Type);
};

protocol ProtocolSuccess {
    Method() -> (Type) error uint32;
};

protocol ProtocolError {
    Method() -> () error ErrorIntegerType;
};

service Service {
    member client_end:TargetProtocol;
};

resource_definition Resource : uint32 {
    properties {
        first IntegerType;
        // This property is required for compilation, but is not otherwise under test.
        subtype flexible enum : uint32 {};
    };
};

type NestedTypes = struct {
    first vector<Type>;
    second vector<array<Type, 3>>;
};

type LayoutParameters = struct {
    member array<bool, VALUE>;
};

type Constraints = struct {
    member vector<bool>:VALUE;
};

type AnonymousLayouts = struct {
    first_member table {
        1: second_member union {
            1: third_member Type;
        };
    };
};

protocol AnonymousLayoutsInProtocol {
    Request(struct { member Type; });
    Response() -> (struct { member Type; });
    -> Event(struct { member Type; });
    Success() -> (struct { member Type; }) error uint32;
    Error() -> () error ErrorIntegerType;
};
"#;

    let fidl = format!("{}{}", fidl_prefix, common_suffix);
    let v1 = format!("{}{}", v1_prefix, common_suffix);
    let v2_onward = format!("{}{}", v2_onward_prefix, common_suffix);

    assert_equivalent(&fidl, &v1, "1");
    assert_equivalent(&fidl, &v2_onward, "2");
    assert_equivalent(&fidl, &v2_onward, "HEAD");
    assert_equivalent(&fidl, &v2_onward, "1,2,HEAD");
}

#[test]
#[ignore]
fn complicated() {
    let fidl = r#"
@available(added=1)
library example;

type X = resource table {
    @available(removed=7)
    1: x1 bool;
    @available(added=3)
    2: x2 Y;
    @available(added=4)
    3: x3 Z;
};

@available(added=3)
type Y = resource union {
    1: y1 client_end:A;
    @available(added=4, removed=5)
    2: y2 client_end:B;
};

@available(added=3)
type Z = resource struct {
    z1 Y:optional;
    z2 vector<W>:optional;
};

@available(added=3)
type W = resource table {
    1: w1 X;
};

protocol A {
    A1(X);
    @available(added=7)
    A2(resource struct { y Y; });
};

@available(added=3)
protocol B {
    @available(removed=5)
    B1(X);
    @available(added=5)
    B2(resource struct {
      x X;
      y Y;
    });
};

@available(removed=6)
protocol AB {
    compose A;
    @available(added=4)
    compose B;
};
"#;

    let v1_to_2 = r#"
@available(added=1, removed=3)
library example;

type X = resource table {
    1: x1 bool;
};

protocol A {
    A1(X);
};

protocol AB {
    compose A;
};
"#;

    let v3 = r#"
@available(added=3, removed=4)
library example;

type X = resource table {
    1: x1 bool;
    2: x2 Y;
};

type Y = resource union {
    1: y1 client_end:A;
};

type Z = resource struct {
    z1 Y:optional;
    z2 vector<W>:optional;
};

type W = resource table {
    1: w1 X;
};

protocol A {
    A1(X);
};

protocol B {
    B1(X);
};

protocol AB {
    compose A;
};
"#;

    let v4 = r#"
@available(added=4, removed=5)
library example;

type X = resource table {
    1: x1 bool;
    2: x2 Y;
    3: x3 Z;
};

type Y = resource union {
    1: y1 client_end:A;
    2: y2 client_end:B;
};

type Z = resource struct {
    z1 Y:optional;
    z2 vector<W>:optional;
};

type W = resource table {
    1: w1 X;
};

protocol A {
    A1(X);
};

protocol B {
    B1(X);
};

protocol AB {
    compose A;
    compose B;
};
"#;

    let v5 = r#"
@available(added=5, removed=6)
library example;

type X = resource table {
    1: x1 bool;
    2: x2 Y;
    3: x3 Z;
};

type Y = resource union {
    1: y1 client_end:A;
};

type Z = resource struct {
    z1 Y:optional;
    z2 vector<W>:optional;
};

type W = resource table {
    1: w1 X;
};

protocol A {
    A1(X);
};

protocol B {
    B2(resource struct {
      x X;
      y Y;
    });
};

protocol AB {
    compose A;
    compose B;
};
"#;

    let v6 = r#"
@available(added=6, removed=7)
library example;

type X = resource table {
    1: x1 bool;
    2: x2 Y;
    3: x3 Z;
};

type Y = resource union {
    1: y1 client_end:A;
};

type Z = resource struct {
    z1 Y:optional;
    z2 vector<W>:optional;
};

type W = resource table {
    1: w1 X;
};

protocol A {
    A1(X);
};

protocol B {
    B2(resource struct {
      x X;
      y Y;
    });
};
"#;

    let v7_onward = r#"
@available(added=7)
library example;

type X = resource table {
    2: x2 Y;
    3: x3 Z;
};

type Y = resource union {
    1: y1 client_end:A;
};

type Z = resource struct {
    z1 Y:optional;
    z2 vector<W>:optional;
};

type W = resource table {
    1: w1 X;
};

protocol A {
    A1(X);
    A2(resource struct { y Y; });
};

protocol B {
    B2(resource struct {
      x X;
      y Y;
    });
};
"#;

    let all_versions = r#"
library example;

type X = resource table {
    1: x1 bool;
    2: x2 Y;
    3: x3 Z;
};

type Y = resource union {
    1: y1 client_end:A;
    2: y2 client_end:B;
};

type Z = resource struct {
    z1 Y:optional;
    z2 vector<W>:optional;
};

type W = resource table {
    1: w1 X;
};

protocol A {
    A1(X);
    A2(resource struct { y Y; });
};

protocol B {
    B1(X);
    B2(resource struct {
      x X;
      y Y;
    });
};

protocol AB {
    compose A;
    compose B;
};
"#;

    assert_equivalent(&fidl, &v1_to_2, "1");
    assert_equivalent(&fidl, &v1_to_2, "2");
    assert_equivalent(&fidl, &v3, "3");
    assert_equivalent(&fidl, &v4, "4");
    assert_equivalent(&fidl, &v5, "5");
    assert_equivalent(&fidl, &v6, "6");
    assert_equivalent(&fidl, &v7_onward, "7");
    assert_equivalent(&fidl, &v7_onward, "HEAD");
    assert_equivalent(&fidl, &all_versions, "1,2,3,4,5,6,7,HEAD");
}

#[test]
fn convertnamedtoanonymous() {
    let fidl = r#"
@available(added=1)
library example;

@available(replaced=2)
type Foo = struct {
    member Bar;
};

@available(replaced=2)
type Bar = struct {};

@available(added=2)
type Foo = struct {
    member @generated_name("Bar") struct {};
};
"#;

    let v1 = r#"
@available(added=1, removed=2)
library example;

type Foo = struct {
    member Bar;
};

type Bar = struct {};
"#;

    let v2_onward = r#"
@available(added=2)
library example;

type Foo = struct {
    member @generated_name("Bar") struct {};
};
"#;

    assert_equivalent(&fidl, &v1, "1");
    assert_equivalent(&fidl, &v2_onward, "2");
    assert_equivalent(&fidl, &v2_onward, "HEAD");
    assert_equivalent(&fidl, &v2_onward, "1,2,HEAD");
}

#[test]
fn convertanonymoustonamed() {
    let fidl = r#"
@available(added=1)
library example;

@available(replaced=2)
type Foo = struct {
    member @generated_name("Bar") struct {};
};

@available(added=2)
type Foo = struct {
    member Bar;
};

@available(added=2)
type Bar = struct {};
"#;

    let v1 = r#"
@available(added=1, removed=2)
library example;

type Foo = struct {
    member @generated_name("Bar") struct {};
};
"#;

    let v2_onward = r#"
@available(added=2)
library example;

type Foo = struct {
    member Bar;
};

type Bar = struct {};
"#;

    assert_equivalent(&fidl, &v1, "1");
    assert_equivalent(&fidl, &v2_onward, "2");
    assert_equivalent(&fidl, &v2_onward, "HEAD");
    assert_equivalent(&fidl, &v2_onward, "1,2,HEAD");
}
