use enumset::{EnumSet, EnumSetType};
use heck::AsKebabCase;
use serde::{Deserialize, Serialize};

#[derive(Debug, Hash, EnumSetType, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CoreFeature {
    /// Allow privileged instructions (disabled by default)
    AllowPrivileged,

    /// Enforce aligned memory access within a page
    MemoryAlignment,

    /// Enforce memory access permissions
    MemoryPermissions,

    /// Fault when reading unallocated memory
    MemoryFaults,

    /// Detect integer division by zero
    DivideByZero,

    /// Detect wrapping operations on signed and unsigned integers (disabled by default)
    IntegerBounds,

    /// Dump all allocated memory just before crash
    MemoryDump,
}

impl std::fmt::Display for CoreFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_variant::to_variant_name(self).unwrap())
    }
}

impl CoreFeature {
    /// Convert from a string, returns None if the feature is not recognized
    pub fn from_str(s: &str) -> Option<Self> {
        serde_yaml_ng::from_str(s).ok()
    }
}

/// Get the default feature sets
pub fn default_features() -> EnumSet<CoreFeature> {
    CoreFeature::MemoryAlignment 
    | CoreFeature::MemoryPermissions 
    | CoreFeature::MemoryFaults
    | CoreFeature::DivideByZero
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_string() {
        assert_eq!("memory-alignment", CoreFeature::MemoryAlignment.to_string());
        assert_eq!("divide-by-zero", CoreFeature::DivideByZero.to_string());
    }
    #[test]
    fn test_from_string() {
        assert_eq!(CoreFeature::from_str("memory-alignment"), Some(CoreFeature::MemoryAlignment));
        assert_eq!(CoreFeature::from_str("divide-by-zero"), Some(CoreFeature::DivideByZero));
        assert_eq!(CoreFeature::from_str("memory"), None);
    }
}
