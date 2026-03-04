#![allow(unused_mut, unused_variables)]
#[cfg(test)]
mod tests {
    use crate::source_file::SourceFile;
    use crate::test_library::TestLibrary;

    #[test]
    #[ignore]
    fn good_populated_fields() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library fidl.test.tables;

type Foo = table {
    1: x int64;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().unwrap();
    }

    #[test]
    #[ignore]
    fn good_out_of_order_fields() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library fidl.test.tables;

type Foo = table {
    3: x int64;
    1: y int64;
    2: z int64;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().unwrap();
    }

    #[test]
    #[ignore]
    fn good_allow_empty_tables() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library fidl.test.tables;

type Foo = table {};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().unwrap();
    }

    #[test]
    #[ignore]
    fn bad_missing_ordinals() {
        let content =
            std::fs::read_to_string("fidlc/tests/bad/fi-0016-a.noformat.test.fidl").unwrap();
        let source = SourceFile::new("bad/fi-0016-a.noformat.test.fidl".to_string(), content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
        let errors = lib.reporter().diagnostics();
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].def,
            crate::diagnostics::Error::ErrMissingOrdinalBeforeMember
        );
    }

    #[test]
    #[ignore]
    fn bad_ordinal_out_of_bounds_negative() {
        let content =
            std::fs::read_to_string("fidlc/tests/bad/fi-0017-a.noformat.test.fidl").unwrap();
        let source = SourceFile::new("bad/fi-0017-a.noformat.test.fidl".to_string(), content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
        let errors = lib.reporter().diagnostics();
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].def,
            crate::diagnostics::Error::ErrOrdinalOutOfBound
        );
    }

    #[test]
    #[ignore]
    fn bad_ordinal_out_of_bounds_large() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library test;

type Foo = union {
  4294967296: foo string;
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
            crate::diagnostics::Error::ErrOrdinalOutOfBound
        );
    }

    #[test]
    #[ignore]
    fn bad_duplicate_field_names() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library test;

type MyTable = table {
    1: my_field string;
    2: my_field uint32;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
        let errors = lib.reporter().diagnostics();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].def, crate::diagnostics::Error::ErrNameCollision);
    }

    #[test]
    #[ignore]
    fn bad_duplicate_ordinals() {
        let content = std::fs::read_to_string("fidlc/tests/bad/fi-0094.test.fidl").unwrap();
        let source = SourceFile::new("bad/fi-0094.test.fidl".to_string(), content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
        let errors = lib.reporter().diagnostics();
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].def,
            crate::diagnostics::Error::ErrDuplicateTableFieldOrdinal
        );
    }

    #[test]
    #[ignore]
    fn good_attributes_on_fields() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library fidl.test.tables;

type Foo = table {
    @foo_attr("bar")
    1: x int64;
    @bar_attr
    2: bar bool;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().unwrap();
    }

    #[test]
    #[ignore]
    fn good_attributes_on_tables() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library fidl.test.tables;

@foo_attr("bar")
type Foo = table {
    1: x int64;
    2: please bool;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().unwrap();
    }

    #[test]
    #[ignore]
    fn good_keywords_as_field_names() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library fidl.test.tables;

type struct = struct {
    field bool;
};

type Foo = table {
    1: table int64;
    2: library bool;
    3: uint32 uint32;
    4: member struct;
    5: reserved bool;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().unwrap();
    }

    #[test]
    #[ignore]
    fn bad_optional_in_struct() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library fidl.test.tables;

type Foo = table {
    1: t int64;
};

type OptionalTableContainer = struct {
    foo Foo:optional;
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
            crate::diagnostics::Error::ErrCannotBeOptional
        );
    }

    #[test]
    #[ignore]
    fn bad_table_multiple_constraints() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library fidl.test.tables;

type Foo = table {
    1: t int64;
};

type OptionalTableContainer = struct {
    foo Foo:<1, 2, 3>;
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
            crate::diagnostics::Error::ErrTooManyConstraints
        );
    }

    #[test]
    #[ignore]
    fn bad_optional_in_union() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library fidl.test.tables;

type Foo = table {
    1: t int64;
};

type OptionalTableContainer = union {
    1: foo Foo:optional;
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
            crate::diagnostics::Error::ErrCannotBeOptional
        );
    }

    #[test]
    #[ignore]
    fn good_table_in_table() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library fidl.test.tables;

type Foo = table {
    1: t int64;
};

type Bar = table {
    1: foo Foo;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().unwrap();
    }

    #[test]
    #[ignore]
    fn good_tables_in_unions() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library fidl.test.tables;

type Foo = table {
    1: t int64;
};

