#[layered_crate::import]
use game::{
    super::vm::VirtualMachine,
    super::env::Environment,
};

pub static NAME: &str = "uking::aoc::Manager";

pub const fn size(_env: Environment) -> u32 {
    0x598
}

pub const fn rel_start(_env: Environment) -> u32 {
    0x52000 // TODO #104
}

pub const fn main_offset(env: Environment) -> u32 {
    if env.is150() {
        0x2600630
    } else {
        0x0 // TODO --160
    }
}

pub fn create_instance<VM: VirtualMachine>(cpu: &mut VM, env: Environment) -> Result<(), VM::Error> {
    if env.is160() {
        log::error!("info_data create_instance 1.6.0 not implemented yet");
        return Ok(()); // TODO --160
    }

    let version = env.dlc_version();

    cpu.v_enter(0x00d69170)?;
    cpu.v_execute_until_then_single_alloc_skip_one(0x00d691a0, rel_start(env), size(env))?;
    cpu.v_execute_until_then_skip_one(0x00d691b0)?;
    // --- ctor
    cpu.v_execute_until_then_skip_one(0x00d69240)?;
    cpu.v_execute_until_then_skip_one(0x00d69294)?;
    cpu.v_execute_until_then_skip_one(0x00d69788)?;
    cpu.v_execute_until(0x00d691ec)?;
    // jump to initialize DLC version
    cpu.v_jump(0x00d6c3f4)?;
    cpu.v_singleton_get(19, rel_start(env))?;
    cpu.v_reg_set(8, version as u64)?;
    cpu.v_execute_until(0x00d6c3f8)?;
    // jump back
    cpu.v_jump_execute(0x00d691ec)?;
    Ok(())
}
