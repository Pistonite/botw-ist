
// macro_rules! processor_flag_array_funcs {
//     ($name:ident, $typ:ty, $mem_type:ty) => {
//         paste::item! {
//             pub fn [<get_ $name _array>](core: &mut Core<'_, '_, '_>) -> Result<(), Error> {
//                 let this_addr = u64::from_register_val(core.cpu.read_arg(0), core.mem)?;
//                 let this: &GdtTriggerParam = core.proxies.get_trigger_param(core.mem, this_addr)?;
//                 // TODO --cleanup: compiler error
//                 let ptr = Ptr::new(0);
//                 // let ptr = Ptr::<$mem_type>::from_register_val(core.cpu.read_arg(1), core.mem)?;
//                 let array_idx = core.cpu.read_arg(2);
//                 let idx = core.cpu.read_arg(3);
//
//                 if array_idx < 0 {
//                     core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                     return Ok(());
//                 }
//                 let array_idx = usize::from_register_val(array_idx, core.mem)?;
//                 if array_idx >= this.[<$name _flags>].len() {
//                     core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                     return Ok(());
//                 }
//
//                 let flag = this
//                     .[<$name _flags>]
//                     .get(array_idx)
//                     .ok_or(TriggerParamError::FlagNotFoundIndex(array_idx))?;
//                 if idx < 0 {
//                     core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                     return Ok(());
//                 }
//                 let idx = usize::from_register_val(idx, core.mem)?;
//                 if idx >= flag.get().len() {
//                     core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                     return Ok(());
//                 }
//
//                 let result_value = flag.get().get(idx).unwrap().generate_mem_value(core.mem)?;
//                 ptr.store(core.mem, result_value)?;
//
//                 core.cpu.write_gen_reg(&RegisterType::XReg(0), 1)?;
//                 Ok(())
//             }
//
//             pub fn [<reset_ $name _array>](core: &mut Core<'_, '_, '_>) -> Result<(), Error> {
//                 let this_addr = u64::from_register_val(core.cpu.read_arg(0), core.mem)?;
//                 let this: &mut GdtTriggerParam = core.proxies.mut_trigger_param(core.mem, this_addr)?;
//                 let idx = core.cpu.read_arg(1);
//                 let sub_idx = core.cpu.read_arg(2);
//                 let check_perms = bool::from_register_val(core.cpu.read_arg(3), core.mem)?;
//                 if idx < 0 {
//                     core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                     return Ok(());
//                 }
//                 let idx = usize::from_register_val(idx, core.mem)?;
//                 if idx >= this.[<$name _flags>].len() {
//                     core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                     return Ok(());
//                 }
//
//                 let flag = this
//                     .[<$name _flags>]
//                     .get_mut(idx)
//                     .ok_or(TriggerParamError::FlagNotFoundIndex(idx))?;
//                 if sub_idx < 0 {
//                     core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                     return Ok(());
//                 }
//                 let sub_idx = usize::from_register_val(sub_idx, core.mem)?;
//                 if sub_idx >= flag.value.len() {
//                     core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                     return Ok(());
//                 }
//
//                 if !flag.is_program_writeable && check_perms {
//                     core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                     return Ok(());
//                 }
//
//                 flag.reset_idx(sub_idx);
//
//                 core.cpu.write_gen_reg(&RegisterType::XReg(0), 1)?;
//                 Ok(())
//             }
//
//             pub fn [<get_ $name _array_size>](core: &mut Core<'_, '_, '_>) -> Result<(), Error> {
//                 let this_addr = u64::from_register_val(core.cpu.read_arg(0), core.mem)?;
//                 let this: &mut GdtTriggerParam = core.proxies.mut_trigger_param(core.mem, this_addr)?;
//                 let result = Ptr::<i32>::from_register_val(core.cpu.read_arg(1), core.mem)?;
//                 let idx = core.cpu.read_arg(1);
//                 if idx < 0 {
//                     core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                     return Ok(());
//                 }
//                 let idx = usize::from_register_val(idx, core.mem)?;
//                 if idx >= this.[<$name _flags>].len() {
//                     core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                     return Ok(());
//                 }
//
//                 let flag = this
//                     .[<$name _flags>]
//                     .get_mut(idx)
//                     .ok_or(TriggerParamError::FlagNotFoundIndex(idx))?;
//
//                 result.store(core.mem, flag.get().len() as i32)?;
//
//                 core.cpu.write_gen_reg(&RegisterType::XReg(0), 1)?;
//                 Ok(())
//             }
//
//             pub fn [<set_ $name>](core: &mut Core<'_, '_, '_>) -> Result<(), Error> {
//                 let this_addr = u64::from_register_val(core.cpu.read_arg(0), core.mem)?;
//                 let this: &mut GdtTriggerParam = core.proxies.mut_trigger_param(core.mem, this_addr)?;
//                 let value = <$typ>::from_register_val(core.cpu.read_arg(1), core.mem)?;
//                 let idx = core.cpu.read_arg(2);
//                 let sub_idx = core.cpu.read_arg(3);
//                 let check_perms = bool::from_register_val(core.cpu.read_arg(4), core.mem)?;
//                 let _bypass_one_trigger_check = bool::from_register_val(core.cpu.read_arg(5), core.mem)?;
//                 if idx < 0 {
//                     core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                     return Ok(());
//                 }
//                 let idx = usize::from_register_val(idx, core.mem)?;
//                 if idx >= this.[<$name _flags>].len() {
//                     core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                     return Ok(());
//                 }
//
//                 let flag = this
//                     .[<$name _flags>]
//                     .get_mut(idx)
//                     .ok_or(TriggerParamError::FlagNotFoundIndex(idx))?;
//
//                 if sub_idx < 0 {
//                     core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                     return Ok(());
//                 }
//                 let sub_idx = usize::from_register_val(sub_idx, core.mem)?;
//                 if sub_idx > flag.get().len() {
//                     core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                     return Ok(());
//                 }
//
//                 if !flag.is_program_writeable && check_perms {
//                     core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                     return Ok(());
//                 }
//
//                 flag.set_idx(sub_idx, value);
//                 core.cpu.write_gen_reg(&RegisterType::XReg(0), 1)?;
//                 Ok(())
//             }
//
//             pub fn [<set_ $name _safe_string>](core: &mut Core<'_, '_, '_>) -> Result<(), Error> {
//                 let this_addr = u64::from_register_val(core.cpu.read_arg(0), core.mem)?;
//                 let this: &mut GdtTriggerParam = core.proxies.mut_trigger_param(core.mem, this_addr)?;
//                 let value = <$typ>::from_register_val(core.cpu.read_arg(1), core.mem)?;
//                 let name_addr = u64::from_register_val(core.cpu.read_arg(2), core.mem)?;
//                 let name = core.mem.mem_read_safe_string(name_addr)?;
//                 let sub_idx = core.cpu.read_arg(3);
//                 let check_perms = bool::from_register_val(core.cpu.read_arg(4), core.mem)?;
//                 let _ = bool::from_register_val(core.cpu.read_arg(5), core.mem)?; // unnamed in decomp
//                 let _bypass_one_trigger_check = bool::from_register_val(core.cpu.read_arg(6), core.mem)?;
//
//                 let flag_opt = this
//                     .[<get_ $name _flag_by_name_mut>](name);
//                 if let Some(flag) = flag_opt {
//                     if !flag.is_program_writeable && check_perms {
//                         core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                         return Ok(());
//                     } else {
//                         if sub_idx < 0 {
//                             core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                             return Ok(());
//                         }
//                         let sub_idx = usize::from_register_val(sub_idx, core.mem)?;
//                         if sub_idx > flag.get().len() {
//                             core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                             return Ok(());
//                         }
//
//                         if !flag.is_program_writeable && check_perms {
//                             core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                             return Ok(());
//                         }
//
//                         flag.set_idx(sub_idx, value);
//                         core.cpu.write_gen_reg(&RegisterType::XReg(0), 1)?;
//                         Ok(())
//                     }
//                 } else {
//                     core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                     Ok(())
//                 }
//             }
//         }
//     }
// }
// macro_rules! processor_flag_funcs {
//     ($name:ident, $typ:ty, $mem_type:ty) => {
//         paste::item! {
//             pub fn [<get_ $name>](core: &mut Core<'_, '_, '_>) -> Result<(), Error> {
//                 let this_addr = u64::from_register_val(core.cpu.read_arg(0), core.mem)?;
//                 let this: &GdtTriggerParam = core.proxies.get_trigger_param(core.mem, this_addr)?;
//                 let ptr = Ptr::<$mem_type>::from_register_val(core.cpu.read_arg(1), core.mem)?;
//                 let idx = core.cpu.read_arg(2);
//                 let check_perms = bool::from_register_val(core.cpu.read_arg(3), core.mem)?;
//                 if idx < 0 {
//                     core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                     return Ok(());
//                 }
//                 let idx = usize::from_register_val(idx, core.mem)?;
//                 if idx >= this.[<$name _flags>].len() {
//                     core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                     return Ok(());
//                 }
//
//                 let flag = this
//                     .[<$name _flags>]
//                     .get(idx)
//                     .ok_or(TriggerParamError::FlagNotFoundIndex(idx))?;
//                 if !flag.is_program_readable && check_perms {
//                     core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                     return Ok(());
//                 }
//
//                 let result_value = flag.get().generate_mem_value(core.mem)?;
//                 ptr.store(core.mem, result_value)?;
//                 core.cpu.write_gen_reg(&RegisterType::XReg(0), 1)?;
//                 Ok(())
//             }
//
//             pub fn [<set_ $name>](core: &mut Core<'_, '_, '_>) -> Result<(), Error> {
//                 let this_addr = u64::from_register_val(core.cpu.read_arg(0), core.mem)?;
//                 let this: &mut GdtTriggerParam = core.proxies.mut_trigger_param(core.mem, this_addr)?;
//                 let value = <$typ>::from_register_val(core.cpu.read_arg(1), core.mem)?;
//                 let idx = core.cpu.read_arg(2);
//                 let check_perms = bool::from_register_val(core.cpu.read_arg(3), core.mem)?;
//                 let _bypass_one_trigger_check = bool::from_register_val(core.cpu.read_arg(4), core.mem)?;
//                 if idx < 0 {
//                     core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                     return Ok(());
//                 }
//                 let idx = usize::from_register_val(idx, core.mem)?;
//                 if idx >= this.[<$name _flags>].len() {
//                     core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                     return Ok(());
//                 }
//
//                 let flag = this
//                     .[<$name _flags>]
//                     .get_mut(idx)
//                     .ok_or(TriggerParamError::FlagNotFoundIndex(idx))?;
//                 if !flag.is_program_writeable && check_perms {
//                     core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                     return Ok(());
//                 }
//
//                 flag.set(value);
//                 core.cpu.write_gen_reg(&RegisterType::XReg(0), 1)?;
//                 Ok(())
//             }
//
//             pub fn [<set_ $name _safe_string>](core: &mut Core<'_, '_, '_>) -> Result<(), Error> {
//                 let this_addr = u64::from_register_val(core.cpu.read_arg(0), core.mem)?;
//                 let this: &mut GdtTriggerParam = core.proxies.mut_trigger_param(core.mem, this_addr)?;
//                 let value = <$typ>::from_register_val(core.cpu.read_arg(1), core.mem)?;
//                 let name_addr = u64::from_register_val(core.cpu.read_arg(2), core.mem)?;
//                 let name = core.mem.mem_read_safe_string(name_addr)?;
//                 let check_perms = bool::from_register_val(core.cpu.read_arg(3), core.mem)?;
//                 let _ = bool::from_register_val(core.cpu.read_arg(4), core.mem)?; // unnamed in decompilation
//                 let _bypass_one_trigger_check = bool::from_register_val(core.cpu.read_arg(5), core.mem)?;
//
//                 let flag_opt = this
//                     .[<get_ $name _flag_by_name_mut>](name);
//                 if let Some(flag) = flag_opt {
//                     if !flag.is_program_writeable && check_perms {
//                         core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                         return Ok(());
//                     } else {
//                         flag.set(value);
//                         core.cpu.write_gen_reg(&RegisterType::XReg(0), 1)?;
//                         Ok(())
//                     }
//                 } else {
//                     core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                     Ok(())
//                 }
//             }
//
//             pub fn [<reset_ $name>](core: &mut Core<'_, '_, '_>) -> Result<(), Error> {
//                 let this_addr = u64::from_register_val(core.cpu.read_arg(0), core.mem)?;
//                 let this: &mut GdtTriggerParam = core.proxies.mut_trigger_param(core.mem, this_addr)?;
//                 let idx = core.cpu.read_arg(1);
//                 let check_perms = bool::from_register_val(core.cpu.read_arg(2), core.mem)?;
//                 if idx < 0 {
//                     core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                     return Ok(());
//                 }
//                 let idx = usize::from_register_val(idx, core.mem)?;
//                 if idx >= this.[<$name _flags>].len() {
//                     core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                     return Ok(());
//                 }
//
//                 let flag = this
//                     .[<$name _flags>]
//                     .get_mut(idx)
//                     .ok_or(TriggerParamError::FlagNotFoundIndex(idx))?;
//                 if !flag.is_program_writeable && check_perms {
//                     core.cpu.write_gen_reg(&RegisterType::XReg(0), 0)?;
//                     return Ok(());
//                 }
//
//                 flag.reset();
//
//             core.cpu.write_gen_reg(&RegisterType::XReg(0), 1)?;
//             Ok(())
//         }
//         }
//     };
// }
    // pub fn init_trigger_param_stubs(processor: &mut Processor) {
    //     macro_rules! reg_flag_stubs {
    //         ($name:ident, $get:expr, $get_idx_from_hash:expr, $reset:expr, $set:expr, $set_by_name:expr) => {
    //             paste! {
    //                 processor.register_stub_function($get, Stub::run_and_ret(Box::new(|c| GdtTriggerParam::[<get_ $name>](c))));
    //                 processor.register_stub_function($get_idx_from_hash, Stub::run_and_ret(Box::new(|c| GdtTriggerParam::[<get_ $name _index>](c))));
    //                 processor.register_stub_function($reset, Stub::run_and_ret(Box::new(|c| GdtTriggerParam::[<reset_ $name>](c))));
    //                 processor.register_stub_function($set, Stub::run_and_ret(Box::new(|c| GdtTriggerParam::[<set_ $name>](c))));
    //                 processor.register_stub_function($set_by_name, Stub::run_and_ret(Box::new(|c| GdtTriggerParam::[<set_ $name _safe_string>](c))));
    //             }
    //         }
    //     }
    //     macro_rules! reg_array_flag_stubs {
    //         ($name: ident, $get:expr, $get_idx_from_hash:expr, $reset:expr, $len:expr, $set:expr, $set_by_name:expr) => {
    //             paste! {
    //                 processor.register_stub_function($get, Stub::run_and_ret(Box::new(|c| GdtTriggerParam::[<get_ $name _array>](c))));
    //                 processor.register_stub_function($get_idx_from_hash, Stub::run_and_ret(Box::new(|c| GdtTriggerParam::[<get_ $name _index>](c))));
    //                 processor.register_stub_function($reset, Stub::run_and_ret(Box::new(|c| GdtTriggerParam::[<reset_ $name _array>](c))));
    //                 processor.register_stub_function($len, Stub::run_and_ret(Box::new(|c| GdtTriggerParam::[<get_ $name _array_size>](c))));
    //                 processor.register_stub_function($set, Stub::run_and_ret(Box::new(|c| GdtTriggerParam::[<set_ $name>](c))));
    //                 processor.register_stub_function($set_by_name, Stub::run_and_ret(Box::new(|c| GdtTriggerParam::[<set_ $name _safe_string>](c))));
    //             }
    //         }
    //     }
    //
    //     reg_flag_stubs!(bool, 0xDDF0F8, 0xDF08B8, 0xDE6F1C, 0xDE1B64, 0xDE59E4);
    //     reg_flag_stubs!(s32, 0xDDF174, 0xDF0970, 0xDE7004, 0xDE22F8, 0xDE5B0C);
    //     reg_flag_stubs!(f32, 0xDDF1EC, 0xDF0A28, 0xDE70EC, 0xDE2908, 0xDE5C34);
    //     reg_flag_stubs!(string32, 0xDDF264, 0xDF0AE0, 0, 0xDE2F20, 0xDE5D64);
    //     reg_flag_stubs!(string64, 0xDDF2F0, 0xDF0B98, 0xDE71D4, 0xDE37B0, 0xDE5E8C);
    //     reg_flag_stubs!(string256, 0xDDF37C, 0xDF0C50, 0, 0xDE4040, 0);
    //     reg_flag_stubs!(vector3f, 0xDDF408, 0xDF0DC0, 0xDE72BC, 0xDE4EA0, 0xDE5FB4);
    //
    //     reg_array_flag_stubs!(
    //         bool_array, 0xDE002C, 0xDF0E78, 0xDE77E4, 0xDE0D3C, 0xDE6170, 0xDE6B04
    //     );
    //     reg_array_flag_stubs!(
    //         s32_array, 0xDE00D0, 0xDF0F08, 0xDE7900, 0xDE0D74, 0xDE625C, 0xDE6C08
    //     );
    //     reg_array_flag_stubs!(
    //         f32_array, 0xDE0170, 0xDF0F98, 0xDE7A1C, 0xDE0DAC, 0xDE63E0, 0xDE6D0C
    //     );
    //     reg_array_flag_stubs!(
    //         string64_array,
    //         0xDE0210,
    //         0xDF1028,
    //         0xDE7C54,
    //         0xDE0E1C,
    //         0xDE656C,
    //         0xDE6E18
    //     );
    //     reg_array_flag_stubs!(
    //         string256_array,
    //         0xDE02C4,
    //         0xDF10B8,
    //         0xDE7D70,
    //         0xDE0E54,
    //         0xDE6758,
    //         0
    //     );
    //     reg_array_flag_stubs!(
    //         vector2f_array,
    //         0xDE0378,
    //         0xDF1148,
    //         0xDE7E8C,
    //         0xDE0E8C,
    //         0xDE6944,
    //         0
    //     );
    //     reg_array_flag_stubs!(
    //         vector3f_array,
    //         0xDE0418,
    //         0xDF11D8,
    //         0xDE7FA8,
    //         0xDE0EC4,
    //         0xDE6A24,
    //         0
    //     );
    //
    //     processor.register_stub_function(
    //         0xDEEB8C,
    //         Stub::simple(Box::new(GdtTriggerParam::reset_all_flags_to_initial_values)),
    //     );
    // }
