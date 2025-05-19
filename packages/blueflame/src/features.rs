use enumset::EnumSetType;

use blueflame_macros::DefaultFeatures;

// re-export to use in proc-macros
#[doc(hidden)]
pub use enumset;

/// BlueFlame features that can be enabled or disabled at init time.
/// They should not be changed after init
#[derive(DefaultFeatures, Debug, Hash, EnumSetType)]
#[allow(non_camel_case_types)] // to be more readable
#[rustfmt::skip] // so the #[enable] is before
pub enum Feature {
    /// If enabled and a region is provided when accessing memory,
    /// it will not allow accessing other regions, even if the address
    /// is in another valid region
    #[on] mem_strict_region,
    /// If read, write and execute permission checks are enabled
    #[on] mem_permission,
    /// If enabled, accessing unallocated location on the heap will not be allowed
    #[on] mem_heap_check_allocated
}

pub type FeatureSet = enumset::EnumSet<Feature>;

static mut FEATURES: FeatureSet = Feature::default_const();

/// Set the features for the BlueFlame core
///
/// # Safety
/// The features is simply a static bitset in memory to allow
/// for efficient reads. There is no locking. Initializing
/// features flag should be done before using anything in the crate.
///
/// Before this is called, the "default" set of features is used.
pub unsafe fn init(features: FeatureSet) {
    FEATURES = features;
}

/// Check if a feature is enabled
///
/// This is somewhat verbose, so there is a `feature!` macro
/// for it. `features::is_enabled(features::Feature::mem_strict_region)`
/// is the same as `enabled!("mem-strict-region")`. Kebab case is used
/// to be consistent with the style used in scripts
#[inline(always)]
pub fn is_enabled(feature: Feature) -> bool {
    unsafe { 
        // SAFETY: we are just reading a number
        // if people read the thing above, it will be safe
        #[allow(static_mut_refs)]
        FEATURES.contains(feature)
    }
}
