use crate::diagnostics::Error;
use crate::tests::test_library::{SharedAmongstLibraries, TestLibrary};

#[test]
fn bad_collision() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0035.test.fidl");
    // EXPECT FAIL
    library.expect_fail(Error::ErrNameCollisionCanonical(
        r#"const"#.to_string(),
        r#"COLOR"#.to_string(),
        r#"protocol"#.to_string(),
        r#"Color"#.to_string(),
        r#"bad/fi-0035.test.fidl:8:1"#.to_string(),
        r#"color"#.to_string(),
    ));

    assert!(library.check_compile());
}

#[test]
fn good_collision_fix_rename() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0035.test.fidl");
    library.compile().expect("compilation failed");
}

#[test]
fn good_top_level() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
    library example;
    alias foobar = bool;
    const f_oobar bool = true;
    type fo_obar = struct {};
    type foo_bar = struct {};
    type foob_ar = table {};
    type fooba_r = strict union {
            1: x bool;
    };
    type FoObAr = strict enum {
            A = 1;
    };
    type FooBaR = strict bits {
            A = 1;
    };
    protocol FoObaR {};
    service FOoBAR {};
    "#,
    );
    library.compile().expect("compilation failed");
}

#[test]
fn good_attributes() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
    library example;
    @foobar
    @foo_bar
    @f_o_o_b_a_r
    type Example = struct {};
    "#,
    );
    library.compile().expect("compilation failed");
}

#[test]
fn good_attribute_arguments() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
    library example;
    @some_attribute(foobar="", foo_bar="", f_o_o_b_a_r="")
    type Example = struct {};
    "#,
    );
    library.compile().expect("compilation failed");
}

#[test]
fn good_struct_members() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
    library example;
    type Example = struct {
            foobar bool;
            foo_bar bool;
            f_o_o_b_a_r bool;
    };
    "#,
    );
    library.compile().expect("compilation failed");
}

#[test]
fn good_table_members() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
    library example;
    type Example = table {
            1: foobar bool;
            2: foo_bar bool;
            3: f_o_o_b_a_r bool;
    };
    "#,
    );
    library.compile().expect("compilation failed");
}

#[test]
fn good_union_members() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
    library example;
    type Example = strict union {
            1: foobar bool;
            2: foo_bar bool;
            3: f_o_o_b_a_r bool;
    };
    "#,
    );
    library.compile().expect("compilation failed");
}

#[test]
fn good_enum_members() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
    library example;
    type Example = strict enum {
            foobar = 1;
            foo_bar = 2;
            f_o_o_b_a_r = 3;
    };
    "#,
    );
    library.compile().expect("compilation failed");
}

#[test]
fn good_bits_members() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
    library example;
    type Example = strict bits {
            foobar = 1;
            foo_bar = 2;
            f_o_o_b_a_r = 4;
    };
    "#,
    );
    library.compile().expect("compilation failed");
}

#[test]
fn good_protocol_methods() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
    library example;
    protocol Example {
            foobar() -> ();
            foo_bar() -> ();
            f_o_o_b_a_r() -> ();
    };
    "#,
    );
    library.compile().expect("compilation failed");
}

#[test]
fn good_method_parameters() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
    library example;
    protocol Example {
            example(struct {
                    foobar bool;
                    foo_bar bool;
                    f_o_o_b_a_r bool;
            }) -> ();
    };
    "#,
    );
    library.compile().expect("compilation failed");
}

#[test]
fn good_method_results() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
    library example;
    protocol Example {
            example() -> (struct {
                    foobar bool;
                    foo_bar bool;
                    f_o_o_b_a_r bool;
            });
    };
    "#,
    );
    library.compile().expect("compilation failed");
}

#[test]
fn good_service_members() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
    library example;
    protocol P {};
    service Example {
            foobar client_end:P;
            foo_bar client_end:P;
            f_o_o_b_a_r client_end:P;
    };
    "#,
    );
    library.compile().expect("compilation failed");
}

#[test]
fn good_resource_properties() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
    library example;
    resource_definition Example {
            properties {
                    // This property is required for compilation, but is not otherwise under test.
                    subtype flexible enum : uint32 {};
                    foobar uint32;
                    foo_bar uint32;
                    f_o_o_b_a_r uint32;
            };
    };
    "#,
    );
    library.compile().expect("compilation failed");
}

#[test]
fn good_upper_acronym() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
    library example;
    type HTTPServer = struct {};
    type httpserver = struct {};
    "#,
    );
    library.compile().expect("compilation failed");
}

#[test]
fn good_current_library() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
    library example;
    type example = struct {};
    "#,
    );
    library.compile().expect("compilation failed");
}

