use crate::game::{gdt, singleton_instance};
use crate::memory::{self, Memory, ProxyObject, Ptr};

// TODO --cleanup: remove if not needed
// pub enum FlagType {
//     Bool = 0,
//     S32 = 1,
//     F32 = 2,
//     String = 3,
//     String64 = 4,
//     String256 = 5,
//     Vector2f = 6,
//     Vector3f = 7,
//     Vector4f = 8,
//     BoolArray = 9,
//     S32Array = 10,
//     F32Array = 11,
//     StringArray = 12,
//     String64Array = 13,
//     String256Array = 14,
//     Vector2fArray = 15,
//     Vector3fArray = 16,
//     Vector4fArray = 17,
// }
pub fn get_hash(name: &str) -> i32 {
    crc32fast::hash(name.as_bytes()) as i32
}

/// Get the trigger param raw pointer from GDTM instance
pub fn trigger_param_ptr(memory: &Memory) -> Result<u64, memory::Error> {
    let gdt_manager = singleton_instance!(gdtm(memory))?;
    let params_ptr = Ptr!(&gdt_manager->mFlagBuffer);
    if params_ptr.is_nullptr() {
        return Ok(0);
    }
    params_ptr.load(memory)
}

#[derive(Debug, Clone, Default)]
pub struct TriggerParam {
    // these are pub(crate) for the flag descriptors to access them
    pub(crate) bool_flags: gdt::FlagList<bool>,
    pub(crate) s32_flags: gdt::FlagList<i32>,
    pub(crate) f32_flags: gdt::FlagList<f32>,
    pub(crate) string32_flags: gdt::FlagList<String>,
    pub(crate) string64_flags: gdt::FlagList<String>,
    pub(crate) string256_flags: gdt::FlagList<String>,
    pub(crate) vector2f_flags: gdt::FlagList<(f32, f32)>,
    pub(crate) vector3f_flags: gdt::FlagList<(f32, f32, f32)>,
    pub(crate) vector4f_flags: gdt::FlagList<(f32, f32, f32, f32)>,

    pub(crate) bool_array_flags: gdt::FlagList<Box<[bool]>>,
    pub(crate) s32_array_flags: gdt::FlagList<Box<[i32]>>,
    pub(crate) f32_array_flags: gdt::FlagList<Box<[f32]>>,
    // no str32[] in the game
    pub(crate) string64_array_flags: gdt::FlagList<Box<[String]>>,
    pub(crate) string256_array_flags: gdt::FlagList<Box<[String]>>,
    pub(crate) vector2f_array_flags: gdt::FlagList<Box<[(f32, f32)]>>,
    pub(crate) vector3f_array_flags: gdt::FlagList<Box<[(f32, f32, f32)]>>,
}

impl TriggerParam {
    /// Create a new trigger param instance with all flags loaded
    pub fn loaded() -> Self {
        Builder.build()
    }
    /// Get flag by CRC32 hash of its name
    pub fn by_hash<Fd: gdt::FlagDescriptor>(&self, hash: i32) -> Option<&gdt::Flag<Fd::T>> {
        self.get::<Fd, usize>(self.index_from_hash::<Fd>(hash)?)
    }

    /// Get flag by CRC32 hash of its name for mutation
    pub fn by_hash_mut<Fd: gdt::FlagDescriptor>(
        &mut self,
        hash: i32,
    ) -> Option<&mut gdt::Flag<Fd::T>> {
        self.get_mut::<Fd, usize>(self.index_from_hash::<Fd>(hash)?)
    }

    /// Get flag by its name
    pub fn by_name<Fd: gdt::FlagDescriptor>(
        &self,
        name: impl AsRef<str>,
    ) -> Option<&gdt::Flag<Fd::T>> {
        self.get::<Fd, usize>(self.index_from_name::<Fd>(name)?)
    }

    /// Get flag by its name for mutation
    pub fn by_name_mut<Fd: gdt::FlagDescriptor>(
        &mut self,
        name: impl AsRef<str>,
    ) -> Option<&mut gdt::Flag<Fd::T>> {
        self.get_mut::<Fd, usize>(self.index_from_name::<Fd>(name)?)
    }

    /// Get flag by index in the flag list
    pub fn get<Fd: gdt::FlagDescriptor, I: gdt::FlagIndex>(
        &self,
        idx: I,
    ) -> Option<&gdt::Flag<Fd::T>> {
        Fd::list(self).get(idx.to_index()?)
    }

    /// Get flag by index in the flag list for mutation
    pub fn get_mut<Fd: gdt::FlagDescriptor, I: gdt::FlagIndex>(
        &mut self,
        idx: I,
    ) -> Option<&mut gdt::Flag<Fd::T>> {
        Fd::list_mut(self).get_mut(idx.to_index()?)
    }

    /// Get the index of the flag from CRC32 hash of its name
    pub fn index_from_hash<Fd: gdt::FlagDescriptor>(&self, hash: i32) -> Option<usize> {
        Fd::list(self)
            .binary_search_by_key(&hash, |flag| flag.hash())
            .ok()
    }

