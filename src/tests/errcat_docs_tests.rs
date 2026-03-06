#[test]
#[ignore]
fn index_is_complete() {
    // Tests that all errors are present in errcat.md
    unimplemented!("Need to implement kAllDiagnosticDefs and index validation");
}

#[test]
#[ignore]
fn redirects_are_complete() {
    // Tests that _redirects.yaml has redirects for all of them
    unimplemented!("Need to implement kAllDiagnosticDefs and _redirects.yaml validation");
}

#[test]
#[ignore]
fn markdown_files_exist() {
    // Tests that there's a file error-catalog/_<id>.md for each definition
    unimplemented!("Need to implement kAllDiagnosticDefs and Markdown existence check");
}

#[test]
#[ignore]
fn docs_are_accurate() {
    // Tests the markdown files themselves for the ## <id>: title
    unimplemented!("Need to implement kAllDiagnosticDefs and Markdown documentation checks");
}

#[test]
#[ignore]
fn all_good_files_are_tested() {
    // Checks errcat_good_tests.cc to see if good cases are tested
    unimplemented!("Need to implement kAllDiagnosticDefs and errcat_good_tests validation");
}

#[test]
#[ignore]
fn errors_are_tested_iff_documented_and_not_retired() {
    // Checks errcat_good_tests.cc to see if testing status matches documentation
    unimplemented!("Need to implement kAllDiagnosticDefs and testing status validation");
}
