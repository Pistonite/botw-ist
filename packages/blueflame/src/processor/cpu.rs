use crate::processor::{self as self_, crate_};

use enum_map::EnumMap;

use self_::{ExecuteCache, Registers, Process, Error, Execute, StackTrace, FrameType, Frame};
use self_::insn::InsnVec;

use crate_::env::{GameVer, enabled};
use crate_::memory::{access, PAGE_SIZE};

/// Level 2 CPU state.
///
/// This is the association of a Cpu1 with the process
/// it is currently running
pub struct Cpu2<'a, 'b> {
    pub cpu: &'a mut Cpu1,
    pub proc: &'b mut Process,
}

/// Level 1 CPU state.
///
/// This is responsible for fetching instructions, then using
/// level 0 to execute them
pub struct Cpu1 {
    pub state: Cpu0,
    pub cache: EnumMap<GameVer, ExecuteCache>,
}

const INTERNAL_RETURN_ADDRESS: u64 = 0xDEAD464C414D45AAu64;
// -----------------------------------------F-L-A-M-E-----


/// The bottom level of CPU state. This is what's needed
/// to execute some instruction (i.e. instructions can read
/// and write to these)
pub struct Cpu0 {
    pub reg: Registers,

    // stores addresses of all functions visited
    // when function branched to, address is added (first entry is where the call was made from, second is where the branch was to)
    // when function returns, address at the back is removed
    // first is pc of branch insn, second is address branched to
    pub stack_trace: StackTrace
}

impl Cpu2<'_, '_> {
    /// Make a jump to the target address and execute until it returns
    pub fn native_jump(&mut self, pc: u64) -> Result<(), Error> {
        const BLOCK_COUNT_LIMIT: usize = 0x1000;
        self.cpu.state.reg.set_lr(INTERNAL_RETURN_ADDRESS);
        self.cpu.state.stack_trace.frames.push(Frame {
            jump_target: pc,
            jump_type: FrameType::Native,
        });
        self.cpu.state.reg.pc = pc;

        let has_limit = enabled!("limited-block-count");

        for count in 0.. {
            if has_limit && count > BLOCK_COUNT_LIMIT {
                return Err(Error::BlockCountLimitReached);
            }
            if self.cpu.state.reg.pc == INTERNAL_RETURN_ADDRESS {
                break;
            }
            self.execute_once()?;
        }

        let x = self.cpu.state.stack_trace.frames.pop();
        if x.is_none() && enabled!("check-stack-frames") {
            return Err(Error::StackFrameCorrupted);
        }
        Ok(())
    }

    /// Run one block of execution
    pub fn execute_once(&mut self) -> Result<(), Error> {
        let ver = self.proc.game_ver();

        let (fetch_max_bytes, is_hook) = match self.cpu.cache[ver].get(self.cpu.state.reg.pc) {
            Ok((exe, step)) => {
                // found in cache - execute
                match exe.execute_from(&mut self.cpu.state, &mut self.proc, step) {
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

        let mut reader = self.proc.memory().read(self.cpu.state.reg.pc, access!(execute))?;
        let mut insns = InsnVec::new();

        let pc_before = self.cpu.state.reg.pc;

        for _ in 0..(bytes/4) {
            let insn_raw = reader.read_u32()?;
            // actually increase the PC so we have correct report
            // if failed to read memory
            self.cpu.state.reg.pc = self.cpu.state.reg.pc.wrapping_add(4);
            if !insns.disassemble(insn_raw) {
                // done with this block
                break;
            }
        }

        // recover the PC
        self.cpu.state.reg.pc = pc_before;

        // if we are in the middle of a replacement hook, then
        // just execute without caching
        if is_hook {
            return insns.execute_from(&mut self.cpu.state, &mut self.proc, 0);
        }

        // add the cache
        self.cpu.cache[ver].insert(false, 
            self.proc.main_start(), 
            pc_before, insns.byte_size(), Box::new(insns))?;

        let Ok((exe, step)) = self.cpu.cache[ver].get(pc_before) else {
            return Err(Error::Unexpected("failed to insert to execute cache".to_string()))
        };
        // execute
        exe.execute_from(&mut self.cpu.state, &mut self.proc, step)
    }

    pub fn reset_stack(&mut self) {
        self.cpu.state.stack_trace.frames.clear();
        // starting from middle of the last page
        // since calling function would store things on the stack before reserving space
        self.cpu.state.reg.sp_el0 = self.proc.memory().stack_end() - (PAGE_SIZE / 2) as u64;
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