    /// Get the index of the flag its name
    pub fn index_from_name<Fd: gdt::FlagDescriptor>(&self, name: impl AsRef<str>) -> Option<usize> {
        self.index_from_hash::<Fd>(get_hash(name.as_ref()))
    }

    /// Reset all flags to initial value
    pub fn reset_all(&mut self) {
        self.reset::<gdt::fd!(bool)>();
        self.reset::<gdt::fd!(s32)>();
        self.reset::<gdt::fd!(f32)>();
        self.reset::<gdt::fd!(str32)>();
        self.reset::<gdt::fd!(str64)>();
        self.reset::<gdt::fd!(str256)>();
        self.reset::<gdt::fd!(vec2f)>();
        self.reset::<gdt::fd!(vec3f)>();
        self.reset::<gdt::fd!(vec4f)>();
        self.reset::<gdt::fd!(bool[])>();
        self.reset::<gdt::fd!(s32[])>();
        self.reset::<gdt::fd!(f32[])>();
        self.reset::<gdt::fd!(str64[])>();
        self.reset::<gdt::fd!(str256[])>();
        self.reset::<gdt::fd!(vec2f[])>();
        self.reset::<gdt::fd!(vec3f[])>();
    }

    /// Reset all flags of a type to initial value
    pub fn reset<Fd: gdt::FlagDescriptor>(&mut self) {
        for flag in Fd::list_mut(self) {
            flag.reset();
        }
    }

    /// Load data from a save
    ///
    /// Only flags with the IsSave bit will be loaded. The other will be kept the same
    #[must_use = "returns false if failed"]
    pub fn load_save(&mut self, other: &Self) -> bool {
        self.load_save_for::<gdt::fd!(bool)>(other)
            && self.load_save_for::<gdt::fd!(s32)>(other)
            && self.load_save_for::<gdt::fd!(f32)>(other)
            && self.load_save_for::<gdt::fd!(str32)>(other)
            && self.load_save_for::<gdt::fd!(str64)>(other)
            && self.load_save_for::<gdt::fd!(str256)>(other)
            && self.load_save_for::<gdt::fd!(vec2f)>(other)
            && self.load_save_for::<gdt::fd!(vec3f)>(other)
            && self.load_save_for::<gdt::fd!(vec4f)>(other)
            && self.load_save_for::<gdt::fd!(bool[])>(other)
            && self.load_save_for::<gdt::fd!(s32[])>(other)
            && self.load_save_for::<gdt::fd!(f32[])>(other)
            && self.load_save_for::<gdt::fd!(str64[])>(other)
            && self.load_save_for::<gdt::fd!(str256[])>(other)
            && self.load_save_for::<gdt::fd!(vec2f[])>(other)
            && self.load_save_for::<gdt::fd!(vec3f[])>(other)
    }

    #[must_use = "returns false if failed"]
    fn load_save_for<Fd: gdt::FlagDescriptor>(&mut self, other: &Self) -> bool {
        let self_list = Fd::list_mut(self);
        let other_list = Fd::list(other);
        if self_list.len() != other_list.len() {
            log::error!(
                "fail to load save: length mismatch, self={}, other={}, descriptor={}",
                self_list.len(),
                other_list.len(),
                std::any::type_name::<Fd>()
            );
            return false;
        }
        for (s, o) in std::iter::zip(self_list.iter_mut(), other_list.iter()) {
            if s.hash() != o.hash() {
                log::error!(
                    "fail to load save: hash mismatch, self={}, other={}, descriptor={}",
                    s.hash(),
                    o.hash(),
                    std::any::type_name::<Fd>()
                );
                return false;
            }
            s.set(o.get().clone())
        }

        true
    }
}
impl ProxyObject for TriggerParam {
    fn mem_size(&self) -> u32 {
        0x3f0
    }
}

