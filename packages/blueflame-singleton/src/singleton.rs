use crate::{Bytecode, DataType, Environment, ProxyType, VirtualMachine};


#[derive(Debug, Clone)]
pub struct Singleton {
    /// Start of the singleton relative to root heap
    pub rel_start: u32,

    /// Size of the singleton in bytes
    pub size: u32,

    /// Bytecode for creating the singleton
    pub bytecode: &'static [Bytecode],
}

impl Singleton {
    /// Create the singleton by executing the bytecode on the virtual machine
    pub fn create<V: VirtualMachine>(&self, vm: &mut V) -> Result<(), V::Error> {
        if self.bytecode.is_empty() {
            return Ok(());
        }
        let mut prev_lo_value = 0u32;
        for bytecode in self.bytecode {
            match *bytecode {
                Bytecode::Enter(target) => {
                    vm.enter(target)?
                },
                Bytecode::SetRegHi(reg, value) => {
                    vm.set_reg(reg, (value as u64) << 32 | prev_lo_value as u64)?;
                    prev_lo_value = 0;
                }
                Bytecode::SetRegLo(reg, value) => {
                    vm.set_reg(reg, value as u64)?
                }
                Bytecode::RegLoNextHi(value) => {
                    prev_lo_value = value;
                },
                Bytecode::CopyReg(from, to) => {
                    vm.copy_reg(from, to)?
                },
                Bytecode::ExecuteUntil(target) => {
                    vm.execute_until(target)?
                }
                Bytecode::ExecuteUntilThenSkipOne(target) => {
                    vm.execute_until(target)?;
                    vm.jump(target + 4)?
                }
                Bytecode::ExecuteUntilThenAllocSingletonSkipOne(target) => {
                    vm.execute_until(target)?;
                    vm.allocate_singleton(self.rel_start, self.size)?;
                    vm.jump(target + 4)?
                }
                Bytecode::Jump(target) => {
                    vm.jump(target)?
                },
                Bytecode::JumpExecute(target) => {
                    vm.jump(target)?;
                    vm.execute_until(target + 4)?
                },
                Bytecode::Allocate(bytes) => {
                    vm.allocate_memory(bytes)?
                },
                Bytecode::AllocateProxy(proxy_type) => {
                    vm.allocate_proxy(proxy_type)?
                },
                Bytecode::AllocateData(data_type) => {
                    vm.allocate_data(data_type)?
                }
                Bytecode::AllocateSingleton => {
                    vm.allocate_singleton(self.rel_start, self.size)?
                }
                Bytecode::GetSingleton(reg) => {
                    vm.get_singleton(reg, self.rel_start)?
                }
                Bytecode::ExecuteToComplete => {
                    vm.execute_to_complete()?
                }
            }
        }
        vm.finish()?;

        Ok(())
    }
}

/// uking::ui::PauseMenuDataMgr
pub fn pmdm(env: Environment) -> Singleton {
    let rel_start = 0xaaaaaaa0; // TODO, based on env
    let size = 0x44808; // should be the same for all envs
                        //
    let bytecode = if env.is150() {
        &[
            Bytecode::Enter(0x0096b1cc),
        
            Bytecode::ExecuteUntilThenAllocSingletonSkipOne(0x0096b200),
            // skip the Disposer ctor
            Bytecode::ExecuteUntilThenSkipOne(0x0096b218),
            // --- enter ctor
            // skip CS ctor
            Bytecode::ExecuteUntilThenSkipOne(0x0096b2e8),
            Bytecode::ExecuteToComplete,
            // no init needed
        ]
    } else {
        todo!()
    };

    Singleton {
        rel_start,
        size,
        bytecode,
    }
}

