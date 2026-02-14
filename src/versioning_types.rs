use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

const UNVERSIONED_NAME: &str = "unversioned";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Platform {
    name: String,
}

impl Platform {
    pub fn unversioned() -> Self {
        Self {
            name: UNVERSIONED_NAME.to_string(),
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        // Just checking if valid component, we'll keep it simple for now and accept any non-empty string.
        // Or if it matches [a-z0-9_]+ (in fidlc it checks that)
        if s.is_empty() {
            None
        } else {
            Some(Self {
                name: s.to_string(),
            })
        }
    }

    pub fn is_unversioned(&self) -> bool {
        self.name == UNVERSIONED_NAME
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version(pub u32);

impl Version {
    pub const NEG_INF: Version = Version(0);
    pub const NEXT: Version = Version(0xFFD00000);
    pub const HEAD: Version = Version(0xFFE00000);
    pub const LEGACY: Version = Version(0xFFF00000);
    pub const POS_INF: Version = Version(u32::MAX);

    pub fn from_number(number: u32) -> Option<Self> {
        if number == Self::NEXT.0 || number == Self::HEAD.0 || number == Self::LEGACY.0 {
            Some(Version(number))
        } else if number > 0 && number < (1 << 31) {
            Some(Version(number))
        } else {
            None
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "NEXT" => Some(Self::NEXT),
            "HEAD" => Some(Self::HEAD),
            "LEGACY" => Some(Self::LEGACY),
            _ => s.parse::<u32>().ok().and_then(Self::from_number),
        }
    }

    pub fn number(&self) -> u32 {
        self.0
    }

    pub fn is_infinite(&self) -> bool {
        *self == Self::NEG_INF || *self == Self::POS_INF
    }

    pub fn to_string(&self) -> String {
        match *self {
            Self::NEG_INF => "-inf".to_string(),
            Self::NEXT => "NEXT".to_string(),
            Self::HEAD => "HEAD".to_string(),
            Self::LEGACY => "LEGACY".to_string(),
            Self::POS_INF => "+inf".to_string(),
            _ => self.0.to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VersionRange {
    pub lower: Version,
    pub upper_exclusive: Version,
}

impl VersionRange {
    pub fn new(lower: Version, upper_exclusive: Version) -> Self {
        assert!(lower < upper_exclusive, "invalid version range");
        Self {
            lower,
            upper_exclusive,
        }
    }

    pub fn contains(&self, version: Version) -> bool {
        self.lower <= version && version < self.upper_exclusive
    }

    pub fn intersect(lhs: Option<Self>, rhs: Option<Self>) -> Option<Self> {
        match (lhs, rhs) {
            (Some(l), Some(r)) => {
                let lower = std::cmp::max(l.lower, r.lower);
                let upper = std::cmp::min(l.upper_exclusive, r.upper_exclusive);
                if lower < upper {
                    Some(Self::new(lower, upper))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VersionSet {
    pub ranges: (VersionRange, Option<VersionRange>),
}

impl VersionSet {
    pub fn new(first: VersionRange, second: Option<VersionRange>) -> Self {
        if let Some(s) = &second {
            assert!(
                first.upper_exclusive < s.lower,
                "ranges must be in order and noncontiguous"
            );
        }
        Self {
            ranges: (first, second),
        }
    }

    pub fn contains(&self, version: Version) -> bool {
        self.ranges.0.contains(version) || self.ranges.1.map_or(false, |r| r.contains(version))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AvailabilityState {
    Unset,
    Initialized,
    Inherited,
    Narrowed,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Legacy {
    NotApplicable,
    No,
    Yes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ending {
    None,
    Removed,
    Replaced,
    Inherited,
    Split,
}

#[derive(Debug, Clone)]
pub struct Availability {
    state: AvailabilityState,
    added: Option<Version>,
    deprecated: Option<Version>,
    removed: Option<Version>,
    ending: Option<Ending>,
    legacy: Option<Legacy>,
}

pub struct InitArgs {
    pub added: Option<Version>,
    pub deprecated: Option<Version>,
    pub removed: Option<Version>,
    pub replaced: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InheritStatus {
    Ok,
    BeforeParentAdded,
    AfterParentDeprecated,
    AfterParentRemoved,
}

pub struct InheritResult {
    pub added: InheritStatus,
    pub deprecated: InheritStatus,
    pub removed: InheritStatus,
}

impl InheritResult {
    pub fn is_ok(&self) -> bool {
        self.added == InheritStatus::Ok
            && self.deprecated == InheritStatus::Ok
            && self.removed == InheritStatus::Ok
    }
}

impl Availability {
    pub fn new() -> Self {
        Self {
            state: AvailabilityState::Unset,
            added: None,
            deprecated: None,
            removed: None,
            ending: None,
            legacy: None,
        }
    }

    pub fn unbounded() -> Self {
        Self {
            state: AvailabilityState::Inherited,
            added: Some(Version::NEG_INF),
            deprecated: None,
            removed: Some(Version::POS_INF),
            ending: Some(Ending::None),
            legacy: Some(Legacy::NotApplicable),
        }
    }

    pub fn state(&self) -> AvailabilityState {
        self.state
    }

    pub fn is_deprecated(&self) -> bool {
        assert!(self.state == AvailabilityState::Narrowed);
        self.deprecated.is_some()
    }

    pub fn fail(&mut self) {
        self.state = AvailabilityState::Failed;
    }

    fn valid_order(&self) -> bool {
        let a = self.added.unwrap_or(Version::NEG_INF);
        let d = self.deprecated.unwrap_or(a);
        let r = self.removed.unwrap_or(Version::POS_INF);
        a <= d && d < r
    }

    pub fn init(&mut self, args: InitArgs) -> bool {
        assert!(self.state == AvailabilityState::Unset);
        if self.legacy.is_some() {
            panic!("cannot process legacy=true during Init");
        }
        if args.replaced && args.removed.is_none() {
            panic!("cannot set replaced without removed");
        }

        self.added = args.added;
        self.deprecated = args.deprecated;
        self.removed = args.removed;
        if args.removed.is_some() {
            self.ending = Some(if args.replaced {
                Ending::Replaced
            } else {
                Ending::Removed
            });
        }

        let valid = self.valid_order();
        self.state = if valid {
            AvailabilityState::Initialized
        } else {
            AvailabilityState::Failed
        };
        valid
    }

    pub fn inherit(&mut self, parent: &Availability) -> InheritResult {
        assert!(self.state == AvailabilityState::Initialized);
        assert!(parent.state == AvailabilityState::Inherited);

        let mut result = InheritResult {
            added: InheritStatus::Ok,
            deprecated: InheritStatus::Ok,
            removed: InheritStatus::Ok,
        };

        if self.added.is_none() {
            self.added = parent.added;
        } else if self.added.unwrap() < parent.added.unwrap() {
            result.added = InheritStatus::BeforeParentAdded;
        } else if self.added.unwrap() >= parent.removed.unwrap() {
            result.added = InheritStatus::AfterParentRemoved;
        }

        if self.removed.is_none() {
            self.removed = parent.removed;
        } else if self.removed.unwrap() <= parent.added.unwrap() {
            result.removed = InheritStatus::BeforeParentAdded;
        } else if self.removed.unwrap() > parent.removed.unwrap() {
            result.removed = InheritStatus::AfterParentRemoved;
        }

        if self.deprecated.is_none() {
            if let Some(pd) = parent.deprecated {
                if pd < self.removed.unwrap() {
                    self.deprecated = Some(std::cmp::max(pd, self.added.unwrap()));
                }
            }
        } else if self.deprecated.unwrap() < parent.added.unwrap() {
            result.deprecated = InheritStatus::BeforeParentAdded;
        } else if self.deprecated.unwrap() >= parent.removed.unwrap() {
            result.deprecated = InheritStatus::AfterParentRemoved;
        } else if parent.deprecated.is_some()
            && self.deprecated.unwrap() > parent.deprecated.unwrap()
        {
            result.deprecated = InheritStatus::AfterParentDeprecated;
        }

        if self.ending.is_none() {
            self.ending = Some(if parent.ending.unwrap() == Ending::None {
                Ending::None
            } else {
                Ending::Inherited
            });
        } else if self.ending.unwrap() == Ending::Replaced
            && self.removed.unwrap() == parent.removed.unwrap()
        {
            result.removed = InheritStatus::AfterParentRemoved;
        }

        assert!(self.legacy.is_none());
        if self.removed.unwrap() == parent.removed.unwrap() {
            self.legacy = parent.legacy;
        } else {
            assert!(self.removed.unwrap() != Version::POS_INF);
            self.legacy = Some(Legacy::No);
        }

        if result.is_ok() {
            assert!(
                self.added.is_some()
                    && self.removed.is_some()
                    && self.ending.is_some()
                    && self.legacy.is_some()
            );
            assert!(self.added.unwrap() != Version::NEG_INF);
            assert!(self.valid_order());
            self.state = AvailabilityState::Inherited;
        } else {
            self.state = AvailabilityState::Failed;
        }
        result
    }

    pub fn set_legacy(&mut self) {
        assert!(self.state == AvailabilityState::Inherited);
        assert!(self.legacy.is_some());
        assert!(self.removed.unwrap() != Version::POS_INF);
        self.legacy = Some(Legacy::Yes);
    }

    pub fn narrow(&mut self, range: VersionRange) {
        assert!(self.state == AvailabilityState::Inherited);
        let a = range.lower;
        let b = range.upper_exclusive;
        if a == Version::LEGACY {
            assert!(b == Version::POS_INF);
            assert!(self.legacy.unwrap() != Legacy::No);
        } else {
            assert!(a >= self.added.unwrap() && b <= self.removed.unwrap());
        }
        if b == Version::POS_INF {
            self.ending = Some(Ending::None);
        } else if self.removed.unwrap() != b {
            self.ending = Some(Ending::Split);
        }
        self.added = Some(a);
        self.removed = Some(b);
        if let Some(d) = self.deprecated {
            if a >= d {
                self.deprecated = Some(a);
            } else {
                self.deprecated = None;
            }
        }
        if a <= Version::LEGACY && b > Version::LEGACY {
            self.legacy = Some(Legacy::NotApplicable);
        } else {
            self.legacy = Some(Legacy::No);
        }
        self.state = AvailabilityState::Narrowed;
    }

    pub fn range(&self) -> VersionRange {
        assert!(self.state == AvailabilityState::Narrowed);
        VersionRange::new(self.added.unwrap(), self.removed.unwrap())
    }

    pub fn points(&self) -> BTreeSet<Version> {
        let mut pts = BTreeSet::new();
        pts.insert(self.added.unwrap());
        if let Some(d) = self.deprecated {
            pts.insert(d);
        }
        pts.insert(self.removed.unwrap());
        if self.legacy == Some(Legacy::Yes) {
            pts.insert(Version::LEGACY);
            pts.insert(Version::POS_INF);
        }
        pts
    }

    pub fn set(&self) -> VersionSet {
        // [added, removed) and possibly [LEGACY, +inf)
        let first = VersionRange::new(self.added.unwrap(), self.removed.unwrap());
        let second = if self.legacy == Some(Legacy::Yes) {
            Some(VersionRange::new(Version::LEGACY, Version::POS_INF))
        } else {
            None
        };
        VersionSet::new(first, second)
    }

    pub fn ending(&self) -> Ending {
        assert!(self.state == AvailabilityState::Narrowed);
        self.ending.unwrap()
    }
}

pub struct VersionSelection {
    map: BTreeMap<Platform, BTreeSet<Version>>,
}

impl VersionSelection {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, platform: Platform, versions: BTreeSet<Version>) -> bool {
        assert!(!platform.is_unversioned());
        assert!(!versions.is_empty());
        assert!(!versions.contains(&Version::LEGACY));
        if versions.len() > 1 {
            assert!(versions.contains(&Version::HEAD));
        }
        if self.map.contains_key(&platform) {
            false
        } else {
            self.map.insert(platform, versions);
            true
        }
    }

    pub fn lookup(&self, platform: &Platform) -> Version {
        if platform.is_unversioned() {
            Version::HEAD
        } else {
            let versions = self.map.get(platform).unwrap();
            if versions.len() == 1 {
                *versions.iter().next().unwrap()
            } else {
                Version::LEGACY
            }
        }
    }

    pub fn contains(&self, platform: &Platform) -> bool {
        assert!(!platform.is_unversioned());
        self.map.contains_key(platform)
    }
}
impl Default for Availability {
    fn default() -> Self {
        Self {
            state: AvailabilityState::Narrowed,
            added: Some(Version::NEG_INF),
            deprecated: None,
            removed: None,
            ending: None,
            legacy: None,
        }
    }
}