#[test]
fn good_dependent_library() {
    let mut shared = SharedAmongstLibraries::new();
    let mut dependency = TestLibrary::with_shared(&mut shared);

    dependency.add_source_file(
        "foobar.fidl",
        r#"
    library foobar;
    type Something = struct {};
    "#,
    );

    dependency.compile().expect("compilation failed");

    let mut library = TestLibrary::with_shared(&mut shared);

    library.add_source_file(
        "example.fidl",
        r#"
    library example;
    using foobar;
    alias f_o_o_b_a_r = foobar.Something;
    const f_oobar bool = true;
    type fo_obar = struct {};
    type foo_bar = struct {};
    type foob_ar = table {};
    type fooba_r = union { 1: x bool; };
    type FoObAr = enum { A = 1; };
    type FooBaR = bits { A = 1; };
    protocol FoObaR {};
    service FOoBAR {};
    "#,
    );
    library.compile().expect("compilation failed");
}

#[test]

fn bad_top_level() {
    // TODO: port manually
}

#[test]

fn bad_attributes() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
    library example;
    @fooBar
    @FooBar
    type Example = struct {};
    "#,
    );
    // EXPECT FAIL
    library.expect_fail(Error::ErrDuplicateAttributeCanonical(
        r#"FooBar"#.to_string(),
        r#"fooBar"#.to_string(),
        r#"fooBar"#.to_string(),
        r#"foo_bar"#.to_string(),
    ));

    assert!(library.check_compile());
}

#[test]

fn bad_attribute_arguments() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
    library example;
    @some_attribute(fooBar="", FooBar="")
    type Example = struct {};
    "#,
    );
    // EXPECT FAIL
    library.expect_fail(Error::ErrDuplicateAttributeArgCanonical(
        r#"some_attribute"#.to_string(),
        r#"FooBar"#.to_string(),
        r#"fooBar="""#.to_string(),
        r#"fooBar="""#.to_string(),
        r#"foo_bar"#.to_string(),
    ));

    assert!(library.check_compile());
}

#[test]
fn bad_struct_members() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
    library example;
    type MyStruct = struct {
            myStructMember string;
            MyStructMember uint64;
    };
    "#,
    );
    // EXPECT FAIL
    library.expect_fail(Error::ErrNameCollisionCanonical(
        r#"struct member"#.to_string(),
        r#"MyStructMember"#.to_string(),
        r#"struct member"#.to_string(),
        r#"myStructMember"#.to_string(),
        r#"example.fidl:4:13"#.to_string(),
        r#"my_struct_member"#.to_string(),
    ));

    assert!(library.check_compile());
}

#[test]
fn bad_table_members() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
    library example;
    type MyTable = table {
            1: myField bool;
            2: MyField bool;
    };
    "#,
    );
    // EXPECT FAIL
    library.expect_fail(Error::ErrNameCollisionCanonical(
        r#"table field"#.to_string(),
        r#"MyField"#.to_string(),
        r#"table field"#.to_string(),
        r#"myField"#.to_string(),
        r#"example.fidl:4:16"#.to_string(),
        r#"my_field"#.to_string(),
    ));

    assert!(library.check_compile());
}

#[test]
fn bad_union_members() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
    library example;
    type MyUnion = union {
            1: myVariant bool;
            2: MyVariant bool;
    };
    "#,
    );
    // EXPECT FAIL
    library.expect_fail(Error::ErrNameCollisionCanonical(
        r#"union member"#.to_string(),
        r#"MyVariant"#.to_string(),
        r#"union member"#.to_string(),
        r#"myVariant"#.to_string(),
        r#"example.fidl:4:16"#.to_string(),
        r#"my_variant"#.to_string(),
    ));

    assert!(library.check_compile());
}

#[test]
fn bad_enum_members() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
    library example;
    type Example = enum {
        fooBar = 1;
        FooBar = 2;
    };
    "#,
    );
    // EXPECT FAIL
    library.expect_fail(Error::ErrNameCollisionCanonical(
        r#"member"#.to_string(),
        r#"FooBar"#.to_string(),
        r#"member"#.to_string(),
        r#"fooBar"#.to_string(),
        r#"example.fidl:4:9"#.to_string(),
        r#"foo_bar"#.to_string(),
    ));

    assert!(library.check_compile());
}

#[test]
fn bad_bits_members() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
    library example;
    type MyBits = bits {
            fooBar = 1;
            FooBar = 2;
    };
    "#,
    );
    // EXPECT FAIL
    library.expect_fail(Error::ErrNameCollisionCanonical(
        r#"member"#.to_string(),
        r#"FooBar"#.to_string(),
        r#"member"#.to_string(),
        r#"fooBar"#.to_string(),
        r#"example.fidl:4:13"#.to_string(),
        r#"foo_bar"#.to_string(),
    ));

    assert!(library.check_compile());
}

#[test]
fn bad_protocol_methods() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
    library example;
    protocol MyProtocol {
            strict myMethod() -> ();
            strict MyMethod() -> ();
    };
    "#,
    );
    // EXPECT FAIL
    library.expect_fail(Error::ErrNameCollisionCanonical(
        r#"method"#.to_string(),
        r#"MyMethod"#.to_string(),
        r#"method"#.to_string(),
        r#"myMethod"#.to_string(),
        r#"example.fidl:4:20"#.to_string(),
        r#"my_method"#.to_string(),
    ));

    assert!(library.check_compile());
}

