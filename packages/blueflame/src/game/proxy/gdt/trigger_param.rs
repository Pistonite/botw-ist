#[layered_crate::import]
use game::{
    super::memory::{self, Memory, Ptr, ProxyObject},
    self::{gdt, singleton_instance},
};


// #[derive(Error, Debug)]
// pub enum TriggerParamError {
//     #[error("Flag not found for index {0}")]
//     FlagNotFoundIndex(usize),
//     #[error("Flag not found for hash {0}")]
//     FlagNotFoundHash(i32),
// }

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
    pub bool_flags: gdt::FlagList<bool>,
    pub s32_flags: gdt::FlagList<i32>,
    pub f32_flags: gdt::FlagList<f32>,
    pub string32_flags: gdt::FlagList<String>,
    pub string64_flags: gdt::FlagList<String>,
    pub string256_flags: gdt::FlagList<String>,
    pub vector2f_flags: gdt::FlagList<(f32, f32)>,
    pub vector3f_flags: gdt::FlagList<(f32, f32, f32)>,
    pub vector4f_flags: gdt::FlagList<(f32, f32, f32, f32)>,


    pub bool_array_flags: gdt::FlagList<Box<[bool]>>,
    pub s32_array_flags: gdt::FlagList<Box<[i32]>>,
    pub f32_array_flags: gdt::FlagList<Box<[f32]>>,

    pub string64_array_flags: gdt::FlagList<Box<[String]>>,
    pub string256_array_flags: gdt::FlagList<Box<[String]>>,
    pub vector2f_array_flags: gdt::FlagList<Box<[(f32, f32)]>>,
    pub vector3f_array_flags: gdt::FlagList<Box<[(f32, f32, f32)]>>,
}

impl TriggerParam {
    /// Create a new trigger param instance with all flags loaded
    pub fn loaded() -> Self {
        let mut builder = Builder::default();
        if let Err(e) = builder.load_all_flags() {
            panic!("Failed to load trigger param flags: {}", e);
        }
        builder.build()
    }
    /// Get flag by CRC32 hash of its name
    pub fn by_hash<Fd: gdt::FlagDescriptor>(&self, hash: i32) -> Option<&gdt::Flag<Fd::T>> {
        self.get::<Fd, usize>(self.index_from_hash::<Fd>(hash)?)
    }

    /// Get flag by CRC32 hash of its name for mutation
    pub fn by_hash_mut<Fd: gdt::FlagDescriptor>(&mut self, hash: i32) -> Option<&mut gdt::Flag<Fd::T>> {
        self.get_mut::<Fd, usize>(self.index_from_hash::<Fd>(hash)?)
    }

    /// Get flag by its name
    pub fn by_name<Fd: gdt::FlagDescriptor>(&self, name: impl AsRef<str>) -> Option<&gdt::Flag<Fd::T>> {
        self.get::<Fd, usize>(self.index_from_name::<Fd>(name)?)
    }

    /// Get flag by its name for mutation
    pub fn by_name_mut<Fd: gdt::FlagDescriptor>(&mut self, name: impl AsRef<str>) -> Option<&mut gdt::Flag<Fd::T>> {
        self.get_mut::<Fd, usize>(self.index_from_name::<Fd>(name)?)
    }

    /// Get flag by index in the flag list
    pub fn get<Fd: gdt::FlagDescriptor, I: gdt::FlagIndex>(&self, idx: I) -> Option<&gdt::Flag<Fd::T>> {
        Fd::list(self).get(idx.to_index()?)
    }


    /// Get flag by index in the flag list for mutation
    pub fn get_mut<Fd: gdt::FlagDescriptor, I: gdt::FlagIndex>(&mut self, idx: I) -> Option<&mut gdt::Flag<Fd::T>> {
        Fd::list_mut(self).get_mut(idx.to_index()?)
    }


    /// Get the index of the flag from CRC32 hash of its name
    pub fn index_from_hash<Fd: gdt::FlagDescriptor>(&self, hash: i32) -> Option<usize> {
        Fd::list(self).binary_search_by_key(&hash, |flag| flag.hash()).ok()
    }
    
