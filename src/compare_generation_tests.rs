#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::process::Command;

    #[test]
    fn test_compare_generation() {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let goldens_dir = manifest_dir.join("goldens");
        fs::create_dir_all(&goldens_dir).unwrap();

        let status = Command::new("cargo")
            .arg("build")
            .arg("--bin")
            .arg("fidlcrs")
            .status()
            .expect("Failed to run cargo build");
        assert!(status.success(), "cargo build failed");

        let fidlcrs_bin = manifest_dir.join("target/debug/fidlcrs");

        struct TestCase {
            filename: &'static str,
            experimental_flags: Vec<&'static str>,
            available: Option<&'static str>,
            public_deps: Vec<&'static str>,
            contains_drivers: bool,
        }

        impl TestCase {
            fn new(filename: &'static str) -> Self {
                Self {
                    filename,
                    experimental_flags: vec![],
                    available: None,
                    public_deps: vec![],
                    contains_drivers: false,
                }
            }
            fn experimental(mut self, flag: &'static str) -> Self {
                self.experimental_flags.push(flag);
                self
            }
            fn available(mut self, avail: &'static str) -> Self {
                self.available = Some(avail);
                self
            }
            fn public_dep(mut self, dep: &'static str) -> Self {
                self.public_deps.push(dep);
                self
            }
            fn contains_drivers(mut self) -> Self {
                self.contains_drivers = true;
                self
            }
        }

        let active_tests = vec![
            TestCase::new("anonymous.test.fidl"),
            TestCase::new("arrays.test.fidl"),
            TestCase::new("byte_and_bytes.test.fidl"),
            TestCase::new("bits.test.fidl"),
            TestCase::new("doc_comments.test.fidl"),
            TestCase::new("empty_struct.test.fidl"),
            TestCase::new("encapsulated_structs.test.fidl"),
            TestCase::new("enum.test.fidl"),
            TestCase::new("error.test.fidl"),
            TestCase::new("escaping.test.fidl"),
            TestCase::new("nullable.test.fidl"),
            TestCase::new("padding.test.fidl"),
            TestCase::new("protocol_request.test.fidl"),
            TestCase::new("protocols.test.fidl"),
            TestCase::new("request_flexible_envelope.test.fidl"),
            TestCase::new("serializable.test.fidl"),
            TestCase::new("service.test.fidl"),
            TestCase::new("struct.test.fidl"),
            TestCase::new("table.test.fidl"),
            TestCase::new("union_sandwich.test.fidl"),
            TestCase::new("union.test.fidl"),
            TestCase::new("vectors.test.fidl"),
            TestCase::new("driver_one_way.test.fidl").contains_drivers(),
            TestCase::new("driver_two_way.test.fidl").contains_drivers(),
            TestCase::new("inheritance.test.fidl"),
            TestCase::new("inheritance_with_recursive_decl.test.fidl"),
            TestCase::new("time.test.fidl"),
        ];

        let disabled_tests = vec![
            TestCase::new("bits_constants.test.fidl"),
            TestCase::new("constants.test.fidl"),
            TestCase::new("consts.test.fidl"),
            TestCase::new("driver_handle.test.fidl").contains_drivers(),
            TestCase::new("driver_service.test.fidl").contains_drivers(),
            TestCase::new("experimental_maybe_from_alias.test.fidl"),
            TestCase::new("experimental_zx_c_types.test.fidl").experimental("zx_c_types"),
            TestCase::new("handles_in_types.test.fidl"),
            TestCase::new("handles.test.fidl")
                .public_dep("//sdk/fidl/fdf")
                .contains_drivers(),
            TestCase::new("new_type.test.fidl").experimental("allow_new_types"),
            TestCase::new("overlay.test.fidl").experimental("zx_c_types"),
            TestCase::new("string_arrays.test.fidl").experimental("zx_c_types"),
            TestCase::new("types_in_protocols.test.fidl"),
            TestCase::new("unknown_interactions.test.fidl").contains_drivers(),
            TestCase::new("versions.test.fidl").available("test:HEAD"),
        ];

        let mut failed = false;

        let check_test = |tc: &TestCase, expect_match: bool| -> bool {
            let file = tc.filename;
            let name = file.strip_suffix(".test.fidl").unwrap();

            let output_json = goldens_dir.join(format!("{}.json", name));

            let mut cmd = Command::new(&fidlcrs_bin);
            cmd.current_dir(&manifest_dir);
            cmd.arg("--json").arg(&output_json);

            for flag in &tc.experimental_flags {
                cmd.arg("--experimental").arg(flag);
            }
            let available = tc.available.unwrap_or("fuchsia:42,NEXT,HEAD");
            if !available.is_empty() {
                cmd.arg("--available").arg(available);
            }

            cmd.arg("--files");
            cmd.arg("vdso-fidl/rights.fidl");
            cmd.arg("vdso-fidl/zx_common.fidl");
            cmd.arg("vdso-fidl/overview.fidl");

            cmd.arg("--files").arg(format!("fidlc/testdata/{}", file));

            let output = cmd.output().unwrap();
            if !output.status.success() {
                println!("fidlcrs failed for {}:", file);
                println!("{}", String::from_utf8_lossy(&output.stderr));
                return false;
            }

            let expected_path = manifest_dir.join(format!("fidlc/goldens/{}.json.golden", name));
            let expected_json = fs::read_to_string(&expected_path).unwrap_or_default();
            let actual_json = fs::read_to_string(&output_json).unwrap_or_default();

            if expect_match {
                if expected_json != actual_json {
                    println!("Golden mismatch for {}", file);
                    let diff_output = Command::new("diff")
                        .arg("-u")
                        .arg(&expected_path)
                        .arg(&output_json)
                        .output()
                        .unwrap();
                    println!("{}", String::from_utf8_lossy(&diff_output.stdout));
                    return false;
                }
            } else {
                if expected_json == actual_json {
                    println!(
                        "Error: Disabled test {} matches golden output. It should be moved from disabled_tests to active_tests.",
                        file
                    );
                    return false;
                }
            }
            true
        };

        for tc in active_tests {
            if !check_test(&tc, true) {
                failed = true;
            }
        }

        for tc in disabled_tests {
            if !check_test(&tc, false) {
                failed = true;
            }
        }

        assert!(
            !failed,
            "One or more golden tests failed. See output above."
        );
    }
}
