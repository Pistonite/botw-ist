use crate::linker::{self as self_, crate_};

use crate_::game::{gdt, SafeString};
use crate_::memory::{MemObject, Memory, Ptr, proxy};
use crate_::processor::{self, reg, Cpu0, Execute, HookProvider, Process};

// this macro is needed because template generic types are not stable
macro_rules! get_flag_impl {
    ($cpu:ident, $proc:ident, type = $value_ty:ty, flag = $fd:ident) => {
        get_flag_impl::<$value_ty, gdt::fd!($fd), _, _>(
            $cpu,
            $proc,
            |x| Ok(*x),
            |value, out_ptr, memory| Ok(Ptr!(<$value_ty>(out_ptr)).store(&value, memory)?),
        )
    };
    (by_name $cpu:ident, $proc:ident, type = $value_ty:ty, flag = $fd:ident) => {
        get_flag_by_name_impl::<$value_ty, gdt::fd!($fd), _, _>(
            $cpu,
            $proc,
            |x| Ok(*x),
            |value, out_ptr, memory| Ok(Ptr!(<$value_ty>(out_ptr)).store(&value, memory)?),
        )
    };
    (array $cpu:ident, $proc:ident, type = $value_ty:ty, flag = $fd:ident[]) => {
        get_flag_array_by_index_impl::<$value_ty, gdt::fd!($fd[]), _, _>(
            $cpu,
            $proc,
            |x| Ok(*x),
            |value, out_ptr, memory| Ok(Ptr!(<$value_ty>(out_ptr)).store(&value, memory)?),
        )
    };
    (array by_name $cpu:ident, $proc:ident, type = $value_ty:ty, flag = $fd:ident[]) => {
        get_flag_array_by_name_impl::<$value_ty, gdt::fd!($fd[]), _, _>(
            $cpu,
            $proc,
            |x| Ok(*x),
            |value, out_ptr, memory| Ok(Ptr!(<$value_ty>(out_ptr)).store(&value, memory)?),
        )
    }
}

