use crate::flat_ast::JsonRoot;
use crate::source_file::SourceFile;
use crate::tests::test_library::TestLibrary;

fn direct_and_composed_dependencies(root: &JsonRoot) -> Vec<String> {
    let mut names: Vec<String> = root
        .library_dependencies
        .iter()
        .map(|d| d.name.clone())
        .collect();
    names.sort();
    names
}

#[test]
fn good_direct_deps_simple() {
    for type_usage in [
        "dep2.Type",
        "vector<dep2.Type>",
        "array<dep2.Type, 1>",
        "box<dep2.Type>",
        "client_end:dep2.Protocol",
        "server_end:dep2.Protocol",
        "vector<uint32>:dep2.Constant",
        "array<uint32, dep2.Constant>",
    ] {
        let mut dep2 = TestLibrary::new();
        let dep2_source = SourceFile::new(
            "dep2.fidl".to_string(),
            r#"
library dep2;

const Constant uint32 = 50;
type Type = struct {};
protocol Protocol {};
"#
            .to_string(),
        );
        dep2.add_source(&dep2_source);
        let _dep2_root = dep2.compile().expect("dep2 compilation failed");

        let mut dep1 = TestLibrary::new();
        let dep1_source = SourceFile::new(
            "dep1.fidl".to_string(),
            format!(
                r#"
library dep1;

using dep2;

protocol ComposedProtocol {{
  UsesDep2(resource struct {{ data {}; }});
}};
"#,
                type_usage
            ),
        );
        dep1.add_source(&dep2_source);
        dep1.add_source(&dep1_source);
        let _dep1_root = dep1.compile().expect("dep1 compilation failed");

        let mut lib = TestLibrary::new();
        let lib_source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

using dep1;

protocol CapturesDependencyThroughCompose {
  compose dep1.ComposedProtocol;
};
"#
            .to_string(),
        );
        lib.add_source(&dep2_source);
        lib.add_source(&dep1_source);
        lib.add_source(&lib_source);
        let root = lib.compile().expect("lib compilation failed");

        let expected: Vec<&str> = vec!["dep1", "dep2"];
        assert_eq!(direct_and_composed_dependencies(&root), expected);
    }
}

#[test]
fn good_does_not_follow_alias() {
    let mut dep2 = TestLibrary::new();
    let dep2_source = SourceFile::new(
        "dep2.fidl".to_string(),
        r#"
    library dep2;
    type Foo = struct {};
    "#
        .to_string(),
    );
    dep2.add_source(&dep2_source);
    let _dep2_root = dep2.compile().expect("dep2 compilation failed");
    let mut dep1 = TestLibrary::new();
    dep1.add_source(&dep2_source);
    let dep1_source = SourceFile::new(
        "dep1.fidl".to_string(),
        r#"
    library dep1;
    using dep2;
    alias Bar = dep2.Foo;
    protocol ComposedProtocol {
    UsesDep2InAlias(struct { foo vector<Bar>; });
    };
    "#
        .to_string(),
    );
    dep1.add_source(&dep1_source);
    let _dep1_root = dep1.compile().expect("dep1 compilation failed");
    let mut lib = TestLibrary::new();
    lib.add_source(&dep2_source);
    lib.add_source(&dep1_source);
    let lib_source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    using dep1;
    protocol CapturesDependencyThroughCompose {
    compose dep1.ComposedProtocol;
    };
    "#
        .to_string(),
    );
    lib.add_source(&lib_source);
    let root = lib.compile().expect("lib compilation failed");
    let expected: Vec<&str> = vec!["dep1"];
    assert_eq!(direct_and_composed_dependencies(&root), expected);
}

#[test]
fn good_does_not_follow_nested_struct() {
    let mut dep2 = TestLibrary::new();
    let dep2_source = SourceFile::new(
        "dep2.fidl".to_string(),
        r#"
    library dep2;
    type Foo = struct {};
    "#
        .to_string(),
    );
    dep2.add_source(&dep2_source);
    let _dep2_root = dep2.compile().expect("dep2 compilation failed");
    let mut dep1 = TestLibrary::new();
    dep1.add_source(&dep2_source);
    let dep1_source = SourceFile::new(
        "dep1.fidl".to_string(),
        r#"
    library dep1;
    using dep2;
    type Bar = struct {
    foo dep2.Foo;
    };
    protocol ComposedProtocol {
    UsesDep2InNestedStruct(struct { foo vector<Bar>; });
    };
    "#
        .to_string(),
    );
    dep1.add_source(&dep1_source);
    let _dep1_root = dep1.compile().expect("dep1 compilation failed");
    let mut lib = TestLibrary::new();
    lib.add_source(&dep2_source);
    lib.add_source(&dep1_source);
    let lib_source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    using dep1;
    protocol CapturesDependencyThroughCompose {
    compose dep1.ComposedProtocol;
    };
    "#
        .to_string(),
    );
    lib.add_source(&lib_source);
    let root = lib.compile().expect("lib compilation failed");
    let expected: Vec<&str> = vec!["dep1"];
    assert_eq!(direct_and_composed_dependencies(&root), expected);
}

