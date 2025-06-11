use crate::memory::{self, MemObject, Memory, Ptr, mem};

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
pub struct OffsetList {
    #[offset(0x0)]
    pub mStartEnd: ListNode,
    #[offset(0x10)]
    pub mCount: i32,
    #[offset(0x14)]
    pub mOffset: i32,
}

impl Ptr![OffsetList] {
    /// Create an iterator pointing to the first element
    ///
    /// The implementation mimics `sead::OffsetList::begin`
    pub fn begin(self, memory: &Memory) -> Result<OffsetListIter, memory::Error> {
        mem! { memory:
            let offset = *(&self->mOffset);
            let curr = *(&self->mStartEnd.mNext);
        }
        Ok(OffsetListIter { offset, curr })
    }

    /// Create an iterator pointing to after the last element
    ///
    /// The implementation mimics `sead::OffsetList::end`
    pub fn end(self, memory: &Memory) -> Result<OffsetListIter, memory::Error> {
        mem! { memory:
            let offset = *(&self->mOffset);
        }
        Ok(OffsetListIter {
            offset,
            curr: Ptr!(&self->mStartEnd),
        })
    }
}

#[derive(PartialEq)]
pub struct OffsetListIter {
    offset: i32,
    pub curr: Ptr![ListNode],
}

impl OffsetListIter {
    /// Get the current location of the iterator as a T* (may be null)
    pub fn get_tptr(&self) -> u64 {
        if self.curr.is_nullptr() {
            return self.curr.to_raw();
        }
        self.curr.to_raw() - self.offset as u64
    }
    /// Advance to the next position
    ///
    /// If the iterator is currently null, it will likely raise a memory error
    pub fn next(&mut self, memory: &Memory) -> Result<(), memory::Error> {
        let curr_ptr = self.curr;
        mem! { memory: let next_ptr = *(&curr_ptr->mNext) };
        self.curr = next_ptr;
        Ok(())
    }
}
