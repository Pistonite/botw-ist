use crate::memory::{self, MemObject, Memory, Ptr, mem};

#[derive(MemObject, Default, Clone, Copy)]
#[size(0x10)]
pub struct ListNode {
    #[offset(0x0)]
    pub mPrev: Ptr![ListNode],
    #[offset(0x8)]
    pub mNext: Ptr![ListNode],
}

impl Ptr![ListNode] {
    /// `sead::ListNode::insertBack_` - add after this node
    pub fn insert_back(
        self,
        node: Ptr![ListNode],
        memory: &mut Memory,
    ) -> Result<(), memory::Error> {
        mem! { memory:
            let next = *(&self->mNext);
            *(&self->mNext) = node;
            *(&node->mPrev) = self;
            *(&node->mNext) = next;
        }
        if !next.is_nullptr() {
            mem! { memory: *(&next->mPrev) = node; }
        }
        Ok(())
    }

    /// `sead::ListNode::insertFront` - add before this node
    pub fn insert_front(
        self,
        node: Ptr![ListNode],
        memory: &mut Memory,
    ) -> Result<(), memory::Error> {
        mem! { memory:
            let prev = *(&self->mPrev);
            *(&self->mPrev) = node;
            *(&node->mPrev) = prev;
            *(&node->mNext) = self;
        }
        if !prev.is_nullptr() {
            mem! { memory: *(&prev->mNext) = node; }
        }
        Ok(())
    }

    /// `sead::ListNode::erase_`: removes the node from the linked list
    pub fn erase(self, memory: &mut Memory) -> Result<(), memory::Error> {
        mem! { memory: let self_ = *self; }
        if !self_.mPrev.is_nullptr() {
            let prev = self_.mPrev;
            // mPrev->mNext = mNext
            mem! { memory: *(&prev->mNext)= self_.mNext ; }
        }
        if !self_.mNext.is_nullptr() {
            let next = self_.mNext;
            // mNext->mPrev = mPrev
            mem! { memory: *(&next->mPrev)= self_.mPrev ; }
        }
        mem! { memory: *self = ListNode::default(); }

        Ok(())
    }
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
    /// ListImpl constructor, and also set the offset
    pub fn construct_with_offset(
        self,
        offset: i32,
        memory: &mut Memory,
    ) -> Result<(), memory::Error> {
        let start_end_ptr = Ptr!(&self->mStartEnd);
        let init = OffsetList {
            mStartEnd: ListNode {
                mPrev: start_end_ptr,
                mNext: start_end_ptr,
            },
            mCount: 0,
            mOffset: offset,
        };
        mem! { memory: *self = init };
        Ok(())
    }
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
        mem! { memory: let offset = *(&self->mOffset); }
        Ok(OffsetListIter {
            offset,
            curr: Ptr!(&self->mStartEnd),
        })
    }

    /// `sead::ListImpl::pushBack` - add after last node
    pub fn push_back(self, node: Ptr![ListNode], memory: &mut Memory) -> Result<(), memory::Error> {
        let start_end = Ptr!(&self->mStartEnd);
        start_end.insert_front(node, memory)?;
        mem! { memory:
            let count = *(&self->mCount);
            *(&self->mCount) = count+1;
        }
        Ok(())
    }

    /// `sead::ListImpl::pushFront` - add before first node
    pub fn push_front(
        self,
        node: Ptr![ListNode],
        memory: &mut Memory,
    ) -> Result<(), memory::Error> {
        let start_end = Ptr!(&self->mStartEnd);
        start_end.insert_back(node, memory)?;
        mem! { memory:
            let count = *(&self->mCount);
            *(&self->mCount) = count+1;
        }
        Ok(())
    }

    /// `sead::ListImpl::popBack` - remove the last node
    pub fn pop_back(self, memory: &mut Memory) -> Result<Ptr![ListNode], memory::Error> {
        mem! { memory: let self_ = *self; }
        if self_.mCount < 1 {
            return Ok(0u64.into());
        }
        let back = self_.mStartEnd.mPrev;
        back.erase(memory)?;
        mem! { memory: *(&self->mCount) = self_.mCount - 1 }

        Ok(back)
    }

    /// `sead::ListImpl::popFront` - remove the first node
    pub fn pop_front(self, memory: &mut Memory) -> Result<Ptr![ListNode], memory::Error> {
        mem! { memory: let self_ = *self; }
        if self_.mCount < 1 {
            return Ok(0u64.into());
        }
        let front = self_.mStartEnd.mNext;
        front.erase(memory)?;
        mem! { memory: *(&self->mCount) = self_.mCount - 1 }

        Ok(front)
    }
}

pub struct OffsetListIter {
    offset: i32,
    pub curr: Ptr![ListNode],
}

impl PartialEq for OffsetListIter {
    fn eq(&self, other: &Self) -> bool {
        self.curr == other.curr
    }
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
