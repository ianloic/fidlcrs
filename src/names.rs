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

    /// Constructs a new OwnedQualifiedName from components
    pub fn new(library: &str, declaration: &str, member: Option<&str>) -> Self {
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
