use crate::tests::test_library::{SharedAmongstLibraries, TestLibrary};

#[test]
fn bad_collision() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0035.test.fidl");
    // EXPECT FAIL
    assert!(library.compile().is_err());
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
    assert!(library.compile().is_err());
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
    assert!(library.compile().is_err());
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
    assert!(library.compile().is_err());
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
    assert!(library.compile().is_err());
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
    assert!(library.compile().is_err());
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
    assert!(library.compile().is_err());
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
    assert!(library.compile().is_err());
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
    assert!(library.compile().is_err());
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
    assert!(library.compile().is_err());
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
    assert!(library.compile().is_err());
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
    assert!(library.compile().is_err());
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
    assert!(library.compile().is_err());
}

#[test]

fn bad_member_values() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0054.test.fidl");
    // EXPECT FAIL
    assert!(library.compile().is_err());
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
    assert!(library.compile().is_err());
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
    assert!(library.compile().is_err());
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
    assert!(library.compile().is_err());
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
