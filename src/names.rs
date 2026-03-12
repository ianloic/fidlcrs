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
impl QualifiedName {
    pub fn as_string(&self) -> String {
        self.to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QualifiedName {
    full_name: String,
    decl_start: usize,
    member_start: Option<usize>,
}

impl QualifiedName {

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

    /// Parses a string into a QualifiedName.
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

impl fmt::Display for QualifiedName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_name)
    }
}

impl PartialEq<str> for LibraryName<'_> {
    fn eq(&self, other: &str) -> bool {
        self.to_string() == other
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

impl PartialEq<str> for QualifiedName {
    fn eq(&self, other: &str) -> bool {
        self.to_string() == other
    }
}

impl PartialEq<&str> for QualifiedName {
    fn eq(&self, other: &&str) -> bool {
        self.to_string() == *other
    }
}

impl PartialEq<String> for QualifiedName {
    fn eq(&self, other: &String) -> bool {
        self.to_string() == *other
    }
}

impl From<&str> for QualifiedName {
    fn from(s: &str) -> Self {
        Self::parse(s)
    }
}

impl From<String> for QualifiedName {
    fn from(s: String) -> Self {
        Self::parse(&s)
    }
}