    /// Get the index of the flag its name
    pub fn index_from_name<Fd: gdt::FlagDescriptor>(&self, name: impl AsRef<str>) -> Option<usize> {
        self.index_from_hash::<Fd>(gdt::get_hash(name.as_ref()))
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

    /// Get the length of the flags array of the type
    pub fn len<Fd: gdt::FlagDescriptor>(&self) -> usize {
        Fd::list(self).len()
    }
}
impl ProxyObject for TriggerParam {
    fn mem_size(&self) -> u32 {
        0x3f0
    }
}

macro_rules! flag_yaml_file {
    ($name:ident) => {
        include_str!(concat!("../../../../res/Flag/", stringify!($name), "_data.yml"))
    };
    ($name:ident array) => {
        include_str!(concat!("../../../../res/Flag/", stringify!($name), "_array_data.yml"))
    };
}
#[derive(Default)]
struct Builder(gdt::TriggerParam);
impl Builder {
    fn push<Fd: gdt::FlagDescriptor>(&mut self, flag: gdt::Flag<Fd::T>) {
        Fd::list_mut(&mut self.0).push(flag);
    }
    fn build(mut self) -> gdt::TriggerParam {
        self.0.bool_flags.sort_by_key(|flag| flag.hash());
        self.0.s32_flags.sort_by_key(|flag| flag.hash());
        self.0.f32_flags.sort_by_key(|flag| flag.hash());
        self.0.string32_flags.sort_by_key(|flag| flag.hash());
        self.0.string64_flags.sort_by_key(|flag| flag.hash());
        self.0.string256_flags.sort_by_key(|flag| flag.hash());
        self.0.vector2f_flags.sort_by_key(|flag| flag.hash());
        self.0.vector3f_flags.sort_by_key(|flag| flag.hash());
        self.0.vector4f_flags.sort_by_key(|flag| flag.hash());
        self.0.bool_array_flags.sort_by_key(|flag| flag.hash());
        self.0.s32_array_flags.sort_by_key(|flag| flag.hash());
        self.0.f32_array_flags.sort_by_key(|flag| flag.hash());
        self.0.string64_array_flags.sort_by_key(|flag| flag.hash());
        self.0.string256_array_flags.sort_by_key(|flag| flag.hash());
        self.0.vector2f_array_flags.sort_by_key(|flag| flag.hash());
        self.0.vector3f_array_flags.sort_by_key(|flag| flag.hash());
        self.0
    }

