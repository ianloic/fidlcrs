use std::fmt;


impl OwnedLibraryName {
    pub fn as_string(&self) -> String {
        self.to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OwnedLibraryName {
    name: String,
}

impl OwnedLibraryName {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn versioning_platform(&self) -> &str {
        self.name.split('.').next().unwrap_or(&self.name)
    }
    
    pub fn as_borrowed(&self) -> LibraryName<'_> {
        LibraryName::new(&self.name)
    }

    pub fn with_declaration(&self, declaration: &str) -> OwnedQualifiedName {
        OwnedQualifiedName::new(&self.name, declaration, None)
    }
}

impl fmt::Display for OwnedLibraryName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl From<String> for OwnedLibraryName {
    fn from(name: String) -> Self {
        Self { name }
    }
}

impl From<&str> for OwnedLibraryName {
    fn from(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LibraryName<'a> {
    name: &'a str,
}

impl<'a> LibraryName<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }

    pub fn versioning_platform(&self) -> &'a str {
        self.name.split('.').next().unwrap_or(self.name)
    }

    pub fn to_owned(&self) -> OwnedLibraryName {
        OwnedLibraryName::new(self.name.to_string())
    }
}

impl<'a> fmt::Display for LibraryName<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl<'a> From<&'a str> for LibraryName<'a> {
    fn from(name: &'a str) -> Self {
        Self { name }
    }
}

impl<'a> From<LibraryName<'a>> for OwnedLibraryName {
    fn from(lib: LibraryName<'a>) -> Self {
        Self { name: lib.name.to_string() }
    }
}

impl<'a> From<&'a OwnedLibraryName> for LibraryName<'a> {
    fn from(lib: &'a OwnedLibraryName) -> Self {
        Self { name: &lib.name }
    }
}

/// Represents a fully resolved name that unambiguously identifies a declaration or a member.
impl OwnedQualifiedName {
    pub fn as_string(&self) -> String {
        self.to_string()
    }
}
#[derive(Debug, Clone)]
pub struct OwnedQualifiedName {
    full_name: String,
    decl_start: usize,
    member_start: Option<usize>,
}

impl OwnedQualifiedName {
    pub fn library(&self) -> LibraryName<'_> {
        let lib_part = if self.decl_start > 0 {
            &self.full_name[0..self.decl_start - 1]
        } else {
            ""
        };
        LibraryName::new(lib_part)
    }

    pub fn declaration(&self) -> &str {
        if let Some(m_start) = self.member_start {
            &self.full_name[self.decl_start..m_start - 1]
        } else {
            &self.full_name[self.decl_start..]
        }
    }

    pub fn member(&self) -> Option<&str> {
        self.member_start.map(|idx| &self.full_name[idx..])
    }
    
    pub fn as_borrowed(&self) -> QualifiedName<'_> {
        QualifiedName {
            full_name: &self.full_name,
            decl_start: self.decl_start,
            member_start: self.member_start,
        }
    }

    pub fn with_member(&self, member: &str) -> Self {
        Self::new(self.library().name, self.declaration(), Some(member))
    }

    /// Constructs a new OwnedQualifiedName from components
    fn new(library: &str, declaration: &str, member: Option<&str>) -> Self {
        let mut full_name = String::with_capacity(
            library.len() + 1 + declaration.len() + member.map_or(0, |m| m.len() + 1),
        );

        if !library.is_empty() {
            full_name.push_str(library);
            full_name.push('/');
        }

        let decl_start = full_name.len();
        full_name.push_str(declaration);

        let member_start = if let Some(m) = member {
            let start = full_name.len() + 1;
            full_name.push('.');
            full_name.push_str(m);
            Some(start)
        } else {
            None
        };

        Self {
            full_name,
            decl_start,
            member_start,
        }
    }

    /// Parses a string into an OwnedQualifiedName.
    /// Expected formats:
    /// - "library/Declaration"
    /// - "library/Declaration.member"
    pub fn parse(s: &str) -> Self {
        let full_name = s.to_string();
        let slash_idx = full_name.rfind('/');

        let decl_start = if let Some(idx) = slash_idx {
            idx + 1
        } else {
            0
        };

        let dot_idx = full_name[decl_start..].find('.');
        let member_start = dot_idx.map(|idx| decl_start + idx + 1);

        Self {
            full_name,
            decl_start,
            member_start,
        }
    }
}

impl fmt::Display for OwnedQualifiedName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_name)
    }
}

impl std::hash::Hash for OwnedQualifiedName {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.full_name.hash(state);
    }
}

impl PartialEq for OwnedQualifiedName {
    fn eq(&self, other: &Self) -> bool {
        self.full_name == other.full_name
    }
}

impl Eq for OwnedQualifiedName {}

impl PartialOrd for OwnedQualifiedName {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OwnedQualifiedName {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.full_name.cmp(&other.full_name)
    }
}

