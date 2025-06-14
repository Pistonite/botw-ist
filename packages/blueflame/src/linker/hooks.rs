use super::gdt_hooks;
use crate::env::{DlcVer, Environment, GameVer};
use crate::game::{gdt, singleton_instance};
use crate::memory::{self, Memory, access};
use crate::processor::insn::paste_insn;
use crate::processor::{self, Cpu0, Execute, HookProvider, Process, reg};

macro_rules! fn_table {
    ($main_offset:ident size fn $( $offset:literal $size:literal $function:expr ),* $(,)?) => {
        match $main_offset {
        $(
            #[allow(clippy::zero_prefixed_literal)]
            $offset => {
                // log::trace!("reached hook function at 0x{:#08x} with size {}: {}", $offset, $size, stringify!($function));
                return Ok(Some((processor::box_execute($function), $size)));
            }
        )*
        _ => {}
        }
    };
}

/// Patch the (instruction) memory before running
pub fn patch_memory(memory: &mut Memory, env: Environment) -> Result<(), memory::Error> {
    let main_start = memory.program_start() + env.main_offset() as u64;

    if env.is160() {
        // TODO --160: patch for 160
        return Ok(());
    }

    let f = access!(force);

    // in uking::ui::PauseMenuDataMgr::createPlayerEquipment and doCreateEquipmentFromItem
    // skip the check for if CreatePlayerEquipActorMgr is null
    memory
        .write(main_start + 0x971540, f)?
        .write_u32(paste_insn!(1F 20 03 D5))?;
    memory
        .write(main_start + 0xaa81ec, f)?
        .write_u32(paste_insn!(1F 20 03 D5))?;

    Ok(())
}