    fn load_all_flags(&mut self) -> anyhow::Result<()> {
        fn parse_bool(v: &serde_yaml::Value) -> anyhow::Result<bool> {
            if let Some(val) = v.as_bool() {
                return Ok(val);
            }
            match v.as_i64() {
                Some(val) => return Ok(val & 1 == 1),
                None => Err(anyhow::anyhow!("Expected boolean value, got: {:?}", v)),
            }
        }
        fn parse_i32(v: &serde_yaml::Value) -> anyhow::Result<i32> {
            if let Some(val) = v.as_i64() {
                return Ok(val as i32);
            }
            match v.as_f64() {
                Some(val) => return Ok(val as i32),
                None => Err(anyhow::anyhow!("Expected integer value, got: {:?}", v)),
            }
        }
        fn parse_f32(v: &serde_yaml::Value) -> anyhow::Result<f32> {
            if let Some(val) = v.as_f64() {
                return Ok(val as f32);
            }
            match v.as_i64() {
                Some(val) => return Ok(val as f32),
                None => Err(anyhow::anyhow!("Expected float value, got: {:?}", v)),
            }
        }
        fn parse_string(v: &serde_yaml::Value) -> anyhow::Result<String> {
            if let Some(val) = v.as_str() {
                return Ok(val.to_string());
            }
            anyhow::bail!("Expected string value, got: {:?}", v)
        }
        fn parse_vec2f(v: &serde_yaml::Value) -> anyhow::Result<(f32, f32)> {
            if let Some(arr) = v.as_sequence() {
                let arr = arr[0].as_sequence()
                    .ok_or_else(|| anyhow::anyhow!("Expected array for vector2f, got: {:?}", v))?;
                if arr.len() != 2 {
                    anyhow::bail!("Expected array of length 2 for vector2f, got: {:?}", v);
                }
                let x = parse_f32(&arr[0])?;
                let y = parse_f32(&arr[1])?;
                return Ok((x, y));
            }
            anyhow::bail!("Expected array for vector2f, got: {:?}", v);
        }
        fn parse_vec3f(v: &serde_yaml::Value) -> anyhow::Result<(f32, f32, f32)> {
            if let Some(arr) = v.as_sequence() {
                let arr = arr[0].as_sequence()
                    .ok_or_else(|| anyhow::anyhow!("Expected array for vector3f, got: {:?}", v))?;
                if arr.len() != 3 {
                    anyhow::bail!("Expected array of length 3 for vector3f, got: {:?}", v);
                }
                let x = parse_f32(&arr[0])?;
                let y = parse_f32(&arr[1])?;
                let z = parse_f32(&arr[2])?;
                return Ok((x, y, z));
            }
            anyhow::bail!("Expected array for vector3f, got: {:?}", v);
        }
        fn parse_vec4f(v: &serde_yaml::Value) -> anyhow::Result<(f32, f32, f32, f32)> {
            if let Some(arr) = v.as_sequence() {
                let arr = arr[0].as_sequence()
                    .ok_or_else(|| anyhow::anyhow!("Expected array for vector4f, got: {:?}", v))?;
                if arr.len() != 4 {
                    anyhow::bail!("Expected array of length 4 for vector4f, got: {:?}", v);
                }
                let w = parse_f32(&arr[0])?;
                let x = parse_f32(&arr[1])?;
                let y = parse_f32(&arr[2])?;
                let z = parse_f32(&arr[3])?;
                return Ok((w, x, y, z));
            }
            anyhow::bail!("Expected array for vector4f, got: {:?}", v);
        }

        self.load_flags::<gdt::fd!(bool)>(flag_yaml_file!(bool), "bool_data", parse_bool)?;
        self.load_flags::<gdt::fd!(bool)>(flag_yaml_file!(revival_bool), "bool_data", parse_bool)?;
        self.load_flags::<gdt::fd!(s32)>(flag_yaml_file!(s32), "s32_data", parse_i32)?;
        self.load_flags::<gdt::fd!(s32)>(flag_yaml_file!(revival_s32), "s32_data", parse_i32)?;
        self.load_flags::<gdt::fd!(f32)>(flag_yaml_file!(f32), "f32_data", parse_f32)?;
        self.load_flags::<gdt::fd!(str32)>(flag_yaml_file!(string32), "string_data", parse_string)?;
        self.load_flags::<gdt::fd!(str64)>(flag_yaml_file!(string64), "string64_data", parse_string)?;
        self.load_flags::<gdt::fd!(str256)>(flag_yaml_file!(string256), "string256_data", parse_string)?;
        self.load_flags::<gdt::fd!(vec2f)>(flag_yaml_file!(vector2f), "vector2f_data", parse_vec2f)?;
        self.load_flags::<gdt::fd!(vec3f)>(flag_yaml_file!(vector3f), "vector3f_data", parse_vec3f)?;
        self.load_flags::<gdt::fd!(vec4f)>(flag_yaml_file!(vector4f), "vector4f_data", parse_vec4f)?;
        self.load_flags_array::<gdt::fd!(bool[])>(flag_yaml_file!(bool array), "bool_array_data", parse_bool)?;
        self.load_flags_array::<gdt::fd!(s32[])>(flag_yaml_file!(s32 array), "s32_array_data", parse_i32)?;
        self.load_flags_array::<gdt::fd!(f32[])>(flag_yaml_file!(f32 array), "f32_array_data", parse_f32)?;
        self.load_flags_array::<gdt::fd!(str64[])>(flag_yaml_file!(string64 array), "string64_array_data", parse_string)?;
        self.load_flags_array::<gdt::fd!(str256[])>(flag_yaml_file!(string256 array), "string256_array_data", parse_string)?;
        self.load_flags_array::<gdt::fd!(vec2f[])>(flag_yaml_file!(vector2f array), "vector2f_array_data", parse_vec2f)?;
        self.load_flags_array::<gdt::fd!(vec3f[])>(flag_yaml_file!(vector3f array), "vector3f_array_data", parse_vec3f)?;

        // TODO: are those loaded without DLC?
        // Add additional flags, missing from the yaml
        let flag = gdt::Flag::from_name_value("AoC_DragonFireChallengeRing_Advent", false);
        self.0.bool_flags.push(flag);
        let flag = gdt::Flag::from_name_value("AoC_RandomSpawnTreasure_Contents", Box::from([]));
        self.0.string64_array_flags.push(flag);
        let flag = gdt::Flag::from_name_value("AoC_RandomSpawnTreasure_IsRandomized", false);
        self.0.bool_flags.push(flag);
        let flag = gdt::Flag::from_name_value("AoC_TestProg_Imoto_Flag_00", false);
        self.0.bool_flags.push(flag);
        let flag = gdt::Flag::from_name_value("AoC_TestProg_Imoto_TagCount_00", 0);
        self.0.s32_flags.push(flag);
        let flag = gdt::Flag::from_name_value("AocTestEx_Omosako_IsPastWorld", false);
        self.0.bool_flags.push(flag);
        let flag = gdt::Flag::from_name_value("AocTestEx_Omosako_ReturnToMainField_Position", (0f32, 0f32, 0f32));
        self.0.vector3f_flags.push(flag);
        let flag = gdt::Flag::from_name_value("AocTestEx_Omosako_ReturnToMainField_Rotation", 0f32);
        self.0.f32_flags.push(flag);
        let flag = gdt::Flag::from_name_value("AocTestEx_Omosako_SandOfTime_Num", 0);
        self.0.s32_flags.push(flag);
        let flag = gdt::Flag::from_name_value("AocTestEx_Omosako_SandOfTime_Rate", 0f32);
        self.0.f32_flags.push(flag);
        let flag = gdt::Flag::from_name_value("Location_DarkDungeon01", 0);
        self.0.s32_flags.push(flag);
        let flag = gdt::Flag::from_name_value("Location_DarkDungeon02", 0);
        self.0.s32_flags.push(flag);
        let flag = gdt::Flag::from_name_value("Location_DarkDungeon03", 0);
        self.0.s32_flags.push(flag);
        let flag = gdt::Flag::from_name_value("Location_DarkDungeon04", 0);
        self.0.s32_flags.push(flag);
        let flag = gdt::Flag::from_name_value("SpurGear_revolve_01", false);
        self.0.bool_flags.push(flag);
        let flag = gdt::Flag::from_name_value("SpurGear_revolve_02", false);
        self.0.bool_flags.push(flag);

        Ok(())
    }