#[test]
fn good_error_syntax_success_type() {
    let mut dep2 = TestLibrary::new();
    let dep2_source = SourceFile::new(
        "dep2.fidl".to_string(),
        r#"
    library dep2;
    type Foo = struct {};
    "#
        .to_string(),
    );
    dep2.add_source(&dep2_source);
    let _dep2_root = dep2.compile().expect("dep2 compilation failed");
    let mut dep1 = TestLibrary::new();
    dep1.add_source(&dep2_source);
    let dep1_source = SourceFile::new(
        "dep1.fidl".to_string(),
        r#"
    library dep1;
    using dep2;
    protocol ComposedProtocol {
    UsesDep2InSuccessType() -> (struct { foo vector<dep2.Foo>; }) error uint32;
    };
    "#
        .to_string(),
    );
    dep1.add_source(&dep1_source);
    let _dep1_root = dep1.compile().expect("dep1 compilation failed");
    let mut lib = TestLibrary::new();
    lib.add_source(&dep2_source);
    lib.add_source(&dep1_source);
    let lib_source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    using dep1;
    protocol CapturesDependencyThroughCompose {
    compose dep1.ComposedProtocol;
    };
    "#
        .to_string(),
    );
    lib.add_source(&lib_source);
    let root = lib.compile().expect("lib compilation failed");
    let expected: Vec<&str> = vec!["dep1", "dep2"];
    assert_eq!(direct_and_composed_dependencies(&root), expected);
}

#[test]
fn good_error_syntax_error_type() {
    let mut dep2 = TestLibrary::new();
    let dep2_source = SourceFile::new(
        "dep2.fidl".to_string(),
        r#"
    library dep2;
    type Foo = flexible enum : uint32 {};
    "#
        .to_string(),
    );
    dep2.add_source(&dep2_source);
    let _dep2_root = dep2.compile().expect("dep2 compilation failed");
    let mut dep1 = TestLibrary::new();
    dep1.add_source(&dep2_source);
    let dep1_source = SourceFile::new(
        "dep1.fidl".to_string(),
        r#"
    library dep1;
    using dep2;
    protocol ComposedProtocol {
    UsesDep2InErrorType() -> () error dep2.Foo;
    };
    "#
        .to_string(),
    );
    dep1.add_source(&dep1_source);
    let _dep1_root = dep1.compile().expect("dep1 compilation failed");
    let mut lib = TestLibrary::new();
    lib.add_source(&dep2_source);
    lib.add_source(&dep1_source);
    let lib_source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    using dep1;
    protocol CapturesDependencyThroughCompose {
    compose dep1.ComposedProtocol;
    };
    "#
        .to_string(),
    );
    lib.add_source(&lib_source);
    let root = lib.compile().expect("lib compilation failed");
    let expected: Vec<&str> = vec!["dep1", "dep2"];
    assert_eq!(direct_and_composed_dependencies(&root), expected);
}

#[test]
fn good_flexible_response() {
    let mut dep2 = TestLibrary::new();
    let dep2_source = SourceFile::new(
        "dep2.fidl".to_string(),
        r#"
    library dep2;
    type Foo = struct {};
    "#
        .to_string(),
    );
    dep2.add_source(&dep2_source);
    let _dep2_root = dep2.compile().expect("dep2 compilation failed");
    let mut dep1 = TestLibrary::new();
    dep1.add_source(&dep2_source);
    let dep1_source = SourceFile::new(
        "dep1.fidl".to_string(),
        r#"
    library dep1;
    using dep2;
    open protocol ComposedProtocol {
    flexible UsesDep2InFlexibleResponse() -> (struct { foo vector<dep2.Foo>; });
    };
    "#
        .to_string(),
    );
    dep1.add_source(&dep1_source);
    let _dep1_root = dep1.compile().expect("dep1 compilation failed");
    let mut lib = TestLibrary::new();
    lib.add_source(&dep2_source);
    lib.add_source(&dep1_source);
    let lib_source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
    library example;
    using dep1;
    open protocol CapturesDependencyThroughCompose {
    compose dep1.ComposedProtocol;
    };
    "#
        .to_string(),
    );
    lib.add_source(&lib_source);
    let root = lib.compile().expect("lib compilation failed");
    let expected: Vec<&str> = vec!["dep1", "dep2"];
    assert_eq!(direct_and_composed_dependencies(&root), expected);
}
