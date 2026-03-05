use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct FidlBuild {
    pub target_name: String,
    pub sources: Vec<String>,
    pub public_deps: Option<Vec<String>>,
    pub experimental_flags: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Clone)]
enum Token<'a> {
    Ident(&'a str),
    String(&'a str),
    Punct(char),
}

fn tokenize(content: &str) -> Vec<Token<'_>> {
    let mut tokens = Vec::new();
    let mut chars = content.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        match c {
            '#' => {
                while let Some(&(_, ch)) = chars.peek() {
                    if ch == '\n' {
                        break;
                    }
                    chars.next();
                }
            }
            '"' => {
                let mut end = i + 1;
                while let Some(&(j, ch)) = chars.peek() {
                    if ch == '"' {
                        end = j;
                        chars.next(); // consume '"'
                        break;
                    }
                    chars.next();
                }
                tokens.push(Token::String(&content[i + 1..end]));
            }
            c if c.is_alphabetic() || c == '_' => {
                let start = i;
                let mut end = start + c.len_utf8();
                while let Some(&(j, ch)) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        end = j + ch.len_utf8();
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Ident(&content[start..end]));
            }
            c if c.is_whitespace() => {}
            c if "=[](){},".contains(c) => {
                tokens.push(Token::Punct(c));
            }
            _ => {}
        }
    }
    tokens
}

pub fn parse_build_gn(content: &str) -> Option<FidlBuild> {
    let tokens = tokenize(content);
    let mut iter = tokens.iter().peekable();

    let mut target_name = String::new();
    let mut sources = Vec::new();
    let mut public_deps = None;
    let mut experimental_flags = None;
    let mut in_fidl = false;

    while let Some(tok) = iter.next() {
        if let Token::Ident("fidl") = tok {
            if let Some(Token::Punct('(')) = iter.next() {
            } else {
                continue;
            }
            if let Some(Token::String(name)) = iter.next() {
                target_name = name.to_string();
            } else {
                continue;
            }
            if let Some(Token::Punct(')')) = iter.next() {
            } else {
                continue;
            }
            if let Some(Token::Punct('{')) = iter.next() {
            } else {
                continue;
            }
            in_fidl = true;
            break;
        }
    }

    if !in_fidl {
        return None;
    }

    while let Some(tok) = iter.next() {
        match tok {
            Token::Punct('}') => break,
            Token::Ident("sources") => {
                if let Some(Token::Punct('=')) = iter.next() {
                } else {
                    continue;
                }
                if let Some(Token::Punct('[')) = iter.next() {
                } else {
                    continue;
                }
                sources = parse_string_list(&mut iter);
            }
            Token::Ident("public_deps") => {
                if let Some(Token::Punct('=')) = iter.next() {
                } else {
                    continue;
                }
                if let Some(Token::Punct('[')) = iter.next() {
                } else {
                    continue;
                }
                public_deps = Some(parse_string_list(&mut iter));
            }
            Token::Ident("experimental_flags") => {
                if let Some(Token::Punct('=')) = iter.next() {
                } else {
                    continue;
                }
                if let Some(Token::Punct('[')) = iter.next() {
                } else {
                    continue;
                }
                experimental_flags = Some(parse_string_list(&mut iter));
            }
            _ => {}
        }
    }

    Some(FidlBuild {
        target_name,
        sources,
        public_deps,
        experimental_flags,
    })
}

fn parse_string_list(
    iter: &mut std::iter::Peekable<std::slice::Iter<'_, Token<'_>>>,
) -> Vec<String> {
    let mut list = Vec::new();
    while let Some(&tok) = iter.peek() {
        match tok {
            Token::Punct(']') => {
                iter.next();
                break;
            }
            Token::String(s) => {
                list.push(s.to_string());
                iter.next();
            }
            Token::Punct(',') => {
                iter.next();
            }
            _ => {
                iter.next();
            }
        }
    }
    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_build_gn() {
        let content = r#"
        import("//build/fidl/fidl.gni")

        fidl("fuchsia.accessibility.scene") {
            sources = [ "provider.fidl" ]
            public_deps = [ "//sdk/fidl/fuchsia.ui.views" ]
            enable_hlcpp = true
        }
        "#;

        let parsed = parse_build_gn(content).unwrap();
        assert_eq!(parsed.target_name, "fuchsia.accessibility.scene");
        assert_eq!(parsed.sources, vec!["provider.fidl"]);
        assert_eq!(
            parsed.public_deps,
            Some(vec!["//sdk/fidl/fuchsia.ui.views".to_string()])
        );
        assert_eq!(parsed.experimental_flags, None);
    }

    #[test]
    fn test_parse_all_sdk_build_files() {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let sdk_fidl_dir = manifest_dir.join("sdk-fidl");

        let entries = std::fs::read_dir(sdk_fidl_dir).unwrap();
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.starts_with("fuchsia.") {
                        let build_gn_path = path.join("BUILD.gn");
                        if build_gn_path.exists() {
                            let content = std::fs::read_to_string(&build_gn_path).unwrap();
                            let parsed = parse_build_gn(&content);
                            assert!(
                                parsed.is_some(),
                                "Failed to parse BUILD.gn at {:?}",
                                build_gn_path
                            );
                            let parsed = parsed.unwrap();
                            assert!(
                                !parsed.sources.is_empty() || content.contains("sources = []"),
                                "Parsed no sources for {:?} (or ensure it's manually empty)",
                                build_gn_path
                            );
                        }
                    }
                }
            }
        }
    }
}
