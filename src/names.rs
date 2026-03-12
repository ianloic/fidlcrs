use std::fmt;


/// Represents a resolved library namespace.
impl LibraryName {
    pub fn as_string(&self) -> String {
        self.to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LibraryName {
    name: String,
}

impl LibraryName {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn versioning_platform(&self) -> &str {
        self.name.split('.').next().unwrap_or(&self.name)
    }
}

impl fmt::Display for LibraryName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl From<String> for LibraryName {
    fn from(name: String) -> Self {
        Self { name }
    }
}

impl From<&str> for LibraryName {
    fn from(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

/// Represents a fully resolved name that unambiguously identifies a declaration or a member.
impl FullyQualifiedName {
    pub fn as_string(&self) -> String {
        self.to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FullyQualifiedName {
    pub library: LibraryName,
    pub declaration: String,
    pub member: Option<String>,
}

impl FullyQualifiedName {
    pub fn new(library: LibraryName, declaration: String, member: Option<String>) -> Self {
        Self {
            library,
            declaration,
            member,
        }
    }

    /// Parses a string into a FullyQualifiedName.
    /// Expected formats:
    /// - "library/Declaration"
    /// - "library/Declaration.member"
    pub fn parse(s: &str) -> Self {
        let (lib_part, rest) = s.rsplit_once('/').unwrap_or(("", s));
        let library = LibraryName::new(lib_part.to_string());

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

impl fmt::Display for FullyQualifiedName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.library, self.declaration)?;
        if let Some(m) = &self.member {
            write!(f, ".{}", m)?;
        }
        Ok(())
    }
}

impl PartialEq<str> for LibraryName {
    fn eq(&self, other: &str) -> bool {
        self.to_string() == other
    }
}

impl PartialEq<&str> for LibraryName {
    fn eq(&self, other: &&str) -> bool {
        self.to_string() == *other
    }
}

impl PartialEq<String> for LibraryName {
    fn eq(&self, other: &String) -> bool {
        self.to_string() == *other
    }
}

impl PartialEq<str> for FullyQualifiedName {
    fn eq(&self, other: &str) -> bool {
        self.to_string() == other
    }
}

impl PartialEq<&str> for FullyQualifiedName {
    fn eq(&self, other: &&str) -> bool {
        self.to_string() == *other
    }
}

impl PartialEq<String> for FullyQualifiedName {
    fn eq(&self, other: &String) -> bool {
        self.to_string() == *other
    }
}

impl From<&str> for FullyQualifiedName {
    fn from(s: &str) -> Self {
        Self::parse(s)
    }
}

impl From<String> for FullyQualifiedName {
    fn from(s: String) -> Self {
        Self::parse(&s)
    }
}


