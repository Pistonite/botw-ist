use crate::game::crate_;

use crate_::vm::VirtualMachine;
use crate_::env::Environment;

pub static NAME: &str = "uking::ui::PauseMenuDataMgr";

pub const fn size(_env: Environment) -> u32 {
    0x44808
}

pub const fn rel_start(_env: Environment) -> u32 {
    0x0 // TODO #104
}

pub const fn main_offset(env: Environment) -> u32 {
    if env.is150() {
        0x25d75b8
    } else {
        0x2ca6d50
    }
}

pub fn create_instance<VM: VirtualMachine>(cpu: &mut VM, env: Environment) -> Result<(), VM::Error> {
    if env.is160() {
        log::error!("pmdm create_instance 1.6.0 not implemented yet");
        return Ok(()); // TODO --160
    }

    cpu.v_enter(0x0096aaa0)?;
    cpu.v_execute_to_complete()?;
    cpu.v_enter(0x0096b1cc)?;
    cpu.v_execute_until_then_single_alloc_skip_one(0x0096b200, rel_start(env), size(env))?;
    // skip the Disposer ctor
    cpu.v_execute_until_then_skip_one(0x0096b218)?;
    // --- enter ctor
    // skip CS ctor
    cpu.v_execute_until_then_skip_one(0x0096b2e8)?;
    cpu.v_execute_to_complete()?;
    // no init() needed since it's empty
    Ok(())
}
