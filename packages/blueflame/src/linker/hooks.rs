use crate::linker::{self as self_, crate_};

use crate_::processor::{self, HookProvider, Execute, Cpu0, Process, reg};
use crate_::processor::insn::paste_insn;
use crate_::memory::{self, Ptr, access, Memory};
use crate_::env::{Environment, GameVer, DlcVer};
use crate_::game::{gdt, singleton, singleton_instance, PauseMenuDataMgr};

use super::gdt_hooks;

macro_rules! fn_table {
    ($main_offset:ident size fn $( $offset:literal $size:literal $function:expr ),* $(,)?) => {
        match $main_offset {
        $(
            $offset => return Ok(Some((processor::box_execute($function), $size))),
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

    let f= access!(force);

    // in uking::ui::PauseMenuDataMgr::createPlayerEquipment and doCreateEquipmentFromItem
    // skip the check for if CreatePlayerEquipActorMgr is null
    memory.write(main_start + 0x971540, f)?.write_u32(paste_insn!(1F 20 03 D5))?;
    memory.write(main_start + 0xaa81ec, f)?.write_u32(paste_insn!(1F 20 03 D5))?;


    Ok(())
}

struct GameHooks;
impl HookProvider for GameHooks {
    fn fetch(&self, main_offset: u32, env: Environment) -> Result<Option<(Box<dyn Execute>, u32)>, processor::Error> {
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
0x0000007100de0d3c,O,000056,_ZNK4ksys3gdt12TriggerParam16getBoolArraySizeEPii
0x0000007100de0d74,O,000056,_ZNK4ksys3gdt12TriggerParam15getS32ArraySizeEPii
0x0000007100de0dac,O,000056,_ZNK4ksys3gdt12TriggerParam15getF32ArraySizeEPii
0x0000007100de0de4,O,000056,_ZNK4ksys3gdt12TriggerParam15getStrArraySizeEPii
0x0000007100de0e1c,O,000056,_ZNK4ksys3gdt12TriggerParam17getStr64ArraySizeEPii
0x0000007100de0e54,O,000056,_ZNK4ksys3gdt12TriggerParam18getStr256ArraySizeEPii
0x0000007100de0e8c,O,000056,_ZNK4ksys3gdt12TriggerParam17getVec2fArraySizeEPii
0x0000007100de0ec4,O,000056,_ZNK4ksys3gdt12TriggerParam17getVec3fArraySizeEPii
0x0000007100de0efc,O,000056,_ZNK4ksys3gdt12TriggerParam17getVec4fArraySizeEPii
0x0000007100de0f34,O,000192,_ZNK4ksys3gdt12TriggerParam22getBoolArraySizeByHashEPij
0x0000007100de0ff4,O,000192,_ZNK4ksys3gdt12TriggerParam21getS32ArraySizeByHashEPij
0x0000007100de10b4,O,000192,_ZNK4ksys3gdt12TriggerParam21getF32ArraySizeByHashEPij
0x0000007100de1174,O,000192,_ZNK4ksys3gdt12TriggerParam21getStrArraySizeByHashEPij
0x0000007100de1234,O,000192,_ZNK4ksys3gdt12TriggerParam23getStr64ArraySizeByHashEPij
0x0000007100de12f4,O,000192,_ZNK4ksys3gdt12TriggerParam24getStr256ArraySizeByHashEPij
0x0000007100de13b4,O,000192,_ZNK4ksys3gdt12TriggerParam23getVec2fArraySizeByHashEPij
0x0000007100de1474,O,000192,_ZNK4ksys3gdt12TriggerParam23getVec3fArraySizeByHashEPij
0x0000007100de1534,O,000192,_ZNK4ksys3gdt12TriggerParam23getVec4fArraySizeByHashEPij
0x0000007100de15f4,O,000248,_ZNK4ksys3gdt12TriggerParam15getS32ArraySizeEPiRKN4sead14SafeStringBaseIcEE
0x0000007100de16ec,O,000248,_ZNK4ksys3gdt12TriggerParam17getStr64ArraySizeEPiRKN4sead14SafeStringBaseIcEE
0x0000007100de17e4,O,000248,_ZNK4ksys3gdt12TriggerParam17getVec3fArraySizeEPiRKN4sead14SafeStringBaseIcEE
0x0000007100de18dc,O,000324,_ZNK4ksys3gdt12TriggerParam17getMinValueForS32EPiRKN4sead14SafeStringBaseIcEE
0x0000007100de1a20,O,000324,_ZNK4ksys3gdt12TriggerParam17getMaxValueForS32EPiRKN4sead14SafeStringBaseIcEE
        0x00de1b64  000236 gdt_hooks::set_bool, // setBool by idx
        0x00de22f8  000332 gdt_hooks::set_s32,  // setS32 by idx
0x0000007100de2908,O,000340,_ZN4ksys3gdt12TriggerParam6setF32Efibb
0x0000007100de2f20,O,000432,_ZN4ksys3gdt12TriggerParam6setStrEPKcibb
0x0000007100de37b0,O,000432,_ZN4ksys3gdt12TriggerParam8setStr64EPKcibb
0x0000007100de4040,O,000440,_ZN4ksys3gdt12TriggerParam9setStr256EPKcibb
0x0000007100de4ea0,O,000180,_ZN4ksys3gdt12TriggerParam8setVec3fERKN4sead7Vector3IfEEibb
        0x00de59e4  000296 gdt_hooks::set_bool_by_name, // setBool by name
        0x00de5b0c  000296 gdt_hooks::set_s32_by_name,  // setS32 by name
0x0000007100de5c34,O,000304,_ZN4ksys3gdt12TriggerParam6setF32EfRKN4sead14SafeStringBaseIcEEbbb
0x0000007100de5d64,O,000296,_ZN4ksys3gdt12TriggerParam6setStrEPKcRKN4sead14SafeStringBaseIcEEbbb
0x0000007100de5e8c,O,000296,_ZN4ksys3gdt12TriggerParam8setStr64EPKcRKN4sead14SafeStringBaseIcEEbbb
0x0000007100de5fb4,m,000444,_ZN4ksys3gdt12TriggerParam8setVec3fERKN4sead7Vector3IfEERKNS2_14SafeStringBaseIcEEbbb
0x0000007100de6170,O,000236,_ZN4ksys3gdt12TriggerParam7setBoolEbiibb
0x0000007100de625c,O,000388,_ZN4ksys3gdt12TriggerParam6setS32Eiiibb
0x0000007100de63e0,O,000396,_ZN4ksys3gdt12TriggerParam6setF32Efiibb
0x0000007100de656c,O,000492,_ZN4ksys3gdt12TriggerParam8setStr64EPKciibb
0x0000007100de6758,O,000492,_ZN4ksys3gdt12TriggerParam9setStr256EPKciibb
0x0000007100de6944,O,000224,_ZN4ksys3gdt12TriggerParam8setVec2fERKN4sead7Vector2IfEEiibb
0x0000007100de6a24,O,000224,_ZN4ksys3gdt12TriggerParam8setVec3fERKN4sead7Vector3IfEEiibb
0x0000007100de6b04,O,000260,_ZN4ksys3gdt12TriggerParam7setBoolEbRKN4sead14SafeStringBaseIcEEibbb
0x0000007100de6c08,O,000260,_ZN4ksys3gdt12TriggerParam6setS32EiRKN4sead14SafeStringBaseIcEEibbb
0x0000007100de6d0c,O,000268,_ZN4ksys3gdt12TriggerParam6setF32EfRKN4sead14SafeStringBaseIcEEibbb
0x0000007100de6e18,O,000260,_ZN4ksys3gdt12TriggerParam8setStr64EPKcRKN4sead14SafeStringBaseIcEEibbb
        0x00de6f1c  000232 gdt_hooks::reset::<gdt::fd!(bool)>, // resetBool
        0x00de7004  000232 gdt_hooks::reset::<gdt::fd!(s32)>,  // resetS32
0x0000007100de70ec,O,000232,_ZN4ksys3gdt12TriggerParam8resetF32Eib
0x0000007100de71d4,O,000232,_ZN4ksys3gdt12TriggerParam10resetStr64Eib
0x0000007100de72bc,O,000232,_ZN4ksys3gdt12TriggerParam10resetVec3fEib
0x0000007100de73a4,O,000272,_ZN4ksys3gdt12TriggerParam9resetBoolERKN4sead14SafeStringBaseIcEEbb
0x0000007100de74b4,O,000272,_ZN4ksys3gdt12TriggerParam8resetS32ERKN4sead14SafeStringBaseIcEEbb
0x0000007100de75c4,O,000272,_ZN4ksys3gdt12TriggerParam10resetStr64ERKN4sead14SafeStringBaseIcEEbb
0x0000007100de76d4,O,000272,_ZN4ksys3gdt12TriggerParam10resetVec3fERKN4sead14SafeStringBaseIcEEbb
0x0000007100de77e4,O,000284,_ZN4ksys3gdt12TriggerParam9resetBoolEiib
0x0000007100de7900,O,000284,_ZN4ksys3gdt12TriggerParam8resetS32Eiib
0x0000007100de7a1c,O,000284,_ZN4ksys3gdt12TriggerParam8resetF32Eiib
0x0000007100de7b38,O,000284,_ZN4ksys3gdt12TriggerParam8resetStrEiib
0x0000007100de7c54,O,000284,_ZN4ksys3gdt12TriggerParam10resetStr64Eiib
0x0000007100de7d70,O,000284,_ZN4ksys3gdt12TriggerParam11resetStr256Eiib
0x0000007100de7e8c,O,000284,_ZN4ksys3gdt12TriggerParam10resetVec2fEiib
0x0000007100de7fa8,O,000284,_ZN4ksys3gdt12TriggerParam10resetVec3fEiib
0x0000007100de80c4,O,000284,_ZN4ksys3gdt12TriggerParam10resetVec4fEiib
            // not doing copyFlags stuff
0x0000007100deeb8c,O,002628,_ZN4ksys3gdt12TriggerParam28resetAllFlagsToInitialValuesEv
        
        0x00df08b8  000184 gdt_hooks::idx_from_hash::<gdt::fd!(bool)>, // getBoolIdx by hash
        0x00df0970  000184 gdt_hooks::idx_from_hash::<gdt::fd!(s32)>,  // getS32Idx by hash
0x0000007100df0a28,O,000184,_ZNK4ksys3gdt12TriggerParam9getF32IdxEj
0x0000007100df0ae0,O,000184,_ZNK4ksys3gdt12TriggerParam9getStrIdxEj
0x0000007100df0b98,O,000184,_ZNK4ksys3gdt12TriggerParam11getStr64IdxEj
0x0000007100df0c50,O,000184,_ZNK4ksys3gdt12TriggerParam12getStr256IdxEj
0x0000007100df0d08,O,000184,_ZNK4ksys3gdt12TriggerParam11getVec2fIdxEj
0x0000007100df0dc0,O,000184,_ZNK4ksys3gdt12TriggerParam11getVec3fIdxEj
0x0000007100df0e78,O,000144,_ZNK4ksys3gdt12TriggerParam15getBoolArrayIdxEj
0x0000007100df0f08,O,000144,_ZNK4ksys3gdt12TriggerParam14getS32ArrayIdxEj
0x0000007100df0f98,O,000144,_ZNK4ksys3gdt12TriggerParam14getF32ArrayIdxEj
0x0000007100df1028,O,000144,_ZNK4ksys3gdt12TriggerParam16getStr64ArrayIdxEj
0x0000007100df10b8,O,000144,_ZNK4ksys3gdt12TriggerParam17getStr256ArrayIdxEj
0x0000007100df1148,O,000144,_ZNK4ksys3gdt12TriggerParam16getVec2fArrayIdxEj
0x0000007100df11d8,O,000144,_ZNK4ksys3gdt12TriggerParam16getVec3fArrayIdxEj
            //
        0x00df0d08  000184 return_neg1_32, // getVec2fIdx -- not used in game
        
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

fn get_player(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
    let player_ptr: u64 = 0xDEAD_AAAA_0001_CCCC; // some value easy to spot in the debugger
    reg! { cpu:
        x[0] = player_ptr,
        return
    };
}

fn do_request_create_weapon_150(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
    do_request_create_weapon(cpu, proc, Environment::new(GameVer::X150, DlcVer::V300))
}

/// uking::act::CreatePlayerEquipActorMgr::doRequestCreateWeapon
/// (this, i32 slot_idx, sead::SafeString* name, int value, WeaponModifierInfo* modifier, _)
fn do_request_create_weapon(cpu: &mut Cpu0, proc: &mut Process, env: Environment) -> Result<(), processor::Error> {
    let main_start = proc.main_start();
    let pmdm_ptr = singleton_instance!(pmdm(proc.memory()))?;

    reg! { cpu:
        x[0] = pmdm_ptr.to_raw(),
        w[1] => slot_idx: i32,
        w[3] => value: i32,
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
        x[0] => dest: u64,
        x[1] => src: u64,
        x[2] => size: u64,
    };

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
        x[0] => start: u64,
        x[1] => value: u8,
        x[2] => size: u64,
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
        x[0] => s1: u64,
        x[1] => s2: u64,
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