impl std::borrow::Borrow<str> for OwnedQualifiedName {
    fn borrow(&self) -> &str {
        &self.full_name
    }
}

#[derive(Debug, Clone, Copy)]
pub struct QualifiedName<'a> {
    full_name: &'a str,
    decl_start: usize,
    member_start: Option<usize>,
}

impl<'a> QualifiedName<'a> {
    pub fn library(&self) -> LibraryName<'a> {
        let lib_part = if self.decl_start > 0 {
            &self.full_name[0..self.decl_start - 1]
        } else {
            ""
        };
        LibraryName::new(lib_part)
    }

    pub fn declaration(&self) -> &'a str {
        if let Some(m_start) = self.member_start {
            &self.full_name[self.decl_start..m_start - 1]
        } else {
            &self.full_name[self.decl_start..]
        }
    }

    pub fn member(&self) -> Option<&'a str> {
        self.member_start.map(|idx| &self.full_name[idx..])
    }

    pub fn to_owned(&self) -> OwnedQualifiedName {
        OwnedQualifiedName {
            full_name: self.full_name.to_string(),
            decl_start: self.decl_start,
            member_start: self.member_start,
        }
    }

    pub fn parse(s: &'a str) -> Self {
        let slash_idx = s.rfind('/');

        let decl_start = if let Some(idx) = slash_idx {
            idx + 1
        } else {
            0
        };

        let dot_idx = s[decl_start..].find('.');
        let member_start = dot_idx.map(|idx| decl_start + idx + 1);

        Self {
            full_name: s,
            decl_start,
            member_start,
        }
    }
}

impl<'a> fmt::Display for QualifiedName<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_name)
    }
}

impl<'a> From<QualifiedName<'a>> for OwnedQualifiedName {
    fn from(name: QualifiedName<'a>) -> Self {
        name.to_owned()
    }
}

impl<'a> From<&'a OwnedQualifiedName> for QualifiedName<'a> {
    fn from(name: &'a OwnedQualifiedName) -> Self {
        name.as_borrowed()
    }
}

impl std::borrow::Borrow<str> for OwnedLibraryName {
    fn borrow(&self) -> &str {
        &self.name
    }
}

impl PartialEq<str> for LibraryName<'_> {
    fn eq(&self, other: &str) -> bool {
        *self == other
    }
}

impl PartialEq<&str> for LibraryName<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.to_string() == *other
    }
}

impl PartialEq<String> for LibraryName<'_> {
    fn eq(&self, other: &String) -> bool {
        self.to_string() == *other
    }
}

impl PartialEq<str> for OwnedQualifiedName {
    fn eq(&self, other: &str) -> bool {
        *self == other
    }
}

impl PartialEq<&str> for OwnedQualifiedName {
    fn eq(&self, other: &&str) -> bool {
        self.to_string() == *other
    }
}

impl PartialEq<String> for OwnedQualifiedName {
    fn eq(&self, other: &String) -> bool {
        self.to_string() == *other
    }
}

impl PartialEq<str> for QualifiedName<'_> {
    fn eq(&self, other: &str) -> bool {
        *self == other
    }
}

impl PartialEq<&str> for QualifiedName<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.to_string() == *other
    }
}

impl PartialEq<String> for QualifiedName<'_> {
    fn eq(&self, other: &String) -> bool {
        self.to_string() == *other
    }
}

impl From<&str> for OwnedQualifiedName {
    fn from(s: &str) -> Self {
        Self::parse(s)
    }
}

impl From<String> for OwnedQualifiedName {
    fn from(s: String) -> Self {
        Self::parse(&s)
    }
}