/// ksys::gdt::Manager
pub fn gdt_manager(env: Environment) -> Singleton {
    let rel_start = 0xaaaaaaa0; // TODO, based on env
    let size = 0xdc8;

    let bytecode = if env.is150() {
        &[
            Bytecode::Enter(0x00dce964),

            Bytecode::ExecuteUntilThenAllocSingletonSkipOne(0x00dce994),
            // skip the Disposer ctor
            Bytecode::ExecuteUntilThenSkipOne(0x00dce9ac),

            // --- enter ctor
            // skip some data ctors
            Bytecode::ExecuteUntilThenSkipOne(0x00dcea24),
            Bytecode::ExecuteUntilThenSkipOne(0x00dcea2c),
            Bytecode::ExecuteUntilThenSkipOne(0x00dcea38),
            Bytecode::ExecuteUntilThenSkipOne(0x00dcea40),
            Bytecode::ExecuteUntilThenSkipOne(0x00dcea48),
            Bytecode::ExecuteUntilThenSkipOne(0x00dcea54),
            // method tree node disposer ctor
            Bytecode::ExecuteUntil(0x00b04390),
            Bytecode::Jump(0x00b043b4),
            // skip mutex ctor
            Bytecode::ExecuteUntilThenSkipOne(0x00dcec0c),
            // finish the function
            Bytecode::ExecuteUntil(0x00dcec24),

            // replace return with a B to init
            Bytecode::Jump(0x00dcf1c4),
            Bytecode::GetSingleton(0),
            Bytecode::SetRegLo(1, 0),
            Bytecode::SetRegLo(2, 0),

            // --- init
            // skip 2 GetSystemTick calls
            Bytecode::ExecuteUntil(0x00dcf1f8),
            Bytecode::Jump(0x00dcf200),

            // skip DualHeap creation, set to null
            Bytecode::ExecuteUntilThenSkipOne(0x00dcf23c),
            Bytecode::SetRegLo(0, 0),

            // allocate increase logger
            Bytecode::ExecuteUntilThenSkipOne(0x00dcf254),
            Bytecode::Allocate(0x3098),

            // skip SaveMgr creation
            Bytecode::ExecuteUntilThenSkipOne(0x00dcf268),
            // skip debug and SaveMgr init
            Bytecode::ExecuteUntil(0x00dcf3ec),
            Bytecode::Jump(0x00dcf3fc),
            Bytecode::ExecuteUntilThenSkipOne(0x00dcf40c),
            // skip entry factory bgdata
            Bytecode::ExecuteUntil(0x00dcf428),
            Bytecode::Jump(0x00dcf4e0),
            Bytecode::ExecuteUntilThenSkipOne(0x00dcf4fc),

            // skip save area DualHeap creation, set to null
            Bytecode::ExecuteUntilThenSkipOne(0x00dcf530),
            Bytecode::SetRegLo(0, 0),

            // skip loading save and some other stuff
            Bytecode::ExecuteUntilThenSkipOne(0x00dcf53c),
            Bytecode::ExecuteUntilThenSkipOne(0x00dcf550),

            // skip loading game data arc
            Bytecode::ExecuteUntilThenSkipOne(0x00dcf5cc),
            // skip loading shop data
            Bytecode::ExecuteUntilThenSkipOne(0x00dcf618),
            // skip unloading resources
            Bytecode::ExecuteUntilThenSkipOne(0x00dcf680),

            // create trigger param and store it in param and param1
            Bytecode::AllocateProxy(ProxyType::TriggerParam),
            Bytecode::CopyReg(0, 21),
            Bytecode::GetSingleton(19),
            Bytecode::JumpExecute(0x00dcfe88),
            Bytecode::JumpExecute(0x00dd2ed4),

            // finish init normally
            Bytecode::Jump(0x00dcf684),
            Bytecode::ExecuteToComplete,
        ]
    } else {
        todo!()
    };

    Singleton {
        rel_start,
        size,
        bytecode,
    }
}

/// ksys::act::InfoData
pub fn info_data(env: Environment) -> Singleton {
    let rel_start = 0xaaaaaaa0; // TODO, based on env
    let size = 0x98;

    let bytecode = if env.is150() {
        &[
            Bytecode::Enter(0x00d2e16c),
            Bytecode::ExecuteUntilThenAllocSingletonSkipOne(0x00d2e19c),
            // finish the function
            Bytecode::ExecuteUntil(0x00d2e220),
            // B to init
            Bytecode::Jump(0x00d2e2d8),
            Bytecode::GetSingleton(0),
            // load data into args
            Bytecode::AllocateData(DataType::ActorInfoData),
            Bytecode::CopyReg(0, 1),
            Bytecode::SetRegLo(2, 0),
            Bytecode::SetRegLo(3, 0),
            // root yaml iter
            Bytecode::ExecuteUntilThenSkipOne(0x00d2e314),
            Bytecode::Allocate(0x10),
            // hash iter
            Bytecode::ExecuteUntilThenSkipOne(0x00d2e334),
            Bytecode::Allocate(0x10),
            // actor iter
            Bytecode::ExecuteUntilThenSkipOne(0x00d2e350),
            Bytecode::Allocate(0x10),
            // finish
            Bytecode::ExecuteToComplete,
        ]
    } else {
        todo!()
    };
    
    Singleton {
        rel_start,
        size,
        bytecode,
    }
}
