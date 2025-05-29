use enumset::EnumSetType;

use blueflame_proc_macros::FeatureFlags;
pub use blueflame_proc_macros::enabled;

// re-export to use in proc-macros
#[doc(hidden)]
pub use enumset;

/// BlueFlame features that can be enabled or disabled at init time.
/// They should not be changed after init
///
/// TODO manual
#[derive(FeatureFlags, Debug, Hash, EnumSetType)]
#[allow(non_camel_case_types)] // to be more readable
pub enum Feature {
    // #[on] means this feature is on by default
    // in scripts, replace `_` with `-` as the feature name
    // (i.e. mem-strict-region instead of mem_strict_region)
    /// Enable strict section checks:
    /// - Memory access may specify which section(s) it is allowed to access
    /// - Access to invalid sections will be denied (bypass will return 0 for reads)
    #[on]
    mem_strict_section,
    /// If read, write and execute permission checks are enabled
    #[on]
    mem_permission,
    /// If enabled, accessing unallocated location on the heap will not be allowed
    #[on]
    mem_strict_heap,

    /// If enabled, jumping to the middle of a replace-hooked code will not be allowed
    #[on]
    strict_replace_hook,

    /// Limit the total number of blocks that can be executed
    /// by one frame of the Core. This helps prevent infinite
    /// calls
    #[on]
    limited_block_count,

    /// Limit the total number of instructions that can be executed
    /// by one block. This helps prevent infinite loops
    #[on]
    limited_block_iteration,

    /// Make sure the stack frames push and pop match
    #[on]
    check_stack_frames,

    /// Make sure the LR matches the return address tracked by stack frames
    #[on]
    check_return_address,

    /// Abort when unable to decode an instruction. If disabled,
    /// the instruction will simply be skipped
    #[on]
    instruction_abort, // TODO --cleanup: implement this

    /// If enabled, objects allocated on the stack will reserve
    /// extra space to check for stack corruption
    #[on]
    check_stack_corruption,
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
pub unsafe fn init_features(features: FeatureSet) {
    FEATURES = features;
}

/// Check if a feature is enabled
///
/// This is somewhat verbose, so there is a `feature!` macro
/// for it. `features::is_enabled(features::Feature::mem_strict_region)`
/// is the same as `enabled!("mem-strict-region")`. Kebab case is used
/// to be consistent with the style used in scripts
#[inline(always)]
pub fn is_feature_enabled(feature: Feature) -> bool {
    unsafe {
        // SAFETY: we are just reading a number
        // if people read the thing above, it will be safe
        #[allow(static_mut_refs)]
        FEATURES.contains(feature)
    }
}