pub struct GameHooks;
impl HookProvider for GameHooks {
    fn fetch(
        &self,
        main_offset: u32,
        env: Environment,
    ) -> Result<Option<(Box<dyn Execute>, u32)>, processor::Error> {
        if env.is160() {
            // TODO --160: hooks for 160
            return Ok(None);
        }

        fn_table! {

        main_offset size   fn
        0x006669f8  000408 return_void, // uking::act::CreatePlayerEquipActorMgr::doRequestCreateWeapon
        0x00666cf8  000688 return_void, // uking::act::CreatePlayerEquipActorMgr::doRequestCreateArmor
        0x00849580  003456 return_void, // Player::equipmentStuff
        0x0085456c  000068 get_player,  // ksys::act::PlayerInfo::getPlayer
        0x00d2e950  000348 return_void, // ksys::act::InfoData::logFailure

        // ksys::gdt::TriggerParam
        0x00ddf0f8  000124 gdt_hooks::get_bool, // getBool by idx
        0x00ddf174  000120 gdt_hooks::get_s32,  // getS32 by idx
        0x00ddf1ec  000120 gdt_hooks::get_f32,  // getF32 by idx
        0x00ddf264  000140 gdt_hooks::get_str::<gdt::fd!(str32)>, // getStr by idx
        0x00ddf2f0  000140 gdt_hooks::get_str::<gdt::fd!(str64)>, // getStr by idx
        0x00ddf37c  000140 gdt_hooks::get_str::<gdt::fd!(str256)>, // getStr by idx
        0x00ddf408  000124 gdt_hooks::get_vec3f, // getVec3f by idx
        0x00ddf484  000368 gdt_hooks::get_bool_by_name,
        0x00ddf5f4  000368 gdt_hooks::get_bool_by_name,
        0x00ddf764  000364 gdt_hooks::get_s32_by_name,
        0x00ddf8d0  000364 gdt_hooks::get_f32_by_name,
        0x00ddfa3c  000384 gdt_hooks::get_str_by_name::<gdt::fd!(str32)>,
        0x00ddfbbc  000384 gdt_hooks::get_str_by_name::<gdt::fd!(str64)>,
        0x00ddfd3c  000384 gdt_hooks::get_str_by_name::<gdt::fd!(str256)>,
        0x00ddfebc  000368 gdt_hooks::get_vec3f_by_name,
        0x00de002c  000164 gdt_hooks::get_bool_array, //_ZNK4ksys3gdt12TriggerParam7getBoolEPbiib
        0x00de00d0  000160 gdt_hooks::get_s32_array, //_ZNK4ksys3gdt12TriggerParam6getS32EPiiib
        0x00de0170  000160 gdt_hooks::get_f32_array, //_ZNK4ksys3gdt12TriggerParam6getF32EPfiib
        0x00de0210  000180 gdt_hooks::get_str_array::<gdt::fd!(str64[])>, //_ZNK4ksys3gdt12TriggerParam8getStr64EPPKciib
        0x00de02c4  000180 gdt_hooks::get_str_array::<gdt::fd!(str256[])>, //_ZNK4ksys3gdt12TriggerParam9getStr256EPPKciib
        0x00de0378  000160 gdt_hooks::get_vec2f_array, //_ZNK4ksys3gdt12TriggerParam8getVec2fEPN4sead7Vector2IfEEiib
        0x00de0418  000164 gdt_hooks::get_vec3f_array, //_ZNK4ksys3gdt12TriggerParam8getVec3fEPN4sead7Vector3IfEEiib
        0x00de04bc  000356 gdt_hooks::get_bool_array_by_name, //_ZNK4ksys3gdt12TriggerParam7getBoolEPbRKN4sead14SafeStringBaseIcEEibb
        0x00de0620  000352 gdt_hooks::get_s32_array_by_name, //_ZNK4ksys3gdt12TriggerParam6getS32EPiRKN4sead14SafeStringBaseIcEEibb
        0x00de0780  000352 gdt_hooks::get_f32_array_by_name, //_ZNK4ksys3gdt12TriggerParam6getF32EPfRKN4sead14SafeStringBaseIcEEibb
        0x00de08e0  000372 return_false,//_ZNK4ksys3gdt12TriggerParam6getStrEPPKcRKN4sead14SafeStringBaseIcEEibb
        0x00de0a54  000372 gdt_hooks::get_str_array_by_name::<gdt::fd!(str64[])>,//_ZNK4ksys3gdt12TriggerParam8getStr64EPPKcRKN4sead14SafeStringBaseIcEEibb
        0x00de0bc8  000372 gdt_hooks::get_str_array_by_name::<gdt::fd!(str256[])>,//_ZNK4ksys3gdt12TriggerParam9getStr256EPPKcRKN4sead14SafeStringBaseIcEEibb
        0x00de0d3c  000056 gdt_hooks::get_array_size::<gdt::fd!(bool[])>,//_ZNK4ksys3gdt12TriggerParam16getBoolArraySizeEPii
        0x00de0d74  000056 gdt_hooks::get_array_size::<gdt::fd!(s32[])>,//_ZNK4ksys3gdt12TriggerParam15getS32ArraySizeEPii
        0x00de0dac  000056 gdt_hooks::get_array_size::<gdt::fd!(f32[])>,//_ZNK4ksys3gdt12TriggerParam15getF32ArraySizeEPii
        0x00de0de4  000056 return_0,//_ZNK4ksys3gdt12TriggerParam15getStrArraySizeEPii
        0x00de0e1c  000056 gdt_hooks::get_array_size::<gdt::fd!(str64[])>,//_ZNK4ksys3gdt12TriggerParam17getStr64ArraySizeEPii
        0x00de0e54  000056 gdt_hooks::get_array_size::<gdt::fd!(str256[])>,//_ZNK4ksys3gdt12TriggerParam18getStr256ArraySizeEPii
        0x00de0e8c  000056 gdt_hooks::get_array_size::<gdt::fd!(vec2f[])>,//_ZNK4ksys3gdt12TriggerParam17getVec2fArraySizeEPii
        0x00de0ec4  000056 gdt_hooks::get_array_size::<gdt::fd!(vec3f[])>,//_ZNK4ksys3gdt12TriggerParam17getVec3fArraySizeEPii
        0x00de0efc  000056 return_0,//_ZNK4ksys3gdt12TriggerParam17getVec4fArraySizeEPii
        0x00de0f34  000192 gdt_hooks::get_array_size_by_hash::<gdt::fd!(bool[])>,//_ZNK4ksys3gdt12TriggerParam22getBoolArraySizeByHashEPij
        0x00de0ff4  000192 gdt_hooks::get_array_size_by_hash::<gdt::fd!(s32[])>,//_ZNK4ksys3gdt12TriggerParam21getS32ArraySizeByHashEPij
        0x00de10b4  000192 gdt_hooks::get_array_size_by_hash::<gdt::fd!(f32[])>,//_ZNK4ksys3gdt12TriggerParam21getF32ArraySizeByHashEPij
        0x00de1174  000192 return_0,//_ZNK4ksys3gdt12TriggerParam21getStrArraySizeByHashEPij
        0x00de1234  000192 gdt_hooks::get_array_size_by_hash::<gdt::fd!(str64[])>,//_ZNK4ksys3gdt12TriggerParam23getStr64ArraySizeByHashEPij
        0x00de12f4  000192 gdt_hooks::get_array_size_by_hash::<gdt::fd!(str256[])>,//_ZNK4ksys3gdt12TriggerParam24getStr256ArraySizeByHashEPij
        0x00de13b4  000192 gdt_hooks::get_array_size_by_hash::<gdt::fd!(vec2f[])>,//_ZNK4ksys3gdt12TriggerParam23getVec2fArraySizeByHashEPij
        0x00de1474  000192 gdt_hooks::get_array_size_by_hash::<gdt::fd!(vec3f[])>,//_ZNK4ksys3gdt12TriggerParam23getVec3fArraySizeByHashEPij
        0x00de1534  000192 return_0, //_ZNK4ksys3gdt12TriggerParam23getVec4fArraySizeByHashEPij
        // 0x00de15f4  000248,_ZNK4ksys3gdt12TriggerParam15getS32ArraySizeEPiRKN4sead14SafeStringBaseIcEE
        // 0x00de16ec  000248,_ZNK4ksys3gdt12TriggerParam17getStr64ArraySizeEPiRKN4sead14SafeStringBaseIcEE
        // 0x00de17e4  000248,_ZNK4ksys3gdt12TriggerParam17getVec3fArraySizeEPiRKN4sead14SafeStringBaseIcEE
        // 0x00de18dc  000324,_ZNK4ksys3gdt12TriggerParam17getMinValueForS32EPiRKN4sead14SafeStringBaseIcEE
        0x00de1a20  000324 gdt_hooks::get_s32_max,

        0x00de1b64  000236 gdt_hooks::set_bool, // setBool by idx
        0x00de22f8  000332 gdt_hooks::set_s32,  // setS32 by idx
        0x00de2908  000340 gdt_hooks::set_f32,//_ZN4ksys3gdt12TriggerParam6setF32Efibb
        0x00de2f20  000432 gdt_hooks::set_str::<gdt::fd!(str32)>,//_ZN4ksys3gdt12TriggerParam6setStrEPKcibb
        0x00de37b0  000432 gdt_hooks::set_str::<gdt::fd!(str64)>,//_ZN4ksys3gdt12TriggerParam8setStr64EPKcibb
        0x00de4040  000440 gdt_hooks::set_str::<gdt::fd!(str256)>,//_ZN4ksys3gdt12TriggerParam9setStr256EPKcibb
        0x00de4ea0  000180 gdt_hooks::set_vec3f,//_ZN4ksys3gdt12TriggerParam8setVec3fERKN4sead7Vector3IfEEibb
        0x00de59e4  000296 gdt_hooks::set_bool_by_name, // setBool by name
        0x00de5b0c  000296 gdt_hooks::set_s32_by_name,  // setS32 by name
        0x00de5c34  000304 gdt_hooks::set_f32_by_name,//_ZN4ksys3gdt12TriggerParam6setF32EfRKN4sead14SafeStringBaseIcEEbbb
        0x00de5d64  000296 gdt_hooks::set_str_by_name::<gdt::fd!(str32)>,//_ZN4ksys3gdt12TriggerParam6setStrEPKcRKN4sead14SafeStringBaseIcEEbbb
        0x00de5e8c  000296 gdt_hooks::set_str_by_name::<gdt::fd!(str64)>,//_ZN4ksys3gdt12TriggerParam8setStr64EPKcRKN4sead14SafeStringBaseIcEEbbb
        0x00de5fb4  000444 gdt_hooks::set_vec3f_by_name,//_ZN4ksys3gdt12TriggerParam8setVec3fERKN4sead7Vector3IfEERKNS2_14SafeStringBaseIcEEbbb
        0x00de6170  000236 gdt_hooks::set_bool_array,//_ZN4ksys3gdt12TriggerParam7setBoolEbiibb
        0x00de625c  000388 gdt_hooks::set_s32_array,//_ZN4ksys3gdt12TriggerParam6setS32Eiiibb
        0x00de63e0  000396 gdt_hooks::set_f32_array,//_ZN4ksys3gdt12TriggerParam6setF32Efiibb
        0x00de656c  000492 gdt_hooks::set_str_array::<gdt::fd!(str64[])>,//_ZN4ksys3gdt12TriggerParam8setStr64EPKciibb
        0x00de6758  000492 gdt_hooks::set_str_array::<gdt::fd!(str256[])>,//_ZN4ksys3gdt12TriggerParam9setStr256EPKciibb
        0x00de6944  000224 gdt_hooks::set_vec2f_array,//_ZN4ksys3gdt12TriggerParam8setVec2fERKN4sead7Vector2IfEEiibb
        0x00de6a24  000224 gdt_hooks::set_vec3f_array,//_ZN4ksys3gdt12TriggerParam8setVec3fERKN4sead7Vector3IfEEiibb
        0x00de6b04  000260 gdt_hooks::set_bool_array_by_name,//_ZN4ksys3gdt12TriggerParam7setBoolEbRKN4sead14SafeStringBaseIcEEibbb
        0x00de6c08  000260 gdt_hooks::set_s32_array_by_name,//_ZN4ksys3gdt12TriggerParam6setS32EiRKN4sead14SafeStringBaseIcEEibbb
        0x00de6d0c  000268 gdt_hooks::set_f32_array_by_name,//_ZN4ksys3gdt12TriggerParam6setF32EfRKN4sead14SafeStringBaseIcEEibbb
        0x00de6e18  000260 gdt_hooks::set_str_array_by_name::<gdt::fd!(str64[])>,//_ZN4ksys3gdt12TriggerParam8setStr64EPKcRKN4sead14SafeStringBaseIcEEibbb
        0x00de6f1c  000232 gdt_hooks::reset::<gdt::fd!(bool)>, // resetBool
        0x00de7004  000232 gdt_hooks::reset::<gdt::fd!(s32)>,  // resetS32
        0x00de70ec  000232 gdt_hooks::reset::<gdt::fd!(f32)>,//_ZN4ksys3gdt12TriggerParam8resetF32Eib
        0x00de71d4  000232 gdt_hooks::reset::<gdt::fd!(str64)>,//_ZN4ksys3gdt12TriggerParam10resetStr64Eib
        0x00de72bc  000232 gdt_hooks::reset::<gdt::fd!(vec3f)>,//_ZN4ksys3gdt12TriggerParam10resetVec3fEib
        0x00de73a4  000272 gdt_hooks::reset_by_name::<gdt::fd!(bool)>,//_ZN4ksys3gdt12TriggerParam9resetBoolERKN4sead14SafeStringBaseIcEEbb
        0x00de74b4  000272 gdt_hooks::reset_by_name::<gdt::fd!(s32)>,//_ZN4ksys3gdt12TriggerParam8resetS32ERKN4sead14SafeStringBaseIcEEbb
        0x00de75c4  000272 gdt_hooks::reset_by_name::<gdt::fd!(f32)>,//_ZN4ksys3gdt12TriggerParam10resetStr64ERKN4sead14SafeStringBaseIcEEbb
        0x00de76d4  000272 gdt_hooks::reset_by_name::<gdt::fd!(vec3f)>,//_ZN4ksys3gdt12TriggerParam10resetVec3fERKN4sead14SafeStringBaseIcEEbb
        0x00de77e4  000284 gdt_hooks::reset_array::<gdt::fd!(bool[])>,//_ZN4ksys3gdt12TriggerParam9resetBoolEiib
        0x00de7900  000284 gdt_hooks::reset_array::<gdt::fd!(s32[])>,//_ZN4ksys3gdt12TriggerParam8resetS32Eiib
        0x00de7a1c  000284 gdt_hooks::reset_array::<gdt::fd!(f32[])>,//_ZN4ksys3gdt12TriggerParam8resetF32Eiib
        0x00de7b38  000284 return_false,//_ZN4ksys3gdt12TriggerParam8resetStrEiib
        0x00de7c54  000284 gdt_hooks::reset_array::<gdt::fd!(str64[])>,//_ZN4ksys3gdt12TriggerParam10resetStr64Eiib
        0x00de7d70  000284 gdt_hooks::reset_array::<gdt::fd!(str256[])>,//_ZN4ksys3gdt12TriggerParam11resetStr256Eiib
        0x00de7e8c  000284 gdt_hooks::reset_array::<gdt::fd!(vec2f[])>,//_ZN4ksys3gdt12TriggerParam10resetVec2fEiib
        0x00de7fa8  000284 gdt_hooks::reset_array::<gdt::fd!(vec3f[])>,//_ZN4ksys3gdt12TriggerParam10resetVec3fEiib
        0x00de80c4  000284 return_false,//_ZN4ksys3gdt12TriggerParam10resetVec4fEiib
            // not doing copyFlags stuff
        0x00deeb8c  002628 gdt_hooks::reset_all,//_ZN4ksys3gdt12TriggerParam28resetAllFlagsToInitialValuesEv

        0x00df08b8  000184 gdt_hooks::idx_from_hash::<gdt::fd!(bool)>, // getBoolIdx by hash
        0x00df0970  000184 gdt_hooks::idx_from_hash::<gdt::fd!(s32)>,  // getS32Idx by hash
        0x00df0a28  000184 gdt_hooks::idx_from_hash::<gdt::fd!(f32)>,//_ZNK4ksys3gdt12TriggerParam9getF32IdxEj
        0x00df0ae0  000184 gdt_hooks::idx_from_hash::<gdt::fd!(str32)>,//_ZNK4ksys3gdt12TriggerParam9getStrIdxEj
        0x00df0b98  000184 gdt_hooks::idx_from_hash::<gdt::fd!(str64)>,//_ZNK4ksys3gdt12TriggerParam11getStr64IdxEj
        0x00df0c50  000184 gdt_hooks::idx_from_hash::<gdt::fd!(str256)>,//_ZNK4ksys3gdt12TriggerParam12getStr256IdxEj
        0x00df0d08  000184 gdt_hooks::idx_from_hash::<gdt::fd!(vec2f)>,//_ZNK4ksys3gdt12TriggerParam11getVec2fIdxEj
        0x00df0dc0  000184 gdt_hooks::idx_from_hash::<gdt::fd!(vec3f)>,//_ZNK4ksys3gdt12TriggerParam11getVec3fIdxEj
        0x00df0e78  000144 gdt_hooks::idx_from_hash::<gdt::fd!(bool[])>,//_ZNK4ksys3gdt12TriggerParam15getBoolArrayIdxEj
        0x00df0f08  000144 gdt_hooks::idx_from_hash::<gdt::fd!(s32[])>,//_ZNK4ksys3gdt12TriggerParam14getS32ArrayIdxEj
        0x00df0f98  000144 gdt_hooks::idx_from_hash::<gdt::fd!(f32[])>,//_ZNK4ksys3gdt12TriggerParam14getF32ArrayIdxEj
        0x00df1028  000144 gdt_hooks::idx_from_hash::<gdt::fd!(str64[])>,//_ZNK4ksys3gdt12TriggerParam16getStr64ArrayIdxEj
        0x00df10b8  000144 gdt_hooks::idx_from_hash::<gdt::fd!(str256[])>,//_ZNK4ksys3gdt12TriggerParam17getStr256ArrayIdxEj
        0x00df1148  000144 gdt_hooks::idx_from_hash::<gdt::fd!(vec2f[])>,//_ZNK4ksys3gdt12TriggerParam16getVec2fArrayIdxEj
        0x00df11d8  000144 gdt_hooks::idx_from_hash::<gdt::fd!(vec3f[])>,//_ZNK4ksys3gdt12TriggerParam16getVec3fArrayIdxEj
        //
        0x00e491d4  000332 return_void, // EventMgr::auto1 (called from doGetItem)

        0x011f3364  000032 return_0, // ksys::util::getDebugHeap

        // --- .plt
        0x018001d0  000016 memcpy,
        0x018001e0  000016 return_true, // __cxa_guard_acquire
        0x01800260  000016 memset,
        0x01800270  000016 return_true, // nn::os::GetSystemTick
        0x01800760  000016 strcmp,
        0x01800a10  000016 return_void, // nn::os::LockMutex
        0x01800a20  000016 return_void, // nn::os::UnlockMutex
        0x01800bf0  000016 vsnprintf,   // nn::util::VSNPrintf

        };

        Ok(None)
    }
}