    // TODO --cleanup: replace yaml stuff with pre generated data
    //
    fn load_flags<Fd: gdt::FlagDescriptor>(
        &mut self, yaml_content: &str,
        top_name: &str,
        init_value_loader: fn(&serde_yaml::Value) -> anyhow::Result<Fd::T>,
    ) -> anyhow::Result<()> {
        let value: serde_yaml::Value = serde_yaml::from_str(yaml_content)?;
        let data_list = Self::read_value_seq(&value, top_name)?;
        for entry in data_list {
            let name = Self::read_value_str(entry, "DataName")?;
            let hash = Self::read_value_i64(entry, "HashValue")?;
            let readable = Self::read_value_bool(entry, "IsProgramReadable")?;
            let writeable = Self::read_value_bool(entry, "IsProgramWritable")?;
            let init_value = entry.get("InitValue")
                .ok_or_else(|| anyhow::anyhow!("InitValue not found in entry"))?;
            let init = init_value_loader(init_value)?;
            let flag = gdt::Flag::new(
                init,
                name.to_string(),
                hash as i32,
                readable,
                writeable,
            );
            self.push::<Fd>(flag);
        }
        Ok(())
    }

    fn load_flags_array<Fd: gdt::ArrayFlagDescriptor>(
        &mut self, yaml_content: &str,
        top_name: &str,
        init_value_loader: fn(&serde_yaml::Value) -> anyhow::Result<Fd::ElemT>,
    ) -> anyhow::Result<()> {
        let value: serde_yaml::Value = serde_yaml::from_str(yaml_content)?;
        let data_list = Self::read_value_seq(&value, top_name)?;
        for entry in data_list {
            let name = Self::read_value_str(entry, "DataName")?;
            let hash = Self::read_value_i64(entry, "HashValue")?;
            let readable = Self::read_value_bool(entry, "IsProgramReadable")?;
            let writeable = Self::read_value_bool(entry, "IsProgramWritable")?;
            let init_value = Self::read_value_seq(entry, "InitValue")?;
            let first = &init_value[0];
            let value_seq = Self::read_value_seq(first, "Values")?;
            let mut init_values = Vec::with_capacity(value_seq.len());
            for v in value_seq {
                init_values.push(init_value_loader(v)?);
            }
            let flag: gdt::Flag<Box<[Fd::ElemT]>> = gdt::Flag::new(
                init_values.into_boxed_slice(),
                String::from(name),
                hash as i32,
                readable,
                writeable,
            );
            self.push::<Fd>(flag);
        }
        Ok(())
    }

