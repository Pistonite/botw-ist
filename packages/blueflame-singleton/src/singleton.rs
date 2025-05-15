use crate::{Bytecode, VirtualMachine};

use blueflame_utils::{DataType, Environment, ProxyType};

/// Information about a singleton
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "deku", derive(deku::DekuRead, deku::DekuWrite))]
#[cfg_attr(feature = "deku", deku(id_type = "u8"))]
pub struct Singleton {
    /// Start of the singleton relative to root heap
    pub rel_start: u32,

    /// Size of the singleton in bytes
    pub size: u32,

    /// Offset of the instance static variable compared to the
    /// start of the main module (i.e. `*((main + main_offset) as u64*)` is the
    /// pointer to the singleton instance)
    pub main_offset: u32,

    _bytecode_len: u32,

    /// Bytecode for creating the singleton
    #[cfg_attr(feature = "deku", deku(count = "_bytecode_len"))]
    bytecode: Vec<Bytecode>,
}

impl Singleton {
    pub fn new(rel_start: u32, size: u32, main_offset: u32, bytecode: Vec<Bytecode>) -> Self {
        let _bytecode_len = bytecode.len() as u32;
        Self {
            rel_start,
            size,
            main_offset,
            bytecode,
            _bytecode_len,
        }
    }
    /// Create and initialize the singleton by executing the bytecode on the virtual machine
    pub fn create<V: VirtualMachine>(&self, vm: &mut V) -> Result<(), V::Error> {
        vm.execute_bytecode_program(&self.bytecode, self.rel_start, self.size)
    }
}

/// uking::ui::PauseMenuDataMgr
pub fn pmdm(env: Environment) -> Singleton {
    let rel_start = 0x0; // TODO, based on env
    let size = 0x44808; // should be the same for all envs
    let main_offset = if env.is150() { 0x25d75b8 } else { 0x2ca6d50 };
    //
    let bytecode = if env.is150() {
        vec![
            Bytecode::Enter(0x0096AAA0),
            Bytecode::ExecuteToComplete,
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
        vec![] //TODO
    };

    Singleton::new(rel_start, size, main_offset, bytecode)
}

/// ksys::gdt::Manager
pub fn gdt_manager(env: Environment) -> Singleton {
    let rel_start = 0x100000; // TODO, based on env
    let size = 0xdc8;
    let main_offset = if env.is150() {
        0x2601c28
    } else {
        0x0 // TODO
    };

    let bytecode = if env.is150() {
        vec![
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
            // skip tree node stuff
            Bytecode::ExecuteUntil(0x00dcf634),
            Bytecode::Jump(0x00dcf670),
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
            // init common flags
            // Bytecode::Enter(0x008BF8A0),
            // Bytecode::ExecuteToComplete
        ]
    } else {
        vec![] //TODO
    };

    Singleton::new(rel_start, size, main_offset, bytecode)
}

/// ksys::act::InfoData
pub fn info_data(env: Environment) -> Singleton {
    let rel_start = 0x200000; // TODO, based on env
    let size = 0x98;
    let main_offset = if env.is150() {
        0x2600020
    } else {
        0x0 // TODO
    };

    let bytecode = if env.is150() {
        vec![
            Bytecode::Enter(0x00d2e16c),
            Bytecode::ExecuteUntilThenAllocSingletonSkipOne(0x00d2e19c),
            // finish the function
            Bytecode::ExecuteUntil(0x00d2e220),
            // B to init
            Bytecode::Jump(0x00d2e2d8),
            Bytecode::GetSingleton(0),
            Bytecode::CopyReg(0, 3),
            // load data into args
            Bytecode::AllocateData(DataType::ActorInfoByml),
            Bytecode::CopyReg(0, 1),
            Bytecode::CopyReg(3, 0),
            Bytecode::SetRegLo(2, 0),
            Bytecode::SetRegLo(3, 0),
            //Bytecode::SetRegLo(0, 0)
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
        vec![Bytecode::Enter(0x00d2e16c)] // TODO
    };

    Singleton::new(rel_start, size, main_offset, bytecode)
}

/// uking::aoc::Manager
pub fn aoc_manager(env: Environment) -> Singleton {
    let rel_start = 0x300000; // TODO, based on env
    let size = 0x598;

    let main_offset = if env.is150() {
        0x2600630
    } else {
        0x0 // TODO
    };

    let version = env.dlc_version();

    let bytecode = if env.is150() {
        vec![
            Bytecode::Enter(0x00d69170),
            Bytecode::ExecuteUntilThenAllocSingletonSkipOne(0x00d691a0),
            Bytecode::ExecuteUntilThenSkipOne(0x00d691b0),
            // --- ctor
            Bytecode::ExecuteUntilThenSkipOne(0x00d69240),
            Bytecode::ExecuteUntilThenSkipOne(0x00d69294),
            Bytecode::ExecuteUntilThenSkipOne(0x00d69788),
            Bytecode::ExecuteUntil(0x00d691ec),
            // initial DLC version
            Bytecode::Jump(0x00d6c3f4),
            Bytecode::GetSingleton(19),
            Bytecode::SetRegLo(8, version),
            Bytecode::ExecuteUntil(0x00d6c3f8),
        ]
    } else {
        vec![] // TODO
    };

    Singleton::new(rel_start, size, main_offset, bytecode)
}
