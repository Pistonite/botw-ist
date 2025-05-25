use crate::game::{self as self_, crate_};

use crate_::vm::VirtualMachine;
use crate_::env::{Environment, ProxyId};

pub static NAME: &str = "ksys::gdt::Manager";

pub type Type = self_::GdtManager;

pub const fn size(_env: Environment) -> u32 {
    0xdc8
}

pub const fn rel_start(_env: Environment) -> u32 {
    0x50000 // TODO #104
}

pub const fn main_offset(env: Environment) -> u32 {
    if env.is150() {
        0x2601c28
    } else {
        0x0 // TODO --160
    }
}

pub fn create_instance<VM: VirtualMachine>(cpu: &mut VM, env: Environment) -> Result<(), VM::Error> {
    if env.is160() {
        log::error!("gdtm create_instance 1.6.0 not implemented yet");
        return Ok(()); // TODO --160
    }

    cpu.v_enter(0x00dce964)?;
    cpu.v_execute_until_then_single_alloc_skip_one(0x00dce9a0, rel_start(env), size(env))?;
    // skip the Disposer ctor
    cpu.v_execute_until_then_skip_one(0x00dce9ac)?;
    // --- enter ctor
    // skip some data ctors
    cpu.v_execute_until_then_skip_one(0x00dcea24)?;
    cpu.v_execute_until_then_skip_one(0x00dcea2c)?;
    cpu.v_execute_until_then_skip_one(0x00dcea38)?;
    cpu.v_execute_until_then_skip_one(0x00dcea40)?;
    cpu.v_execute_until_then_skip_one(0x00dcea48)?;
    cpu.v_execute_until_then_skip_one(0x00dcea54)?;
    // method tree node disposer ctor
    cpu.v_execute_until(0x00b04390)?;
    cpu.v_jump(0x00b043b4)?;
    // skip mutex ctor
    cpu.v_execute_until_then_skip_one(0x00dcec0c)?;
    // finish the function
    cpu.v_execute_until(0x00dcec24)?;
    // replace return with a B to init
    cpu.v_jump(0x00dcf1c4)?;
    cpu.v_singleton_get(0, rel_start(env))?;
    cpu.v_reg_set(1, 0)?;
    cpu.v_reg_set(2, 0)?;
    // --- init
    // skip 2 GetSystemTick calls
    cpu.v_execute_until(0x00dcf1f8)?;
    cpu.v_jump(0x00dcf200)?;
    // skip DualHeap creation, set to null
    cpu.v_execute_until_then_skip_one(0x00dcf23c)?;
    cpu.v_reg_set(0, 0)?;
    // allocate increase logger
    cpu.v_execute_until(0x00dcf254)?;
    cpu.v_mem_alloc(0x3098)?;
    // skip SaveMgr creation
    cpu.v_execute_until_then_skip_one(0x00dcf268)?;
    // skip debug and SaveMgr init
    cpu.v_execute_until(0x00dcf3ec)?;
    cpu.v_jump(0x00dcf3fc)?;
    cpu.v_execute_until_then_skip_one(0x00dcf40c)?;
    // skip entry factory bgdata
    cpu.v_execute_until(0x00dcf428)?;
    cpu.v_jump(0x00dcf4e0)?;
    cpu.v_execute_until_then_skip_one(0x00dcf4fc)?;
    // skip save area DualHeap creation, set to null
    cpu.v_execute_until_then_skip_one(0x00dcf530)?;
    cpu.v_reg_set(0, 0)?;
    // skip loading save and some other stuff
    cpu.v_execute_until_then_skip_one(0x00dcf53c)?;
    cpu.v_execute_until_then_skip_one(0x00dcf550)?;
    // skip loading game data arc
    cpu.v_execute_until_then_skip_one(0x00dcf5cc)?;
    // skip loading shop data
    cpu.v_execute_until_then_skip_one(0x00dcf618)?;
    // skip tree node stuff
    cpu.v_execute_until_then_skip_one(0x00dcf634)?;
    cpu.v_jump(0x00dcf670)?;
    // skip unloading resources
    cpu.v_execute_until_then_skip_one(0x00dcf680)?;
    // create trigger param and store it in param and param1
    cpu.v_proxy_alloc(ProxyId::TriggerParam)?;
    cpu.v_reg_copy(0, 21)?;
    cpu.v_singleton_get(19, rel_start(env))?;
    cpu.v_jump_execute(0x00dcfe88)?;
    cpu.v_jump_execute(0x00dd2ed4)?;
    // finish init normally
    cpu.v_jump(0x00dcf684)?;
    cpu.v_execute_to_complete()?;
            // TODO --cleanup init common flags
            // Bytecode::Enter(0x008BF8A0),
            // Bytecode::ExecuteToComplete
    Ok(())
}
