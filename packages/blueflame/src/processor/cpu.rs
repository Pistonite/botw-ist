use std::ops::ControlFlow;

use crate::processor::{self as self_, crate_};

use derive_more::derive::{Deref, DerefMut};
use enum_map::EnumMap;

use self_::{ExecuteCache, Registers, Process, Error, Execute, StackTrace, reg, BLOCK_COUNT_LIMIT};
use self_::insn::InsnVec;

use crate_::env::{GameVer, enabled};
use crate_::memory::{access, PAGE_SIZE};

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
#[derive(Deref, DerefMut)]
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

        let mut reader = self.proc.memory().read(self.pc, access!(execute))?;
        let mut insns = InsnVec::new();

        for _ in 0..(bytes/4) {
            let insn_raw = reader.read_u32()?;

            if let ControlFlow::Break(_) = insns.disassemble(insn_raw) {
                break;
            }
        }

        // if we are in the middle of a replacement hook, then
        // just execute without caching
        if is_hook {
            return insns.execute_from(&mut self.cpu1.cpu0, self.proc, 0);
        }

        let pc = self.pc;

        // add the cache
        self.cpu1.cache[ver].insert(false, 
            self.proc.main_start(), 
            pc, insns.byte_size(), Box::new(insns))?;

        let Ok((exe, step)) = self.cpu1.cache[ver].get(pc) else {
            return Err(Error::Unexpected("failed to insert to execute cache".to_string()))
        };
        debug_assert!(step == 0, "step should be 0 after inserting to cache with the same PC");
        // execute
        exe.execute_from(&mut self.cpu1.cpu0, self.proc, step)
    }

    pub fn reset_stack(&mut self) {
        self.stack_trace.reset();
        // starting from middle of the last page
        // since calling function would store things on the stack before reserving space
        let sp = self.proc.memory().stack_end() - (PAGE_SIZE /2)as u64;
        self.write(reg!(sp), sp);
    }
}

// impl Cpu0 {
//     pub fn push_stack_trace(&mut self, target: u64, jump_type: FrameType) {
//         // TODO --cleanup: move to stack
//         self.stack_trace.frames.push(Frame {
//             jump_target: pc,
//             jump_type,
//         });
//     }
// }
