#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ExperimentalFlag {
    AllowNewTypes = 1 << 0,
    OutputIndexJson = 1 << 1,
    ZxCTypes = 1 << 2,
    AllowArbitraryErrorTypes = 1 << 3,
    NoResourceAttribute = 1 << 4,
}

impl std::str::FromStr for ExperimentalFlag {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "allow_new_types" => Ok(ExperimentalFlag::AllowNewTypes),
            "output_index_json" => Ok(ExperimentalFlag::OutputIndexJson),
            "zx_c_types" => Ok(ExperimentalFlag::ZxCTypes),
            "allow_arbitrary_error_types" => Ok(ExperimentalFlag::AllowArbitraryErrorTypes),
            "no_resource_attribute" => Ok(ExperimentalFlag::NoResourceAttribute),
            _ => Err(format!("Unknown experimental flag: {}", s)),
        }
    }
}

impl ExperimentalFlag {
    pub const ALL: [ExperimentalFlag; 5] = [
        ExperimentalFlag::AllowNewTypes,
        ExperimentalFlag::OutputIndexJson,
        ExperimentalFlag::ZxCTypes,
        ExperimentalFlag::AllowArbitraryErrorTypes,
        ExperimentalFlag::NoResourceAttribute,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            ExperimentalFlag::AllowNewTypes => "allow_new_types",
            ExperimentalFlag::OutputIndexJson => "output_index_json",
            ExperimentalFlag::ZxCTypes => "zx_c_types",
            ExperimentalFlag::AllowArbitraryErrorTypes => "allow_arbitrary_error_types",
            ExperimentalFlag::NoResourceAttribute => "no_resource_attribute",
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ExperimentalFlags(u8);

impl ExperimentalFlags {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn enable_flag(&mut self, flag: ExperimentalFlag) {
        self.0 |= flag as u8;
    }

    pub fn is_enabled(&self, flag: ExperimentalFlag) -> bool {
        (self.0 & (flag as u8)) != 0
    }

    pub fn into_vec(self) -> Vec<String> {
        ExperimentalFlag::ALL
            .iter()
            .filter(|&f| self.is_enabled(*f))
            .map(|f| f.name().to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_parse() {
        assert_eq!(
            ExperimentalFlag::from_str("allow_new_types").unwrap(),
            ExperimentalFlag::AllowNewTypes
        );
        assert_eq!(
            ExperimentalFlag::from_str("output_index_json").unwrap(),
            ExperimentalFlag::OutputIndexJson
        );
        assert_eq!(
            ExperimentalFlag::from_str("zx_c_types").unwrap(),
            ExperimentalFlag::ZxCTypes
        );
        assert_eq!(
            ExperimentalFlag::from_str("allow_arbitrary_error_types").unwrap(),
            ExperimentalFlag::AllowArbitraryErrorTypes
        );
        assert_eq!(
            ExperimentalFlag::from_str("no_resource_attribute").unwrap(),
            ExperimentalFlag::NoResourceAttribute
        );
        assert!(ExperimentalFlag::from_str("unknown_flag").is_err());
    }

    #[test]
    fn test_name() {
        assert_eq!(ExperimentalFlag::AllowNewTypes.name(), "allow_new_types");
        assert_eq!(ExperimentalFlag::OutputIndexJson.name(), "output_index_json");
        assert_eq!(ExperimentalFlag::ZxCTypes.name(), "zx_c_types");
        assert_eq!(
            ExperimentalFlag::AllowArbitraryErrorTypes.name(),
            "allow_arbitrary_error_types"
        );
        assert_eq!(
            ExperimentalFlag::NoResourceAttribute.name(),
            "no_resource_attribute"
        );
    }

    #[test]
    fn test_flags_operations() {
        let mut flags = ExperimentalFlags::new();
        
        assert!(!flags.is_enabled(ExperimentalFlag::AllowNewTypes));
        flags.enable_flag(ExperimentalFlag::AllowNewTypes);
        assert!(flags.is_enabled(ExperimentalFlag::AllowNewTypes));
        assert!(!flags.is_enabled(ExperimentalFlag::ZxCTypes));

        flags.enable_flag(ExperimentalFlag::ZxCTypes);
        assert!(flags.is_enabled(ExperimentalFlag::ZxCTypes));
        assert!(flags.is_enabled(ExperimentalFlag::AllowNewTypes));
        
        let vec = flags.into_vec();
        assert_eq!(vec.len(), 2);
        assert!(vec.contains(&"allow_new_types".to_string()));
        assert!(vec.contains(&"zx_c_types".to_string()));
    }
}