#[test]
fn bad_method_parameters() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
    library example;
    protocol Example {
        example(struct { fooBar bool; FooBar bool; }) -> ();
    };
    "#,
    );
    // EXPECT FAIL
    library.expect_fail(Error::ErrNameCollisionCanonical(
        r#"struct member"#.to_string(),
        r#"FooBar"#.to_string(),
        r#"struct member"#.to_string(),
        r#"fooBar"#.to_string(),
        r#"example.fidl:4:26"#.to_string(),
        r#"foo_bar"#.to_string(),
    ));

    assert!(library.check_compile());
}

#[test]
fn bad_method_results() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
    library example;
    protocol Example {
        example() -> (struct { fooBar bool; FooBar bool; });
    };
    "#,
    );
    // EXPECT FAIL
    library.expect_fail(Error::ErrNameCollisionCanonical(
        r#"struct member"#.to_string(),
        r#"FooBar"#.to_string(),
        r#"struct member"#.to_string(),
        r#"fooBar"#.to_string(),
        r#"example.fidl:4:32"#.to_string(),
        r#"foo_bar"#.to_string(),
    ));

    assert!(library.check_compile());
}

#[test]
fn bad_service_members() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
    library example;
    protocol MyProtocol {};
    service MyService {
            myServiceMember client_end:MyProtocol;
            MyServiceMember client_end:MyProtocol;
    };
    "#,
    );
    // EXPECT FAIL
    library.expect_fail(Error::ErrNameCollisionCanonical(
        r#"service member"#.to_string(),
        r#"MyServiceMember"#.to_string(),
        r#"service member"#.to_string(),
        r#"myServiceMember"#.to_string(),
        r#"example.fidl:5:13"#.to_string(),
        r#"my_service_member"#.to_string(),
    ));

    assert!(library.check_compile());
}

#[test]
fn bad_resource_properties() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
    library example;
    resource_definition MyResource : uint32 {
            properties {
                    subtype flexible enum : uint32 {};
                    rights uint32;
                    Rights uint32;
            };
    };
    "#,
    );
    // EXPECT FAIL
    library.expect_fail(Error::ErrNameCollisionCanonical(
        r#"resource property"#.to_string(),
        r#"Rights"#.to_string(),
        r#"resource property"#.to_string(),
        r#"rights"#.to_string(),
        r#"example.fidl:6:21"#.to_string(),
        r#"rights"#.to_string(),
    ));

    assert!(library.check_compile());
}

#[test]

fn bad_member_values() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0054.test.fidl");
    // EXPECT FAIL
    library.expect_fail(Error::ErrCouldNotResolveMember(r#"enum"#.to_string()));

    assert!(library.check_compile());
}

#[test]
fn bad_upper_acronym() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
    library example;
    type HTTPServer = struct {};
    type HttpServer = struct {};
    "#,
    );
    // EXPECT FAIL
    library.expect_fail(Error::ErrNameCollisionCanonical(
        r#"type"#.to_string(),
        r#"HttpServer"#.to_string(),
        r#"type"#.to_string(),
        r#"HTTPServer"#.to_string(),
        r#"example.fidl:3:5"#.to_string(),
        r#"http_server"#.to_string(),
    ));

    assert!(library.check_compile());
}

#[test]

fn bad_dependent_library() {
    let mut shared = SharedAmongstLibraries::new();
    let mut dependency = TestLibrary::with_shared(&mut shared);

    dependency.add_source_file(
        "foobar.fidl",
        r#"
    library foobar;
    type Something = struct {};
    "#,
    );
    dependency.compile().expect("dep failed");
    let mut library = TestLibrary::with_shared(&mut shared);

    library.add_source_file(
        "lib.fidl",
        r#"
    library example;
    using foobar;
    alias FOOBAR = foobar.Something;
    "#,
    );
    // EXPECT FAIL
    library.expect_fail(Error::ErrDeclNameConflictsWithLibraryImportCanonical(
        r#"FOOBAR"#.to_string(),
        r#"foobar"#.to_string(),
    ));

    assert!(library.check_compile());
}

#[test]

fn bad_various_collisions() {
    // TODO: port manually
}

#[test]
fn bad_consecutive_underscores() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
    library example;
    type it_is_the_same = struct {};
    type it__is___the____same = struct {};
    "#,
    );
    // EXPECT FAIL
    library.expect_fail(Error::ErrNameCollisionCanonical(
        r#"type"#.to_string(),
        r#"it__is___the____same"#.to_string(),
        r#"type"#.to_string(),
        r#"it_is_the_same"#.to_string(),
        r#"example.fidl:3:5"#.to_string(),
        r#"it_is_the_same"#.to_string(),
    ));

    assert!(library.check_compile());
}

#[test]

fn bad_inconsistent_type_spelling() {
    // TODO: port manually
}

#[test]

fn bad_inconsistent_const_spelling() {
    // TODO: port manually
}

#[test]

fn bad_inconsistent_enum_member_spelling() {
    // TODO: port manually
}

#[test]

fn bad_inconsistent_bits_member_spelling() {
    // TODO: port manually
}
