use crate::cli::{Cli, run};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_unused_library_provided_via_files() {
    let dir = tempdir().unwrap();
    let main_path = dir.path().join("main.fidl");
    let dep_path = dir.path().join("dep.fidl");
    fs::write(&main_path, "library main;").unwrap();
    fs::write(&dep_path, "library dep;").unwrap();

    let cli = Cli {
        ..Default::default()
    };
    let source_managers = vec![
        vec![dep_path.to_str().unwrap().to_string()],
        vec![main_path.to_str().unwrap().to_string()],
    ];

    let result = run(&cli, &source_managers);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Unused libraries provided via --files: dep"));
}

#[test]
fn test_used_library_provided_via_files() {
    let dir = tempdir().unwrap();
    let main_path = dir.path().join("main.fidl");
    let dep_path = dir.path().join("dep.fidl");
    fs::write(
        &main_path,
        "library main; using dep; type Foo = struct { x dep.Type; };",
    )
    .unwrap();
    fs::write(&dep_path, "library dep; type Type = struct {};").unwrap();

    let cli = Cli {
        ..Default::default()
    };
    let source_managers = vec![
        vec![dep_path.to_str().unwrap().to_string()],
        vec![main_path.to_str().unwrap().to_string()],
    ];

    let result = run(&cli, &source_managers);
    assert!(result.is_ok());
}

#[test]
fn test_zx_library_is_ignored() {
    let dir = tempdir().unwrap();
    let main_path = dir.path().join("main.fidl");
    let zx_path = dir.path().join("zx.fidl");
    fs::write(&main_path, "library main;").unwrap();
    fs::write(&zx_path, "library zx;").unwrap();

    let cli = Cli {
        ..Default::default()
    };
    let source_managers = vec![
        vec![zx_path.to_str().unwrap().to_string()],
        vec![main_path.to_str().unwrap().to_string()],
    ];

    let result = run(&cli, &source_managers);
    // zx shouldn't trigger unused library error
    assert!(result.is_ok());
}