fn get_player(cpu: &mut Cpu0, _: &mut Process) -> Result<(), processor::Error> {
    let player_ptr: u64 = 0xDEAD_AAAA_0001_CCCC; // some value easy to spot in the debugger
    reg! { cpu: x[0] = player_ptr, return };
}

#[allow(dead_code)]
fn do_request_create_weapon_150(
    cpu: &mut Cpu0,
    proc: &mut Process,
) -> Result<(), processor::Error> {
    do_request_create_weapon(cpu, proc, Environment::new(GameVer::X150, DlcVer::V300))
}

#[allow(dead_code)]
/// uking::act::CreatePlayerEquipActorMgr::doRequestCreateWeapon
/// (this, i32 slot_idx, sead::SafeString* name, int value, WeaponModifierInfo* modifier, _)
fn do_request_create_weapon(
    cpu: &mut Cpu0,
    proc: &mut Process,
    _env: Environment,
) -> Result<(), processor::Error> {
    let main_start = proc.main_start();
    let pmdm_ptr = singleton_instance!(pmdm(proc.memory()))?;

    reg! { cpu:
        x[0] = pmdm_ptr.to_raw(),
        w[1] => let slot_idx: i32,
        w[3] => let value: i32,
        w[1] = value,
        w[2] = match slot_idx {
            0 => 0, // sword
            1 => 3, // shield
            _ => 1, // bow
        }
    };

    // jump to uking::ui::PauseMenuDataMgr::setEquippedWeaponItemValue
    // directly to update the created weapon value in pouch
    cpu.pc = main_start + 0x971438;
    Ok(())
}

