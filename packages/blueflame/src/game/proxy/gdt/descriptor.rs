#[layered_crate::import]
use game::gdt;

/// Descriptor for accessing flag types from TriggerParam
///
/// Use [`fd!`] macro to create a descriptor for a specific flag type.
pub trait FlagDescriptor {
    type T: Clone;

    fn list(trigger_param: &gdt::TriggerParam) -> &gdt::FlagList<Self::T>;
    fn list_mut(trigger_param: &mut gdt::TriggerParam) -> &mut gdt::FlagList<Self::T>;
}

pub trait ArrayFlagDescriptor {
    type ElemT: Clone;

    fn list(trigger_param: &gdt::TriggerParam) -> &gdt::FlagList<Box<[Self::ElemT]>>;
    fn list_mut(trigger_param: &mut gdt::TriggerParam) -> &mut gdt::FlagList<Box<[Self::ElemT]>>;
}

impl<T: ArrayFlagDescriptor> FlagDescriptor for T {
    type T = Box<[T::ElemT]>;

    #[inline(always)]
    fn list(trigger_param: &gdt::TriggerParam) -> &gdt::FlagList<Self::T> {
        Self::list(trigger_param)
    }

    #[inline(always)]
    fn list_mut(trigger_param: &mut gdt::TriggerParam) -> &mut gdt::FlagList<Self::T> {
        Self::list_mut(trigger_param)
    }
}

#[doc(hidden)]
macro_rules! make_descriptor {
    ($name:ident, $type:ty, $field:ident) => {
        #[doc(hidden)]
        pub struct $name;
        #[doc(hidden)]
        impl FlagDescriptor for $name {
            type T = $type;
            #[inline(always)]
            fn list(trigger_param: &gdt::TriggerParam) -> &gdt::FlagList<Self::T> {
                &trigger_param.$field
            }
            #[inline(always)]
            fn list_mut(trigger_param: &mut gdt::TriggerParam) -> &mut gdt::FlagList<Self::T> {
                &mut trigger_param.$field
            }
        }
    };
    (array, $name:ident, $type:ty, $field:ident) => {
        #[doc(hidden)]
        pub struct $name;
        #[doc(hidden)]
        impl ArrayFlagDescriptor for $name {
            type ElemT = $type;
            #[inline(always)]
            fn list(trigger_param: &gdt::TriggerParam) -> &gdt::FlagList<Box<[Self::ElemT]>> {
                &trigger_param.$field
            }
            #[inline(always)]
            fn list_mut(trigger_param: &mut gdt::TriggerParam) -> &mut gdt::FlagList<Box<[Self::ElemT]>> {
                &mut trigger_param.$field
            }
        }
    };
}
make_descriptor!(FdBool, bool, bool_flags);
make_descriptor!(FdS32, i32, s32_flags);
make_descriptor!(FdF32, f32, f32_flags);
make_descriptor!(FdString32, String, string32_flags);
make_descriptor!(FdString64, String, string64_flags);
make_descriptor!(FdString256, String, string256_flags);
make_descriptor!(FdVector2f, (f32, f32), vector2f_flags);
make_descriptor!(FdVector3f, (f32, f32, f32), vector3f_flags);
make_descriptor!(FdVector4f, (f32, f32, f32, f32), vector4f_flags);
make_descriptor!(array, FdBoolArray, bool, bool_array_flags);
make_descriptor!(array, FdS32Array, i32, s32_array_flags);
make_descriptor!(array, FdF32Array, f32, f32_array_flags);
make_descriptor!(array, FdString64Array, String, string64_array_flags);
make_descriptor!(array, FdString256Array, String, string256_array_flags);
make_descriptor!(array, FdVector2fArray, (f32, f32), vector2f_array_flags);
make_descriptor!(array, FdVector3fArray, (f32, f32, f32), vector3f_array_flags);
pub use blueflame_macros::fd;
