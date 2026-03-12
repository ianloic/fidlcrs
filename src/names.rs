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
    library: OwnedLibraryName,
    declaration: String,
    member: Option<String>,
}

impl QualifiedName {
    pub fn new(library: OwnedLibraryName, declaration: String, member: Option<String>) -> Self {
        Self {
            library,
            declaration,
            member,
        }
    }

    pub fn library(&self) -> LibraryName<'_> {
        self.library.as_borrowed()
    }

    pub fn declaration(&self) -> &str {
        &self.declaration
    }

    pub fn member(&self) -> Option<&str> {
        self.member.as_deref()
    }

    /// Parses a string into a FullyQualifiedName.
    /// Expected formats:
    /// - "library/Declaration"
    /// - "library/Declaration.member"
    pub fn parse(s: &str) -> Self {
        let (lib_part, rest) = s.rsplit_once('/').unwrap_or(("", s));
        let library = OwnedLibraryName::new(lib_part.to_string());

        let (declaration, member) = if let Some((decl, mem)) = rest.split_once('.') {
            (decl.to_string(), Some(mem.to_string()))
        } else {
            (rest.to_string(), None)
        };

        Self {
            library,
            declaration,
            member,
        }
    }
}

impl fmt::Display for QualifiedName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.library, self.declaration)?;
        if let Some(m) = &self.member {
            write!(f, ".{}", m)?;
        }
        Ok(())
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


