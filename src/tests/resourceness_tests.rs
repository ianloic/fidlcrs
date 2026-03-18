#![allow(unused_mut, unused_variables)]

use crate::diagnostics::Error;
use crate::tests::test_library::TestLibrary;

#[test]
fn bad_bits_resourceness() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;
type Foo = resource bits {
    BAR = 1;
};
"#,
    );
    lib.expect_fail(
        Error::ErrCannotSpecifyModifier,
        &["\"resource\"", "\"bits\""],
    );
    assert!(lib.check_compile());
}

#[test]
fn bad_enum_resourceness() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;
type Foo = resource enum {
    BAR = 1;
};
"#,
    );
    lib.expect_fail(
        Error::ErrCannotSpecifyModifier,
        &["\"resource\"", "\"enum\""],
    );
    assert!(lib.check_compile());
}

#[test]
fn bad_const_resourceness() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

resource const BAR uint32 = 1;
"#,
    );
    lib.expect_fail(
        Error::ErrCannotSpecifyModifier,
        &["\"resource\"", "\"const\""],
    );
    assert!(lib.check_compile());
}

#[test]
fn bad_protocol_resourceness() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

resource protocol Foo {};
"#,
    );
    lib.expect_fail(
        Error::ErrCannotSpecifyModifier,
        &["\"resource\"", "\"protocol\""],
    );
    assert!(lib.check_compile());
}

#[test]
fn bad_alias_resourceness() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

resource alias B = bool;
"#,
    );
    lib.expect_fail(
        Error::ErrCannotSpecifyModifier,
        &["\"resource\"", "\"alias\""],
    );
    assert!(lib.check_compile());
}

#[test]
fn bad_duplicate_modifier() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type One = resource struct {};
type Two = resource resource struct {};
type Three = resource resource resource struct {};
"#,
    );
    lib.expect_fail(Error::ErrDuplicateModifier, &["\"resource\""]);
    lib.expect_fail(Error::ErrDuplicateModifier, &["\"resource\""]);
    lib.expect_fail(Error::ErrDuplicateModifier, &["\"resource\""]);
    assert!(lib.check_compile());
}