//     processor_flag_funcs!(bool, bool, bool);
//     processor_flag_funcs!(f32, f32, f32);
//     processor_flag_funcs!(s32, i32, i32);
//     processor_flag_funcs!(string256, String, u64);
//     processor_flag_funcs!(string32, String, u64);
//     processor_flag_funcs!(string64, String, u64);
//     processor_flag_funcs!(vector2f, (f32, f32), (f32, f32));
//     processor_flag_funcs!(vector3f, (f32, f32, f32), (f32, f32, f32));
//     processor_flag_funcs!(vector4f, (f32, f32, f32, f32), (f32, f32, f32, f32));
//
//     processor_flag_array_funcs!(bool_array, bool, bool);
//     processor_flag_array_funcs!(f32_array, f32, f32);
//     processor_flag_array_funcs!(s32_array, i32, i32);
//     processor_flag_array_funcs!(string256_array, String, u64);
//     processor_flag_array_funcs!(string64_array, String, u64);
//     processor_flag_array_funcs!(vector2f_array, (f32, f32), (f32, f32));
//     processor_flag_array_funcs!(vector3f_array, (f32, f32, f32), (f32, f32, f32));
//     pub fn reset_all_flags_to_initial_values(core: &mut Core<'_, '_, '_>) -> Result<(), Error> {
//         let this_addr = u64::from_register_val(core.cpu.read_arg(0), core.mem)?;
//         let this: &mut TriggerParam = core.proxies.mut_trigger_param(core.mem, this_addr)?;
//
//
//         -- impl hidden --
//
//         Ok(())
//     }
// trait GenerateMemValue<T>
// where
//     T: MemWrite,
// {
//     fn generate_mem_value(&self, mem: &mut Memory) -> anyhow::Result<T>;
// }
//
// macro_rules! impl_trivial_gen_mem_value {
//     ($typ:ty) => {
//         impl GenerateMemValue<$typ> for $typ {
//             fn generate_mem_value(&self, _mem: &mut Memory) -> anyhow::Result<$typ> {
//                 Ok(self.clone())
//             }
//         }
//     };
// }
// impl_trivial_gen_mem_value!(bool);
// impl_trivial_gen_mem_value!(i32);
// impl_trivial_gen_mem_value!(f32);
// impl_trivial_gen_mem_value!((f32, f32));
// impl_trivial_gen_mem_value!((f32, f32, f32));
// impl_trivial_gen_mem_value!((f32, f32, f32, f32));
// impl_trivial_gen_mem_value!(Box<[bool]>);
// impl_trivial_gen_mem_value!(Box<[i32]>);
// impl_trivial_gen_mem_value!(Box<[f32]>);
// impl_trivial_gen_mem_value!(Box<[(f32, f32)]>);
// impl_trivial_gen_mem_value!(Box<[(f32, f32, f32)]>);
//
// impl GenerateMemValue<u64> for String {
//     fn generate_mem_value(&self, mem: &mut Memory) -> anyhow::Result<u64> {
//         let len = self.len() + 1; // full length + null terminator
//         let addr = mem.heap_mut().alloc(len as u32)?;
//         let mut writer = mem.write(addr, None)?;
//         self.clone().write_to_mem(&mut writer)?;
//         Ok(addr)
//     }
// }
