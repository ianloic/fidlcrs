
use crate::source_file::SourceFile;
use crate::tests::test_library::TestLibrary;
use std::collections::HashMap;

fn mangle(input: &str) -> String {
    let mut letters: HashMap<String, char> = HashMap::new();
    let mut current_char = b'A';
    let mut result = input.to_string();

    while let Some(start) = result.find('#') {
        if let Some(mut end) = result[start + 1..].find('#') {
            end += start + 1;
            let key = &result[start + 1..end];
            let key_str = key.to_string();
            let letter = if let Some(&c) = letters.get(&key_str) {
                c
            } else {
                let c = current_char as char;
                letters.insert(key_str.clone(), c);
                current_char += 1;
                c
            };
            let replacement = format!("{}__{}", letter, key_str);
            result.replace_range(start..=end, &replacement);
        } else {
            break;
        }
    }
    result
}

fn unmangle_decls(decls: &[String]) -> Vec<String> {
    let mut result = Vec::new();
    for decl in decls {
        let name_without_lib = if let Some(idx) = decl.find('/') {
            &decl[idx + 1..]
        } else {
            decl.as_str()
        };

        if let Some(idx) = name_without_lib.find("__") {
            assert_eq!(idx, 1, "{}", name_without_lib);
            result.push(name_without_lib[idx + 2..].to_string());
        } else if name_without_lib.len() >= 2
            && name_without_lib.chars().nth(0).unwrap().is_uppercase()
            && name_without_lib.chars().nth(1).unwrap().is_uppercase()
        {
            // Heuristic: NamingContext consumes `__`, leaving `AProtocol`.
            // If we see two uppercase letters start, strip the first one (the prefix).
            result.push(name_without_lib[1..].to_string());
        } else {
            // Anonymous types and other things without __ prefix
            result.push(name_without_lib.to_string());
        }
    }
    result
}

fn is_union_of(actual: &[String], suborders: &[Vec<&str>]) -> bool {
    let mut used = vec![false; actual.len()];
    for suborder in suborders {
        let mut prev_index = None;
        for &name in suborder {
            if let Some(index) = actual.iter().position(|x| x == name) {
                if used[index] {
                    println!("'{}' used twice", name);
                    return false;
                }
                used[index] = true;
                if let Some(prev) = prev_index {
                    if index < prev {
                        println!("'{}' came before previous item in suborder", name);
                        return false;
                    }
                }
                prev_index = Some(index);
            } else {
                println!("'{}' not found in actual: {:?}", name, actual);
                return false;
            }
        }
    }
    for (i, is_used) in used.iter().enumerate() {
        if !is_used {
            println!("unexpected '{}' in actual sequence", actual[i]);
            return false;
        }
    }
    true
}

const REPEAT_TEST_COUNT: usize = 10;

#[test]
fn good_no_unused_anonymous_names() {
    for _ in 0..REPEAT_TEST_COUNT {
        let source = mangle(
            r#"
library example;

protocol #Protocol# {
    strict Method() -> ();
};
"#,
        );
        let source_file = SourceFile::new("example.fidl".to_string(), source);
        let library = TestLibrary::with_source(&source_file);
        let root = library.compile().unwrap();
        let expected = vec!["Protocol"];
        assert_eq!(unmangle_decls(&root.declaration_order), expected);
    }
}

#[test]
fn good_nonnullable_ref() {
    for _ in 0..REPEAT_TEST_COUNT {
        let source = mangle(
            r#"
library example;

type #Request# = struct {
  req array<#Element#, 4>;
};

type #Element# = struct {};

protocol #Protocol# {
  SomeMethod(struct { req #Request#; });
};
"#,
        );
        let source_file = SourceFile::new("example.fidl".to_string(), source);
        let library = TestLibrary::with_source(&source_file);
        let root = library.compile().unwrap();
        let expected = vec![
            "Element",
            "Request",
            "ProtocolSomeMethodRequest",
            "Protocol",
        ];
        assert_eq!(unmangle_decls(&root.declaration_order), expected);
    }
}

