use crate::source_file::SourceFile;
use crate::tests::test_library::TestLibrary;

#[test]
fn good_error() {
    let mut library = TestLibrary::new();
    let source0 = SourceFile::new(
        "example0.fidl".to_string(),
        r#"

library example;

protocol Example {
    strict Method() -> (struct {
        foo string;
    }) error int32;
};
"#
        .to_string(),
    );
    library.add_source(&source0);
    library.compile().expect("compilation failed");
    // TODO: port AST checks: strict Method() -> (struct {
    // TODO: port AST checks: auto methods = &library.LookupProtocol("Example")->methods;
    // TODO: port AST checks: ASSERT_EQ(methods->size(), 1u);
    // TODO: port AST checks: auto method = &methods->at(0);
    // TODO: port AST checks: auto response = method->maybe_response.get();
    // TODO: port AST checks: ASSERT_NE(response, nullptr);
    // TODO: port AST checks: auto id = static_cast<const IdentifierType*>(response->type);
    // TODO: port AST checks: auto result_union = static_cast<const Union*>(id->type_decl);
    // TODO: port AST checks: ASSERT_NE(result_union, nullptr);
    // TODO: port AST checks: ASSERT_EQ(result_union->members.size(), 2u);
    // TODO: port AST checks: auto anonymous = result_union->name.as_anonymous();
    // TODO: port AST checks: ASSERT_NE(anonymous, nullptr);
    // TODO: port AST checks: ASSERT_EQ(anonymous->provenance, Name::Provenance::kGeneratedResultUnion);
    // TODO: port AST checks: const auto& success = result_union->members.at(0);
    // TODO: port AST checks: ASSERT_EQ("response", success.name.data());
    // TODO: port AST checks: const Union::Member& error = result_union->members.at(1);
    // TODO: port AST checks: ASSERT_EQ("err", error.name.data());
    // TODO: port AST checks: ASSERT_EQ(error.type_ctor->type->kind, Type::Kind::kPrimitive);
    // TODO: port AST checks: auto primitive_type = static_cast<const PrimitiveType*>(error.type_ctor->type);
    // TODO: port AST checks: ASSERT_EQ(primitive_type->subtype, PrimitiveSubtype::kInt32);
}

#[test]
fn good_error_unsigned() {
    let mut library = TestLibrary::new();
    let source0 = SourceFile::new(
        "example0.fidl".to_string(),
        r#"

library example;

protocol Example {
    Method() -> (struct {
        foo string;
    }) error uint32;
};
"#
        .to_string(),
    );
    library.add_source(&source0);
    library.compile().expect("compilation failed");
    // TODO: port AST checks: Method() -> (struct {
}

#[test]
fn good_error_empty_struct_as_success() {
    let mut library = TestLibrary::new();
    let source0 = SourceFile::new(
        "example0.fidl".to_string(),
        r#"

library example;

protocol MyProtocol {
  strict MyMethod() -> () error uint32;
};
"#
        .to_string(),
    );
    library.add_source(&source0);
    library.compile().expect("compilation failed");
    // TODO: port AST checks: strict MyMethod() -> () error uint32;
    // TODO: port AST checks: auto protocol = library.LookupProtocol("MyProtocol");
    // TODO: port AST checks: ASSERT_NE(protocol, nullptr);
    // TODO: port AST checks: ASSERT_EQ(protocol->methods.size(), 1u);
    // TODO: port AST checks: auto& method = protocol->methods[0];
    // TODO: port AST checks: ASSERT_NE(method.maybe_response, nullptr);
    // TODO: port AST checks: auto id = static_cast<const IdentifierType*>(method.maybe_response->type);
    // TODO: port AST checks: auto response = static_cast<const Union*>(id->type_decl);
    // TODO: port AST checks: EXPECT_TRUE(response->kind == Decl::Kind::kUnion);
    // TODO: port AST checks: ASSERT_EQ(response->members.size(), 2u);
    // TODO: port AST checks: auto empty_struct_name = response->members[0].type_ctor->type->name.decl_name();
    // TODO: port AST checks: auto empty_struct = library.LookupStruct(empty_struct_name);
    // TODO: port AST checks: ASSERT_NE(empty_struct, nullptr);
    // TODO: port AST checks: auto anonymous = empty_struct->name.as_anonymous();
    // TODO: port AST checks: ASSERT_EQ(anonymous->provenance, Name::Provenance::kGeneratedEmptySuccessStruct);
}

#[test]
fn good_error_enum() {
    let mut library = TestLibrary::new();
    let source0 = SourceFile::new(
        "example0.fidl".to_string(),
        r#"

library example;

type ErrorType = enum : int32 {
    GOOD = 1;
    BAD = 2;
    UGLY = 3;
};

protocol Example {
    Method() -> (struct {
        foo string;
    }) error ErrorType;
};
"#
        .to_string(),
    );
    library.add_source(&source0);
    library.compile().expect("compilation failed");
    // TODO: port AST checks: Method() -> (struct {
}