#[derive(Default)]
struct Builder;
impl Builder {
    fn build(self) -> gdt::TriggerParam {
        TriggerParam {
            bool_flags: blueflame_deps::gdt::unpack_bool_flags(),
            s32_flags: blueflame_deps::gdt::unpack_s32_flags(),
            f32_flags: blueflame_deps::generated::gdt::generate_F32_yaml_flags(),
            string32_flags: blueflame_deps::generated::gdt::generate_String32_yaml_flags(),
            string64_flags: blueflame_deps::generated::gdt::generate_String64_yaml_flags(),
            string256_flags: blueflame_deps::generated::gdt::generate_String256_yaml_flags(),
            vector2f_flags: blueflame_deps::generated::gdt::generate_Vector2f_yaml_flags(),
            vector3f_flags: blueflame_deps::generated::gdt::generate_Vector3f_yaml_flags(),
            vector4f_flags: blueflame_deps::generated::gdt::generate_Vector4f_yaml_flags(),
            bool_array_flags: blueflame_deps::generated::gdt::generate_ArrayBool_yaml_flags(),
            s32_array_flags: blueflame_deps::generated::gdt::generate_ArrayS32_yaml_flags(),
            f32_array_flags: blueflame_deps::generated::gdt::generate_ArrayF32_yaml_flags(),
            string64_array_flags: blueflame_deps::generated::gdt::generate_ArrayString64_yaml_flags(
            ),
            string256_array_flags:
                blueflame_deps::generated::gdt::generate_ArrayString256_yaml_flags(),
            vector2f_array_flags: blueflame_deps::generated::gdt::generate_ArrayVector2f_yaml_flags(
            ),
            vector3f_array_flags: blueflame_deps::generated::gdt::generate_ArrayVector3f_yaml_flags(
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sorted_bool() {
        let params = TriggerParam::loaded();

        let mut vec = params.bool_flags.clone();
        vec.sort_by_key(|flag| flag.hash());
        assert_eq!(vec, params.bool_flags);
        let mut vec = params.s32_flags.clone();
        vec.sort_by_key(|flag| flag.hash());
        assert_eq!(vec, params.s32_flags);
        let mut vec = params.f32_flags.clone();
        vec.sort_by_key(|flag| flag.hash());
        assert_eq!(vec, params.f32_flags);
        let mut vec = params.string32_flags.clone();
        vec.sort_by_key(|flag| flag.hash());
        assert_eq!(vec, params.string32_flags);
        let mut vec = params.string64_flags.clone();
        vec.sort_by_key(|flag| flag.hash());
        assert_eq!(vec, params.string64_flags);
        let mut vec = params.string256_flags.clone();
        vec.sort_by_key(|flag| flag.hash());
        assert_eq!(vec, params.string256_flags);
        let mut vec = params.vector2f_flags.clone();
        vec.sort_by_key(|flag| flag.hash());
        assert_eq!(vec, params.vector2f_flags);
        let mut vec = params.vector3f_flags.clone();
        vec.sort_by_key(|flag| flag.hash());
        assert_eq!(vec, params.vector3f_flags);
        let mut vec = params.vector4f_flags.clone();
        vec.sort_by_key(|flag| flag.hash());
        assert_eq!(vec, params.vector4f_flags);
        let mut vec = params.bool_array_flags.clone();
        vec.sort_by_key(|flag| flag.hash());
        assert_eq!(vec, params.bool_array_flags);
        let mut vec = params.s32_array_flags.clone();
        vec.sort_by_key(|flag| flag.hash());
        assert_eq!(vec, params.s32_array_flags);
        let mut vec = params.f32_array_flags.clone();
        vec.sort_by_key(|flag| flag.hash());
        assert_eq!(vec, params.f32_array_flags);
        let mut vec = params.string64_array_flags.clone();
        vec.sort_by_key(|flag| flag.hash());
        assert_eq!(vec, params.string64_array_flags);
        let mut vec = params.string256_array_flags.clone();
        vec.sort_by_key(|flag| flag.hash());
        assert_eq!(vec, params.string256_array_flags);
        let mut vec = params.vector2f_array_flags.clone();
        vec.sort_by_key(|flag| flag.hash());
        assert_eq!(vec, params.vector2f_array_flags);
        let mut vec = params.vector3f_array_flags.clone();
        vec.sort_by_key(|flag| flag.hash());
        assert_eq!(vec, params.vector3f_array_flags);
    }
    #[test]
    fn test_init() -> anyhow::Result<()> {
        let params = gdt::TriggerParam::loaded();
        let flag1 = params
            .by_hash::<gdt::fd!(bool)>(530692287)
            .expect("flag not found");
        assert!(!flag1.get());
        // assert_eq!(flag1.name(), "BarrelErrand_Intro_Finished");

        let flag2 = params
            .by_hash::<gdt::fd!(bool[])>(-1649503087)
            .expect("flag not found");
        assert_eq!(
            &Box::from([false, false, false, false, false, false, false, false]),
            flag2.get()
        );
        // assert_eq!("dummy_bool_array", flag2.name());

        let flag3 = params
            .by_hash::<gdt::fd!(vec3f)>(-1542741757)
            .expect("flag not found");
        assert_eq!((-1130.0, 237.4, 1914.5), *flag3.get());
        // assert_eq!("PlayerSavePos", flag3.name());

        let flag4 = params
            .by_hash::<gdt::fd!(bool)>(595714052)
            .expect("flag not found");
        assert!(!flag4.get());
        // assert_eq!("MainField_LinkTagAnd_02894606454", flag4.name());

        let flag = params
            .by_name::<gdt::fd!(s32)>("KorokNutsNum")
            .expect("flag not found");
        assert_eq!(*flag.get(), 0);
        Ok(())
    }

    #[test]
    fn test_get_set_reset() -> Result<(), Box<dyn std::error::Error>> {
        let mut params = gdt::TriggerParam::loaded();

        let flag = params
            .by_hash_mut::<gdt::fd!(bool)>(530692287)
            .expect("flag not found");
        flag.set(true);
        assert!(flag.get());
        flag.reset();
        assert!(!flag.get());
        Ok(())
    }
}
