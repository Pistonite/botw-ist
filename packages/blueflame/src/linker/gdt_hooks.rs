use crate::game::{SafeString, gdt};
use crate::memory::{Memory, Ptr, proxy, mem};
use crate::processor::{self, Cpu0, Process, reg};

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
    // TODO --optimize: we can probably avoid having to alloc new string every time here
    pub fn get_str<Fd: gdt::FlagDescriptor<T=String>>(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
        get_flag_impl::<Vec<u8>, Fd, _, _>(cpu, proc, get_str_extractor, get_str_storer)
    }
    pub fn get_str_by_name<Fd: gdt::FlagDescriptor<T=String>>(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
        get_flag_by_name_impl::<Vec<u8>, Fd, _, _>( cpu, proc, get_str_extractor, get_str_storer,)
    }
    pub fn get_str_array<Fd: gdt::ArrayFlagDescriptor<ElemT=String>>(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
        get_flag_array_by_index_impl::<Vec<u8>, Fd, _, _>( cpu, proc, get_str_extractor, get_str_storer)
    }
    pub fn get_str_array_by_name<Fd: gdt::ArrayFlagDescriptor<ElemT=String>>(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
        get_flag_array_by_name_impl::<Vec<u8>, Fd, _, _>( cpu, proc, get_str_extractor, get_str_storer,)
    }
    fn get_str_extractor(s: &String) -> Result<Vec<u8>, processor::Error> {
        let mut v = s.as_bytes().to_vec();
        v.push(0); // null-terminate
        Ok(v)
    }
    fn get_str_storer(value: Vec<u8>, out_ptr: u64, memory: &mut Memory) -> Result<(), processor::Error> {
        let ptr = memory.alloc_with(&value)?;
        // out_ptr is char**
        Ok(Ptr!(<u64>(out_ptr)).store(&ptr, memory)?)
    }
}
pub use __impl_get::*;

