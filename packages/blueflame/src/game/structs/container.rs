use crate::memory::{MemObject, Ptr};

#[derive(MemObject, Default, Clone, Copy)]
#[size(0x10)]
pub struct ListNode {
    #[offset(0x0)]
    pub mPrev: Ptr![ListNode],
    #[offset(0x8)]
    pub mNext: Ptr![ListNode],
}

#[derive(MemObject, Default, Clone)]
#[size(0x18)]
pub struct OffsetList{
    #[offset(0x0)]
    pub mStartEnd: ListNode,
    #[offset(0x10)]
    pub mCount: i32,
    #[offset(0x14)]
    pub mOffset: i32,
}
