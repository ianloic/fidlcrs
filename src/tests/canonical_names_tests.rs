use crate::source_file::SourceFile;
use crate::tests::test_library::TestLibrary;
use std::fs;

fn get_file_content(path: &str) -> String {
    let full_path = format!("fidlc/tests/fidl/{}", path);
    fs::read_to_string(&full_path).unwrap_or_else(|_| panic!("Failed to read file {}", full_path))
}

#[test]
#[ignore]
fn bad_collision() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "bad/fi-0035.test.fidl".to_string(),
        get_file_content("bad/fi-0035.test.fidl"),
    );
    library.add_source(&source);
    // EXPECT FAIL
    assert!(library.compile().is_err());
}

#[test]
fn good_collision_fix_rename() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "good/fi-0035.test.fidl".to_string(),
        get_file_content("good/fi-0035.test.fidl"),
    );
    library.add_source(&source);
    library.compile().expect("compilation failed");
}

#[test]
fn good_top_level() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example.fidl".to_string(),
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
    "#
        .to_string(),
    );
    library.add_source(&source);
    library.compile().expect("compilation failed");
}

#[test]
fn good_attributes() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    @foobar
    @foo_bar
    @f_o_o_b_a_r
    type Example = struct {};
    "#
        .to_string(),
    );
    library.add_source(&source);
    library.compile().expect("compilation failed");
}

#[test]
fn good_attribute_arguments() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    @some_attribute(foobar="", foo_bar="", f_o_o_b_a_r="")
    type Example = struct {};
    "#
        .to_string(),
    );
    library.add_source(&source);
    library.compile().expect("compilation failed");
}

#[test]
fn good_struct_members() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    type Example = struct {
            foobar bool;
            foo_bar bool;
            f_o_o_b_a_r bool;
    };
    "#
        .to_string(),
    );
    library.add_source(&source);
    library.compile().expect("compilation failed");
}

#[test]
fn good_table_members() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    type Example = table {
            1: foobar bool;
            2: foo_bar bool;
            3: f_o_o_b_a_r bool;
    };
    "#
        .to_string(),
    );
    library.add_source(&source);
    library.compile().expect("compilation failed");
}

#[test]
fn good_union_members() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    type Example = strict union {
            1: foobar bool;
            2: foo_bar bool;
            3: f_o_o_b_a_r bool;
    };
    "#
        .to_string(),
    );
    library.add_source(&source);
    library.compile().expect("compilation failed");
}

#[test]
fn good_enum_members() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    type Example = strict enum {
            foobar = 1;
            foo_bar = 2;
            f_o_o_b_a_r = 3;
    };
    "#
        .to_string(),
    );
    library.add_source(&source);
    library.compile().expect("compilation failed");
}

#[test]
fn good_bits_members() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    type Example = strict bits {
            foobar = 1;
            foo_bar = 2;
            f_o_o_b_a_r = 4;
    };
    "#
        .to_string(),
    );
    library.add_source(&source);
    library.compile().expect("compilation failed");
}

#[test]
fn good_protocol_methods() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    protocol Example {
            foobar() -> ();
            foo_bar() -> ();
            f_o_o_b_a_r() -> ();
    };
    "#
        .to_string(),
    );
    library.add_source(&source);
    library.compile().expect("compilation failed");
}

#[test]
fn good_method_parameters() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    protocol Example {
            example(struct {
                    foobar bool;
                    foo_bar bool;
                    f_o_o_b_a_r bool;
            }) -> ();
    };
    "#
        .to_string(),
    );
    library.add_source(&source);
    library.compile().expect("compilation failed");
}

#[test]
fn good_method_results() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    protocol Example {
            example() -> (struct {
                    foobar bool;
                    foo_bar bool;
                    f_o_o_b_a_r bool;
            });
    };
    "#
        .to_string(),
    );
    library.add_source(&source);
    library.compile().expect("compilation failed");
}

#[test]
fn good_service_members() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    protocol P {};
    service Example {
            foobar client_end:P;
            foo_bar client_end:P;
            f_o_o_b_a_r client_end:P;
    };
    "#
        .to_string(),
    );
    library.add_source(&source);
    library.compile().expect("compilation failed");
}

#[test]
#[ignore]
fn good_resource_properties() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example.fidl".to_string(),
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
    "#
        .to_string(),
    );
    library.add_source(&source);
    library.compile().expect("compilation failed");
}

#[test]
fn good_upper_acronym() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    type HTTPServer = struct {};
    type httpserver = struct {};
    "#
        .to_string(),
    );
    library.add_source(&source);
    library.compile().expect("compilation failed");
}

#[test]
fn good_current_library() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    type example = struct {};
    "#
        .to_string(),
    );
    library.add_source(&source);
    library.compile().expect("compilation failed");
}

#[test]
#[ignore]
fn good_dependent_library() {
    // SharedLibrary is not fully translated
    let mut dependency = TestLibrary::new();
    let dep_source = SourceFile::new(
        "foobar.fidl".to_string(),
        r#"
    library foobar;
    type Something = struct {};
    "#
        .to_string(),
    );
    dependency.add_source(&dep_source);
    dependency.compile().expect("dep failed");
    // ASSERT_COMPILED(dependency);
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example.fidl".to_string(),
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
    "#
        .to_string(),
    );
    library.add_source(&source);
    library.compile().expect("compilation failed");
}

#[test]
#[ignore]
fn bad_top_level() {
    // TODO: port manually
}

