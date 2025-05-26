use derive_more::derive::{Deref, DerefMut};
use enum_map::EnumMap;

#[layered_crate::import]
use processor::{
    super::env::{GameVer, enabled, ProxyId, DataId},
    super::vm::VirtualMachine,
    super::memory::{Ptr, PAGE_SIZE},
    super::program::Program,
    super::game::gdt,
    self::{RegName, ExecuteCache, Registers, Process, Error, StackTrace, reg, BLOCK_COUNT_LIMIT},
};

const INTERNAL_RETURN_ADDRESS: u64 = 0xDEAD464C414D45AAu64;
// -----------------------------------------F-L-A-M-E-----

// Note for using Deref/DerefMut for CPU layers:
//
// It's semanticaly not correct to use Deref/DerefMut here, 
// because the structs are not smart pointers. 
// We are using Deref/DerefMut to transparently allowing
// higher layer to access lower layer, without having
// to write long access chains. 
//
// Please do not copy this pattern without first considering the implications.
// See https://doc.rust-lang.org/std/ops/trait.Deref.html#when-to-implement-deref-or-derefmut

/// Level 3 CPU state.
///
/// This layer also has access to the static data from the program image
#[derive(Deref, DerefMut)]
pub struct Cpu3<'a, 'b, 'c> {
    #[deref]
    #[deref_mut]
    pub cpu2: Cpu2<'a, 'b>,
    pub program: &'c Program,
    // adding singleton rel_start to this gets the physical
    // address of the singleton
    heap_start_adjusted: u64
}

/// Level 2 CPU state.
///
/// This is the association of a Cpu1 with the process
/// it is currently running
#[derive(Deref, DerefMut)]
pub struct Cpu2<'a, 'b> {
    #[deref]
    #[deref_mut]
    pub cpu1: &'a mut Cpu1,
    pub proc: &'b mut Process,
}

/// Level 1 CPU state.
///
/// This is responsible for fetching instructions, then using
/// level 0 to execute them
#[derive(Default, Deref, DerefMut)]
pub struct Cpu1 {
    // See NOTE at top about using Deref/DerefMut
    #[deref]
    #[deref_mut]
    pub cpu0: Cpu0,
    pub cache: EnumMap<GameVer, ExecuteCache>,
}

/// The bottom level of CPU state. This is what's needed
/// to execute some instruction (i.e. instructions can read
/// and write to these)
#[derive(Debug, Default, Clone, Deref, DerefMut)]
pub struct Cpu0 {
    // See NOTE at top about using Deref/DerefMut
    #[deref]
    #[deref_mut]
    inner: Registers,

    // stores addresses of all functions visited
    // when function branched to, address is added (first entry is where the call was made from, second is where the branch was to)
    // when function returns, address at the back is removed
    // first is pc of branch insn, second is address branched to
    pub stack_trace: StackTrace
}

impl<'a, 'b, 'c> Cpu3<'a, 'b, 'c> {
    pub fn new(
        cpu1: &'a mut Cpu1, 
        process: &'b mut Process, 
        program: &'c Program,
        heap_start_adjusted: u64,
    ) -> Self {
        let cpu2 = Cpu2 {
            cpu1,
            proc: process,
        };
        Self {
            cpu2,
            program,
            heap_start_adjusted,
        }
    }
}

impl VirtualMachine for Cpu3<'_, '_, '_> {
    type Error = Error;

    fn v_enter(&mut self, target: u32) -> Result<(), Self::Error> {
        self.write(reg!(lr), INTERNAL_RETURN_ADDRESS);
        self.pc = target as u64 + self.proc.main_start();
        Ok(())
    }

    fn v_reg_set(&mut self, reg: u8, value: u64) -> Result<(), Self::Error> {
        match reg {
            0..31 => self.write(reg!(x[reg]), value),
            31 => {
                log::error!("attempt to write to X31 in VM, which is a reserved register");
            },
            _ => self.write(reg!(d[reg - 32]), value),
        }
        Ok(())
    }

    fn v_reg_copy(&mut self, from: u8, to: u8) -> Result<(), Self::Error> {
        let value = match from {
            0..31 => self.read(reg!(x[from])),
            31 => {
                log::error!("attempt to copy from X31 in VM, which is a reserved register");
                return Ok(());
            },
            _ => self.read(reg!(d[from - 32]))
        };
        self.v_reg_set(to, value)
    }

    fn v_execute_until(&mut self, target: u32) -> Result<(), Self::Error> {
        let target_abs = target as u64 + self.proc.main_start();
        while self.pc != target_abs {
            self.execute_one_insn()?;
        }
        Ok(())
    }

    fn v_jump(&mut self, target: u32) -> Result<(), Self::Error> {
        self.pc = target as u64 + self.proc.main_start();
        Ok(())
    }

    fn v_mem_alloc(&mut self, bytes: u32) -> Result<(), Self::Error> {
        let ptr = self.proc.memory_mut().heap_mut().alloc(bytes)?;
        self.write(reg!(x[0]), ptr);
        Ok(())
    }

    fn v_proxy_alloc(&mut self, proxy: ProxyId) -> Result<(), Self::Error> {
        let ptr = match proxy {
            ProxyId::TriggerParam => {
                let mut proxy_list = self.proc.proxies_mut(|p| &mut p.trigger_param);
                proxy_list.alloc(gdt::TriggerParam::loaded())?
            }
        };
        self.write(reg!(x[0]), ptr);
        Ok(())
    }

    fn v_data_alloc(&mut self, id: DataId) -> Result<(), Self::Error> {
        let Some(data) = self.program.data(id) else {
            return Err(Error::Unexpected(format!(
                "data {:?} not found in program",
                id
            )));
        };
        let bytes = data.bytes();
        let ptr = self.proc.memory_mut().heap_mut().alloc(bytes.len() as u32)?;
        let ptr = Ptr!(<u8>(ptr));
        ptr.store_slice(bytes, self.proc.memory_mut())?;
        Ok(())
    }

    fn v_singleton_get(&mut self, reg: u8, rel_start: u32) -> Result<(), Self::Error> {
        let addr = self.heap_start_adjusted + rel_start as u64;
        self.write(reg!(x[reg]), addr);
        Ok(())
    }

    fn v_execute_to_complete(&mut self) -> Result<(), Self::Error> {
        while self.pc != INTERNAL_RETURN_ADDRESS {
            self.execute_once()?;
        }
        Ok(())
    }
}