#[rustfmt::skip]
mod __impl_set {
    use super::*;
    pub fn set_bool(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { set_flag_by_index_impl::<gdt::fd!(bool), _>(cpu, proc, false, read_set_bool_reg_arg) }
    pub fn set_s32(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { set_flag_by_index_impl::<gdt::fd!(s32), _>(cpu, proc, false, read_set_s32_reg_arg) }
    pub fn set_f32(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { set_flag_by_index_impl::<gdt::fd!(f32), _>(cpu, proc, true, read_set_f32_reg_arg) }
    pub fn set_str<Fd: gdt::FlagDescriptor<T=String>>( cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { set_flag_by_index_impl::<Fd, _>( cpu, proc, false, read_set_str_reg_arg) }
    pub fn set_vec3f(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { set_flag_by_index_impl::<gdt::fd!(vec3f), _>(cpu, proc, false, read_set_vec3f_reg_arg) }
    pub fn set_bool_by_name(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { set_flag_by_name_impl::<gdt::fd!(bool), _>(cpu, proc, false, read_set_bool_reg_arg) }
    pub fn set_s32_by_name(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { set_flag_by_name_impl::<gdt::fd!(s32), _>(cpu, proc, false, read_set_s32_reg_arg) }
    pub fn set_f32_by_name(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { set_flag_by_name_impl::<gdt::fd!(f32), _>(cpu, proc, true, read_set_f32_reg_arg) }
    pub fn set_str_by_name<Fd: gdt::FlagDescriptor<T=String>>(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { set_flag_by_name_impl::<Fd, _>( cpu, proc, false, read_set_str_reg_arg) }
    pub fn set_vec3f_by_name(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { set_flag_by_name_impl::<gdt::fd!(vec3f), _>(cpu, proc, false, read_set_vec3f_reg_arg) }
    pub fn set_bool_array(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { set_flag_array_by_index_impl::<gdt::fd!(bool[]), _>(cpu, proc, false, read_set_bool_reg_arg) }
    pub fn set_s32_array(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { set_flag_array_by_index_impl::<gdt::fd!(s32[]), _>(cpu, proc, false, read_set_s32_reg_arg) }
    pub fn set_f32_array(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { set_flag_array_by_index_impl::<gdt::fd!(f32[]), _>(cpu, proc, true, read_set_f32_reg_arg) }
    pub fn set_str_array<Fd: gdt::ArrayFlagDescriptor<ElemT=String>>(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { set_flag_array_by_index_impl::<Fd, _>(cpu, proc, false, read_set_str_reg_arg) }
    pub fn set_vec2f_array(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { set_flag_array_by_index_impl::<gdt::fd!(vec2f[]), _>(cpu, proc, false, read_set_vec2f_reg_arg) }
    pub fn set_vec3f_array(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { set_flag_array_by_index_impl::<gdt::fd!(vec3f[]), _>(cpu, proc, false, read_set_vec3f_reg_arg) }
    pub fn set_bool_array_by_name(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { set_flag_array_by_name_impl::<gdt::fd!(bool[]), _>(cpu, proc, false, read_set_bool_reg_arg) }
    pub fn set_s32_array_by_name(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { set_flag_array_by_name_impl::<gdt::fd!(s32[]), _>(cpu, proc, false, read_set_s32_reg_arg) }
    pub fn set_f32_array_by_name(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { set_flag_array_by_name_impl::<gdt::fd!(f32[]), _>(cpu, proc, true, read_set_f32_reg_arg) }
    pub fn set_str_array_by_name<Fd: gdt::ArrayFlagDescriptor<ElemT=String>>(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> { set_flag_array_by_name_impl::<Fd, _>(cpu, proc, false, read_set_str_reg_arg) }
    fn read_set_bool_reg_arg(cpu: &mut Cpu0, _: &mut Process) -> Result<bool, processor::Error> {
        Ok(cpu.read(reg!(w[1])))
    }
    fn read_set_s32_reg_arg(cpu: &mut Cpu0, _: &mut Process) -> Result<i32, processor::Error> {
        Ok(cpu.read(reg!(w[1])))
    }
    fn read_set_f32_reg_arg(cpu: &mut Cpu0, _: &mut Process) -> Result<f32, processor::Error> {
        Ok(cpu.read(reg!(s[0])))
    }
    fn read_set_str_reg_arg(cpu: &mut Cpu0, proc: &mut Process) -> Result<String, processor::Error> {
        reg! { cpu: 
            x[1] => let ptr: Ptr![u8]
        };
        Ok(ptr.load_utf8_lossy(proc.memory())?)
    }
    fn read_set_vec2f_reg_arg(cpu: &mut Cpu0, proc: &mut Process) -> Result<(f32, f32), processor::Error> {
        reg! { cpu:
            x[1] => let ptr: Ptr![(f32, f32)],
        };
        Ok(ptr.load(proc.memory())?)
    }
    fn read_set_vec3f_reg_arg(cpu: &mut Cpu0, proc: &mut Process) -> Result<(f32, f32, f32), processor::Error> {
        reg! { cpu:
            x[1] => let ptr: Ptr![(f32, f32, f32)],
        };
        Ok(ptr.load(proc.memory())?)
    }



}
pub use __impl_set::*;

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
        x[0] => let this_ptr: u64,
        x[1] => let out_ptr: u64,
        w[2] => let idx: i32,
        w[3] => let check_perms: bool,
    };

    proxy! { let params = *this_ptr as trigger_param in proc };
    let Some(flag) = params.get::<Fd, _>(idx) else {
        reg! { cpu: x[0] = false, return };
    };
    if check_perms && !flag.readable() {
        reg! { cpu: x[0] = false, return };
    }
    let value = extractor(flag.get())?;

    storer(value, out_ptr, proc.memory_mut())?;
    reg! { cpu: x[0] = true, return };
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
        x[0] => let this_ptr: u64,
        x[1] => let out_ptr: u64,
        x[2] => let name_ptr: Ptr![SafeString],
        w[3] => let check_perms: bool,
    };

    let m = proc.memory();
    let name = name_ptr.cstr(m)?.load_utf8_lossy(m)?;
    if name.is_empty() {
        reg! { cpu: x[0] = false, return };
    }

    proxy! { let params = *this_ptr as trigger_param in proc };
    let Some(flag) = params.by_name::<Fd>(name) else {
        reg! { cpu: x[0] = false, return };
    };
    if check_perms && !flag.readable() {
        reg! { cpu: x[0] = false, return };
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
        x[0] => let this_ptr: u64,
        x[1] => let out_ptr: u64,
        w[2] => let idx: i32,
        w[3] => let array_idx: i32,
        w[4] => let check_perms: bool,
    };

    proxy! {let params = *this_ptr as trigger_param in proc};
    let Some(flag) = params.get::<Fd, _>(idx) else {
        reg! { cpu: x[0] = false, return };
    };

    if check_perms && !flag.readable() {
        reg! { cpu: x[0] = false, return };
    }

    let Some(entry) = flag.get_at(array_idx) else {
        reg! { cpu: x[0] = false, return };
    };
    let value = extractor(entry)?;
    storer(value, out_ptr, proc.memory_mut())?;
    reg! { cpu: x[0] = true, return };
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
        x[0] => let this_ptr: u64,
        x[1] => let out_ptr: u64,
        x[2] => let name_ptr: Ptr![SafeString],
        w[3] => let array_idx: i32,
        w[4] => let check_perms: bool,
    };

    let m = proc.memory();
    let name = name_ptr.cstr(m)?.load_utf8_lossy(m)?;

    proxy! {let params = *this_ptr as trigger_param in proc};
    let Some(flag) = params.by_name::<Fd>(name) else {
        reg! { cpu: x[0] = false, return };
    };

    if check_perms && !flag.readable() {
        reg! { cpu: x[0] = false, return };
    }

    let Some(entry) = flag.get_at(array_idx) else {
        reg! { cpu: x[0] = false, return };
    };
    let value = extractor(entry)?;
    storer(value, out_ptr, proc.memory_mut())?;
    reg! { cpu: x[0] = true, return };
}

/// Implements getXXXArraySize
pub fn get_array_size<Fd: gdt::ArrayFlagDescriptor>(
    cpu: &mut Cpu0,
    proc: &mut Process,
) -> Result<(), processor::Error> {
    reg! { cpu:
        x[0] => let this_ptr: u64,
        x[1] => let out_ptr: Ptr![i32],
        w[2] => let idx: i32,
    };

    proxy! { let params = *this_ptr as trigger_param in proc };
    let Some(array) = params.get::<Fd, _>(idx) else {
        reg! { cpu: x[0] = false, return };
    };
    let result = array.len() as i32;
    out_ptr.store(&result, proc.memory_mut())?;
    reg! { cpu: x[0] = true, return };
}

/// Implements getXXXArraySizeByHash
pub fn get_array_size_by_hash<Fd: gdt::ArrayFlagDescriptor>(
    cpu: &mut Cpu0,
    proc: &mut Process,
) -> Result<(), processor::Error> {
    reg! { cpu:
        x[0] => let this_ptr: u64,
        x[1] => let out_ptr: Ptr![i32],
        w[2] => let hash: i32,
    };

    proxy! { let params = *this_ptr as trigger_param in proc };
    let Some(array) = params.by_hash::<Fd>(hash) else {
        reg! { cpu: x[0] = false, return };
    };
    let result = array.len() as i32;
    out_ptr.store(&result, proc.memory_mut())?;
    reg! { cpu: x[0] = true, return };
}

/// Implements setXXX
///
/// Args:(this, T/T* in, i32 idx, bool check_perms, bool ignore_one_trigger)
///
/// since T-in is by value, if it's a float, it uses s0 and the rest of the register
/// shifts
fn set_flag_by_index_impl<
    Fd: gdt::FlagDescriptor,
    Rd: FnOnce(&mut Cpu0, &mut Process) -> Result<Fd::T, processor::Error>,
>(
    cpu: &mut Cpu0,
    proc: &mut Process,
    is_float: bool,
    reader: Rd,
) -> Result<(), processor::Error> {
    reg! { cpu:
        x[0] =>                        let this_ptr: u64,
        x[if is_float {1} else {2}] => let idx: i32,
        w[if is_float {2} else {3}] => let check_perms: bool,
        w[if is_float {3} else {4}] => let _ignore_one_trigger: bool,
    };
    let value = reader(cpu, proc)?;

    proxy! { let mut params = *this_ptr as trigger_param in proc };
    let Some(flag) = params.get_mut::<Fd, _>(idx) else {
        reg! { cpu: x[0] = false, return };
    };
    if check_perms && !flag.writable() {
        reg! { cpu: x[0] = false, return };
    }
    // we ignore one_trigger and initial value check
    flag.set(value);
    reg! { cpu: x[0] = true, return };
}

/// Implements setXXX
///
/// Args: (this, T/T* in, sead::SafeString* name, bool check_perms, bool unused, bool ignore_one_trigger)
///
/// Since T-in is by value, if it's a float, it uses s0 and the rest of the register
fn set_flag_by_name_impl<
    Fd: gdt::FlagDescriptor,
    Rd: FnOnce(&mut Cpu0, &mut Process) -> Result<Fd::T, processor::Error>,
>(
    cpu: &mut Cpu0,
    proc: &mut Process,
    is_float: bool,
    reader: Rd,
) -> Result<(), processor::Error> {
    reg! { cpu:
        x[0] =>                        let this_ptr:    u64,
        x[if is_float {1} else {2}] => let name_ptr:    Ptr![SafeString],
        w[if is_float {2} else {3}] => let check_perms: bool,
        // one slot for UNUSED variable
        w[if is_float {4} else {5}] => let _ignore_one_trigger: bool,
    };
    let value = reader(cpu, proc)?;

    let m = proc.memory();
    let name = name_ptr.cstr(m)?.load_utf8_lossy(m)?;
    if name.is_empty() {
        reg! { cpu: x[0] = false, return };
    }

    proxy! { let mut params = *this_ptr as trigger_param in proc };
    let Some(flag) = params.by_name_mut::<Fd>(name) else {
        reg! { cpu: x[0] = false, return };
    };
    if check_perms && !flag.writable() {
        reg! { cpu: x[0] = false, return };
    }
    // we ignore one_trigger and initial value check
    flag.set(value);
    reg! { cpu: x[0] = true, return };
}

/// Implements setXXX
///
/// Args:(this, T/T* in, i32 idx, i32 array_idx, bool check_perms, bool ignore_one_trigger)
///
/// since T-in is by value, if it's a float, it uses s0 and the rest of the register
/// shifts
fn set_flag_array_by_index_impl<
    Fd: gdt::ArrayFlagDescriptor,
    Rd: FnOnce(&mut Cpu0, &mut Process) -> Result<Fd::ElemT, processor::Error>,
>(
    cpu: &mut Cpu0,
    proc: &mut Process,
    is_float: bool,
    reader: Rd,
) -> Result<(), processor::Error> {
    reg! { cpu:
        x[0] =>                        let this_ptr: u64,
        x[if is_float {1} else {2}] => let idx: i32,
        w[if is_float {2} else {3}] => let array_idx: i32,
        w[if is_float {3} else {4}] => let check_perms: bool,
        w[if is_float {4} else {5}] => let _ignore_one_trigger: bool,
    };
    let value = reader(cpu, proc)?;

    proxy! { let mut params = *this_ptr as trigger_param in proc };
    let Some(flag) = params.get_mut::<Fd, _>(idx) else {
        reg! { cpu: x[0] = false, return };
    };
    if check_perms && !flag.writable() {
        reg! { cpu: x[0] = false, return };
    }
    // we ignore one_trigger and initial value check
    let result = flag.set_at(array_idx, value);
    reg! { cpu: x[0] = result, return };
}

/// Implements setXXX
///
/// Args:(this, T/T* in, sead::SafeString* name, i32 idx, bool check_perms, bool unused, bool ignore_ot)
/// since T-in is by value, if it's a float, it uses s0 and the rest of the register
/// shifts
fn set_flag_array_by_name_impl<
    Fd: gdt::ArrayFlagDescriptor,
    Rd: FnOnce(&mut Cpu0, &mut Process) -> Result<Fd::ElemT, processor::Error>,
>(
    cpu: &mut Cpu0,
    proc: &mut Process,
    is_float: bool,
    reader: Rd,
) -> Result<(), processor::Error> {
    reg! { cpu:
        x[0]                        => let this_ptr:    u64,
        x[if is_float {1} else {2}] => let name_ptr:    Ptr![SafeString],
        w[if is_float {2} else {3}] => let array_idx:   i32,
        w[if is_float {3} else {4}] => let check_perms: bool,
        // one slot for UNUSED variable
        w[if is_float {5} else {6}] => let _ignore_one_trigger: bool,
    };
    let value = reader(cpu, proc)?;

    let m = proc.memory();
    let name = name_ptr.cstr(m)?.load_utf8_lossy(m)?;
    if name.is_empty() {
        reg! { cpu: x[0] = false, return };
    }

    proxy! { let mut params = *this_ptr as trigger_param in proc };
    let Some(flag) = params.by_name_mut::<Fd>(name) else {
        reg! { cpu: x[0] = false, return };
    };
    if check_perms && !flag.writable() {
        reg! { cpu: x[0] = false, return };
    }
    // we ignore one_trigger and initial value check
    let result = flag.set_at(array_idx, value);
    reg! { cpu: x[0] = result, return };
}

/// Implements resetXXX
pub fn reset<Fd: gdt::FlagDescriptor>(
    cpu: &mut Cpu0,
    proc: &mut Process,
) -> Result<(), processor::Error> {
    reg! { cpu:
        x[0] => let this_ptr: u64,
        w[1] => let idx: i32,
        w[2] => let check_perms: bool,
    }

    proxy! { let mut params = *this_ptr as trigger_param in proc };
    let Some(flag) = params.get_mut::<Fd, _>(idx) else {
        reg! { cpu: x[0] = false, return };
    };
    if check_perms && !flag.writable() {
        reg! { cpu: x[0] = false, return };
    }
    flag.reset();
    reg! { cpu: x[0] = true, return }
}

/// Implements resetXXX by name
pub fn reset_by_name<Fd: gdt::FlagDescriptor>(
    cpu: &mut Cpu0,
    proc: &mut Process,
) -> Result<(), processor::Error> {
    reg! { cpu:
        x[0] => let this_ptr: u64,
        x[1] => let name_ptr: Ptr![SafeString],
        w[2] => let check_perms: bool,
    }

    let m = proc.memory();
    let name = name_ptr.cstr(m)?.load_utf8_lossy(m)?;
    if name.is_empty() {
        reg! { cpu: x[0] = false, return };
    }

    proxy! { let mut params = *this_ptr as trigger_param in proc };
    let Some(flag) = params.by_name_mut::<Fd>(name) else {
        reg! { cpu: x[0] = false, return };
    };
    if check_perms && !flag.writable() {
        reg! { cpu: x[0] = false, return };
    }
    flag.reset();
    reg! { cpu: x[0] = true, return }
}

/// Implements resetXXX array
pub fn reset_array<Fd: gdt::ArrayFlagDescriptor>(
    cpu: &mut Cpu0,
    proc: &mut Process,
) -> Result<(), processor::Error> {
    reg! { cpu:
        x[0] => let this_ptr: u64,
        w[1] => let idx: i32,
        w[2] => let array_idx: i32,
        w[3] => let check_perms: bool,
    }

    proxy! { let mut params = *this_ptr as trigger_param in proc };
    let Some(flag) = params.get_mut::<Fd, _>(idx) else {
        reg! { cpu: x[0] = false, return };
    };
    if check_perms && !flag.writable() {
        reg! { cpu: x[0] = false, return };
    }
    let result = flag.reset_at(array_idx);
    reg! { cpu: x[0] = result, return }
}

pub fn reset_all(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
    reg! { cpu:
        x[0] => let this_ptr: u64,
    };
    proxy! { let mut params = *this_ptr as trigger_param in proc };
    params.reset_all();
    Ok(())
}

/// ksys::gdt::TriggerParam::getXXXIdx(this, i32 hash) -> int
pub fn idx_from_hash<Fd: gdt::FlagDescriptor>(
    cpu: &mut Cpu0,
    proc: &mut Process,
) -> Result<(), processor::Error> {
    reg! { cpu:
        x[0] => let this_ptr: u64,
        w[1] => let hash: i32,
    }
    proxy! { let params = *this_ptr as trigger_param in proc };
    let index = match params.index_from_hash::<Fd>(hash) {
        Some(i) => i as i32,
        None => -1,
    };

    reg! { cpu: w[0] = index, return };
}

/// ksys::get::TriggerParam::getMaxValueForS32
pub fn get_s32_max(
    cpu: &mut Cpu0,
    proc: &mut Process,
) -> Result<(), processor::Error> {
    reg! { cpu:
        x[0] => let this_ptr: u64,
        x[1] => let out_ptr: Ptr![i32],
        x[2] => let name_ptr: Ptr![SafeString]
    };

    let m = proc.memory();
    let name = name_ptr.cstr(m)?.load_utf8_lossy(m)?;
    if name.is_empty() {
        reg! { cpu: x[0] = false, return };
    }

    let value = {
        proxy! { let mut params = *this_ptr as trigger_param in proc };
        let Some(flag) = params.by_name_mut::<gdt::fd!(s32)>(name) else {
            reg! { cpu: x[0] = false, return };
        };
        flag.max()
    };
    mem! { (proc.memory_mut()): *out_ptr = value; };
    reg! { cpu: x[0] = true, return }
}