#[test]
fn good_nullable_ref_breaks_dependency() {
    for _ in 0..REPEAT_TEST_COUNT {
        let source = mangle(
            r#"
library example;

type #Request# = resource struct {
  req array<box<#Element#>, 4>;
};

type #Element# = resource struct {
  prot client_end:#Protocol#;
};

protocol #Protocol# {
  SomeMethod(resource struct { req #Request#; });
};
"#,
        );
        let source_file = SourceFile::new("example.fidl".to_string(), source);
        let library = TestLibrary::with_source(&source_file);
        let root = library.compile().unwrap();
        let expected_suborders = vec![
            vec!["Element"],
            vec!["Request", "ProtocolSomeMethodRequest", "Protocol"],
        ];
        assert!(is_union_of(
            &unmangle_decls(&root.declaration_order),
            &expected_suborders
        ));
    }
}

#[test]
fn good_request_type_breaks_dependency_graph() {
    for _ in 0..REPEAT_TEST_COUNT {
        let source = mangle(
            r#"
library example;

type #Request# = resource struct {
  req server_end:#Protocol#;
};

protocol #Protocol# {
  SomeMethod(resource struct { req #Request#; });
};
"#,
        );
        let source_file = SourceFile::new("example.fidl".to_string(), source);
        let library = TestLibrary::with_source(&source_file);
        let root = library.compile().unwrap();
        let expected = vec!["Request", "ProtocolSomeMethodRequest", "Protocol"];
        assert_eq!(unmangle_decls(&root.declaration_order), expected);
    }
}

#[test]
fn good_nonnullable_union() {
    for _ in 0..REPEAT_TEST_COUNT {
        let source = mangle(
            r#"
library example;

type #Union# = resource union {
  1: req server_end:#Protocol#;
  2: foo #Payload#;
};

protocol #Protocol# {
  SomeMethod(resource struct { req #Union#; });
};

type #Payload# = struct {
  a int32;
};
"#,
        );
        let source_file = SourceFile::new("example.fidl".to_string(), source);
        let library = TestLibrary::with_source(&source_file);
        let root = library.compile().unwrap();
        let expected = vec!["Payload", "Union", "ProtocolSomeMethodRequest", "Protocol"];
        assert_eq!(unmangle_decls(&root.declaration_order), expected);
    }
}

#[test]
fn good_nullable_union() {
    for _ in 0..REPEAT_TEST_COUNT {
        let source = mangle(
            r#"
library example;

type #Union# = resource union {
  1: req server_end:#Protocol#;
  2: foo #Payload#;
};

protocol #Protocol# {
  SomeMethod(resource struct { req #Union#:optional; });
};

type #Payload# = struct {
  a int32;
};
"#,
        );
        let source_file = SourceFile::new("example.fidl".to_string(), source);
        let library = TestLibrary::with_source(&source_file);
        let root = library.compile().unwrap();
        let expected_suborders = vec![
            vec!["Payload", "Union"],
            vec!["ProtocolSomeMethodRequest", "Protocol"],
        ];
        assert!(is_union_of(
            &unmangle_decls(&root.declaration_order),
            &expected_suborders
        ));
    }
}

#[test]
fn good_nonnullable_union_in_struct() {
    for _ in 0..REPEAT_TEST_COUNT {
        let source = mangle(
            r#"
library example;

type #Payload# = struct {
  a int32;
};

protocol #Protocol# {
  SomeMethod(struct { req #Request#; });
};

type #Request# = struct {
  u #Union#;
};

type #Union# = union {
  1: foo #Payload#;
};
"#,
        );
        let source_file = SourceFile::new("example.fidl".to_string(), source);
        let library = TestLibrary::with_source(&source_file);
        let root = library.compile().unwrap();
        let expected = vec![
            "Payload",
            "Union",
            "Request",
            "ProtocolSomeMethodRequest",
            "Protocol",
        ];
        assert_eq!(unmangle_decls(&root.declaration_order), expected);
    }
}

#[test]
fn good_nullable_union_in_struct() {
    for _ in 0..REPEAT_TEST_COUNT {
        let source = mangle(
            r#"
library example;

type #Payload# = struct {
  a int32;
};

protocol #Protocol# {
  SomeMethod(struct { req #Request#; });
};

type #Request# = struct {
  u #Union#:optional;
};

type #Union# = union {
  1: foo #Payload#;
};
"#,
        );
        let source_file = SourceFile::new("example.fidl".to_string(), source);
        let library = TestLibrary::with_source(&source_file);
        let root = library.compile().unwrap();
        let expected_suborders = vec![
            vec!["Payload", "Union"],
            vec!["Request", "ProtocolSomeMethodRequest", "Protocol"],
        ];
        assert!(is_union_of(
            &unmangle_decls(&root.declaration_order),
            &expected_suborders
        ));
    }
}