impl Cpu2<'_, '_> {
    /// Make a jump to the target address and execute until it returns
    pub fn native_jump(&mut self, pc: u64) -> Result<(), Error> {
        let pc_before = self.pc;

        self.write(reg!(lr), INTERNAL_RETURN_ADDRESS);
        self.stack_trace.push_native(pc);
        self.pc = pc;

        let has_limit = enabled!("limited-block-count");

        for count in 0.. {
            if has_limit && count > BLOCK_COUNT_LIMIT {
                return Err(Error::BlockCountLimitReached);
            }
            if self.pc == INTERNAL_RETURN_ADDRESS {
                break;
            }
            self.execute_once()?;
        }

        self.pc = pc_before;

        Ok(())
    }

    /// Run one block of execution
    pub fn execute_once(&mut self) -> Result<(), Error> {
        let ver = self.proc.game_ver();

        let (fetch_max_bytes, is_hook) = match self.cpu1.cache[ver].get(self.pc) {
            Ok((exe, step)) => {
                // found in cache - execute
                match exe.execute_from(&mut self.cpu1.cpu0, self.proc, step) {
                    Ok(_) => return Ok(()),
                    Err(e) => {
                        if !matches!(e, Error::StrictReplacement { .. }) {
                            return Err(e);
                        }
                        // executing the middle of replacement code
                        if enabled!("proc-strict-replace-hook") {
                            return Err(e);
                        }
                        // probably doesn't need to fetch that much
                        (0x4000, true)
                    }
                }
            },
            Err(bytes) => (bytes, false)
        };
        // not found in cache - load from process memory
        let bytes = fetch_max_bytes.max(4);

        let (exe, bytes) = self.proc.fetch_execute_block(self.pc, bytes, true)?;

        // if we are in the middle of a replacement hook, then
        // just execute without caching
        if is_hook {
            return exe.execute_from(&mut self.cpu1.cpu0, self.proc, 0);
        }

        let pc = self.pc;

        // add the cache
        self.cpu1.cache[ver].insert(false, self.proc.main_start(), pc, bytes, exe)?;

        let Ok((exe, step)) = self.cpu1.cache[ver].get(pc) else {
            return Err(Error::Unexpected("failed to insert to execute cache".to_string()))
        };
        debug_assert!(step == 0, "step should be 0 after inserting to cache with the same PC");
        // execute
        exe.execute_from(&mut self.cpu1.cpu0, self.proc, step)
    }

    /// Execute a single instruction at PC, ignoring caches
    /// PC is still updated depending on the instruction
    pub fn execute_one_insn(&mut self) -> Result<(), Error> {
        // fetch one instruction directly from process
        // not allowing hooks
        let (exe, bytes) = self.proc.fetch_execute_block(self.pc, 4, false)?;
        if bytes != 4 {
            return Err(Error::Unexpected(format!(
                "expected 4 bytes for instruction, got {}",
                bytes
            )));
        }
        exe.execute_from(&mut self.cpu1.cpu0, self.proc, 0)
    }

    pub fn reset_stack(&mut self) {
        self.stack_trace.reset();
        // starting from middle of the last page
        // since calling function would store things on the stack before reserving space
        let sp = self.proc.memory().stack_end() - (PAGE_SIZE /2)as u64;
        self.write(reg!(sp), sp);
    }
}

impl Cpu0 {
    /// Simulate the `ret` instruction.
    pub fn ret(&mut self) -> Result<(), Error> {
        self.retr(reg!(lr))
    }

    /// Simulate the `ret` instruction with a register
    pub fn retr(&mut self, reg: RegName) -> Result<(), Error> {
        let xn_val: u64 = self.read(reg);
        let new_pc = xn_val - 4;
        self.stack_trace.pop_checked(xn_val)?;
        self.pc = new_pc as u64;
        Ok(())
    }
}