#[test]
fn good_error_enum_after() {
    let mut library = TestLibrary::new();
    let source0 = SourceFile::new(
        "example0.fidl".to_string(),
        r#"

library example;

protocol Example {
    Method() -> (struct {
        foo string;
    }) error ErrorType;
};

type ErrorType = enum : int32 {
    GOOD = 1;
    BAD = 2;
    UGLY = 3;
};
"#
        .to_string(),
    );
    library.add_source(&source0);
    library.compile().expect("compilation failed");
    // TODO: port AST checks: Method() -> (struct {
}

#[test]
#[ignore]
fn bad_error_unknown_identifier() {
    let mut library = TestLibrary::new();
    let result = library.compile();
    assert!(result.is_err(), "expected compilation to fail");
}

#[test]
#[ignore]
fn bad_error_wrong_primitive() {
    let mut library = TestLibrary::new();
    let result = library.compile();
    assert!(result.is_err(), "expected compilation to fail");
}

#[test]
#[ignore]
fn bad_error_missing_type() {
    let mut library = TestLibrary::new();
    let source0 = SourceFile::new(
        "example0.fidl".to_string(),
        r#"

library example;
protocol Example {
    Method() -> (flub int32) error;
};
"#
        .to_string(),
    );
    library.add_source(&source0);
    let result = library.compile();
    assert!(result.is_err(), "expected compilation to fail");
    // TODO: port error checks: Method() -> (flub int32) error;
}

#[test]
fn bad_error_not_a_type() {
    let mut library = TestLibrary::new();
    let source0 = SourceFile::new(
        "example0.fidl".to_string(),
        r#"

library example;
protocol Example {
    Method() -> (flub int32) error "hello";
};
"#
        .to_string(),
    );
    library.add_source(&source0);
    let result = library.compile();
    assert!(result.is_err(), "expected compilation to fail");
    // TODO: port error checks: Method() -> (flub int32) error "hello";
}

#[test]
fn bad_error_no_response() {
    let mut library = TestLibrary::new();
    let source0 = SourceFile::new(
        "example0.fidl".to_string(),
        r#"

library example;
protocol Example {
    Method() -> error int32;
};
"#
        .to_string(),
    );
    library.add_source(&source0);
    let result = library.compile();
    assert!(result.is_err(), "expected compilation to fail");
    // TODO: port error checks: Method() -> error int32;
}

#[test]
fn bad_error_unexpected_end_of_file() {
    let mut library = TestLibrary::new();
    let source0 = SourceFile::new(
        "example0.fidl".to_string(),
        r#"

library example;
type ForgotTheSemicolon = table {}
"#
        .to_string(),
    );
    library.add_source(&source0);
    let result = library.compile();
    assert!(result.is_err(), "expected compilation to fail");
}

#[test]
#[ignore]
fn bad_incorrect_identifier() {
    let mut library = TestLibrary::new();
    let result = library.compile();
    assert!(result.is_err(), "expected compilation to fail");
}

#[test]
#[ignore]
fn bad_error_empty_file() {
    let mut library = TestLibrary::new();
    let result = library.compile();
    assert!(result.is_err(), "expected compilation to fail");
}

#[test]
#[ignore]
fn experimental_allow_arbitrary_error_types() {
    let mut library = TestLibrary::new();
    let source0 = SourceFile::new(
        "example0.fidl".to_string(),
        r#"

library example;
protocol Example {
    Method() -> () error table {};
};
"#
        .to_string(),
    );
    library.add_source(&source0);
    library.compile().expect("compilation failed");
    // TODO: port AST checks: Method() -> () error table {};
    // TODO: port AST checks: auto result_id = static_cast<const IdentifierType*>(
    // TODO: port AST checks: library.LookupProtocol("Example")->methods.at(0).maybe_response->type);
    // TODO: port AST checks: auto result_union = static_cast<const Union*>(result_id->type_decl);
    // TODO: port AST checks: auto error_id = static_cast<const IdentifierType*>(result_union->members.at(1).type_ctor->type);
    // TODO: port AST checks: ASSERT_EQ(error_id->type_decl->kind, Decl::Kind::kTable);
}

#[test]
#[ignore]
fn transitional_removed() {
    let mut library = TestLibrary::new();
    let source0 = SourceFile::new(
        "example0.fidl".to_string(),
        r#"

library example;
protocol Example {
    @transitional
    Method();
};
"#
        .to_string(),
    );
    library.add_source(&source0);
    let result = library.compile();
    assert!(result.is_err(), "expected compilation to fail");
}
