use crate::processor::{self as self_, crate_};

use crate_::env::enabled;
use self_::Error;


#[derive(Debug, Clone, Default)]
pub struct StackTrace {
    pub frames: Vec<Frame>,
}

impl StackTrace {
    pub fn reset(&mut self) {
        self.frames.clear();
    }

    /// Push a jump to target address from native implementation
    pub fn push_native(&mut self, target: u64) {
        self.frames.push(Frame {
            jump_target: target,
            jump_type: FrameType::Native,
        });
    }

    /// Pop the stack frame
    pub fn pop_checked(&mut self, lr: u64) -> Result<(), Error> {
        match self.frames.pop() {
            Some(frame) => {
                if enabled!("check-return-address") {
                    let source = match frame.jump_type {
                        FrameType::Bl(source) => source,
                        FrameType::Blr(source, _) => source,
                        _ => return Ok(())
                    };
                    if source + 4 != lr {
                        return Err(Error::ReturnAddressMismatch(lr, source + 4));
                    }
                }
            }
            None => {
                log::error!("stack frames popped while empty");
                if enabled!("check-stack-frames") {
                    return Err(Error::StackFrameCorrupted);
                }
            }
        }
        Ok(())
    }

}

#[derive(Debug, Clone)]
pub struct Frame {
    pub jump_target: u64,
    pub jump_type: FrameType,
}

#[derive(Debug, Clone)]
pub enum FrameType {
    /// Branch with BL instruction
    Bl(u64),
    /// Branch with BLR instruction (TODO --cleanup: second is the reg)
    Blr(u64, ()),
    /// Called from native implementation
    Native,
}