/// memcpy(dest, src, size)
fn memcpy(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
    reg! { cpu:
        x[0] => let dest: u64,
        x[1] => let src: u64,
        x[2] => let size: u64,
    };

    // TODO --optimize: Vec isn't needed here

    let mut buf = Vec::with_capacity(size as usize);
    let mut reader = proc.memory().read(src, access!(read))?;
    for _ in 0..size {
        let byte: u8 = reader.read_u8()?;
        buf.push(byte);
    }
    let mut writer = proc.memory_mut().write(dest, access!(write))?;
    for byte in buf {
        writer.write_u8(byte)?;
    }
    reg! { cpu: return }; // returning x0 -- already in there
}

/// memset(start, value, size)
fn memset(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
    reg! { cpu:
        x[0] => let start: u64,
        x[1] => let value: u8,
        x[2] => let size: u64,
    };

    let mut writer = proc.memory_mut().write(start, access!(write))?;
    for _ in 0..size {
        writer.write_u8(value)?;
    }
    reg! { cpu: return }; // returning x0
}

/// strcmp(s1, s2)
fn strcmp(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
    reg! { cpu:
        x[0] => let s1: u64,
        x[1] => let s2: u64,
    };
    let mut reader1 = proc.memory().read(s1, access!(read))?;
    let mut reader2 = proc.memory().read(s2, access!(read))?;

    loop {
        let b1: u8 = reader1.read_u8()?;
        let b2: u8 = reader2.read_u8()?;
        if b1 != b2 || b1 == 0 {
            let result = b1 as i64 - b2 as i64;
            reg! { cpu: x[0] = result, return };
        }
    }
}

/// vsnprintf(char* buffer, size_t size, const char* format, ...)
///
/// Right now, only debug stuff uses it, so we nop
fn vsnprintf(cpu: &mut Cpu0, _: &mut Process) -> Result<(), processor::Error> {
    reg! { cpu: w[0] = -1i32, return }
}

#[allow(dead_code)]
fn return_neg1_32(cpu: &mut Cpu0, _: &mut Process) -> Result<(), processor::Error> {
    reg! { cpu: w[0] = -1i32, return }
}

fn return_void(cpu: &mut Cpu0, _: &mut Process) -> Result<(), processor::Error> {
    reg! { cpu: return }
}

fn return_true(cpu: &mut Cpu0, _: &mut Process) -> Result<(), processor::Error> {
    reg! { cpu: x[0] = true, return }
}

fn return_false(cpu: &mut Cpu0, _: &mut Process) -> Result<(), processor::Error> {
    reg! { cpu: x[0] = false, return }
}

fn return_0(cpu: &mut Cpu0, _: &mut Process) -> Result<(), processor::Error> {
    reg! { cpu: x[0] = 0, return }
}