#[test]
fn good_resource_simple() {
    let content = std::fs::read_to_string("fidlc/tests/fidl/good/fi-0110-a.test.fidl").unwrap();

    let mut lib = TestLibrary::new();
    lib.use_library_zx();
    lib.add_source_file("good/fi-0110-a.test.fidl", &(content));
    lib.compile().unwrap();
}
#[test]
fn bad_resource_modifier_missing() {
    let content = std::fs::read_to_string("fidlc/tests/fidl/bad/fi-0110.test.fidl").unwrap();

    let mut lib = TestLibrary::new();
    lib.use_library_zx();
    lib.add_source_file("bad/fi-0110.test.fidl", &(content));
    lib.expect_fail(
        Error::ErrTypeMustBeResource,
        &[
            "\"struct\"",
            "\"Foo\"",
            "\"handle\"",
            "\"struct\"",
            "\"struct\"",
            "\"Foo\"",
        ],
    );
    assert!(lib.check_compile());
}
#[test]
fn good_resource_struct() {
    let definitions = vec![
        "type Foo =  resource struct {};",
        "type Foo = resource struct { b bool; };",
        "using zx;\ntype Foo = resource struct{ h zx.Handle; };",
        "using zx;\ntype Foo = resource struct{ a array<zx.Handle, 1>; };",
        "using zx;\ntype Foo = resource struct{ v vector<zx.Handle>; };",
    ];
    for definition in definitions {
        let fidl_library = format!("library example;\n{}", definition);

        let mut lib = TestLibrary::new();
        lib.use_library_zx();
        lib.add_source_file("example.fidl", &(fidl_library));
        lib.compile().unwrap();
    }
}
#[test]
fn good_resource_table() {
    let definitions = vec![
        "type Foo = resource table {};",
        "type Foo = resource table { 1: b bool; };",
        "using zx;\ntype Foo = resource table { 1: h zx.Handle; };",
        "using zx;\ntype Foo = resource table { 1: a array<zx.Handle, 1>; };",
        "using zx;\ntype Foo = resource table { 1: v vector<zx.Handle>; };",
    ];
    for definition in definitions {
        let fidl_library = format!("library example;\n{}", definition);

        let mut lib = TestLibrary::new();
        lib.use_library_zx();
        lib.add_source_file("example.fidl", &(fidl_library));
        lib.compile().unwrap();
    }
}
#[test]
fn good_resource_union() {
    let definitions = vec![
        "type Foo = resource union { 1: b bool; };",
        "using zx;\ntype Foo = resource union { 1: h zx.Handle; };",
        "using zx;\ntype Foo = resource union { 1: a array<zx.Handle, 1>; };",
        "using zx;\ntype Foo = resource union { 1: v vector<zx.Handle>; };",
    ];
    for definition in definitions {
        let fidl_library = format!("library example;\n{}", definition);

        let mut lib = TestLibrary::new();
        lib.use_library_zx();
        lib.add_source_file("example.fidl", &(fidl_library));
        lib.compile().unwrap();
    }
}
#[test]
fn bad_handles_in_value_struct() {
    let definitions = vec![
        "type Foo = struct { bad_member zx.Handle; };",
        "type Foo = struct { bad_member zx.Handle:optional; };",
        "type Foo = struct { bad_member array<zx.Handle, 1>; };",
        "type Foo = struct { bad_member vector<zx.Handle>; };",
        "type Foo = struct { bad_member vector<zx.Handle>:0; };",
    ];
    for definition in definitions {
        let fidl_library = format!("library example;\nusing zx;\n\n{}\n", definition);

        let mut lib = TestLibrary::new();
        lib.use_library_zx();
        lib.add_source_file("example.fidl", &(fidl_library));
        lib.expect_fail(
            Error::ErrTypeMustBeResource,
            &[
                "\"struct\"",
                "\"Foo\"",
                "\"bad_member\"",
                "\"struct\"",
                "\"struct\"",
                "\"Foo\"",
            ],
        );
        assert!(lib.check_compile());
    }
}
#[test]
fn bad_handles_in_value_table() {
    let definitions = vec![
        "type Foo = table { 1: bad_member zx.Handle; };",
        "type Foo = table { 1: bad_member array<zx.Handle, 1>; };",
        "type Foo = table { 1: bad_member vector<zx.Handle>; };",
        "type Foo = table { 1: bad_member vector<zx.Handle>:0; };",
    ];
    for definition in definitions {
        let fidl_library = format!("library example;\nusing zx;\n\n{}\n", definition);

        let mut lib = TestLibrary::new();
        lib.use_library_zx();
        lib.add_source_file("example.fidl", &(fidl_library));
        lib.expect_fail(
            Error::ErrTypeMustBeResource,
            &[
                "\"table\"",
                "\"Foo\"",
                "\"bad_member\"",
                "\"table\"",
                "\"table\"",
                "\"Foo\"",
            ],
        );
        assert!(lib.check_compile());
    }
}
#[test]
fn bad_handles_in_value_union() {
    let definitions = vec![
        "type Foo = union { 1: bad_member zx.Handle; };",
        "type Foo = union { 1: bad_member array<zx.Handle, 1>; };",
        "type Foo = union { 1: bad_member vector<zx.Handle>; };",
        "type Foo = union { 1: bad_member vector<zx.Handle>:0; };",
    ];
    for definition in definitions {
        let fidl_library = format!("library example;\nusing zx;\n\n{}\n", definition);

        let mut lib = TestLibrary::new();
        lib.use_library_zx();
        lib.add_source_file("example.fidl", &(fidl_library));
        lib.expect_fail(
            Error::ErrTypeMustBeResource,
            &[
                "\"union\"",
                "\"Foo\"",
                "\"bad_member\"",
                "\"union\"",
                "\"union\"",
                "\"Foo\"",
            ],
        );
        assert!(lib.check_compile());
    }
}
#[test]
fn bad_protocols_in_value_type() {
    let mut lib = TestLibrary::new();
    lib.use_library_zx();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;
using zx;

protocol Protocol {};

type Foo = struct { bad_member client_end:Protocol; };

"#,
    );
    lib.expect_fail(
        Error::ErrTypeMustBeResource,
        &[
            "\"struct\"",
            "\"Foo\"",
            "\"bad_member\"",
            "\"struct\"",
            "\"struct\"",
            "\"Foo\"",
        ],
    );
    assert!(lib.check_compile());
}

#[test]
fn bad_resource_types_in_value_type() {
    let mut lib = TestLibrary::new();
    lib.use_library_zx();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type ResourceStruct = resource struct {};
type ResourceTable = resource table {};
type ResourceUnion = resource union { 1: b bool; };

type Foo = struct { bad_member ResourceStruct; };

"#,
    );
    lib.expect_fail(
        Error::ErrTypeMustBeResource,
        &[
            "\"struct\"",
            "\"Foo\"",
            "\"bad_member\"",
            "\"struct\"",
            "\"struct\"",
            "\"Foo\"",
        ],
    );
    assert!(lib.check_compile());
}

