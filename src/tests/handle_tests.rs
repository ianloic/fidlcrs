use crate::flat_ast::Type;
use crate::flat_ast::TypeKind;
use crate::source_file::SourceFile;
use crate::tests::test_library::{LookupHelpers, TestLibrary};

#[test]
fn good_handle_rights_test() {
    let mut lib = TestLibrary::new();
    lib.use_library_zx();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

using zx;

type MyStruct = resource struct {
    h zx.Handle:<THREAD, zx.Rights.DUPLICATE | zx.Rights.TRANSFER>;
};
"#
        ,
    );
    let root = lib.compile().expect("compilation failed");

    let type_decl = root
        .lookup_struct("example/MyStruct")
        .expect("struct not found");
    let h_type = &type_decl.members[0].type_;

    assert_eq!(h_type.kind(), TypeKind::Handle);
    assert_eq!(
        match h_type {
            Type::Handle(t) => t.subtype.as_deref(),
            _ => None,
        },
        Some("thread")
    );
    assert_eq!(h_type.rights(), Some(3));
}

#[test]
fn good_no_handle_rights_test() {
    let mut lib = TestLibrary::new();
    lib.use_library_zx();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

using zx;

type MyStruct = resource struct {
    h zx.Handle:VMO;
};
"#
        ,
    );
    let root = lib.compile().expect("compilation failed");

    let type_decl = root
        .lookup_struct("example/MyStruct")
        .expect("struct not found");
    let h_type = &type_decl.members[0].type_;

    assert_eq!(h_type.kind(), TypeKind::Handle);
    assert_eq!(
        match h_type {
            Type::Handle(t) => t.subtype.as_deref(),
            _ => None,
        },
        Some("vmo")
    );
    assert_eq!(h_type.rights(), Some(2147483648));
}

#[test]
fn bad_invalid_handle_rights_test() {
    let mut lib = TestLibrary::new();
    lib.use_library_zx();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

using zx;

protocol P {
    Method(struct { h zx.Handle:<VMO, 1>; });  // rights must be zx.Rights-typed.
};
"#
        ,
    );
    assert!(lib.compile().is_err(), "expected compilation to fail");
}

#[test]
fn good_plain_handle_test() {
    let mut lib = TestLibrary::new();
    lib.use_library_zx();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

using zx;

type MyStruct = resource struct {
    h zx.Handle;
};
"#
        ,
    );
    let root = lib.compile().expect("compilation failed");

    let type_decl = root
        .lookup_struct("example/MyStruct")
        .expect("struct not found");
    let h_type = &type_decl.members[0].type_;

    assert_eq!(h_type.kind(), TypeKind::Handle);
    assert_eq!(
        match h_type {
            Type::Handle(t) => t.subtype.as_deref(),
            _ => None,
        },
        Some("handle")
    );
    assert_eq!(h_type.rights(), Some(2147483648)); // kHandleSameRights
}

#[test]
fn good_handle_fidl_defined_test() {
    let mut lib = TestLibrary::new();
    lib.use_library_zx();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

using zx;

type MyStruct = resource struct {
  a zx.Handle:THREAD;
  b zx.Handle:<PROCESS>;
  c zx.Handle:<VMO, zx.Rights.TRANSFER>;
};
"#
        ,
    );
    let root = lib.compile().expect("compilation failed");

    let type_decl = root
        .lookup_struct("example/MyStruct")
        .expect("struct not found");

    let a_type = &type_decl.members[0].type_;
    assert_eq!(a_type.kind(), TypeKind::Handle);
    assert_eq!(
        match a_type {
            Type::Handle(t) => t.subtype.as_deref(),
            _ => None,
        },
        Some("thread")
    );
    assert_eq!(a_type.rights(), Some(2147483648));

    let b_type = &type_decl.members[1].type_;
    assert_eq!(b_type.kind(), TypeKind::Handle);
    assert_eq!(
        match b_type {
            Type::Handle(t) => t.subtype.as_deref(),
            _ => None,
        },
        Some("process")
    );
    assert_eq!(b_type.rights(), Some(2147483648));

    let c_type = &type_decl.members[2].type_;
    assert_eq!(c_type.kind(), TypeKind::Handle);
    assert_eq!(
        match c_type {
            Type::Handle(t) => t.subtype.as_deref(),
            _ => None,
        },
        Some("vmo")
    );
    assert_eq!(c_type.rights(), Some(2));
}

#[test]
fn bad_invalid_fidl_defined_handle_subtype() {
    let mut lib = TestLibrary::new();
    lib.use_library_zx();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

using zx;

type MyStruct = struct {
  a zx.Handle:ZIPPY;
};
"#
        ,
    );
    assert!(lib.compile().is_err(), "expected compilation to fail");
}

#[test]
fn bad_disallow_old_handles() {
    let mut lib = TestLibrary::new();
    lib.use_library_zx();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

using zx;

type MyStruct = struct {
    h handle<vmo>;
};
"#
        ,
    );
    assert!(lib.compile().is_err(), "expected compilation to fail");
}

#[test]
fn good_resource_definition_only_subtype_no_rights_test() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type ObjType = strict enum : uint32 {
    NONE = 0;
    VMO = 3;
};

resource_definition handle : uint32 {
    properties {
        subtype ObjType;
    };
};

type MyStruct = resource struct {
    h handle:VMO;
};
"#
        ,
    );
    let root = lib.compile().expect("compilation failed");

    let type_decl = root
        .lookup_struct("example/MyStruct")
        .expect("struct not found");
    let h_type = &type_decl.members[0].type_;

    assert_eq!(h_type.kind(), TypeKind::Handle);
    assert_eq!(
        match h_type {
            Type::Handle(t) => t.subtype.as_deref(),
            _ => None,
        },
        Some("vmo")
    );
    assert_eq!(h_type.rights(), Some(2147483648));
}

#[test]
fn bad_invalid_subtype_at_use_site() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type ObjType = enum : uint32 {
    NONE = 0;
    VMO = 3;
};

resource_definition handle : uint32 {
    properties {
        subtype ObjType;
    };
};

type MyStruct = resource struct {
    h handle:<1, optional>;
};
"#
        ,
    );
    assert!(lib.compile().is_err(), "expected compilation to fail");
}

#[test]
fn bad_invalid_rights_at_use_site() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type ObjType = enum : uint32 {
    NONE = 0;
    VMO = 3;
};

resource_definition handle : uint32 {
    properties {
        subtype ObjType;
        rights uint32;
    };
};

type MyStruct = resource struct {
    h handle:<VMO, "my_improperly_typed_rights", optional>;
};
"#
        ,
    );
    assert!(lib.compile().is_err(), "expected compilation to fail");
}

#[test]
fn bad_bare_handle_no_constraints() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type MyStruct = resource struct {
    h handle;
};
"#
        ,
    );
    assert!(lib.compile().is_err(), "expected compilation to fail");
}

#[test]
fn bad_bare_handle_with_constraints() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type MyStruct = resource struct {
    h handle:VMO;
};
"#
        ,
    );
    assert!(lib.compile().is_err(), "expected compilation to fail");
}

#[test]
fn bad_bare_handle_with_constraints_through_alias() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

alias my_handle = handle;

type MyStruct = resource struct {
    h my_handle:VMO;
};
"#
        ,
    );
    assert!(lib.compile().is_err(), "expected compilation to fail");
}
