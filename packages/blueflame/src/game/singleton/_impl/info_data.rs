#[layered_crate::import]
use game::{
    super::vm::VirtualMachine,
    super::env::{Environment, DataId},
};

pub static NAME: &str = "ksys::act::InfoData";

pub const fn size(_env: Environment) -> u32 {
    0x98
}

pub const fn rel_start(_env: Environment) -> u32 {
    0x51000 // TODO #104
}

pub const fn main_offset(env: Environment) -> u32 {
    if env.is150() {
        0x2600020
    } else {
        0x0 // TODO --160
    }
}

pub fn create_instance<VM: VirtualMachine>(cpu: &mut VM, env: Environment) -> Result<(), VM::Error> {
    if env.is160() {
        log::error!("info_data create_instance 1.6.0 not implemented yet");
        return Ok(()); // TODO --160
    }

    cpu.v_enter(0x00d2e16c)?;
    cpu.v_execute_until_then_single_alloc_skip_one(0x00d2e19c, rel_start(env), size(env))?;
    // finish the function
    cpu.v_execute_until(0x00d2e220)?;
    // B to init
    cpu.v_jump(0x00d2e2d8)?;
    cpu.v_singleton_get(0, rel_start(env))?;
    cpu.v_reg_copy(0, 3)?;
    // load data into args
    cpu.v_data_alloc(DataId::ActorInfoByml)?;
    cpu.v_reg_copy(0, 1)?;
    cpu.v_reg_copy(3, 0)?;
    cpu.v_reg_set(2, 0)?;
    cpu.v_reg_set(3, 0)?;
    // cpu.v_reg_set(0, 0)?; -- nop
    // root yaml iter
    cpu.v_execute_until_then_skip_one(0x00d2e314)?;
    cpu.v_mem_alloc(0x10)?;
    // hash iter
    cpu.v_execute_until_then_skip_one(0x00d2e334)?;
    cpu.v_mem_alloc(0x10)?;
    // actor iter
    cpu.v_execute_until_then_skip_one(0x00d2e350)?;
    cpu.v_mem_alloc(0x10)?;
    // finish
    cpu.v_execute_to_complete()?;
    Ok(())
}