#[test]
fn bad_resource_aliases_in_value_type() {
    let mut lib = TestLibrary::new();
    lib.use_library_zx();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;
using zx;

alias HandleAlias = zx.Handle;
alias ProtocolAlias = client_end:Protocol;
alias ResourceStructAlias = ResourceStruct;
alias ResourceTableAlias = ResourceStruct;
alias ResourceUnionAlias = ResourceStruct;

protocol Protocol {};
type ResourceStruct = resource struct {};
type ResourceTable = resource table {};
type ResourceUnion = resource union { 1: b bool; };

type Foo = struct { bad_member HandleAlias; };

"#,
    );
    lib.expect_fail(
        Error::ErrTypeMustBeResource,
        &[
            "\"struct\"",
            "\"Foo\"",
            "\"bad_member\"",
            "\"struct\"",
            "\"struct\"",
            "\"Foo\"",
        ],
    );
    assert!(lib.check_compile());
}

#[test]
fn bad_resources_in_nested_containers() {
    let mut lib = TestLibrary::new();
    lib.use_library_zx();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;
using zx;

protocol Protocol {};
type ResourceStruct = resource struct {};
type ResourceTable = resource table {};
type ResourceUnion = resource union { 1: b bool; };

type Foo = struct { bad_member vector<vector<zx.Handle>>; };

"#,
    );
    lib.expect_fail(
        Error::ErrTypeMustBeResource,
        &[
            "\"struct\"",
            "\"Foo\"",
            "\"bad_member\"",
            "\"struct\"",
            "\"struct\"",
            "\"Foo\"",
        ],
    );
    assert!(lib.check_compile());
}

#[test]
fn bad_multiple_resource_types_in_value_type() {
    let mut lib = TestLibrary::new();
    lib.use_library_zx();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;
using zx;

type Foo = struct {
  first zx.Handle;
  second zx.Handle:optional;
  third ResourceStruct;
};

type ResourceStruct = resource struct {};
"#,
    );
    lib.expect_fail(
        Error::ErrTypeMustBeResource,
        &[
            "\"struct\"",
            "\"Foo\"",
            "\"first\"",
            "\"struct\"",
            "\"struct\"",
            "\"Foo\"",
        ],
    );
    lib.expect_fail(
        Error::ErrTypeMustBeResource,
        &[
            "\"struct\"",
            "\"Foo\"",
            "\"second\"",
            "\"struct\"",
            "\"struct\"",
            "\"Foo\"",
        ],
    );
    lib.expect_fail(
        Error::ErrTypeMustBeResource,
        &[
            "\"struct\"",
            "\"Foo\"",
            "\"third\"",
            "\"struct\"",
            "\"struct\"",
            "\"Foo\"",
        ],
    );
    assert!(lib.check_compile());
}

#[test]
fn good_transitive_resource_member() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type Top = resource struct {
    middle Middle;
};
type Middle = resource struct {
    bottom Bottom;
};
type Bottom = resource struct {};
"#,
    );
    lib.compile().unwrap();
}

#[test]
fn bad_transitive_resource_member() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type Top = struct {
  middle Middle;
};
type Middle = struct {
  bottom Bottom;
};
type Bottom = resource struct {};
"#,
    );
    lib.expect_fail(
        Error::ErrTypeMustBeResource,
        &[
            "\"struct\"",
            "\"Middle\"",
            "\"bottom\"",
            "\"struct\"",
            "\"struct\"",
            "\"Middle\"",
        ],
    );
    assert!(lib.check_compile());
}

#[test]
fn good_recursive_value_types() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type Ouro = struct {
    b box<Boros>;
};

type Boros = struct {
    o box<Ouro>;
};
"#,
    );
    lib.compile().unwrap();
}

#[test]
fn good_recursive_resource_types() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type Ouro = resource struct {
    b box<Boros>;
};

type Boros = resource struct {
    o box<Ouro>;
};
"#,
    );
    lib.compile().unwrap();
}

#[test]
fn bad_recursive_resource_types() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type Ouro = resource struct {
  b box<Boros>;
};

type Boros = struct {
  bad_member box<Ouro>;
};
"#,
    );
    lib.expect_fail(
        Error::ErrTypeMustBeResource,
        &[
            "\"struct\"",
            "\"Boros\"",
            "\"bad_member\"",
            "\"struct\"",
            "\"struct\"",
            "\"Boros\"",
        ],
    );
    assert!(lib.check_compile());
}

#[test]
fn good_strict_resource_order_independent() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type SR = strict resource union {
    1: b bool;
};
type RS = strict resource union {
    1: b bool;
};
"#,
    );
    lib.compile().unwrap();
}
