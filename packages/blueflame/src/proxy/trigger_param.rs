use crate::Core;
use crate::{
    memory::{
        traits::{FromRegisterVal, MemWrite, Ptr},
        Memory, ProxyObject,
    },
    processor::instruction_registry::RegisterType,
};
use anyhow::Error;
use serde_yaml::Value;
use thiserror::Error;

type FlagList<T> = Vec<Flag<T>>;

#[derive(Error, Debug)]
pub enum TriggerParamError {
    #[error("Flag not found for index {0}")]
    FlagNotFoundIndex(usize),
    #[error("Flag not found for hash {0}")]
    FlagNotFoundHash(i32),
}

#[derive(Clone, Default)]
pub struct GdtTriggerParam {
    bool_flags: FlagList<bool>,
    bool_array_flags: FlagList<Box<[bool]>>,
    f32_flags: FlagList<f32>,
    f32_array_flags: FlagList<Box<[f32]>>,
    s32_flags: FlagList<i32>,
    s32_array_flags: FlagList<Box<[i32]>>,
    string256_flags: FlagList<String>,
    string256_array_flags: FlagList<Box<[String]>>,
    string32_flags: FlagList<String>,
    string64_flags: FlagList<String>,
    string64_array_flags: FlagList<Box<[String]>>,
    // Note: Vectors stored directly as one value after the other (like a struct { x: f32, y: f32 }),
    // so using tuples is valid since we read/write them to memory in the same format
    vector2f_flags: FlagList<(f32, f32)>,
    vector2f_array_flags: FlagList<Box<[(f32, f32)]>>,
    vector3f_flags: FlagList<(f32, f32, f32)>,
    vector3f_array_flags: FlagList<Box<[(f32, f32, f32)]>>,
    vector4f_flags: FlagList<(f32, f32, f32, f32)>,
}

macro_rules! common_flag_funcs {
    ($name:ident, $typ:ty) => {
        paste::item! {
            pub fn [< get_ $name _flag_by_index >](&self, index: usize) -> &Flag<$typ> {
                self.[<$name _flags>].get(index).unwrap()
            }
            pub fn [< get_ $name _flag_by_index_mut >](&mut self, index: usize) -> &mut Flag<$typ> {
                self.[<$name _flags>].get_mut(index).unwrap()
            }

            pub fn [< get_ $name _flag_index_from_name >](&self, name: String) -> Option<usize> {
                let hash = crc32fast::hash(name.as_str().as_bytes());
                let hash_signed = i32::from_ne_bytes(hash.to_ne_bytes());
                self.[<get_ $name _flag_index_from_hash>](hash_signed)
            }

            pub fn [< get_ $name _flag_index_from_hash >](&self, hash: i32) -> Option<usize> {
                let flag = self.[<$name _flags>].binary_search_by(|x| x.hash().cmp(&hash));
                if let Ok(idx) = flag {
                    Some(idx)
                } else {
                    None
                }
            }

            pub fn [< get_ $name _flag_by_name >](&self, name: String) -> Option<&Flag<$typ>> {
                let res = self.[< get_ $name _flag_index_from_name >](name);
                if let Some(idx) = res {
                    Some(self.[<$name _flags>].get(idx).unwrap())
                } else {
                    None
                }
            }
            pub fn [< get_ $name _flag_by_name_mut >](&mut self, name: String) -> Option<&mut Flag<$typ>> {
                let res = self.[< get_ $name _flag_index_from_name >](name);
                if let Some(idx) = res {
                    Some(self.[<$name _flags>].get_mut(idx).unwrap())
                } else {
                    None
                }
            }

            pub fn [<add_ $name _flag>](&mut self, flag: Flag<$typ>) {
                self.[<$name _flags>].push(flag);
            }

            pub fn [<get_ $name _index>](core: &mut Core<'_, '_, '_>) -> Result<(), Error> {
                let this_addr = u64::from_register_val(core.cpu.read_arg(0), core.mem)?;
                let this: &GdtTriggerParam = core.proxies.get_trigger_param(core.mem, this_addr)?;
                let hash = core.cpu.read_arg(1) as i32;
                let hash = i32::from_le_bytes(hash.to_le_bytes()[..4].try_into().unwrap());
                let index = this.[<get_ $name _flag_index_from_hash>](hash);
                if let Some(idx) = index {
                    core.cpu.write_gen_reg(&RegisterType::XReg(0), idx as i64)?;
                    Ok(())
                } else {
                    Err(TriggerParamError::FlagNotFoundHash(hash).into())
                }
            }
        }
    }
}

macro_rules! array_flag_funcs {
    ($name:ident, $typ:ty) => {
        paste::item! {
            pub fn [<get_ $name _value>](&self, name: &str, index: usize) -> Option<$typ> {
                let flag_list = self.[< get_ $name _flag_by_name >](String::from(name))?;
                let list = flag_list.get();
                if index < list.len() {
                    Some(list[index].clone())
                } else {
                    None
                }
            }
        }
    };
}

