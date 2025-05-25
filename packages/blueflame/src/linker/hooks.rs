use crate::game::{self as self_, crate_};

use crate_::processor::{self, HookProvider, Execute, Cpu0, Process, reg};
use crate_::processor::insn::paste_insn;
use crate_::memory::{self, Ptr, access, Memory};
use crate_::env::{Environment, GameVer, DlcVer};
use self_::{singleton, singleton_instance, PauseMenuDataMgr};

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

        #[rustfmt::skip] let function = { use processor::box_execute as wrap;
        match main_offset {
            // --- .text
            0x006669f8 => (wrap(return_void), 408), // uking::act::CreatePlayerEquipActorMgr::doRequestCreateWeapon
            0x00666cf8 => (wrap(return_void), 688), // uking::act::CreatePlayerEquipActorMgr::doRequestCreateArmor
            0x00849580 => (wrap(return_void), 3456),// Player::equipmentStuff
            0x0085456c => (wrap(get_player), 68),   // ksys::act::PlayerInfo::getPlayer
            0x00d2e950 => (wrap(return_void), 348), // ksys::act::InfoData::logFailure
                                                       // (vec2f is not used in game
            0x00df0d08 => (wrap(return_neg1_32), 184), // ksys::act::TriggerParam::getVec2fIdx 
            0x011f3364 => (wrap(return_0), 32),     // ksys::util::getDebugHeap

            // --- .plt
            0x018001d0 => (wrap(memcpy), 16),
            0x018001e0 => (wrap(return_true), 16), // __cxa_guard_acquire
            0x01800260 => (wrap(memset), 16),
            0x01800270 => (wrap(return_true), 16), // nn::os::GetSystemTick
            0x01800760 => (wrap(strcmp), 16),
            0x01800a10 => (wrap(return_void), 16), // nn::os::LockMutex
            0x01800a20 => (wrap(return_void), 16), // nn::os::UnlockMutex
            0x01800bf0 => (wrap(vsnprintf), 16),   // nn::util::VSNPrintf
            _ => return Ok(None),
        }};

        Ok(Some(function))
    }
}

fn get_player(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
    let player_ptr: u64 = 0xDEAD_AAAA_0001_CCCC;
    cpu.write(reg!(x[0]), player_ptr);
    cpu.ret();
    Ok(())
}

fn do_request_create_weapon_150(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
    do_request_create_weapon(cpu, proc, Environment::new(GameVer::X150, DlcVer::V300))
}

/// uking::act::CreatePlayerEquipActorMgr::doRequestCreateWeapon
/// (this, i32 slot_idx, sead::SafeString* name, int value, WeaponModifierInfo* modifier, _)
fn do_request_create_weapon(cpu: &mut Cpu0, proc: &mut Process, env: Environment) -> Result<(), processor::Error> {
    let main_start = proc.main_start();
    let pmdm_ptr = singleton_instance!(singleton::pmdm(proc, env))?;
    cpu.write(reg!(x[0]), pmdm_ptr.to_raw()); // this
    let slot: i32 = cpu.read(reg!(w[1])); // slot_idx
    let pouch_item_type = match slot {
        0 => 0, // sword
        1 => 3, // shield
        _ => 1, // bow
    };
    let value: i32 = cpu.read(reg!(w[3])); // value
    //
    cpu.write(reg!(w[1]), value);
    cpu.write(reg!(w[2]), pouch_item_type);

    // jump to uking::ui::PauseMenuDataMgr::setEquippedWeaponItemValue
    // directly to update the created weapon value in pouch
    cpu.pc = main_start + 0x971438;
    Ok(())
}

/// memcpy(dest, src, size)
fn memcpy(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
    let dest: u64 = cpu.read(reg!(x[0]));
    let src: u64 = cpu.read(reg!(x[1]));
    let size: u64 = cpu.read(reg!(x[2]));

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
    // cpu.write(reg!(x[0]), dest); -- already in there
    cpu.ret();
    Ok(())
}


/// memset(start, value, size)
fn memset(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
    let start: u64 = cpu.read(reg!(x[0]));
    let value: u8 = cpu.read(reg!(x[1]));
    let size: u64 = cpu.read(reg!(x[2]));

    let mut writer = proc.memory_mut().write(start, access!(write))?;
    for _ in 0..size {
        writer.write_u8(value)?;
    }
    cpu.ret();
    Ok(())
}

/// strcmp(s1, s2)
fn strcmp(cpu: &mut Cpu0, proc: &mut Process) -> Result<(), processor::Error> {
    let s1: u64 = cpu.read(reg!(x[0]));
    let s2: u64 = cpu.read(reg!(x[1]));
    let mut reader1 = proc.memory().read(s1, access!(read))?;
    let mut reader2 = proc.memory().read(s2, access!(read))?;

    loop {
        let b1: u8 = reader1.read_u8()?;
        let b2: u8 = reader2.read_u8()?;
        if b1 != b2 || b1 == 0 {
            let result = b1 as i64 - b2 as i64;
            cpu.write(reg!(x[0]), result);
            cpu.ret();
            return Ok(());
        }
    }
}

/// vsnprintf(char* buffer, size_t size, const char* format, ...)
///
/// Right now, only debug stuff uses it, so we nop
fn vsnprintf(cpu: &mut Cpu0, _: &mut Process) -> Result<(), processor::Error> {
    cpu.write(reg!(w[0]), -1i32); // fail
    cpu.ret();
    Ok(())
}

fn return_neg1_32(cpu: &mut Cpu0, _: &mut Process) -> Result<(), processor::Error> {
    cpu.write(reg!(w[0]), -1i32);
    cpu.ret();
    Ok(())
}

fn return_void(cpu: &mut Cpu0, _: &mut Process) -> Result<(), processor::Error> {
    cpu.ret();
    Ok(())
}

fn return_true(cpu: &mut Cpu0, _: &mut Process) -> Result<(), processor::Error> {
    cpu.write(reg!(x[0]), true);
    cpu.ret();
    Ok(())
}

fn return_0(cpu: &mut Cpu0, _: &mut Process) -> Result<(), processor::Error> {
    cpu.write(reg!(x[0]), 0);
    cpu.ret();
    Ok(())
}
