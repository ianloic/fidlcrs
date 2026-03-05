#![allow(unused_mut, unused_variables)]

    use crate::source_file::SourceFile;
    use crate::tests::test_library::TestLibrary;

    #[test]
    fn good_root_types_unqualified() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

const b bool = false;
const i8 int8 = 0;
const i16 int16 = 0;
const i32 int32 = 0;
const i64 int64 = 0;
const u8 uint8 = 0;
const u16 uint16 = 0;
const u32 uint32 = 0;
const u64 uint64 = 0;
const us usize64 = 0;
const up uintptr64 = 0;
const uc uchar = 0;
const f32 float32 = 0;
const f64 float64 = 0;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.enable_flag("zx_c_types");
        lib.add_source(&source);
        lib.compile().unwrap();
    }

    #[test]
    fn good_root_types_qualified() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

const bool fidl.bool = false;
const int8 fidl.int8 = 0;
const int16 fidl.int16 = 0;
const int32 fidl.int32 = 0;
const int64 fidl.int64 = 0;
const uint8 fidl.uint8 = 0;
const uint16 fidl.uint16 = 0;
const uint32 fidl.uint32 = 0;
const uint64 fidl.uint64 = 0;
const usize64 fidl.usize64 = 0;
const uintptr64 fidl.uintptr64 = 0;
const uchar fidl.uchar = 0;
const float32 fidl.float32 = 0;
const float64 fidl.float64 = 0;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.enable_flag("zx_c_types");
        lib.add_source(&source);
        lib.compile().unwrap();
    }

    #[test]
    fn good_handle_subtype() {
        // static_asserts or pure C++ testing not ported directly
    }

    #[test]
    fn good_rights() {
        // static_asserts or pure C++ testing not ported directly
    }

    #[test]
    fn good_type_decl_of_anonymous_layouts() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;
type TypeDecl = struct {
    f0 bits {
      FOO = 1;
    };
    f1 enum {
      BAR = 1;
    };
    f2 struct {
      i0 vector<uint8>;
      @allow_deprecated_struct_defaults
      i1 string = "foo";
    };
    f3 table {
      1: i0 bool;
    };
    f4 union {
      1: i0 bool;
    };
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().unwrap();
    }

    #[test]
    fn bad_type_decl_of_new_type_errors() {
        let content = std::fs::read_to_string("fidlc/tests/fidl/bad/fi-0062.test.fidl").unwrap();
        let source = SourceFile::new("bad/fi-0062.test.fidl".to_string(), content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
        let errors = lib.reporter().diagnostics();
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].def,
            crate::diagnostics::Error::ErrNewTypesNotAllowed
        );
    }

    #[test]
    fn good_type_parameters() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;
type Inner = struct{};
alias Alias = Inner;

type TypeDecl = struct {
  // vector of primitive
  v0 vector<uint8>;
  // vector of sourced
  v1 vector<Inner>;
  // vector of alias
  v2 vector<Alias>;
  // vector of anonymous layout
  v3 vector<struct{
       i0 struct{};
       i1 vector<struct{}>;
     }>;
  // array of primitive
  a0 array<uint8,5>;
  // array of sourced
  a1 array<Inner,5>;
  // array of alias
  a2 array<Alias,5>;
  // array of anonymous layout
  a3 array<struct{
       i2 struct{};
       i3 array<struct{},5>;
     },5>;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().unwrap();
    }

    #[test]
    fn good_layout_member_constraints() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

alias Alias = vector<uint8>;
type t1 = resource struct {
  u0 union { 1: b bool; };
  u1 union { 1: b bool; }:optional;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().unwrap();
    }

    #[test]
    fn good_constraints_on_vectors() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

alias Alias = vector<uint8>;
type TypeDecl= struct {
  v0 vector<bool>;
  v1 vector<bool>:16;
  v2 vector<bool>:optional;
  v3 vector<bool>:<16,optional>;
  b4 vector<uint8>;
  b5 vector<uint8>:16;
  b6 vector<uint8>:optional;
  b7 vector<uint8>:<16,optional>;
  s8 string;
  s9 string:16;
  s10 string:optional;
  s11 string:<16,optional>;
  a12 Alias;
  a13 Alias:16;
  a14 Alias:optional;
  a15 Alias:<16,optional>;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().unwrap();
    }

    #[test]
    fn good_constraints_on_unions() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