macro_rules! processor_flag_funcs {
    ($name:ident, $typ:ty, $mem_type:ty) => {
        paste::item! {
            pub fn [<get_ $name>](core: &mut Core<'_, '_, '_>) -> Result<(), Error> {
                let this_addr = u64::from_register_val(core.cpu.read_arg(0), core.mem)?;
                let this: &GdtTriggerParam = core.proxies.get_trigger_param(core.mem, this_addr)?;
                let ptr = Ptr::<$mem_type>::from_register_val(core.cpu.read_arg(1), core.mem)?;
                let idx = core.cpu.read_arg(2);
                let check_perms = bool::from_register_val(core.cpu.read_arg(3), core.mem)?;
                if idx < 0 {
                    core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                    return Ok(());
                }
                let idx = usize::from_register_val(idx, core.mem)?;
                if idx >= this.[<$name _flags>].len() {
                    core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                    return Ok(());
                }

                let flag = this
                    .[<$name _flags>]
                    .get(idx)
                    .ok_or(TriggerParamError::FlagNotFoundIndex(idx))?;
                if !flag.is_program_readable && check_perms {
                    core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                    return Ok(());
                }

                let result_value = flag.get().generate_mem_value(core.mem)?;
                ptr.store(core.mem, result_value)?;
                core.cpu.write_gen_reg(&RegisterType::XReg(0), 1)?;
                Ok(())
            }

            pub fn [<set_ $name>](core: &mut Core<'_, '_, '_>) -> Result<(), Error> {
                let this_addr = u64::from_register_val(core.cpu.read_arg(0), core.mem)?;
                let this: &mut GdtTriggerParam = core.proxies.mut_trigger_param(core.mem, this_addr)?;
                let value = <$typ>::from_register_val(core.cpu.read_arg(1), core.mem)?;
                let idx = core.cpu.read_arg(2);
                let check_perms = bool::from_register_val(core.cpu.read_arg(3), core.mem)?;
                let _bypass_one_trigger_check = bool::from_register_val(core.cpu.read_arg(4), core.mem)?;
                if idx < 0 {
                    core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                    return Ok(());
                }
                let idx = usize::from_register_val(idx, core.mem)?;
                if idx >= this.[<$name _flags>].len() {
                    core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                    return Ok(());
                }

                let flag = this
                    .[<$name _flags>]
                    .get_mut(idx)
                    .ok_or(TriggerParamError::FlagNotFoundIndex(idx))?;
                if !flag.is_program_writeable && check_perms {
                    core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                    return Ok(());
                }

                flag.set(value);
                core.cpu.write_gen_reg(&RegisterType::XReg(0), 1)?;
                Ok(())
            }

            pub fn [<set_ $name _safe_string>](core: &mut Core<'_, '_, '_>) -> Result<(), Error> {
                let this_addr = u64::from_register_val(core.cpu.read_arg(0), core.mem)?;
                let this: &mut GdtTriggerParam = core.proxies.mut_trigger_param(core.mem, this_addr)?;
                let value = <$typ>::from_register_val(core.cpu.read_arg(1), core.mem)?;
                let name_addr = u64::from_register_val(core.cpu.read_arg(2), core.mem)?;
                let name = core.mem.mem_read_safe_string(name_addr)?;
                let check_perms = bool::from_register_val(core.cpu.read_arg(3), core.mem)?;
                let _ = bool::from_register_val(core.cpu.read_arg(4), core.mem)?; // unnamed in decompilation
                let _bypass_one_trigger_check = bool::from_register_val(core.cpu.read_arg(5), core.mem)?;

                let flag_opt = this
                    .[<get_ $name _flag_by_name_mut>](name);
                if let Some(flag) = flag_opt {
                    if !flag.is_program_writeable && check_perms {
                        core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                        return Ok(());
                    } else {
                        flag.set(value);
                        core.cpu.write_gen_reg(&RegisterType::XReg(0), 1)?;
                        Ok(())
                    }
                } else {
                    core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                    Ok(())
                }
            }

            pub fn [<reset_ $name>](core: &mut Core<'_, '_, '_>) -> Result<(), Error> {
                let this_addr = u64::from_register_val(core.cpu.read_arg(0), core.mem)?;
                let this: &mut GdtTriggerParam = core.proxies.mut_trigger_param(core.mem, this_addr)?;
                let idx = core.cpu.read_arg(1);
                let check_perms = bool::from_register_val(core.cpu.read_arg(2), core.mem)?;
                if idx < 0 {
                    core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                    return Ok(());
                }
                let idx = usize::from_register_val(idx, core.mem)?;
                if idx >= this.[<$name _flags>].len() {
                    core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                    return Ok(());
                }

                let flag = this
                    .[<$name _flags>]
                    .get_mut(idx)
                    .ok_or(TriggerParamError::FlagNotFoundIndex(idx))?;
                if !flag.is_program_writeable && check_perms {
                    core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                    return Ok(());
                }

                flag.reset();

            core.cpu.write_gen_reg(&RegisterType::XReg(0), 1)?;
            Ok(())
        }
        }
    };
}

