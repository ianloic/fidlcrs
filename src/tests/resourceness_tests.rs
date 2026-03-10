#![allow(unused_mut, unused_variables)]

use crate::diagnostics::Error;
use crate::source_file::SourceFile;
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
"#
        ,
    );
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].def, Error::ErrCannotSpecifyModifier);
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
"#
        ,
    );
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].def, Error::ErrCannotSpecifyModifier);
}

#[test]
fn bad_const_resourceness() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

resource const BAR uint32 = 1;
"#
        ,
    );
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].def, Error::ErrCannotSpecifyModifier);
}

#[test]
fn bad_protocol_resourceness() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

resource protocol Foo {};
"#
        ,
    );
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].def, Error::ErrCannotSpecifyModifier);
}

#[test]
fn bad_alias_resourceness() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

resource alias B = bool;
"#
        ,
    );
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].def, Error::ErrCannotSpecifyModifier);
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
"#
        ,
    );
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors.len(), 3);
    assert_eq!(errors[0].def, Error::ErrDuplicateModifier);
    assert_eq!(errors[1].def, Error::ErrDuplicateModifier);
    assert_eq!(errors[2].def, Error::ErrDuplicateModifier);
}

#[test]
fn good_resource_simple() {
    let content = std::fs::read_to_string("fidlc/tests/fidl/good/fi-0110-a.test.fidl").unwrap();

    let mut lib = TestLibrary::new();
    lib.use_library_zx();
    lib.add_source(SourceFile::new(
        "good/fi-0110-a.test.fidl".to_string(),
        content,
    ));
    lib.compile().unwrap();
}
#[test]
fn bad_resource_modifier_missing() {
    let content = std::fs::read_to_string("fidlc/tests/fidl/bad/fi-0110.test.fidl").unwrap();

    let mut lib = TestLibrary::new();
    lib.use_library_zx();
    lib.add_source(SourceFile::new(
        "bad/fi-0110.test.fidl".to_string(),
        content,
    ));
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].def, Error::ErrTypeMustBeResource);
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
        lib.add_source(SourceFile::new("example.fidl".to_string(), fidl_library));
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
        lib.add_source(SourceFile::new("example.fidl".to_string(), fidl_library));
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
        lib.add_source(SourceFile::new("example.fidl".to_string(), fidl_library));
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
        lib.add_source(SourceFile::new("example.fidl".to_string(), fidl_library));
        assert!(lib.compile().is_err());
        let errors = lib.reporter().diagnostics();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].def, Error::ErrTypeMustBeResource);
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
        lib.add_source(SourceFile::new("example.fidl".to_string(), fidl_library));
        assert!(lib.compile().is_err());
        let errors = lib.reporter().diagnostics();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].def, Error::ErrTypeMustBeResource);
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
        lib.add_source(SourceFile::new("example.fidl".to_string(), fidl_library));
        assert!(lib.compile().is_err());
        let errors = lib.reporter().diagnostics();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].def, Error::ErrTypeMustBeResource);
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

"#
        ,
    );
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].def, Error::ErrTypeMustBeResource);
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

"#
        ,
    );
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].def, Error::ErrTypeMustBeResource);
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

"#
        ,
    );
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].def, Error::ErrTypeMustBeResource);
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

"#
        ,
    );
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].def, Error::ErrTypeMustBeResource);
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
"#
        ,
    );
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors.len(), 3);
    assert_eq!(errors[0].def, Error::ErrTypeMustBeResource);
    assert_eq!(errors[1].def, Error::ErrTypeMustBeResource);
    assert_eq!(errors[2].def, Error::ErrTypeMustBeResource);
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
"#
        ,
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
"#
        ,
    );
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].def, Error::ErrTypeMustBeResource);
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
"#
        ,
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
"#
        ,
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
"#
        ,
    );
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].def, Error::ErrTypeMustBeResource);
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
"#
        ,
    );
    lib.compile().unwrap();
}
