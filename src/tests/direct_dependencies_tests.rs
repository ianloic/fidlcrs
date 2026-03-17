use crate::flat_ast::Root;
use crate::tests::test_library::TestLibrary;

fn direct_and_composed_dependencies(root: &Root) -> Vec<String> {
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

        dep2.add_source_file(
            "dep2.fidl",
            r#"
library dep2;

const Constant uint32 = 50;
type Type = struct {};
protocol Protocol {};
"#,
        );
        let _dep2_root = dep2.compile().expect("dep2 compilation failed");

        let mut dep1 = TestLibrary::new();

        dep1.add_dependency_file(
            "dep2.fidl",
            r#"
library dep2;

const Constant uint32 = 50;
type Type = struct {};
protocol Protocol {};
"#,
        );
        dep1.add_source_file(
            "dep1.fidl",
            &(format!(
                r#"
library dep1;

using dep2;

protocol ComposedProtocol {{
  UsesDep2(resource struct {{ data {}; }});
}};
"#,
                type_usage
            )),
        );
        let _dep1_root = dep1.compile().expect("dep1 compilation failed");

        let mut lib = TestLibrary::new();

        lib.add_dependency_file(
            "dep2.fidl",
            r#"
library dep2;

const Constant uint32 = 50;
type Type = struct {};
protocol Protocol {};
"#,
        );
        lib.add_dependency_file(
            "dep1.fidl",
            &(format!(
                r#"
library dep1;

using dep2;

protocol ComposedProtocol {{
  UsesDep2(resource struct {{ data {}; }});
}};
"#,
                type_usage
            )),
        );
        lib.add_source_file(
            "example.fidl",
            r#"
library example;

using dep1;

protocol CapturesDependencyThroughCompose {
  compose dep1.ComposedProtocol;
};
"#,
        );
        let root = lib.compile().expect("lib compilation failed");

        let expected: Vec<&str> = vec!["dep1", "dep2"];
        assert_eq!(direct_and_composed_dependencies(&root), expected);
    }
}

#[test]
fn good_does_not_follow_alias() {
    let mut dep2 = TestLibrary::new();

    dep2.add_source_file(
        "dep2.fidl",
        r#"
    library dep2;
    type Foo = struct {};
    "#,
    );
    let _dep2_root = dep2.compile().expect("dep2 compilation failed");
    let mut dep1 = TestLibrary::new();
    dep1.add_dependency_file(
        "dep2.fidl",
        r#"
    library dep2;
    type Foo = struct {};
    "#,
    );

    dep1.add_source_file(
        "dep1.fidl",
        r#"
    library dep1;
    using dep2;
    alias Bar = dep2.Foo;
    protocol ComposedProtocol {
    UsesDep2InAlias(struct { foo vector<Bar>; });
    };
    "#,
    );
    let _dep1_root = dep1.compile().expect("dep1 compilation failed");
    let mut lib = TestLibrary::new();
    lib.add_dependency_file(
        "dep2.fidl",
        r#"
    library dep2;
    type Foo = struct {};
    "#,
    );
    lib.add_dependency_file(
        "dep1.fidl",
        r#"
    library dep1;
    using dep2;
    alias Bar = dep2.Foo;
    protocol ComposedProtocol {
    UsesDep2InAlias(struct { foo vector<Bar>; });
    };
    "#,
    );

    lib.add_source_file(
        "example.fidl",
        r#"
    library example;
    using dep1;
    protocol CapturesDependencyThroughCompose {
    compose dep1.ComposedProtocol;
    };
    "#,
    );
    let root = lib.compile().expect("lib compilation failed");
    let expected: Vec<&str> = vec!["dep1"];
    assert_eq!(direct_and_composed_dependencies(&root), expected);
}

#[test]
fn good_does_not_follow_nested_struct() {
    let mut dep2 = TestLibrary::new();

    dep2.add_source_file(
        "dep2.fidl",
        r#"
    library dep2;
    type Foo = struct {};
    "#,
    );
    let _dep2_root = dep2.compile().expect("dep2 compilation failed");
    let mut dep1 = TestLibrary::new();
    dep1.add_dependency_file(
        "dep2.fidl",
        r#"
    library dep2;
    type Foo = struct {};
    "#,
    );

    dep1.add_source_file(
        "dep1.fidl",
        r#"
    library dep1;
    using dep2;
    type Bar = struct {
    foo dep2.Foo;
    };
    protocol ComposedProtocol {
    UsesDep2InNestedStruct(struct { foo vector<Bar>; });
    };
    "#,
    );
    let _dep1_root = dep1.compile().expect("dep1 compilation failed");
    let mut lib = TestLibrary::new();
    lib.add_dependency_file(
        "dep2.fidl",
        r#"
    library dep2;
    type Foo = struct {};
    "#,
    );
    lib.add_dependency_file(
        "dep1.fidl",
        r#"
    library dep1;
    using dep2;
    type Bar = struct {
    foo dep2.Foo;
    };
    protocol ComposedProtocol {
    UsesDep2InNestedStruct(struct { foo vector<Bar>; });
    };
    "#,
    );

    lib.add_source_file(
        "example.fidl",
        r#"
    library example;
    using dep1;
    protocol CapturesDependencyThroughCompose {
    compose dep1.ComposedProtocol;
    };
    "#,
    );
    let root = lib.compile().expect("lib compilation failed");
    let expected: Vec<&str> = vec!["dep1"];
    assert_eq!(direct_and_composed_dependencies(&root), expected);
}