macro_rules! processor_flag_array_funcs {
    ($name:ident, $typ:ty, $mem_type:ty) => {
        paste::item! {
            pub fn [<get_ $name _array>](core: &mut Core<'_, '_, '_>) -> Result<(), Error> {
                let this_addr = u64::from_register_val(core.cpu.read_arg(0), core.mem)?;
                let this: &GdtTriggerParam = core.proxies.get_trigger_param(core.mem, this_addr)?;
                let ptr = Ptr::<$mem_type>::from_register_val(core.cpu.read_arg(1), core.mem)?;
                let array_idx = core.cpu.read_arg(2);
                let idx = core.cpu.read_arg(3);

                if array_idx < 0 {
                    core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                    return Ok(());
                }
                let array_idx = usize::from_register_val(array_idx, core.mem)?;
                if array_idx >= this.[<$name _flags>].len() {
                    core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                    return Ok(());
                }

                let flag = this
                    .[<$name _flags>]
                    .get(array_idx)
                    .ok_or(TriggerParamError::FlagNotFoundIndex(array_idx))?;
                if idx < 0 {
                    core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                    return Ok(());
                }
                let idx = usize::from_register_val(idx, core.mem)?;
                if idx >= flag.get().len() {
                    core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                    return Ok(());
                }

                let result_value = flag.get().get(idx).unwrap().generate_mem_value(core.mem)?;
                ptr.store(core.mem, result_value)?;

                core.cpu.write_gen_reg(&RegisterType::XReg(0), 1)?;
                Ok(())
            }

            pub fn [<reset_ $name _array>](core: &mut Core<'_, '_, '_>) -> Result<(), Error> {
                let this_addr = u64::from_register_val(core.cpu.read_arg(0), core.mem)?;
                let this: &mut GdtTriggerParam = core.proxies.mut_trigger_param(core.mem, this_addr)?;
                let idx = core.cpu.read_arg(1);
                let sub_idx = core.cpu.read_arg(2);
                let check_perms = bool::from_register_val(core.cpu.read_arg(3), core.mem)?;
                if idx < 0 {
                    core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                    return Ok(());
                }
                let idx = usize::from_register_val(idx, core.mem)?;
                if idx >= this.[<$name _flags>].len() {
                    core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                    return Ok(());
                }

                let flag = this
                    .[<$name _flags>]
                    .get_mut(idx)
                    .ok_or(TriggerParamError::FlagNotFoundIndex(idx))?;
                if sub_idx < 0 {
                    core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                    return Ok(());
                }
                let sub_idx = usize::from_register_val(sub_idx, core.mem)?;
                if sub_idx >= flag.value.len() {
                    core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                    return Ok(());
                }

                if !flag.is_program_writeable && check_perms {
                    core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                    return Ok(());
                }

                flag.reset_idx(sub_idx);

                core.cpu.write_gen_reg(&RegisterType::XReg(0), 1)?;
                Ok(())
            }

            pub fn [<get_ $name _array_size>](core: &mut Core<'_, '_, '_>) -> Result<(), Error> {
                let this_addr = u64::from_register_val(core.cpu.read_arg(0), core.mem)?;
                let this: &mut GdtTriggerParam = core.proxies.mut_trigger_param(core.mem, this_addr)?;
                let result = Ptr::<i32>::from_register_val(core.cpu.read_arg(1), core.mem)?;
                let idx = core.cpu.read_arg(1);
                if idx < 0 {
                    core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                    return Ok(());
                }
                let idx = usize::from_register_val(idx, core.mem)?;
                if idx >= this.[<$name _flags>].len() {
                    core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                    return Ok(());
                }

                let flag = this
                    .[<$name _flags>]
                    .get_mut(idx)
                    .ok_or(TriggerParamError::FlagNotFoundIndex(idx))?;

                result.store(core.mem, flag.get().len() as i32)?;

                core.cpu.write_gen_reg(&RegisterType::XReg(0), 1)?;
                Ok(())
            }

            pub fn [<set_ $name>](core: &mut Core<'_, '_, '_>) -> Result<(), Error> {
                let this_addr = u64::from_register_val(core.cpu.read_arg(0), core.mem)?;
                let this: &mut GdtTriggerParam = core.proxies.mut_trigger_param(core.mem, this_addr)?;
                let value = <$typ>::from_register_val(core.cpu.read_arg(1), core.mem)?;
                let idx = core.cpu.read_arg(2);
                let sub_idx = core.cpu.read_arg(3);
                let check_perms = bool::from_register_val(core.cpu.read_arg(4), core.mem)?;
                let _bypass_one_trigger_check = bool::from_register_val(core.cpu.read_arg(5), core.mem)?;
                if idx < 0 {
                    core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                    return Ok(());
                }
                let idx = usize::from_register_val(idx, core.mem)?;
                if idx >= this.[<$name _flags>].len() {
                    core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                    return Ok(());
                }

                let flag = this
                    .[<$name _flags>]
                    .get_mut(idx)
                    .ok_or(TriggerParamError::FlagNotFoundIndex(idx))?;

                if sub_idx < 0 {
                    core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                    return Ok(());
                }
                let sub_idx = usize::from_register_val(sub_idx, core.mem)?;
                if sub_idx > flag.get().len() {
                    core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                    return Ok(());
                }

                if !flag.is_program_writeable && check_perms {
                    core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                    return Ok(());
                }

                flag.set_idx(sub_idx, value);
                core.cpu.write_gen_reg(&RegisterType::XReg(0), 1)?;
                Ok(())
            }

            pub fn [<set_ $name _safe_string>](core: &mut Core<'_, '_, '_>) -> Result<(), Error> {
                let this_addr = u64::from_register_val(core.cpu.read_arg(0), core.mem)?;
                let this: &mut GdtTriggerParam = core.proxies.mut_trigger_param(core.mem, this_addr)?;
                let value = <$typ>::from_register_val(core.cpu.read_arg(1), core.mem)?;
                let name_addr = u64::from_register_val(core.cpu.read_arg(2), core.mem)?;
                let name = core.mem.mem_read_safe_string(name_addr)?;
                let sub_idx = core.cpu.read_arg(3);
                let check_perms = bool::from_register_val(core.cpu.read_arg(4), core.mem)?;
                let _ = bool::from_register_val(core.cpu.read_arg(5), core.mem)?; // unnamed in decomp
                let _bypass_one_trigger_check = bool::from_register_val(core.cpu.read_arg(6), core.mem)?;

                let flag_opt = this
                    .[<get_ $name _flag_by_name_mut>](name);
                if let Some(flag) = flag_opt {
                    if !flag.is_program_writeable && check_perms {
                        core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                        return Ok(());
                    } else {
                        if sub_idx < 0 {
                            core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                            return Ok(());
                        }
                        let sub_idx = usize::from_register_val(sub_idx, core.mem)?;
                        if sub_idx > flag.get().len() {
                            core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                            return Ok(());
                        }

                        if !flag.is_program_writeable && check_perms {
                            core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                            return Ok(());
                        }

                        flag.set_idx(sub_idx, value);
                        core.cpu.write_gen_reg(&RegisterType::XReg(0), 1)?;
                        Ok(())
                    }
                } else {
                    core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
                    Ok(())
                }
            }
        }
    }
}

