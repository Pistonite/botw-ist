
pub struct StackTrace {
    pub frames: Vec<Frame>,
}

pub struct Frame {
    pub jump_target: u64,
    pub jump_type: FrameType,
}

pub enum FrameType {
    /// Branch with BL instruction
    Bl(u64),
    /// Branch with BLR instruction (TODO --cleanup: second is the reg)
    Blr(u64, ()),
    /// Called from native implementation
    Native,
}