#[test]
#[ignore]
fn bad_attributes() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    @fooBar
    @FooBar
    type Example = struct {};
    "#
        .to_string(),
    );
    library.add_source(&source);
    // EXPECT FAIL
    assert!(library.compile().is_err());
}

#[test]
#[ignore]
fn bad_attribute_arguments() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    @some_attribute(fooBar="", FooBar="")
    type Example = struct {};
    "#
        .to_string(),
    );
    library.add_source(&source);
    // EXPECT FAIL
    assert!(library.compile().is_err());
}

#[test]
#[ignore]
fn bad_struct_members() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    type MyStruct = struct {
            myStructMember string;
            MyStructMember uint64;
    };
    "#
        .to_string(),
    );
    library.add_source(&source);
    // EXPECT FAIL
    assert!(library.compile().is_err());
}

#[test]
#[ignore]
fn bad_table_members() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    type MyTable = table {
            1: myField bool;
            2: MyField bool;
    };
    "#
        .to_string(),
    );
    library.add_source(&source);
    // EXPECT FAIL
    assert!(library.compile().is_err());
}

#[test]
#[ignore]
fn bad_union_members() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    type MyUnion = union {
            1: myVariant bool;
            2: MyVariant bool;
    };
    "#
        .to_string(),
    );
    library.add_source(&source);
    // EXPECT FAIL
    assert!(library.compile().is_err());
}

#[test]
#[ignore]
fn bad_enum_members() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    type Example = enum {
        fooBar = 1;
        FooBar = 2;
    };
    "#
        .to_string(),
    );
    library.add_source(&source);
    // EXPECT FAIL
    assert!(library.compile().is_err());
}

#[test]
#[ignore]
fn bad_bits_members() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    type MyBits = bits {
            fooBar = 1;
            FooBar = 2;
    };
    "#
        .to_string(),
    );
    library.add_source(&source);
    // EXPECT FAIL
    assert!(library.compile().is_err());
}

#[test]
#[ignore]
fn bad_protocol_methods() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    protocol MyProtocol {
            strict myMethod() -> ();
            strict MyMethod() -> ();
    };
    "#
        .to_string(),
    );
    library.add_source(&source);
    // EXPECT FAIL
    assert!(library.compile().is_err());
}

#[test]
#[ignore]
fn bad_method_parameters() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    protocol Example {
        example(struct { fooBar bool; FooBar bool; }) -> ();
    };
    "#
        .to_string(),
    );
    library.add_source(&source);
    // EXPECT FAIL
    assert!(library.compile().is_err());
}

#[test]
#[ignore]
fn bad_method_results() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    protocol Example {
        example() -> (struct { fooBar bool; FooBar bool; });
    };
    "#
        .to_string(),
    );
    library.add_source(&source);
    // EXPECT FAIL
    assert!(library.compile().is_err());
}

#[test]
#[ignore]
fn bad_service_members() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    protocol MyProtocol {};
    service MyService {
            myServiceMember client_end:MyProtocol;
            MyServiceMember client_end:MyProtocol;
    };
    "#
        .to_string(),
    );
    library.add_source(&source);
    // EXPECT FAIL
    assert!(library.compile().is_err());
}

#[test]
#[ignore]
fn bad_resource_properties() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    resource_definition MyResource : uint32 {
            properties {
                    subtype flexible enum : uint32 {};
                    rights uint32;
                    Rights uint32;
            };
    };
    "#
        .to_string(),
    );
    library.add_source(&source);
    // EXPECT FAIL
    assert!(library.compile().is_err());
}

#[test]
#[ignore]
fn bad_member_values() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "bad/fi-0054.test.fidl".to_string(),
        get_file_content("bad/fi-0054.test.fidl"),
    );
    library.add_source(&source);
    // EXPECT FAIL
    assert!(library.compile().is_err());
}

#[test]
#[ignore]
fn bad_upper_acronym() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    type HTTPServer = struct {};
    type HttpServer = struct {};
    "#
        .to_string(),
    );
    library.add_source(&source);
    // EXPECT FAIL
    assert!(library.compile().is_err());
}

#[test]
#[ignore]
fn bad_dependent_library() {
    // SharedLibrary is not fully translated
    let mut dependency = TestLibrary::new();
    let dep_source = SourceFile::new(
        "foobar.fidl".to_string(),
        r#"
    library foobar;
    type Something = struct {};
    "#
        .to_string(),
    );
    dependency.add_source(&dep_source);
    dependency.compile().expect("dep failed");
    // ASSERT_COMPILED(dependency);
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "lib.fidl".to_string(),
        r#"
    library example;
    using foobar;
    alias FOOBAR = foobar.Something;
    "#
        .to_string(),
    );
    library.add_source(&source);
    // EXPECT FAIL
    assert!(library.compile().is_err());
}

#[test]
#[ignore]
fn bad_various_collisions() {
    // TODO: port manually
}

#[test]
#[ignore]
fn bad_consecutive_underscores() {
    let mut library = TestLibrary::new();
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    type it_is_the_same = struct {};
    type it__is___the____same = struct {};
    "#
        .to_string(),
    );
    library.add_source(&source);
    // EXPECT FAIL
    assert!(library.compile().is_err());
}

#[test]
#[ignore]
fn bad_inconsistent_type_spelling() {
    // TODO: port manually
}

#[test]
#[ignore]
fn bad_inconsistent_const_spelling() {
    // TODO: port manually
}

#[test]
#[ignore]
fn bad_inconsistent_enum_member_spelling() {
    // TODO: port manually
}

#[test]
#[ignore]
fn bad_inconsistent_bits_member_spelling() {
    // TODO: port manually
}
