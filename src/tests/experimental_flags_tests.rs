#[cfg(test)]
mod tests {
    use crate::experimental_flags::{ExperimentalFlag, ExperimentalFlags};
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