#[test]
fn good_error_syntax_success_type() {
    let mut dep2 = TestLibrary::new();

    dep2.add_source_file(
        "dep2.fidl",
        r#"
    library dep2;
    type Foo = struct {};
    "#,
    );
    let _dep2_root = dep2.compile().expect("dep2 compilation failed");
    let mut dep1 = TestLibrary::new();
    dep1.add_dependency_file(
        "dep2.fidl",
        r#"
    library dep2;
    type Foo = struct {};
    "#,
    );

    dep1.add_source_file(
        "dep1.fidl",
        r#"
    library dep1;
    using dep2;
    protocol ComposedProtocol {
    UsesDep2InSuccessType() -> (struct { foo vector<dep2.Foo>; }) error uint32;
    };
    "#,
    );
    let _dep1_root = dep1.compile().expect("dep1 compilation failed");
    let mut lib = TestLibrary::new();
    lib.add_dependency_file(
        "dep2.fidl",
        r#"
    library dep2;
    type Foo = struct {};
    "#,
    );
    lib.add_dependency_file(
        "dep1.fidl",
        r#"
    library dep1;
    using dep2;
    protocol ComposedProtocol {
    UsesDep2InSuccessType() -> (struct { foo vector<dep2.Foo>; }) error uint32;
    };
    "#,
    );

    lib.add_source_file(
        "example.fidl",
        r#"
    library example;
    using dep1;
    protocol CapturesDependencyThroughCompose {
    compose dep1.ComposedProtocol;
    };
    "#,
    );
    let root = lib.compile().expect("lib compilation failed");
    let expected: Vec<&str> = vec!["dep1", "dep2"];
    assert_eq!(direct_and_composed_dependencies(&root), expected);
}

#[test]
fn good_error_syntax_error_type() {
    let mut dep2 = TestLibrary::new();

    dep2.add_source_file(
        "dep2.fidl",
        r#"
    library dep2;
    type Foo = flexible enum : uint32 {};
    "#,
    );
    let _dep2_root = dep2.compile().expect("dep2 compilation failed");
    let mut dep1 = TestLibrary::new();
    dep1.add_dependency_file(
        "dep2.fidl",
        r#"
    library dep2;
    type Foo = flexible enum : uint32 {};
    "#,
    );

    dep1.add_source_file(
        "dep1.fidl",
        r#"
    library dep1;
    using dep2;
    protocol ComposedProtocol {
    UsesDep2InErrorType() -> () error dep2.Foo;
    };
    "#,
    );
    let _dep1_root = dep1.compile().expect("dep1 compilation failed");
    let mut lib = TestLibrary::new();
    lib.add_dependency_file(
        "dep2.fidl",
        r#"
    library dep2;
    type Foo = flexible enum : uint32 {};
    "#,
    );
    lib.add_dependency_file(
        "dep1.fidl",
        r#"
    library dep1;
    using dep2;
    protocol ComposedProtocol {
    UsesDep2InErrorType() -> () error dep2.Foo;
    };
    "#,
    );

    lib.add_source_file(
        "example.fidl",
        r#"
    library example;
    using dep1;
    protocol CapturesDependencyThroughCompose {
    compose dep1.ComposedProtocol;
    };
    "#,
    );
    let root = lib.compile().expect("lib compilation failed");
    let expected: Vec<&str> = vec!["dep1", "dep2"];
    assert_eq!(direct_and_composed_dependencies(&root), expected);
}

#[test]
fn good_flexible_response() {
    let mut dep2 = TestLibrary::new();

    dep2.add_source_file(
        "dep2.fidl",
        r#"
    library dep2;
    type Foo = struct {};
    "#,
    );
    let _dep2_root = dep2.compile().expect("dep2 compilation failed");
    let mut dep1 = TestLibrary::new();
    dep1.add_dependency_file(
        "dep2.fidl",
        r#"
    library dep2;
    type Foo = struct {};
    "#,
    );

    dep1.add_source_file(
        "dep1.fidl",
        r#"
    library dep1;
    using dep2;
    open protocol ComposedProtocol {
    flexible UsesDep2InFlexibleResponse() -> (struct { foo vector<dep2.Foo>; });
    };
    "#,
    );
    let _dep1_root = dep1.compile().expect("dep1 compilation failed");
    let mut lib = TestLibrary::new();
    lib.add_dependency_file(
        "dep2.fidl",
        r#"
    library dep2;
    type Foo = struct {};
    "#,
    );
    lib.add_dependency_file(
        "dep1.fidl",
        r#"
    library dep1;
    using dep2;
    open protocol ComposedProtocol {
    flexible UsesDep2InFlexibleResponse() -> (struct { foo vector<dep2.Foo>; });
    };
    "#,
    );

    lib.add_source_file(
        "example.fidl",
        r#"
    library example;
    using dep1;
    open protocol CapturesDependencyThroughCompose {
    compose dep1.ComposedProtocol;
    };
    "#,
    );
    let root = lib.compile().expect("lib compilation failed");
    let expected: Vec<&str> = vec!["dep1", "dep2"];
    assert_eq!(direct_and_composed_dependencies(&root), expected);
}
