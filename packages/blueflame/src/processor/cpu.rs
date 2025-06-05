use derive_more::derive::{Deref, DerefMut};
use enum_map::EnumMap;

use blueflame_deps::trace_call;

use crate::env::{DataId, GameVer, ProxyId, enabled};
use crate::game::gdt;
use crate::memory::{MemObject, Ptr};
use crate::processor::{
    BLOCK_COUNT_LIMIT, BLOCK_ITERATION_LIMIT, CrashReport, Error, ExecuteCache, Process, Registers,
    STACK_RESERVATION, StackTrace, reg,
};
use crate::program::Program;
use crate::vm::VirtualMachine;

const INTERNAL_RETURN_ADDRESS: u64 = 0xDEAD464C414D45AAu64;
// -----------------------------------------F-L-A-M-E-----
const STACK_CHECK: u64 = 0xDEADBEEFCAFEAAAA;

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
    heap_start_adjusted: u64,
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

    pub stack_trace: StackTrace,
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
        let target_abs = target as u64 + self.proc.main_start();
        self.stack_trace.push_native(target_abs);
        self.pc = target_abs;
        Ok(())
    }

    fn v_reg_set(&mut self, reg: u8, value: u64) -> Result<(), Self::Error> {
        match reg {
            0..31 => self.write(reg!(x[reg]), value),
            31 => {
                log::error!("attempt to write to X31 in VM, which is a reserved register");
            }
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
            }
            _ => self.read(reg!(d[from - 32])),
        };
        self.v_reg_set(to, value)
    }

    fn v_execute_until(&mut self, target: u32) -> Result<(), Self::Error> {
        let target_abs = target as u64 + self.proc.main_start();
        let has_limit = enabled!("limited-block-iteration");

        for count in 0.. {
            if has_limit && count > BLOCK_ITERATION_LIMIT {
                log::error!("Block iteration limit reached in v_execute_until");
                return Err(Error::BlockIterationLimitReached);
            }
            if self.pc == target_abs {
                break;
            }
            self.execute_one_insn()?;
        }
        Ok(())
    }

    fn v_jump(&mut self, target: u32) -> Result<(), Self::Error> {
        log::debug!("v_jump to 0x{target:08x}");
        self.pc = target as u64 + self.proc.main_start();
        Ok(())
    }

    fn v_mem_alloc(&mut self, bytes: u32) -> Result<(), Self::Error> {
        log::debug!("v_mem_alloc {bytes} bytes");
        let ptr = self.proc.memory_mut().alloc(bytes)?;
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
        let Some(data) = self.program.data.iter().find(|d| d.id == id) else {
            return Err(Error::MissingData(id));
        };
        let bytes = &data.bytes;
        log::debug!("allocating data, length={}", bytes.len());
        let ptr = self.proc.memory_mut().alloc(bytes.len() as u32)?;
        let ptr = Ptr!(<u8>(ptr));
        ptr.store_slice(bytes, self.proc.memory_mut())?;
        self.write(reg!(x[0]), ptr.to_raw());
        Ok(())
    }

    fn v_singleton_get(&mut self, reg: u8, rel_start: u32) -> Result<(), Self::Error> {
        let addr = self.heap_start_adjusted + rel_start as u64;
        self.write(reg!(x[reg]), addr);
        Ok(())
    }

    fn v_execute_to_complete(&mut self) -> Result<(), Self::Error> {
        let target_abs = INTERNAL_RETURN_ADDRESS;
        let has_limit = enabled!("limited-block-iteration");

        for count in 0.. {
            if has_limit && count > BLOCK_ITERATION_LIMIT {
                log::error!("Block iteration limit reached in v_execute_to_complete");
                return Err(Error::BlockCountLimitReached);
            }
            if self.pc == target_abs {
                break;
            }
            self.execute_one_insn()?;
        }
        Ok(())
    }
}

impl Cpu3<'_, '_, '_> {
    /// Execute the function, and turn any error that happened into a [`CrashReport`]
    pub fn with_crash_report<T, F: FnOnce(&mut Self) -> Result<T, Error>>(
        &mut self,
        f: F,
    ) -> Result<T, CrashReport> {
        match f(self) {
            Ok(result) => Ok(result),
            Err(e) => Err(self.make_crash_report(e)),
        }
    }
}