// name: the prefix of the name of the variable in the struct (like "f32"_flags)
// data_name: the name of the datatype as listed in the yaml file (like "f32"_data:)
// type: the Rust datatype to annotate the Flag instance with
// read_value: the yaml type to read in (the function read_value_$typ is called)
// converter: a function to convert the read type from the yaml file to the Rust type
macro_rules! load_yaml {
    ($file_name:ident, $data_name:ident, $flag_list_name:ident, $typ:ty, $read_value:ident, $converter:expr) => {
        paste::item! {
            fn [<load_ $file_name _flags>](&mut self) -> LoadResult {
                let file = include_str!(concat!("../../res/Flag/", stringify!($file_name), "_data.yml"));
                // let file = include_str!(path);
                let value: Value = serde_yaml::from_str(file)?;
                let top_name = concat!(stringify!($data_name), "_data");
                let data_list = Self::read_value_seq(&value, top_name)?;
                for entry in data_list {
                    let name = Self::read_value_str(entry, "DataName")?;
                    let hash = Self::read_value_i64(entry, "HashValue")?;
                    let readable = Self::read_value_bool(entry, "IsProgramReadable")?;
                    let writeable = Self::read_value_bool(entry, "IsProgramWritable")?;
                    let init = Self::[<read_value_ $read_value>](entry, "InitValue")?;
                    let flag = Flag::<$typ>::new(
                        $converter(init)?,
                        String::from(name),
                        hash as i32,
                        readable,
                        writeable,
                    );
                    self.[<add_ $flag_list_name _flag>](flag);
                }
                self.[<$flag_list_name _flags>].sort_by(|a, b| a.hash().cmp(&b.hash()));
                Ok(())
            }
        }
    };
}
macro_rules! load_yaml_array {
    ($name:ident, $data_name:ident, $typ:ty, $read_value:ident, $converter:expr) => {
        paste::item! {
            fn [<load_ $name _array_flags>](&mut self) -> LoadResult {
                let file = include_str!(concat!("../../res/Flag/", stringify!($name), "_array_data.yml"));
                // let file = std::fs::File::open(path)?;
                let value: Value = serde_yaml::from_str(file)?;
                let top_name = concat!(stringify!($data_name), "_array_data");
                let data_list = Self::read_value_seq(&value, top_name)?;
                for entry in data_list {
                    let name = Self::read_value_str(entry, "DataName")?;
                    let hash = Self::read_value_i64(entry, "HashValue")?;
                    let readable = Self::read_value_bool(entry, "IsProgramReadable")?;
                    let writeable = Self::read_value_bool(entry, "IsProgramWritable")?;
                    let init_value = Self::read_value_seq(entry, "InitValue")?;
                    let first = init_value.get(0).unwrap();
                    let value_seq = Self::read_value_seq(first, "Values")?;
                    let mut vec = value_seq
                        .iter()
                        .filter_map(|v| v.[<as_ $read_value>]().map($converter))
                        .collect::<Vec<_>>();
                    let mut unwrapped_inits = Vec::new();
                    while !vec.is_empty() {
                        let itm = vec.pop().unwrap();
                        if let Err(e) = itm {
                            return Err(e);
                        } else if let Ok(o) = itm {
                            unwrapped_inits.push(o);
                        }
                    }
                    let init: Box<[$typ]> = unwrapped_inits.into_boxed_slice();
                    let flag: Flag<Box<[$typ]>> = Flag::<Box<[$typ]>>::new(
                        init,
                        String::from(name),
                        hash as i32,
                        readable,
                        writeable,
                    );
                    self.[<add_ $name _array_flag>](flag);
                }
                self.[<$name _array_flags>].sort_by(|a, b| a.hash().cmp(&b.hash()));
                Ok(())
            }
        }
    };
}