type OptionalTableContainer = flexible union {
    1: foo Foo;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().unwrap();
    }

    #[test]
    #[ignore]
    fn bad_optional_table_member() {
        let content = std::fs::read_to_string("fidlc/tests/bad/fi-0048.test.fidl").unwrap();
        let source = SourceFile::new("bad/fi-0048.test.fidl".to_string(), content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
        let errors = lib.reporter().diagnostics();
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].def,
            crate::diagnostics::Error::ErrOptionalTableMember
        );
    }

    #[test]
    #[ignore]
    fn bad_optional_non_optional_table_member() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library fidl.test.tables;

type Foo = table {
    // Integers can never be optional.
    1: t int64:optional;
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
            crate::diagnostics::Error::ErrCannotBeOptional
        );
    }

    #[test]
    #[ignore]
    fn bad_default_not_allowed() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library fidl.test.tables;

type Foo = table {
    1: t int64 = 1;
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
            crate::diagnostics::Error::ErrUnexpectedTokenOfKind
        );
        assert_eq!(
            errors[1].def,
            crate::diagnostics::Error::ErrMissingOrdinalBeforeMember
        );
    }

    #[test]
    #[ignore]
    fn good_ordinal_gap_start() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

type MyTable = table {
    2: two int64;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().unwrap();
    }

    #[test]
    #[ignore]
    fn good_ordinal_gap_middle() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

type MyTable = table {
    1: one int64;
    3: three int64;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().unwrap();
    }

    #[test]
    #[ignore]
    fn good64_ordinals_max_is_table() {
        let content = std::fs::read_to_string("fidlc/tests/good/fi-0093.test.fidl").unwrap();
        let source = SourceFile::new("good/fi-0093.test.fidl".to_string(), content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().unwrap();
    }

    #[test]
    #[ignore]
    fn bad_max_ordinal_not_table() {
        let content = std::fs::read_to_string("fidlc/tests/bad/fi-0093.test.fidl").unwrap();
        let source = SourceFile::new("bad/fi-0093.test.fidl".to_string(), content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
        let errors = lib.reporter().diagnostics();
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].def,
            crate::diagnostics::Error::ErrMaxOrdinalNotTable
        );
    }

    #[test]
    #[ignore]
    fn bad_max_ordinal_not_table_not_primitive() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

type MyStruct = struct {};

type Example = table {
    1: v1 int64;
    2: v2 int64;
    3: v3 int64;
    4: v4 int64;
    5: v5 int64;
    6: v6 int64;
    7: v7 int64;
    8: v8 int64;
    9: v9 int64;
    10: v10 int64;
    11: v11 int64;
    12: v12 int64;
    13: v13 int64;
    14: v14 int64;
    15: v15 int64;
    16: v16 int64;
    17: v17 int64;
    18: v18 int64;
    19: v19 int64;
    20: v20 int64;
    21: v21 int64;
    22: v22 int64;
    23: v23 int64;
    24: v24 int64;
    25: v25 int64;
    26: v26 int64;
    27: v27 int64;
    28: v28 int64;
    29: v29 int64;
    30: v30 int64;
    31: v31 int64;
    32: v32 int64;
    33: v33 int64;
    34: v34 int64;
    35: v35 int64;
    36: v36 int64;
    37: v37 int64;
    38: v38 int64;
    39: v39 int64;
    40: v40 int64;
    41: v41 int64;
    42: v42 int64;
    43: v43 int64;
    44: v44 int64;
    45: v45 int64;
    46: v46 int64;
    47: v47 int64;
    48: v48 int64;
    49: v49 int64;
    50: v50 int64;
    51: v51 int64;
    52: v52 int64;
    53: v53 int64;
    54: v54 int64;
    55: v55 int64;
    56: v56 int64;
    57: v57 int64;
    58: v58 int64;
    59: v59 int64;
    60: v60 int64;
    61: v61 int64;
    62: v62 int64;
    63: v63 int64;
    64: v64 MyStruct;
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
            crate::diagnostics::Error::ErrMaxOrdinalNotTable
        );
    }

    #[test]
    #[ignore]
    fn bad_too_many_ordinals() {
        let content = std::fs::read_to_string("fidlc/tests/bad/fi-0092.test.fidl").unwrap();
        let source = SourceFile::new("bad/fi-0092.test.fidl".to_string(), content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
        let errors = lib.reporter().diagnostics();
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].def,
            crate::diagnostics::Error::ErrTableOrdinalTooLarge
        );
    }

    #[test]
    #[ignore]
    fn bad_recursion_disallowed() {
        let content = std::fs::read_to_string("fidlc/tests/bad/fi-0057-d.test.fidl").unwrap();
        let source = SourceFile::new("bad/fi-0057-d.test.fidl".to_string(), content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
        let errors = lib.reporter().diagnostics();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].def, crate::diagnostics::Error::ErrIncludeCycle);
    }
}
