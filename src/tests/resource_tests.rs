use crate::diagnostics::Error;
use crate::tests::test_library::TestLibrary;

#[test]

fn good_valid_without_rights() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type MyEnum = strict enum : uint32 {
    NONE = 0;
};

resource_definition SomeResource : uint32 {
    properties {
        subtype MyEnum;
    };
};
"#,
    );
    let _root = match lib.compile() {
        Ok(root) => root,
        Err(e) => {
            lib.reporter().print_reports();
            panic!("compilation failed: {}", e);
        }
    };
    // We will need to check ExperimentalResourceDeclaration once it is implemented.
}

#[test]

fn good_valid_with_rights() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type MyEnum = strict enum : uint32 {
    NONE = 0;
};

resource_definition SomeResource : uint32 {
    properties {
        subtype MyEnum;
        rights uint32;
    };
};
"#,
    );
    let _root = match lib.compile() {
        Ok(root) => root,
        Err(e) => {
            lib.reporter().print_reports();
            panic!("compilation failed: {}", e);
        }
    };
}

#[test]

fn good_aliased_base_type_without_rights() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type MyEnum = strict enum : uint32 {
    NONE = 0;
};

alias via = uint32;

resource_definition SomeResource : via {
    properties {
        subtype MyEnum;
    };
};
"#,
    );
    let _root = match lib.compile() {
        Ok(root) => root,
        Err(e) => {
            lib.reporter().print_reports();
            panic!("compilation failed: {}", e);
        }
    };
}

#[test]

fn good_aliased_base_type_with_rights() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type MyEnum = strict enum : uint32 {
    NONE = 0;
};

alias via = uint32;

resource_definition SomeResource : via {
    properties {
        subtype MyEnum;
        rights via;
    };
};
"#,
    );
    let _root = match lib.compile() {
        Ok(root) => root,
        Err(e) => {
            lib.reporter().print_reports();
            panic!("compilation failed: {}", e);
        }
    };
}

#[test]

fn bad_empty() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

resource_definition SomeResource : uint32 {
};
"#,
    );
    lib.expect_fail(Error::ErrUnexpectedTokenOfKind(
        r#"None"#.into(),
        r#"Properties"#.into(),
    ));
    lib.expect_fail(Error::ErrExpectedDeclaration(r#";"#.into()));

    assert!(lib.check_compile());
}

#[test]

fn bad_no_properties() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0029.noformat.test.fidl");
    lib.expect_fail(Error::ErrMustHaveOneProperty);

    assert!(lib.check_compile());
}

#[test]

fn bad_duplicate_property() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

resource_definition MyResource : uint32 {
    properties {
        subtype flexible enum : uint32 {};
        rights uint32;
        rights uint32;
    };
};
"#,
    );
    lib.expect_fail(Error::ErrNameCollision(
        r#"resource property"#.into(),
        r#"rights"#.into(),
        r#"resource property"#.into(),
        r#"example.fidl:7:9"#.into(),
    ));

    assert!(lib.check_compile());
}

#[test]

fn bad_not_uint32() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0172.test.fidl");
    lib.expect_fail(Error::ErrResourceMustBeUint32Derived(
        r#"MyResource"#.into(),
    ));

    assert!(lib.check_compile());
}

#[test]

fn bad_missing_subtype_property_test() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0173.test.fidl");
    lib.expect_fail(Error::ErrResourceMissingSubtypeProperty(
        r#"MyResource"#.into(),
    ));

    assert!(lib.check_compile());
}

#[test]

fn bad_subtype_not_enum() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0175.test.fidl");
    lib.expect_fail(Error::ErrResourceSubtypePropertyMustReferToEnum(
        r#"MyResource"#.into(),
    ));

    assert!(lib.check_compile());
}

#[test]

fn bad_subtype_not_identifier() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

resource_definition handle : uint32 {
    properties {
        subtype uint32;
    };
};
"#,
    );
    lib.expect_fail(Error::ErrResourceSubtypePropertyMustReferToEnum(
        r#"handle"#.into(),
    ));

    assert!(lib.check_compile());
}

#[test]

fn bad_non_bits_rights() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0177.test.fidl");
    lib.expect_fail(Error::ErrResourceRightsPropertyMustReferToBits(
        r#"MyResource"#.into(),
    ));

    assert!(lib.check_compile());
}

#[test]

fn bad_include_cycle() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

resource_definition handle : uint32 {
    properties {
        subtype handle;
    };
};
"#,
    );
    lib.expect_fail(Error::ErrIncludeCycle(
        r#"resource 'handle' -> resource 'handle'"#.into(),
    ));

    lib.expect_fail(Error::ErrResourceSubtypePropertyMustReferToEnum(
        r#"handle"#.into(),
    ));

    assert!(lib.check_compile());
}