macro_rules! tuple_converter {
    ($tuple_func:ident, $typ:ty) => {
        paste::item! {
            |v: &Vec<Value>| -> GenericLoadResult<$typ> {
                Self::$tuple_func(
                    v.get(0).unwrap()
                        .as_sequence().unwrap()
                        .into_iter()
                        .map(|e| e.as_f64().map(|a| a as f32))
                        .collect::<Vec<Option<f32>>>()
                )
            }
        }
    };
}

type LoadError = Box<dyn std::error::Error>;
type LoadResult = GenericLoadResult<()>;
type GenericLoadResult<T> = Result<T, LoadError>;
impl GdtTriggerParam {
    common_flag_funcs!(bool, bool);
    common_flag_funcs!(f32, f32);
    common_flag_funcs!(s32, i32);
    common_flag_funcs!(string256, String);
    common_flag_funcs!(string32, String);
    common_flag_funcs!(string64, String);
    common_flag_funcs!(vector2f, (f32, f32));
    common_flag_funcs!(vector3f, (f32, f32, f32));
    common_flag_funcs!(vector4f, (f32, f32, f32, f32));
    common_flag_funcs!(bool_array, Box<[bool]>);
    common_flag_funcs!(f32_array, Box<[f32]>);
    common_flag_funcs!(s32_array, Box<[i32]>);
    common_flag_funcs!(string256_array, Box<[String]>);
    common_flag_funcs!(string64_array, Box<[String]>);
    common_flag_funcs!(vector2f_array, Box<[(f32, f32)]>);
    common_flag_funcs!(vector3f_array, Box<[(f32, f32, f32)]>);

    array_flag_funcs!(bool_array, bool);
    array_flag_funcs!(f32_array, f32);
    array_flag_funcs!(s32_array, i32);
    array_flag_funcs!(string256_array, String);
    array_flag_funcs!(string64_array, String);
    array_flag_funcs!(vector2f_array, (f32, f32));
    array_flag_funcs!(vector3f_array, (f32, f32, f32));

    processor_flag_funcs!(bool, bool, bool);
    processor_flag_funcs!(f32, f32, f32);
    processor_flag_funcs!(s32, i32, i32);
    processor_flag_funcs!(string256, String, u64);
    processor_flag_funcs!(string32, String, u64);
    processor_flag_funcs!(string64, String, u64);
    processor_flag_funcs!(vector2f, (f32, f32), (f32, f32));
    processor_flag_funcs!(vector3f, (f32, f32, f32), (f32, f32, f32));
    processor_flag_funcs!(vector4f, (f32, f32, f32, f32), (f32, f32, f32, f32));

    processor_flag_array_funcs!(bool_array, bool, bool);
    processor_flag_array_funcs!(f32_array, f32, f32);
    processor_flag_array_funcs!(s32_array, i32, i32);
    processor_flag_array_funcs!(string256_array, String, u64);
    processor_flag_array_funcs!(string64_array, String, u64);
    processor_flag_array_funcs!(vector2f_array, (f32, f32), (f32, f32));
    processor_flag_array_funcs!(vector3f_array, (f32, f32, f32), (f32, f32, f32));

