use super::test_library::TestLibrary;

#[test]
fn bad_recover_in_library_consume() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol P {};
protocol P {};              // Error: name collision

type foo = struct {};
type Foo = struct {};       // Error: canonical name collision
"#,
    );
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]

fn bad_recover_in_library_compile() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
library example;

type Union = union {
    1: string_value string;
    2: unknown_value vector;      // Error: expected 1 layout parameter
};

type Enum = enum {
    ZERO = 0;
    ONE = 1;
    TWO = 1;                      // Error: duplicate value
    THREE = 3;
};

type OtherEnum = enum {
    NONE = 0;
    ONE = 1;
    TWO = "2";                    // Error: invalid type
};

type NonDenseTable = table {
    65: s string;                 // Error: too many ordinals
};
"#,
    );
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]

fn bad_recover_in_library_verify_attribute_placement() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
library example;

@unknown            // Error: invalid placement
type Table = table {
    1: foo string;
};

type Struct = struct {
    foo uint16;
};
"#,
    );
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]

fn bad_recover_in_attribute_compile() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
library example;

@foo(first="a", first="b")   // Error: duplicate args
@bar(first=3, second=4)      // Error: x2 can only use string or bool
@foo                         // Error: duplicate attribute
type Enum = enum {
    FOO                      // Error: cannot resolve enum member
        = "not a number";    // Error: cannot be interpreted as uint32
};
"#,
    );
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]

fn bad_recover_in_const() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
library example;

@attr(1)
const FOO string = 2;
"#,
    );
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]

fn bad_recover_in_bits() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
library example;

type Foo = bits {
    BAR                    // Error: cannot resolve bits member
        = "not a number";  // Error: cannot interpret as uint32
    QUX = vector;          // Error: cannot resolve bits member
    TWO = 2;
    BAZ = 2;               // Error: duplicate value 2
    XYZ = 3;               // Error: not a power of two
};
"#,
    );
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]

fn bad_recover_in_enum() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
library example;

type Foo = flexible enum : uint8 {
    BAR                    // Error: cannot resolve enum member
        = "not a number";  // Error: cannot interpret as uint32
    QUX = vector;          // Error: cannot resolve enum member
    TWO = 2;
    BAZ = 2;               // Error: duplicate value 2
    XYZ = 255;             // Error: max value on flexible enum
};
"#,
    );
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]

fn bad_recover_in_struct() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
library example;

type Foo = struct {
    bar string<1>;     // Error: unexpected layout parameter
    qux vector;        // Error: expected 1 layout parameter
    @allow_deprecated_struct_defaults
    baz bool           // Error: cannot resolve default value
        = "not bool";  // Error: cannot interpret as bool
};
"#,
    );
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]

fn bad_recover_in_table() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
library example;

type Foo = table {
    1: bar string:optional;  // Error: table member cannot be optional
    1: qux                   // Error: duplicate ordinal
       vector;               // Error: expected 1 layout parameter
    65: s string;            // Error: too many ordinals
};
"#,
    );
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]

fn bad_recover_in_union() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
library example;

type Foo = union {
    1: bar string:optional;  // Error: union member cannot be optional
    1: qux                   // Error: duplicate ordinal
        vector;              // Error: expected 1 layout parameter
};
"#,
    );
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]

fn bad_recover_in_protocol() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol Foo {
    compose vector;              // Error: expected protocol
    @selector("not good")        // Error: invalid selector
    Bar(struct {}) -> (struct {  // Error: empty struct invalid
        b bool:optional;         // Error: bool cannot be optional
    }) error vector;             // Error: expected 1 layout parameter
};
"#,
    );
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]
fn bad_recover_in_service() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol P {};
service Foo {
    bar string;                   // Error: must be client_end
    baz vector;                   // Error: expected 1 layout parameter
    qux server_end:P;             // Error: must be client_end
    opt client_end:<P,optional>;  // Error: cannot be optional
};
"#,
    );
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}