impl AsRef<str> for OwnedQualifiedName {
    fn as_ref(&self) -> &str {
        &self.full_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_owned_library_name() {
        let lib = OwnedLibraryName::new("fuchsia.math".to_string());
        assert_eq!(lib.as_string(), "fuchsia.math");
        assert_eq!(lib.versioning_platform(), "fuchsia");
        assert_eq!(lib.to_string(), "fuchsia.math");

        let borrowed = lib.as_borrowed();
        assert_eq!(borrowed.to_string(), "fuchsia.math");
        assert_eq!(borrowed.versioning_platform(), "fuchsia");

        let from_str: OwnedLibraryName = "fuchsia.io".into();
        assert_eq!(from_str.to_string(), "fuchsia.io");

        let fqn = lib.with_declaration("Matrix");
        assert_eq!(fqn.to_string(), "fuchsia.math/Matrix");
    }

    #[test]
    fn test_library_name() {
        let lib = LibraryName::new("fuchsia.math");
        assert_eq!(lib.to_string(), "fuchsia.math");
        assert_eq!(lib.versioning_platform(), "fuchsia");

        let owned = lib.to_owned();
        assert_eq!(owned.to_string(), "fuchsia.math");
    }

    #[test]
    fn test_owned_qualified_name_new() {
        // Without member
        let fqn = OwnedQualifiedName::new("fuchsia.math", "Matrix", None);
        assert_eq!(fqn.to_string(), "fuchsia.math/Matrix");
        assert_eq!(fqn.library().to_string(), "fuchsia.math");
        assert_eq!(fqn.declaration(), "Matrix");
        assert_eq!(fqn.member(), None);

        // With member
        let fqn_mem = OwnedQualifiedName::new("fuchsia.math", "Matrix", Some("m00"));
        assert_eq!(fqn_mem.to_string(), "fuchsia.math/Matrix.m00");
        assert_eq!(fqn_mem.library().to_string(), "fuchsia.math");
        assert_eq!(fqn_mem.declaration(), "Matrix");
        assert_eq!(fqn_mem.member(), Some("m00"));
        
        // Empty library
        let fqn_empty_lib = OwnedQualifiedName::new("", "Int32", None);
        assert_eq!(fqn_empty_lib.to_string(), "Int32");
        assert_eq!(fqn_empty_lib.library().to_string(), "");
        assert_eq!(fqn_empty_lib.declaration(), "Int32");
    }

    #[test]
    fn test_owned_qualified_name_parse() {
        // Without member
        let fqn = OwnedQualifiedName::parse("fuchsia.math/Matrix");
        assert_eq!(fqn, "fuchsia.math/Matrix");
        assert_eq!(fqn.library(), "fuchsia.math");
        assert_eq!(fqn.declaration(), "Matrix");
        assert_eq!(fqn.member(), None);

        // With member
        let fqn_mem = OwnedQualifiedName::parse("fuchsia.math/Matrix.m00");
        assert_eq!(fqn_mem, "fuchsia.math/Matrix.m00");
        assert_eq!(fqn_mem.library(), "fuchsia.math");
        assert_eq!(fqn_mem.declaration(), "Matrix");
        assert_eq!(fqn_mem.member(), Some("m00"));

        // Builtin/Primitive (no library)
        let fqn_builtin = OwnedQualifiedName::parse("uint32");
        assert_eq!(fqn_builtin, "uint32");
        assert_eq!(fqn_builtin.library(), "");
        assert_eq!(fqn_builtin.declaration(), "uint32");
        assert_eq!(fqn_builtin.member(), None);
    }

    #[test]
    fn test_owned_qualified_name_with_member() {
        let fqn = OwnedQualifiedName::new("fuchsia.math", "Matrix", None);
        let fqn_mem = fqn.with_member("m00");
        assert_eq!(fqn_mem, "fuchsia.math/Matrix.m00");
        assert_eq!(fqn_mem.member(), Some("m00"));
        assert_eq!(fqn_mem.declaration(), "Matrix");
        assert_eq!(fqn_mem.library(), "fuchsia.math");
    }

    #[test]
    fn test_qualified_name_borrowed() {
        let owned = OwnedQualifiedName::parse("fuchsia.math/Matrix.m00");
        let borrowed = owned.as_borrowed();
        
        assert_eq!(borrowed, "fuchsia.math/Matrix.m00");
        assert_eq!(borrowed.library(), "fuchsia.math");
        assert_eq!(borrowed.declaration(), "Matrix");
        assert_eq!(borrowed.member(), Some("m00"));

        let owned_again = borrowed.to_owned();
        assert_eq!(owned_again, "fuchsia.math/Matrix.m00");
    }

    #[test]
    fn test_qualified_name_parse() {
        let fqn_str = "fuchsia.math/Matrix.m00";
        let borrowed = QualifiedName::parse(fqn_str);
        
        assert_eq!(borrowed, "fuchsia.math/Matrix.m00");
        assert_eq!(borrowed.library(), "fuchsia.math");
        assert_eq!(borrowed.declaration(), "Matrix");
        assert_eq!(borrowed.member(), Some("m00"));
    }

    #[test]
    fn test_eq_and_cmp() {
        // LibraryName
        let lib1 = LibraryName::new("fuchsia.math");
        let lib2 = OwnedLibraryName::new("fuchsia.math".to_string());
        assert_eq!(lib1, "fuchsia.math");
        assert!(lib1 == *"fuchsia.math");
        assert!(lib1 == "fuchsia.math".to_string());
        assert_eq!(lib2.to_string(), "fuchsia.math");

        // OwnedQualifiedName
        let fqn1 = OwnedQualifiedName::parse("fuchsia.math/Matrix");
        assert_eq!(fqn1, "fuchsia.math/Matrix");
        assert!(fqn1 == *"fuchsia.math/Matrix");
        assert!(fqn1 == "fuchsia.math/Matrix".to_string());

        // QualifiedName
        let fqn2 = QualifiedName::parse("fuchsia.math/Matrix.m00");
        assert_eq!(fqn2, "fuchsia.math/Matrix.m00");
        assert!(fqn2 == *"fuchsia.math/Matrix.m00");
        assert!(fqn2 == "fuchsia.math/Matrix.m00".to_string());
    }
}