    load_yaml!(bool, bool, bool, bool, i64, |v| Ok::<bool, LoadError>(
        v == 1
    ));
    load_yaml!(
        revival_bool,
        bool,
        bool,
        bool,
        i64,
        |v| Ok::<bool, LoadError>(v == 1)
    );
    load_yaml!(s32, s32, s32, i32, i64, |v| Ok::<i32, LoadError>(v as i32));
    load_yaml!(revival_s32, s32, s32, i32, i64, |v| Ok::<i32, LoadError>(
        v as i32
    ));
    load_yaml!(f32, f32, f32, f32, f64, |v| Ok::<f32, LoadError>(v as f32));
    load_yaml!(string32, string, string32, String, str, |v| Ok::<
        String,
        LoadError,
    >(
        String::from(v)
    ));
    load_yaml!(string64, string64, string64, String, str, |v| Ok::<
        String,
        LoadError,
    >(
        String::from(v)
    ));
    load_yaml!(string256, string256, string256, String, str, |v| Ok::<
        String,
        LoadError,
    >(
        String::from(v)
    ));
    load_yaml!(
        vector2f,
        vector2f,
        vector2f,
        (f32, f32),
        seq,
        tuple_converter!(tuple2, (f32, f32))
    );
    load_yaml!(
        vector3f,
        vector3f,
        vector3f,
        (f32, f32, f32),
        seq,
        tuple_converter!(tuple3, (f32, f32, f32))
    );
    load_yaml!(
        vector4f,
        vector4f,
        vector4f,
        (f32, f32, f32, f32),
        seq,
        tuple_converter!(tuple4, (f32, f32, f32, f32))
    );

    load_yaml_array!(bool, bool, bool, i64, |v| Ok::<bool, LoadError>(v == 1));
    load_yaml_array!(s32, s32, i32, i64, |v| Ok::<i32, LoadError>(v as i32));
    load_yaml_array!(f32, f32, f32, f64, |v| Ok::<f32, LoadError>(v as f32));
    load_yaml_array!(
        string64,
        string64,
        String,
        str,
        |v| Ok::<String, LoadError>(String::from(v))
    );
    load_yaml_array!(
        string256,
        string256,
        String,
        str,
        |v| Ok::<String, LoadError>(String::from(v))
    );
    load_yaml_array!(
        vector2f,
        vector2f,
        (f32, f32),
        sequence,
        tuple_converter!(tuple2, (f32, f32))
    );
    load_yaml_array!(
        vector3f,
        vector3f,
        (f32, f32, f32),
        sequence,
        tuple_converter!(tuple3, (f32, f32, f32))
    );

    pub fn load_yaml_files(&mut self) -> LoadResult {
        // Add additional flags, missing from the yaml
        // Do this first, since the load_X_flags functions sort the flags afterwards
        self.add_bool_flag(Self::make_flag(false, "AoC_DragonFireChallengeRing_Advent"));
        self.add_string64_array_flag(Self::make_flag(
            Box::new([]),
            "AoC_RandomSpawnTreasure_Contents",
        ));
        self.add_bool_flag(Self::make_flag(
            false,
            "AoC_RandomSpawnTreasure_IsRandomized",
        ));
        self.add_bool_flag(Self::make_flag(false, "AoC_TestProg_Imoto_Flag_00"));
        self.add_s32_flag(Self::make_flag(0, "AoC_TestProg_Imoto_TagCount_00"));
        self.add_bool_flag(Self::make_flag(false, "AocTestEx_Omosako_IsPastWorld"));
        self.add_vector3f_flag(Self::make_flag(
            (0f32, 0f32, 0f32),
            "AocTestEx_Omosako_ReturnToMainField_Position",
        ));
        self.add_f32_flag(Self::make_flag(
            0f32,
            "AocTestEx_Omosako_ReturnToMainField_Rotation",
        ));
        self.add_s32_flag(Self::make_flag(0, "AocTestEx_Omosako_SandOfTime_Num"));
        self.add_f32_flag(Self::make_flag(0f32, "AocTestEx_Omosako_SandOfTime_Rate"));
        self.add_s32_flag(Self::make_flag(0, "Location_DarkDungeon01"));
        self.add_s32_flag(Self::make_flag(0, "Location_DarkDungeon02"));
        self.add_s32_flag(Self::make_flag(0, "Location_DarkDungeon03"));
        self.add_s32_flag(Self::make_flag(0, "Location_DarkDungeon04"));
        self.add_bool_flag(Self::make_flag(false, "SpurGear_revolve_01"));
        self.add_bool_flag(Self::make_flag(false, "SpurGear_revolve_02"));

        self.load_bool_flags()?;
        self.load_revival_bool_flags()?;
        self.load_f32_flags()?;
        self.load_s32_flags()?;
        self.load_revival_s32_flags()?;
        self.load_string32_flags()?;
        self.load_string64_flags()?;
        self.load_string256_flags()?;
        self.load_vector2f_flags()?;
        self.load_vector3f_flags()?;
        self.load_vector4f_flags()?;

        self.load_bool_array_flags()?;
        self.load_s32_array_flags()?;
        self.load_f32_array_flags()?;
        self.load_string64_array_flags()?;
        self.load_string256_array_flags()?;
        self.load_vector2f_array_flags()?;
        self.load_vector3f_array_flags()?;

        Ok(())
    }