#[rustfmt::skip]
mod __impl_get {
    use super::*;
    pub fn get_bool(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { get_flag_impl!(cpu, proc, type = bool, flag = bool) }
    pub fn get_s32(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { get_flag_impl!(cpu, proc, type = i32, flag =s32) }
    pub fn get_f32(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { get_flag_impl!(cpu, proc, type = f32, flag = f32) }
    pub fn get_vec3f(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { get_flag_impl!(cpu, proc, type = (f32, f32, f32), flag = vec3f) }
    pub fn get_bool_by_name(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { get_flag_impl!(by_name cpu, proc, type = bool, flag = bool) }
    pub fn get_s32_by_name(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { get_flag_impl!(by_name cpu, proc, type = i32, flag = s32) }
    pub fn get_f32_by_name(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { get_flag_impl!(by_name cpu, proc, type = f32, flag = f32) }
    pub fn get_vec3f_by_name(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { get_flag_impl!(by_name cpu, proc, type = (f32, f32, f32), flag = vec3f) }
    pub fn get_bool_array(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { get_flag_impl!(array cpu, proc, type = bool, flag = bool[]) }
    pub fn get_s32_array(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { get_flag_impl!(array cpu, proc, type = i32, flag = s32[]) }
    pub fn get_f32_array(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { get_flag_impl!(array cpu, proc, type = f32, flag = f32[]) }
    pub fn get_vec2f_array(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { get_flag_impl!(array cpu, proc, type = (f32, f32), flag = vec2f[]) }
    pub fn get_vec3f_array(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { get_flag_impl!(array cpu, proc, type = (f32, f32, f32), flag = vec3f[]) }
    pub fn get_bool_array_by_name(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { get_flag_impl!(array by_name cpu, proc, type = bool, flag = bool[]) }
    pub fn get_s32_array_by_name(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { get_flag_impl!(array by_name cpu, proc, type = i32, flag = s32[]) }
    pub fn get_f32_array_by_name(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { get_flag_impl!(array by_name cpu, proc, type = f32, flag = f32[]) }
}
pub use __impl_get::*;
pub fn get_str<Fd: gdt::FlagDescriptor<T=String>>(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
    // TODO --optimize: we can probably avoid having to alloc new string every time
    // here
    get_flag_impl::<Vec<u8>, Fd, _, _>(cpu, proc, get_str_extractor, get_str_storer)
}
pub fn get_str_by_name<Fd: gdt::FlagDescriptor<T=String>>(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
    // TODO --optimize: we can probably avoid having to alloc new string every time
    // here
    get_flag_by_name_impl::<Vec<u8>, Fd, _, _>( cpu, proc, get_str_extractor, get_str_storer,)
}
pub fn get_str_array<Fd: gdt::ArrayFlagDescriptor<ElemT=String>>(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
    // TODO --optimize: we can probably avoid having to alloc new string every time
    // here
    get_flag_array_by_index_impl::<Vec<u8>, Fd, _, _>(
        cpu,
        proc,
        get_str_extractor,
        get_str_storer,
    )
}
pub fn get_str_array_by_name<Fd: gdt::ArrayFlagDescriptor<ElemT=String>>(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
    // TODO --optimize: we can probably avoid having to alloc new string every time
    // here
    get_flag_array_by_name_impl::<Vec<u8>, Fd, _, _>(
        cpu,
        proc,
        get_str_extractor,
        get_str_storer,
    )
}


fn get_str_extractor(
    s: &String,
) -> Result<Vec<u8>, processor::Error> {
    let mut v = s.as_bytes().to_vec();
    v.push(0); // null-terminate
    Ok(v)
}

fn get_str_storer(
    value: Vec<u8>,
    out_ptr: u64,
    memory: &mut Memory,
) -> Result<(), processor::Error> {
    let ptr = memory.alloc_with(&value)?;
    // out_ptr is char**
    Ok(Ptr!(<u64>(out_ptr)).store(&ptr, memory)?)
}

/// ksys::gdt::TriggerParam::getXXXIdx(this, i32 hash) -> int
pub fn idx_from_hash< Fd: gdt::FlagDescriptor, >(
    cpu: &mut Cpu0,
    proc: &mut Process,
) -> Result<(), processor::Error> {
    reg! { cpu:
        x[0] => this_ptr: u64,
        w[1] => hash: i32,
    }
    proxy! {
        let params = *this_ptr as trigger_param in proc
    };

    let index = match params.index_from_hash::<Fd>(hash) {
        Some(i) => i as i32,
        None => -1
    };

    cpu.write(reg!(w[0]), index);
    cpu.ret();
    Ok(())
}

pub fn reset<Fd: gdt::FlagDescriptor>(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
    reg! { cpu:
        x[0] => this_ptr: u64,
        w[1] => idx: i32,
        w[2] => check_perms: bool,
    }

    proxy! { let mut params = *this_ptr as trigger_param in proc };

    let Some(flag) = params.get_mut::<Fd, _>(idx) else {
        reg!{ cpu: x[0] = false, return };
    };
    if check_perms && !flag.writable() {
        reg!{ cpu: x[0] = false, return };
    }
    flag.reset();
    reg!{ cpu: x[0] = true, return }
}

fn reset_array_impl<Fd: gdt::ArrayFlagDescriptor>(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
    reg! { cpu:
        x[0] => this_ptr: u64,
        w[1] => idx: i32,
        w[2] => array_idx: i32,
        w[3] => check_perms: bool,
    }

    proxy! { let mut params = *this_ptr as trigger_param in proc };

    let Some(flag) = params.get_mut::<Fd, _>(idx) else {
        reg!{ cpu: x[0] = false, return };
    };
    if check_perms && !flag.writable() {
        reg!{ cpu: x[0] = false, return };
    }
    let result = flag.reset_at(array_idx);
    reg!{ cpu: x[0] = true, return }
}

pub fn set_bool(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
    set_flag_impl::<gdt::fd!(bool), _>(cpu, proc, false, read_set_bool_reg_arg)
}
pub fn set_bool_by_name(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
    set_flag_by_name_impl::<gdt::fd!(bool), _>(cpu, proc, false, read_set_bool_reg_arg)
}
fn read_set_bool_reg_arg(cpu: &mut Cpu0, _: &mut Process) -> Result<bool, processor::Error> {
    Ok(cpu.read(reg!(w[1])))
}

pub fn set_s32(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
    set_flag_impl::<gdt::fd!(s32), _>(cpu, proc, false, read_set_s32_reg_arg)
}
pub fn set_s32_by_name(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
    set_flag_by_name_impl::<gdt::fd!(s32), _>(cpu, proc, false, read_set_s32_reg_arg)
}
fn read_set_s32_reg_arg(cpu: &mut Cpu0, _: &mut Process) -> Result<i32, processor::Error> {
    Ok(cpu.read(reg!(w[1])))
}

pub fn set_f32(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
    set_flag_impl::<gdt::fd!(f32), _>(cpu, proc, true, read_set_f32_reg_arg)
}
pub fn set_f32_by_name(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
    set_flag_by_name_impl::<gdt::fd!(f32), _>(cpu, proc, true, read_set_f32_reg_arg)
}
fn read_set_f32_reg_arg(cpu: &mut Cpu0, _: &mut Process) -> Result<f32, processor::Error> {
    Ok(cpu.read(reg!(s[0])))
}

pub fn set_str_impl<Fd: gdt::FlagDescriptor<T=String>>(
    cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
    set_flag_impl::<Fd, _>( cpu, proc, false, read_set_str_reg_arg)
}
pub fn set_str_by_name_impl<Fd: gdt::FlagDescriptor<T=String>>(
    cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
    set_flag_by_name_impl::<Fd, _>( cpu, proc, false, read_set_str_reg_arg)
}
fn read_set_str_reg_arg(cpu: &mut Cpu0, proc: &mut Process) -> Result<String, processor::Error> {
    reg! { cpu: x[1] => ptr: u64 };
    let ptr = Ptr!(<u8>(ptr));
    let bytes = ptr.load_zero_terminated(proc.memory())?;
    // TODO --optimize: clone
    let value = match String::from_utf8(bytes.clone()) {
        Ok(s) => s,
        Err(_) => {
            let lossy_s = String::from_utf8_lossy(&bytes).to_string();
            log::error!("Invalid UTF-8 string in read_set_str_reg_arg: {lossy_s}");
            lossy_s
        }
    };
    Ok(value)
}

fn set_vec3f_flag(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
    set_flag_impl::<gdt::fd!(vec3f), _>(cpu, proc, false, read_set_vec3f_reg_arg)
}
fn set_vec3f_by_name(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
    set_flag_by_name_impl::<gdt::fd!(vec3f), _>(cpu, proc, false, read_set_vec3f_reg_arg)
}
fn read_set_vec3f_reg_arg(
    cpu: &mut Cpu0, proc: &mut Process) -> Result<(f32, f32, f32), processor::Error> {
    let ptr: u64 = cpu.read(reg!(x[1]));
    let ptr = Ptr!(<(f32, f32, f32)>(ptr));
    let value = ptr.load(proc.memory())?;
    Ok(value)
}

/// ksys::gdt::TriggerParam::setXXX(this, T/T* in, i32 idx, bool check_perms, bool
/// ignore_one_trigger)
fn set_flag_impl<
Fd: gdt::FlagDescriptor,
Rd: FnOnce(&mut Cpu0, &mut Process) -> Result<Fd::T, processor::Error>,
>(cpu: &mut Cpu0, proc: &mut Process, is_float: bool, reader: Rd) -> Result<(), processor::Error> {
    let this_ptr: u64 = cpu.read(reg!(x[0]));
    let value = reader(cpu, proc)?;
    let (idx, check_perms, ignore_one_trigger) = if is_float {
        let idx: i32 = cpu.read(reg!(w[1]));
        let check_perms: bool = cpu.read(reg!(w[2]));
        let ignore_one_trigger: bool = cpu.read(reg!(w[3]));
        (idx, check_perms, ignore_one_trigger)
    } else {
        let idx: i32 = cpu.read(reg!(w[2]));
        let check_perms: bool = cpu.read(reg!(w[3]));
        let ignore_one_trigger: bool = cpu.read(reg!(w[4]));
        (idx, check_perms, ignore_one_trigger)
    };

    let mut params_proxy = proc.proxies_mut(|p| &mut p.trigger_param);
    let mut params = params_proxy.get_mut(this_ptr)?;
    let Some(flag) = params.get_mut::<Fd, _>(idx) else {
        cpu.write(reg!(x[0]), false);
        cpu.ret();
        return Ok(());
    };
    if check_perms && !flag.writable() {
        cpu.write(reg!(x[0]), false);
        cpu.ret();
        return Ok(());
    }
    // we ignore one_trigger and initial value check
    flag.set(value);
    cpu.write(reg!(x[0]), true);
    cpu.ret();
    Ok(())
}
/// setXXX(this, T/T* in, sead::SafeString* name, bool check_perms, bool unchecked, bool ignore_one_trigger)
fn set_flag_by_name_impl<
Fd: gdt::FlagDescriptor,
Rd: FnOnce(&mut Cpu0, &mut Process) -> Result<Fd::T, processor::Error>,
>(cpu: &mut Cpu0, proc: &mut Process, is_float: bool, reader: Rd) -> Result<(), processor::Error> {
    let this_ptr: u64 = cpu.read(reg!(x[0]));
    let value = reader(cpu, proc)?;
    let (name_ptr, check_perms, ignore_one_trigger) = if is_float {
        reg! {
        cpu:
            x[1] => name_ptr:    u64,
            w[2] => check_perms: bool,
            // NOTE W3 is UNUSED here !!!
            w[4] => ignore_one_trigger: bool,
        };
        (name_ptr, check_perms, ignore_one_trigger)
    } else {
        reg! {
        cpu:
            x[2] => name_ptr:    u64,
            w[3] => check_perms: bool,
            // NOTE W4 is UNUSED here !!!
            w[5] => ignore_one_trigger: bool,
        };
        (name_ptr, check_perms, ignore_one_trigger)
    };

    // the game has no null check here
    let name_ptr = Ptr!(<SafeString>(name_ptr));
    let buf_ptr = Ptr!(&name_ptr->mStringTop).load(proc.memory())?;
    let name = buf_ptr.load_zero_terminated(proc.memory())?;
    if name.is_empty() {
        cpu.write(reg!(x[0]), false);
        cpu.ret();
        return Ok(());
    }
    let name = String::from_utf8_lossy(&name);

    let mut params_proxy = proc.proxies_mut(|p| &mut p.trigger_param);
    let mut params = params_proxy.get_mut(this_ptr)?;
    let Some(flag) = params.by_name_mut::<Fd>(name) else {
        cpu.write(reg!(x[0]), false);
        cpu.ret();
        return Ok(());
    };
    if check_perms && !flag.writable() {
        cpu.write(reg!(x[0]), false);
        cpu.ret();
        return Ok(());
    }
    // we ignore one_trigger and initial value check
    flag.set(value);
    cpu.write(reg!(x[0]), true);
    cpu.ret();
    Ok(())
}

// setXXXArray(
// this,              x0
// T/T* in,           x1 s0
// sead::SafeString* name, w2
// i32 idx,           w2 w1
// bool check_perms, bool unused, bool)
fn set_flag_array_by_name_impl<
Fd: gdt::ArrayFlagDescriptor,
Rd: FnOnce(&mut Cpu0, &mut Process) -> Result<Fd::ElemT, processor::Error>,
>(cpu: &mut Cpu0, proc: &mut Process, is_float: bool, reader: Rd) -> Result<(), processor::Error> {
    reg! { cpu:
        x[0]                        => this_ptr:    u64,
        x[if is_float {1} else {2}] => name_ptr:    u64,
        w[if is_float {2} else {3}] => array_idx:   i32,
        w[if is_float {3} else {4}] => check_perms: bool,
        // one slot for UNUSED variable
        w[if is_float {5} else {6}] => ignore_one_trigger: bool,
    };
    let value = reader(cpu, proc)?;

    // the game has no null check here
    let name_ptr = Ptr!(<SafeString>(name_ptr));
    let buf_ptr = Ptr!(&name_ptr->mStringTop).load(proc.memory())?;
    let name = buf_ptr.load_zero_terminated(proc.memory())?;
    if name.is_empty() {
        reg!{ cpu: x[0] = false, return };
    }
    let name = String::from_utf8_lossy(&name);

    let mut params_proxy = proc.proxies_mut(|p| &mut p.trigger_param);
    let mut params = params_proxy.get_mut(this_ptr)?;
    let Some(flag) = params.by_name_mut::<Fd>(name) else {
        reg!{ cpu: x[0] = false, return };
    };
    if check_perms && !flag.writable() {
        reg!{ cpu: x[0] = false, return };
    }
    // we ignore one_trigger and initial value check
    let result = flag.set_at(array_idx, value);
    reg!{ cpu: x[0] = result, return };
}

fn get_len_impl<Fd: gdt::ArrayFlagDescriptor> (cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
    reg! { cpu:
        x[0] => this_ptr: u64,
        x[1] => out_ptr: u64,
        w[2] => idx: i32,
    };

    let params_proxy = proc.proxies().trigger_param.read(proc.memory());
    let params = params_proxy.get(this_ptr)?;
    let Some(array) = params.get::<Fd, _>(idx) else {
        reg!{ cpu: x[0] = false, return };
    };
    let result = array.len() as i32;
    Ptr!(<i32>(out_ptr)).store(&result, proc.memory_mut())?;
    reg!{ cpu: x[0] = true, return };
}
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
//
//     reg_flag_stubs!(f32, 0xDDF1EC, 0xDF0A28, 0xDE70EC, 0xDE2908, 0xDE5C34);
//     reg_flag_stubs!(string32, 0xDDF264, 0xDF0AE0, 0,        0xDE2F20, 0xDE5D64);
//     reg_flag_stubs!(string64, 0xDDF2F0, 0xDF0B98, 0xDE71D4, 0xDE37B0, 0xDE5E8C);
//     reg_flag_stubs!(string256, 0xDDF37C, 0xDF0C50, 0,       0xDE4040, 0);
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

/// Implements getXXX by index 
///
/// Args: (this, T* out, i32 idx, bool check_perms)
fn get_flag_impl<
    T,
    Fd: gdt::FlagDescriptor,
    Ex: FnOnce(&Fd::T) -> Result<T, processor::Error>,
    St: FnOnce(T, u64, &mut Memory) -> Result<(), processor::Error>,
>(
    cpu: &mut Cpu0,
    proc: &mut Process,
    extractor: Ex,
    storer: St,
) -> Result<(), processor::Error> {
    reg! { cpu:
        x[0] => this_ptr: u64,
        x[1] => out_ptr: u64,
        w[2] => idx: i32,
        w[3] => check_perms: bool,
    };

    proxy!{ let params = *this_ptr as trigger_param in proc };
    let Some(flag) = params.get::<Fd, _>(idx) else {
        reg!{ cpu: x[0] = false, return };
    };
    if check_perms && !flag.readable() {
        reg!{ cpu: x[0] = false, return };
    }
    let value = extractor(flag.get())?;

    storer(value, out_ptr, proc.memory_mut())?;
    reg!{ cpu: x[0] = true, return };
}

/// Impelements getXXX by name
///
/// Args: (this, T* out, sead::SafeString* name, bool check_perms, bool unused)
fn get_flag_by_name_impl<
    T,
    Fd: gdt::FlagDescriptor,
    Ex: FnOnce(&Fd::T) -> Result<T, processor::Error>,
    St: FnOnce(T, u64, &mut Memory) -> Result<(), processor::Error>,
>(
    cpu: &mut Cpu0,
    proc: &mut Process,
    extractor: Ex,
    storer: St,
) -> Result<(), processor::Error> {
    reg! { cpu:
        x[0] => this_ptr: u64,
        x[1] => out_ptr: u64,
        x[2] => name_ptr: Ptr![SafeString],
        w[3] => check_perms: bool,
    };

    let m = proc.memory();
    let name = name_ptr.cstr(m)?.load_utf8_lossy(m)?;

    proxy!{ let params = *this_ptr as trigger_param in proc };
    let Some(flag) = params.by_name::<Fd>(name) else {
        reg!{ cpu: x[0] = false, return };
    };
    if check_perms && !flag.readable() {
        reg!{ cpu: x[0] = false, return };
    }
    let value = extractor(flag.get())?;

    storer(value, out_ptr, proc.memory_mut())?;
    reg! {  cpu: x[0] = true, return };
}

/// Implements getXXX by index with array index
///
/// Args: (this, T* out, i32 idx, i32 array_idx, bool check_perms)
fn get_flag_array_by_index_impl<
    T,
    Fd: gdt::ArrayFlagDescriptor,
    Ex: FnOnce(&Fd::ElemT) -> Result<T, processor::Error>,
    St: FnOnce(T, u64, &mut Memory) -> Result<(), processor::Error>,
>(
    cpu: &mut Cpu0,
    proc: &mut Process,
    extractor: Ex,
    storer: St,
) -> Result<(), processor::Error> {
    reg! { cpu:
        x[0] => this_ptr: u64,
        x[1] => out_ptr: u64,
        w[2] => idx: i32,
        w[3] => array_idx: i32,
        w[4] => check_perms: bool,
    };

    proxy! {let params = *this_ptr as trigger_param in proc};
    let Some(flag) = params.get::<Fd, _>(idx) else {
        reg!{ cpu: x[0] = false, return };
    };

    if check_perms && !flag.readable() {
        reg!{ cpu: x[0] = false, return };
    }

    let Some(entry) = flag.get_at(array_idx) else {
        reg!{ cpu: x[0] = false, return };
    };
    let value = extractor(entry)?;
    storer(value, out_ptr, proc.memory_mut())?;
    reg!{ cpu: x[0] = true, return };
}

/// Implements getXXX by name with array index
///
/// Args: (this, T* out, sead::SafeString* name, i32 array_idx, bool check_perms)
fn get_flag_array_by_name_impl<
    T,
    Fd: gdt::ArrayFlagDescriptor,
    Ex: FnOnce(&Fd::ElemT) -> Result<T, processor::Error>,
    St: FnOnce(T, u64, &mut Memory) -> Result<(), processor::Error>,
>(
    cpu: &mut Cpu0,
    proc: &mut Process,
    extractor: Ex,
    storer: St,
) -> Result<(), processor::Error> {
    reg! { cpu:
        x[0] => this_ptr: u64,
        x[1] => out_ptr: u64,
        x[2] => name_ptr: Ptr![SafeString],
        w[3] => array_idx: i32,
        w[4] => check_perms: bool,
    };

    let m = proc.memory();
    let name = name_ptr.cstr(m)?.load_utf8_lossy(m)?;

    proxy! {let params = *this_ptr as trigger_param in proc};
    let Some(flag) = params.by_name::<Fd>(name) else {
        reg!{ cpu: x[0] = false, return };
    };

    if check_perms && !flag.readable() {
        reg!{ cpu: x[0] = false, return };
    }

    let Some(entry) = flag.get_at(array_idx) else {
        reg!{ cpu: x[0] = false, return };
    };
    let value = extractor(entry)?;
    storer(value, out_ptr, proc.memory_mut())?;
    reg!{ cpu: x[0] = true, return };
}