#[test]
fn good_multiple_libraries() {
    for _ in 0..REPEAT_TEST_COUNT {
        let sources = mangle(
            r#"
// dependency.fidl
library dependency;

type #Decl1# = struct {};

// example.fidl
library example;

using dependency;

type #Decl0# = struct {};
type #Decl2# = struct {};

protocol #Decl1# {
  Method(struct { arg dependency.#Decl1#; });
};
"#,
        );
        // We need to split this into two source files.
        let idx = sources.find("// example.fidl").unwrap();
        let source_dep = &sources[..idx];
        let source_ex = &sources[idx..];

        let source_file_dep =
            SourceFile::new("dependency.fidl".to_string(), source_dep.to_string());
        let source_file_ex = SourceFile::new("example.fidl".to_string(), source_ex.to_string());

        let dependency = TestLibrary::with_source(&source_file_dep);
        let root_dep = dependency.compile().unwrap();

        let mut library = TestLibrary::new();
        // TODO: How does TestLibrary handle multiple libraries?
        // Currently, TestLibrary compiles everything into one JSON root.
        // Let's add both sources to `library`.
        library.add_source(&source_file_dep);
        library.add_source(&source_file_ex);

        let root = library.compile().unwrap();

        // Check dependency
        let expected_dep = vec!["Decl1"];
        // Wait, root_dep is from just compiling the first file.
        assert_eq!(unmangle_decls(&root_dep.declaration_order), expected_dep);

        // Check example
        let expected_suborders = vec![
            vec!["Decl0"],
            vec!["Decl2"],
            vec!["Decl1MethodRequest", "Decl1"],
        ];

        // Note: `unmangle_decls(&root.declaration_order)` will contain `Decl1` (from dependency) as well,
        // since fidlcrs might return all declarations. BUT the expected outputs only contain the decls from `example`.
        // Wait, C++ `library.declaration_order()` only returns decls from the requested library.
        // Our implementation filters `declaration_order` by `library_name == self.library_name` which is correct!

        assert!(is_union_of(
            &unmangle_decls(&root.declaration_order),
            &expected_suborders
        ));
    }
}

#[test]
fn good_const_type_comes_first() {
    for _ in 0..REPEAT_TEST_COUNT {
        let source = mangle(
            r#"
library example;

const #Constant# #Alias# = 42;

alias #Alias# = uint32;
"#,
        );
        let source_file = SourceFile::new("example.fidl".to_string(), source);
        let library = TestLibrary::with_source(&source_file);
        let root = library.compile().unwrap();
        let expected = vec!["Alias", "Constant"];
        assert_eq!(unmangle_decls(&root.declaration_order), expected);
    }
}

#[test]
fn good_enum_ordinal_type_comes_first() {
    for _ in 0..REPEAT_TEST_COUNT {
        let source = mangle(
            r#"
library example;

type #Enum# = enum : #Alias# { A = 1; };

alias #Alias# = uint32;
"#,
        );
        let source_file = SourceFile::new("example.fidl".to_string(), source);
        let library = TestLibrary::with_source(&source_file);
        let root = library.compile().unwrap();
        let expected = vec!["Alias", "Enum"];
        assert_eq!(unmangle_decls(&root.declaration_order), expected);
    }
}

#[test]
fn good_bits_ordinal_type_comes_first() {
    for _ in 0..REPEAT_TEST_COUNT {
        let source = mangle(
            r#"
library example;

type #Bits# = bits : #Alias# { A = 1; };

alias #Alias# = uint32;
"#,
        );
        let source_file = SourceFile::new("example.fidl".to_string(), source);
        let library = TestLibrary::with_source(&source_file);
        let root = library.compile().unwrap();
        let expected = vec!["Alias", "Bits"];
        assert_eq!(unmangle_decls(&root.declaration_order), expected);
    }
}