type UnionDecl = union{1: foo bool;};
alias UnionAlias = UnionDecl;
type TypeDecl= struct {
  u0 union{1: bar bool;};
  u1 union{1: baz bool;}:optional;
  u2 UnionDecl;
  u3 UnionDecl:optional;
  u4 UnionAlias;
  u5 UnionAlias:optional;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().unwrap();
    }

    #[test]
    fn good_constraints_on_handles() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;
using zx;

type TypeDecl = resource struct {
  h0 zx.Handle;
  h1 zx.Handle:VMO;
  h2 zx.Handle:optional;
  h3 zx.Handle:<VMO,optional>;
  h4 zx.Handle:<VMO,zx.Rights.TRANSFER>;
  h5 zx.Handle:<VMO,zx.Rights.TRANSFER,optional>;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.use_library_zx();
        lib.add_source(&source);
        lib.compile().unwrap();
    }

    #[test]
    fn bad_too_many_layout_parameters() {
        let content = std::fs::read_to_string("fidlc/tests/fidl/bad/fi-0162-b.test.fidl").unwrap();
        let source = SourceFile::new("bad/fi-0162-b.test.fidl".to_string(), content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
        let errors = lib.reporter().diagnostics();
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].def,
            crate::diagnostics::Error::ErrWrongNumberOfLayoutParameters
        );
    }

    #[test]
    fn bad_zero_parameters() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

type Foo = struct {
  foo array;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
        let errors = lib.reporter().diagnostics();
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].def,
            crate::diagnostics::Error::ErrWrongNumberOfLayoutParameters
        );
    }

    #[test]
    fn bad_not_enough_parameters() {
        let content = std::fs::read_to_string("fidlc/tests/fidl/bad/fi-0162-a.test.fidl").unwrap();
        let source = SourceFile::new("bad/fi-0162-a.test.fidl".to_string(), content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
        let errors = lib.reporter().diagnostics();
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].def,
            crate::diagnostics::Error::ErrWrongNumberOfLayoutParameters
        );
    }

    #[test]
    fn bad_too_many_constraints() {
        let content = std::fs::read_to_string("fidlc/tests/fidl/bad/fi-0164.test.fidl").unwrap();
        let source = SourceFile::new("bad/fi-0164.test.fidl".to_string(), content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
        let errors = lib.reporter().diagnostics();
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].def,
            crate::diagnostics::Error::ErrTooManyConstraints
        );
    }

    #[test]
    fn bad_parameterized_anonymous_layout() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

type Foo = struct {
  bar struct {}<1>;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
        let errors = lib.reporter().diagnostics();
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].def,
            crate::diagnostics::Error::ErrWrongNumberOfLayoutParameters
        );
    }

    #[test]
    fn bad_constrain_twice() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

using zx;

alias MyVmo = zx.Handle:VMO;

type Foo = struct {
    foo MyVmo:zx.ObjType.CHANNEL;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.use_library_zx();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
        let errors = lib.reporter().diagnostics();
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].def,
            crate::diagnostics::Error::ErrCannotConstrainTwice
        );
    }

    #[test]
    fn good_no_overlapping_constraints() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

using zx;

alias MyVmo = zx.Handle:<VMO, zx.Rights.TRANSFER>;

type Foo = resource struct {
    foo MyVmo:optional;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.use_library_zx();
        lib.add_source(&source);
        lib.compile().unwrap();
    }

    #[test]
    fn bad_want_type_layout_parameter() {
        let content = std::fs::read_to_string("fidlc/tests/fidl/bad/fi-0165.test.fidl").unwrap();
        let source = SourceFile::new("bad/fi-0165.test.fidl".to_string(), content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
        let errors = lib.reporter().diagnostics();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].def, crate::diagnostics::Error::ErrExpectedType);
    }

    #[test]
    fn bad_want_value_layout_parameter() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

