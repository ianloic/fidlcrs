use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

use crate::diagnostics::{Error, ErrorKind, all_errors};

fn test_file_path(filename: &str) -> PathBuf {
    let fuchsia_dir = std::env::var("FUCHSIA_DIR").expect("FUCHSIA_DIR must be set");
    if filename.starts_with("error-catalog/") || filename == "errcat.md" {
        PathBuf::from(fuchsia_dir)
            .join("docs/reference/fidl/language")
            .join(filename)
    } else if filename == "_redirects.yaml" {
        PathBuf::from(fuchsia_dir).join("docs/_redirects.yaml")
    } else {
        // Defaults to fidlc/tests/
        PathBuf::from(fuchsia_dir)
            .join("tools/fidl/fidlc/tests")
            .join(filename)
    }
}

fn read_file(path: &Path) -> Option<String> {
    fs::read_to_string(path).ok()
}

fn good_fidl_paths(def: &Error) -> Option<Vec<String>> {
    let id = def.format_id();
    let path = test_file_path(&format!("error-catalog/_{}.md", id));
    let content = read_file(&path)?;

    let pattern = Regex::new(
        r#"gerrit_path="tools/fidl/fidlc/tests/fidl/(good/fi-(\d+)(?:-[a-z])?\.test\.fidl)""#,
    )
    .unwrap();
    let mut result = Vec::new();

    for cap in pattern.captures_iter(&content) {
        let group1 = &cap[1];
        let group2 = &cap[2];
        let parsed_id: usize = group2.parse().unwrap();
        assert_eq!(
            parsed_id,
            def.id(),
            "{:?} references {} which is for the wrong error ID",
            path,
            group1
        );
        result.push(group1.to_string());
    }

    assert!(
        !result.is_empty(),
        "{:?} does not reference any good .test.fidl files",
        path
    );
    Some(result)
}

#[test]
#[ignore]
fn index_is_complete() {
    let path = test_file_path("errcat.md");
    let content = match read_file(&path) {
        Some(c) => c,
        None => return, // Ignore if missing
    };

    let errors_vec = all_errors();
    let mut errors_iter = errors_vec.iter().filter(|e| e.documented()).peekable();
    let prefix = "<<error-catalog/";

    for line in content.lines() {
        if line.starts_with(prefix) {
            let def = errors_iter.next().expect("unexpected entry in errcat.md; did not contain all diagnostics or missing elements");
            let id = def.format_id();
            assert_eq!(
                line,
                format!("<<error-catalog/_{}.md>>", id),
                "unexpected entry in {:?}; either {} was not next in sequence, or it is marked documented=false",
                path,
                id
            );
        }
    }

    if let Some(missing) = errors_iter.next() {
        panic!(
            "{:?} did not contain all diagnostics; missing {}",
            path,
            missing.format_id()
        );
    }
}

#[test]
#[ignore]
fn redirects_are_complete() {
    let path = test_file_path("_redirects.yaml");
    let redirects = match read_file(&path) {
        Some(c) => c,
        None => return,
    };

    for def in all_errors() {
        let id = def.format_id();
        let entry = format!(
            "- from: /fuchsia-src/error/{}\n  to: /fuchsia-src/reference/fidl/language/errcat.md#{}",
            id, id
        );

        if def.documented() {
            assert!(
                redirects.contains(&entry),
                "{:?} is missing a redirect for {}",
                path,
                id
            );
        } else {
            assert!(
                !redirects.contains(&entry),
                "{:?} unexpectedly has a redirect for {}, which is marked documented=false",
                path,
                id
            );
        }
    }
}

#[test]
#[ignore]
fn markdown_files_exist() {
    for def in all_errors() {
        if !def.documented() {
            continue;
        }
        let id = def.format_id();
        let path = test_file_path(&format!("error-catalog/_{}.md", id));
        assert!(path.exists(), "missing Markdown file {:?} for {}", path, id);
    }
}

#[test]

fn docs_are_accurate() {
    for def in all_errors() {
        if !def.documented() {
            continue;
        }
        let id = def.format_id();
        let path = test_file_path(&format!("error-catalog/_{}.md", id));
        let content = match read_file(&path) {
            Some(c) => c,
            None => continue, // Caught by markdown_files_exist test
        };

        if def.kind() == ErrorKind::Retired {
            let prefix = format!("## {} {{:#{} .hide-from-toc}}", id, id);
            assert!(
                content.starts_with(&prefix),
                "first line of {:?} is incorrect",
                path
            );
            assert!(
                content.contains("Deprecated: This error code has been retired."),
                "{} is Retired, but the Markdown file does not say it is retired: {:?}",
                id,
                path
            );
        } else {
            let prefix = format!("## {}:", id);
            assert!(
                content.starts_with(&prefix),
                "first line of {:?} is incorrect",
                path
            );
            assert!(
                content.contains(&format!("{{:#{}}}", id)),
                "missing the expected heading id attribute in {:?}",
                path
            );
        }
    }
}

#[test]

fn all_good_files_are_tested() {
    let path = test_file_path("errcat_good_tests.cc");
    let source_file = match read_file(&path) {
        Some(c) => c,
        None => return,
    };

    for def in all_errors() {
        if !def.documented() || def.kind() == ErrorKind::Retired {
            continue;
        }
        if def.id() == Error::ErrGeneratedZeroValueOrdinal.id() {
            continue; // This error has no examples because it is impossible to test.
        }

        let fidl_paths = match good_fidl_paths(&def) {
            Some(p) => p,
            None => continue,
        };

        for fidl in fidl_paths {
            let target = format!("\"{}\"", fidl);
            assert!(
                source_file.contains(&target),
                "{:?} does not contain a test for {}",
                path,
                fidl
            );
        }
    }
}

#[test]
#[ignore]
fn errors_are_tested_iff_documented_and_not_retired() {
    let path = test_file_path("errcat_good_tests.cc");
    let source_file = match read_file(&path) {
        Some(c) => c,
        None => return,
    };

    for def in all_errors() {
        if def.id() == Error::ErrGeneratedZeroValueOrdinal.id() {
            continue;
        }

        let id = def.format_id();
        let target = format!("\"good/{}", id);
        let tested = source_file.contains(&target);

        if def.documented() && def.kind() != ErrorKind::Retired {
            assert!(
                tested,
                "{} (documented=true, retired=false) is missing a test in {:?}",
                id, path
            );
        } else if !def.documented() {
            assert!(!tested, "{} (documented=false) unexpectedly has a test", id);
        } else {
            assert!(!tested, "{} (retired=true) unexpectedly has a test", id);
        }
    }
}