    fn make_flag<T: Clone>(value: T, name: &str) -> Flag<T> {
        Flag::<T> {
            value: value.clone(),
            initial_value: value,
            hash: Self::hash(name),
            name: String::from(name),
            is_program_readable: true,
            is_program_writeable: true,
        }
    }
    fn hash(s: &str) -> i32 {
        let hash = crc32fast::hash(s.as_bytes());
        i32::from_ne_bytes(hash.to_ne_bytes())
    }

    pub fn reset_all_flags_to_initial_values(core: &mut Core<'_, '_, '_>) -> Result<(), Error> {
        let this_addr = u64::from_register_val(core.cpu.read_arg(0), core.mem)?;
        let this: &mut GdtTriggerParam = core.proxies.mut_trigger_param(core.mem, this_addr)?;
        for f in &mut this.bool_flags {
            f.reset();
        }
        for f in &mut this.f32_flags {
            f.reset();
        }
        for f in &mut this.s32_flags {
            f.reset();
        }
        for f in &mut this.string32_flags {
            f.reset();
        }
        for f in &mut this.string64_flags {
            f.reset();
        }
        for f in &mut this.string256_flags {
            f.reset();
        }
        for f in &mut this.vector2f_flags {
            f.reset();
        }
        for f in &mut this.vector3f_flags {
            f.reset();
        }
        for f in &mut this.vector4f_flags {
            f.reset();
        }
        for f in &mut this.bool_array_flags {
            f.reset();
        }
        for f in &mut this.s32_array_flags {
            f.reset();
        }
        for f in &mut this.f32_array_flags {
            f.reset();
        }
        for f in &mut this.string64_array_flags {
            f.reset();
        }
        for f in &mut this.string256_array_flags {
            f.reset();
        }
        for f in &mut this.vector2f_array_flags {
            f.reset();
        }
        for f in &mut this.vector3f_array_flags {
            f.reset();
        }

        Ok(())
    }

    fn read_value_i64(value: &Value, name: &str) -> Result<i64, Box<dyn std::error::Error>> {
        let result = value.get(name).and_then(|v| v.as_i64());
        if let Some(v) = result {
            Ok(v)
        } else {
            Err(Error::msg(format!("Could not read {}!", name)).into())
        }
    }
    fn read_value_f64(value: &Value, name: &str) -> Result<f64, Box<dyn std::error::Error>> {
        let result = value.get(name).and_then(|v| v.as_f64());
        if let Some(v) = result {
            Ok(v)
        } else {
            Err(Error::msg(format!("Could not read {}!", name)).into())
        }
    }
    fn read_value_bool(value: &Value, name: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let result = value.get(name).and_then(|v| v.as_bool());
        if let Some(v) = result {
            Ok(v)
        } else {
            Err(Error::msg(format!("Could not read {}!", name)).into())
        }
    }
    fn read_value_str<'a>(
        value: &'a Value,
        name: &str,
    ) -> Result<&'a str, Box<dyn std::error::Error>> {
        let result = value.get(name).and_then(|v| v.as_str());
        if let Some(v) = result {
            Ok(v)
        } else {
            Err(Error::msg(format!("Could not read {}!", name)).into())
        }
    }
    fn read_value_seq<'a>(
        value: &'a Value,
        name: &str,
    ) -> Result<&'a Vec<Value>, Box<dyn std::error::Error>> {
        let result = value.get(name).and_then(|v| v.as_sequence());
        if let Some(v) = result {
            Ok(v)
        } else {
            Err(Error::msg(format!("Could not read {}!", name)).into())
        }
    }

    fn tuple2(a: Vec<Option<f32>>) -> GenericLoadResult<(f32, f32)> {
        let mut vec = Vec::<f32>::new();
        for item in a {
            if let Some(v) = item {
                vec.push(v);
            } else {
                return Err(Error::msg("Could not read value in sequence for tuple!").into());
            }
        }
        Ok((*vec.first().unwrap(), *vec.get(1).unwrap()))
    }
    fn tuple3(a: Vec<Option<f32>>) -> GenericLoadResult<(f32, f32, f32)> {
        let mut vec = Vec::<f32>::new();
        for item in a {
            if let Some(v) = item {
                vec.push(v);
            } else {
                return Err(Error::msg("Could not read value in sequence for tuple!").into());
            }
        }
        Ok((
            *vec.first().unwrap(),
            *vec.get(1).unwrap(),
            *vec.get(2).unwrap(),
        ))
    }
    fn tuple4(a: Vec<Option<f32>>) -> GenericLoadResult<(f32, f32, f32, f32)> {
        let mut vec = Vec::<f32>::new();
        for item in a {
            if let Some(v) = item {
                vec.push(v);
            } else {
                return Err(Error::msg("Could not read value in sequence for tuple!").into());
            }
        }
        Ok((
            *vec.first().unwrap(),
            *vec.get(1).unwrap(),
            *vec.get(2).unwrap(),
            *vec.get(3).unwrap(),
        ))
    }
}