type Foo = struct {
    foo array<uint8, uint8>;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
        let errors = lib.reporter().diagnostics();
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].def,
            crate::diagnostics::Error::ErrExpectedValueButGotType
        );
    }

    #[test]
    fn bad_unresolvable_constraint() {
        let content = std::fs::read_to_string("fidlc/tests/fidl/bad/fi-0166.test.fidl").unwrap();
        let source = SourceFile::new("bad/fi-0166.test.fidl".to_string(), content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
        let errors = lib.reporter().diagnostics();
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].def,
            crate::diagnostics::Error::ErrUnexpectedConstraint
        );
    }

    #[test]
    fn bad_shadowed_optional() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

const optional uint8 = 3;

type Foo = resource struct {
    foo vector<uint8>:<10, optional>;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
        let errors = lib.reporter().diagnostics();
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].def,
            crate::diagnostics::Error::ErrUnexpectedConstraint
        );
    }

    #[test]
    fn bad_wrong_constraint_type() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

type Foo = resource struct {
    foo vector<uint8>:"hello";
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
        let errors = lib.reporter().diagnostics();
        assert_eq!(errors.len(), 2);
        assert_eq!(
            errors[0].def,
            crate::diagnostics::Error::ErrTypeCannotBeConvertedToType
        );
        assert_eq!(
            errors[1].def,
            crate::diagnostics::Error::ErrCouldNotResolveSizeBound
        );
    }

    #[test]
    fn cannot_refer_to_unqualified_internal_type() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

type Foo = struct {
    foo FrameworkErr;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
        let errors = lib.reporter().diagnostics();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].def, crate::diagnostics::Error::ErrNameNotFound);
    }

    #[test]
    fn cannot_refer_to_qualified_internal_type() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

type Foo = struct {
    foo fidl.FrameworkErr;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
        let errors = lib.reporter().diagnostics();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].def, crate::diagnostics::Error::ErrNameNotFound);
    }

    #[test]
    fn bad_usize64_without_flag() {
        let content = std::fs::read_to_string("fidlc/tests/fidl/bad/fi-0180.test.fidl").unwrap();
        let source = SourceFile::new("bad/fi-0180.test.fidl".to_string(), content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
        let errors = lib.reporter().diagnostics();
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].def,
            crate::diagnostics::Error::ErrExperimentalZxCTypesDisallowed
        );
    }

    #[test]
    fn bad_uintptr64_without_flag() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example; alias T = uintptr64;"#.to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
        let errors = lib.reporter().diagnostics();
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].def,
            crate::diagnostics::Error::ErrExperimentalZxCTypesDisallowed
        );
    }

    #[test]
    fn bad_uchar_without_flag() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example; alias T = uchar;"#.to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
        let errors = lib.reporter().diagnostics();
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].def,
            crate::diagnostics::Error::ErrExperimentalZxCTypesDisallowed
        );
    }

    #[test]
    fn bad_experimental_pointer_without_flag() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example; alias T = experimental_pointer<uint32>;"#.to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
        let errors = lib.reporter().diagnostics();
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].def,
            crate::diagnostics::Error::ErrExperimentalZxCTypesDisallowed
        );
    }

    #[test]
    fn good_usize64_with_flag() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example; alias T = usize64;"#.to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.enable_flag("zx_c_types");
        lib.add_source(&source);
        lib.compile().unwrap();
    }

    #[test]
    fn good_uintptr64_with_flag() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example; alias T = uintptr64;"#.to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.enable_flag("zx_c_types");
        lib.add_source(&source);
        lib.compile().unwrap();
    }

    #[test]
    fn good_uchar_with_flag() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example; alias T = uchar;"#.to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.enable_flag("zx_c_types");
        lib.add_source(&source);
        lib.compile().unwrap();
    }

    #[test]
    fn good_experimental_pointer_with_flag() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example; alias T = experimental_pointer<uint32>;"#.to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.enable_flag("zx_c_types");
        lib.add_source(&source);
        lib.compile().unwrap();
    }