impl<'a, 'b> Cpu2<'a, 'b> {
    pub fn new(cpu1: &'a mut Cpu1, proc: &'b mut Process) -> Self {
        Self { cpu1, proc }
    }
}
impl Cpu2<'_, '_> {
    pub fn native_jump_to_main_offset(&mut self, off: u32) -> Result<(), Error> {
        let main_start = self.proc.main_start();
        self.native_jump(main_start + off as u64)
    }
    /// Make a jump to the target address and execute until it returns
    pub fn native_jump(&mut self, pc: u64) -> Result<(), Error> {
        trace_call!(
            "            native jump >>>>> main+0x{:08x}",
            pc - self.proc.main_start()
        );
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
        trace_call!(
            "   native jump finished >>>>> 0x{:016x} (main+0x{:08x})",
            self.pc,
            self.pc - self.proc.main_start()
        );

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
                        if enabled!("strict-replace-hook") {
                            return Err(e);
                        }
                        // probably doesn't need to fetch that much
                        (0x4000, true)
                    }
                }
            }
            Err(bytes) => (bytes, false),
        };
        // not found in cache - load from process memory
        let bytes = fetch_max_bytes.max(4);

        let (exe, bytes) = self.proc.fetch_execute_block(self.pc, Some(bytes))?;

        // if we are in the middle of a replacement hook, then
        // just execute without caching
        if is_hook {
            return exe.execute_from(&mut self.cpu1.cpu0, self.proc, 0);
        }

        let pc = self.pc;

        // add the cache
        self.cpu1.cache[ver].insert(false, self.proc.main_start(), pc, bytes, exe)?;

        let Ok((exe, step)) = self.cpu1.cache[ver].get(pc) else {
            return Err(Error::Unexpected(
                "failed to insert to execute cache".to_string(),
            ));
        };
        debug_assert!(
            step == 0,
            "step should be 0 after inserting to cache with the same PC"
        );
        // execute
        exe.execute_from(&mut self.cpu1.cpu0, self.proc, step)
    }

    /// Execute a single instruction at PC, ignoring caches,
    /// but still using hooks
    pub fn execute_one_insn(&mut self) -> Result<(), Error> {
        // fetch one instruction directly from process and bypass size
        // check for hooks
        let (exe, _) = self.proc.fetch_execute_block(self.pc, None)?;
        exe.execute_from(&mut self.cpu1.cpu0, self.proc, 0)
    }

    pub fn reset_stack(&mut self) {
        self.stack_trace.reset();
        let stack_end = self.proc.memory().stack_end();

        // starting from 0x100 because some function reuses stack frames
        let sp = stack_end - STACK_RESERVATION;
        self.write(reg!(sp), sp);
    }

    /// Execute the function, and turn any error that happened into a [`CrashReport`]
    pub fn with_crash_report<T, F: FnOnce(&mut Self) -> Result<T, Error>>(
        &mut self,
        f: F,
    ) -> Result<T, CrashReport> {
        match f(self) {
            Ok(result) => Ok(result),
            Err(e) => Err(self.make_crash_report(e)),
        }
    }

    /// Create a crash report with the error and CPU state dump
    ///
    /// You can use the [`with_crash_report`](Self::with_crash_report) to define a scope
    /// where processor errors will be turned into a [`CrashReport`]
    pub fn make_crash_report(&self, error: Error) -> CrashReport {
        let main_start = self.proc.main_start();
        let cpu0 = self.cpu1.cpu0.clone();
        CrashReport::new(Box::new(cpu0), main_start, error)
    }

    pub fn stack_alloc<T: MemObject>(&mut self) -> Result<u64, Error> {
        self.stack_alloc_size(T::SIZE)
    }

    /// Reserve `size` bytes on the stack, moving the stack pointer
    /// accordingly.
    ///
    /// When done, the address should be returned with [`stack_check`](Self::stack_check)
    /// to check for stack corruption
    pub fn stack_alloc_size(&mut self, size: u32) -> Result<u64, Error> {
        let sp = self.read::<u64>(reg!(sp));
        // reserve 0x100 from current SP
        let sp = sp - STACK_RESERVATION;
        // write integrity data
        for i in 0..(STACK_RESERVATION / std::mem::size_of::<u64>() as u64) {
            let sp_ptr = Ptr!(<u64>(sp + i * std::mem::size_of::<u64>() as u64));
            sp_ptr.store(&STACK_CHECK, self.proc.memory_mut())?;
        }
        // allocate the data
        let sp = sp - size as u64;
        let addr = sp;
        // reserve another 0x100
        let sp = sp - STACK_RESERVATION;
        self.write(reg!(sp), sp);

        Ok(addr)
    }

    pub fn stack_check<T: MemObject>(&mut self, addr: u64) -> Result<(), Error> {
        self.stack_free_size(addr, T::SIZE)
    }

    pub fn stack_free_size(&mut self, addr: u64, size: u32) -> Result<(), Error> {
        if !enabled!("check-stack-corruption") {
            return Ok(());
        }
        if addr == 0 {
            // nullptr, no check
            return Ok(());
        }
        let start = addr + size as u64;
        // check
        for i in 0..(STACK_RESERVATION / std::mem::size_of::<u64>() as u64) {
            let sp_ptr = Ptr!(<u64>(start + i * std::mem::size_of::<u64>() as u64));
            let x = sp_ptr.load(self.proc.memory())?;
            if x != STACK_CHECK {
                return Err(Error::StackCorruption(addr, size));
            }
        }
        Ok(())
    }
}

impl Cpu0 {
    /// Load the LR, pop and check the stack frame, and set PC to LR
    ///
    /// Note that this is different from the implementation of the `ret`
    /// instruction, because `ret` instruction sets the PC to `LR - 4`,
    /// and then it's increment by the executor
    pub fn return_to_lr(&mut self) -> Result<(), Error> {
        let lr = self.read(reg!(lr));
        self.stack_trace.pop_checked(lr)?;
        self.pc = lr;
        Ok(())
    }
}