impl ProxyObject for GdtTriggerParam {
    fn mem_size(&self) -> u32 {
        0x3f0
    }
}

type Hash = i32;
#[derive(Clone)]
pub struct Flag<T>
where
    T: Clone,
{
    value: T,
    initial_value: T,
    hash: Hash,
    name: String, // Even if not required to run, useful for debugging purposes
    is_program_readable: bool,
    is_program_writeable: bool,
}
impl<T> Flag<T>
where
    T: Clone,
{
    pub fn new(
        initial_value: T,
        name: String,
        hash: Hash,
        readable: bool,
        writeable: bool,
    ) -> Self {
        Flag {
            value: initial_value.clone(),
            initial_value,
            hash,
            name,
            is_program_readable: readable,
            is_program_writeable: writeable,
        }
    }

    pub fn get(&self) -> &T {
        &self.value
    }
    pub fn set(&mut self, value: T) {
        self.value = value;
    }
    pub fn reset(&mut self) {
        self.value = self.initial_value.clone();
    }

    pub fn hash(&self) -> Hash {
        self.hash
    }
    pub fn name(&self) -> &String {
        &self.name
    }
}

impl<T> Flag<Box<[T]>>
where
    T: Clone,
{
    pub fn reset_idx(&mut self, idx: usize) {
        let init_value = self.initial_value[idx].clone();
        self.value[idx] = init_value;
    }
    pub fn set_idx(&mut self, idx: usize, value: T) {
        self.value[idx] = value;
    }
}

trait GenerateMemValue<T>
where
    T: MemWrite,
{
    fn generate_mem_value(&self, mem: &mut Memory) -> anyhow::Result<T>;
}

macro_rules! impl_trivial_gen_mem_value {
    ($typ:ty) => {
        impl GenerateMemValue<$typ> for $typ {
            fn generate_mem_value(&self, _mem: &mut Memory) -> anyhow::Result<$typ> {
                Ok(self.clone())
            }
        }
    };
}
impl_trivial_gen_mem_value!(bool);
impl_trivial_gen_mem_value!(i32);
impl_trivial_gen_mem_value!(f32);
impl_trivial_gen_mem_value!((f32, f32));
impl_trivial_gen_mem_value!((f32, f32, f32));
impl_trivial_gen_mem_value!((f32, f32, f32, f32));
impl_trivial_gen_mem_value!(Box<[bool]>);
impl_trivial_gen_mem_value!(Box<[i32]>);
impl_trivial_gen_mem_value!(Box<[f32]>);
impl_trivial_gen_mem_value!(Box<[(f32, f32)]>);
impl_trivial_gen_mem_value!(Box<[(f32, f32, f32)]>);

impl GenerateMemValue<u64> for String {
    fn generate_mem_value(&self, mem: &mut Memory) -> anyhow::Result<u64> {
        let len = self.len() + 1; // full length + null terminator
        let addr = mem.heap_mut().alloc(len as u32)?;
        let mut writer = mem.write(addr, None)?;
        self.clone().write_to_mem(&mut writer)?;
        Ok(addr)
    }
}

#[test]
fn test_init() -> Result<(), Box<dyn std::error::Error>> {
    let mut tp = GdtTriggerParam::default();
    tp.load_yaml_files()?;
    let flag1 = tp.get_bool_flag_by_index(tp.get_bool_flag_index_from_hash(530692287).unwrap());
    assert!(!(*flag1.get()));
    assert_eq!("BarrelErrand_Intro_Finished", &flag1.name()[..]);
    let flag2 = tp
        .get_bool_array_flag_by_index(tp.get_bool_array_flag_index_from_hash(-1649503087).unwrap());
    assert_eq!(
        &Box::from([false, false, false, false, false, false, false, false]),
        flag2.get()
    );
    assert_eq!("dummy_bool_array", &flag2.name()[..]);
    let flag3 =
        tp.get_vector3f_flag_by_index(tp.get_vector3f_flag_index_from_hash(-1542741757).unwrap());
    assert_eq!((-1130.0, 237.4, 1914.5), *flag3.get());
    assert_eq!("PlayerSavePos", &flag3.name()[..]);
    let flag4 = tp.get_bool_flag_by_index(tp.get_bool_flag_index_from_hash(595714052).unwrap());
    assert!(!(*flag4.get()));
    assert_eq!("MainField_LinkTagAnd_02894606454", &flag4.name()[..]);
    Ok(())
}
#[test]
fn test_get_set_reset() -> Result<(), Box<dyn std::error::Error>> {
    let mut tp = GdtTriggerParam::default();
    tp.load_yaml_files()?;
    let flag1 = tp.get_bool_flag_by_index_mut(tp.get_bool_flag_index_from_hash(530692287).unwrap());
    flag1.set(true);
    assert!(*flag1.get());
    flag1.reset();
    assert!(!(*flag1.get()));
    Ok(())
}
