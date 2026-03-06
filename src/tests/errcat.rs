use std::fs;
use std::path::PathBuf;

pub struct Errcat;

impl Errcat {
    /// Returns the path to an error catalog documentation file for a given ID (e.g., "fi-0001")
    pub fn doc_path(id: &str) -> PathBuf {
        PathBuf::from("errcat-docs").join(format!("_{}.md", id))
    }

    /// Read an error catalog documentation file for a given ID (if it exists)
    pub fn get_doc(id: &str) -> Option<String> {
        fs::read_to_string(Self::doc_path(id)).ok()
    }

    /// Returns the path to the general error catalog index file
    pub fn index_path() -> PathBuf {
        PathBuf::from("errcat-docs").join("errcat.md")
    }

    /// Returns the path to the redirects YAML file
    pub fn redirects_path() -> PathBuf {
        PathBuf::from("errcat-docs").join("_redirects.yaml")
    }

    /// Returns the path to a good/bad FIDL example file, e.g., "good/fi-0001.test.fidl"
    pub fn fidl_path(path: &str) -> PathBuf {
        PathBuf::from("fidlc").join("tests").join("fidl").join(path)
    }

    /// Read a good/bad FIDL example file
    pub fn get_fidl(path: &str) -> Option<String> {
        fs::read_to_string(Self::fidl_path(path)).ok()
    }
}