    fn read_value_seq<'a>(value: &'a serde_yaml::Value, name: &str) -> anyhow::Result<&'a Vec<serde_yaml::Value>> {
        let result = value.get(name).and_then(|v| v.as_sequence());
        if let Some(v) = result {
            Ok(v)
        } else {
            anyhow::bail!(format!("{} is not a sequence!", name))
        }
    }

    fn read_value_str<'a>( value: &'a serde_yaml::Value, name: &str,) -> anyhow::Result<&'a str> {
        let result = value.get(name).and_then(|v| v.as_str());
        if let Some(v) = result {
            Ok(v)
        } else {
            anyhow::bail!(format!("{} is not a string!", name))
        }
    }
    fn read_value_i64(value: &serde_yaml::Value, name: &str) -> anyhow::Result<i64> {
        let result = value.get(name).and_then(|v| v.as_i64());
        if let Some(v) = result {
            Ok(v)
        } else {
            anyhow::bail!(format!("{} is not an integer!", name))
        }
    }
    fn read_value_f64(value: &serde_yaml::Value, name: &str) -> anyhow::Result<f64> {
        let result = value.get(name).and_then(|v| v.as_f64());
        if let Some(v) = result {
            Ok(v)
        } else {
            anyhow::bail!(format!("{} is not a float!", name))
        }
    }
    fn read_value_bool(value: &serde_yaml::Value, name: &str) -> anyhow::Result<bool> {
        let result = value.get(name).and_then(|v| v.as_bool());
        if let Some(v) = result {
            Ok(v)
        } else {
            anyhow::bail!(format!("{} is not a boolean!", name))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_init() -> anyhow::Result<()> {
        let params = gdt::TriggerParam::loaded();
        let flag1 = params.by_hash::<gdt::fd!(bool)>(530692287).expect("flag not found");
        assert!(!flag1.get());
        assert_eq!(flag1.name(), "BarrelErrand_Intro_Finished");

        let flag2 = params.by_hash::<gdt::fd!(bool[])>(-1649503087).expect("flag not found");
        assert_eq!(&Box::from([false, false, false, false, false, false, false, false]), flag2.get());
        assert_eq!("dummy_bool_array", flag2.name());

        let flag3 = params.by_hash::<gdt::fd!(vec3f)>(-1542741757).expect("flag not found");
        assert_eq!((-1130.0, 237.4, 1914.5), *flag3.get());
        assert_eq!("PlayerSavePos", flag3.name());

        let flag4 = params.by_hash::<gdt::fd!(bool)>(595714052).expect("flag not found");
        assert!(!flag4.get());
        assert_eq!("MainField_LinkTagAnd_02894606454", flag4.name());

        Ok(())
    }

    #[test]
    fn test_get_set_reset() -> Result<(), Box<dyn std::error::Error>> {
        let mut params = gdt::TriggerParam::loaded();

        let flag = params.by_hash_mut::<gdt::fd!(bool)>(530692287).expect("flag not found");
        flag.set(true);
        assert!(flag.get());
        flag.reset();
        assert!(!flag.get());
        Ok(())
    }
}
