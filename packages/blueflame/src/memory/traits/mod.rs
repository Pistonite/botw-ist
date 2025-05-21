mod traits;
pub use traits::*;

/// Implementation of traits for primitive types
mod _impl;

#[cfg(test)]
mod tests {
    use crate::memory::{self as self_};

    use std::sync::Arc;

    use self_::{Ptr, Memory, Region, RegionType, SimpleHeap, MemObject};

    #[derive(MemObject)]
    #[size(0x40)]
    struct TestSub {
        #[offset(0x10)]
        one: u32,
        #[offset(0x30)]
        two: u64,
    }

    #[derive(MemObject)]
    #[size(0x40)]
    struct TestSubTwo {
        #[offset(0x0)]
        one: u64,
        #[offset(0x15)]
        two: u64,
    }

    #[derive(MemObject)]
    #[size(0xC0)]
    struct Test {
        #[offset(0x40)]
        t: TestSub,
        #[offset(0x80)]
        t2: TestSubTwo,
    }

    #[test]
    pub fn test_memread() -> anyhow::Result<()> {
        let p = Arc::new(Region::new_rw(RegionType::Program, 0, 0));
        let s = Arc::new(Region::new_rw(RegionType::Stack, 0x100, 0x1000));
        let h = Arc::new(SimpleHeap::new(0x2000, 0, 0));
        let mut mem = Memory::new(p, s, h, None, 0, None);
        let ts = TestSub {
            one: 0x15,
            two: 0x20,
        };
        let ts2 = TestSubTwo { one: 0x4, two: 0x5 };
        let value_to_store = Test { t: ts, t2: ts2 };
        let p: Ptr<Test> = Ptr::new(0x500);
        p.store(&mut mem, &value_to_store)?;
        let loaded_value = p.load(&mem)?;
        assert_eq!(loaded_value.t.one, 0x15);
        assert_eq!(loaded_value.t.two, 0x20);
        assert_eq!(loaded_value.t2.one, 0x4);
        assert_eq!(loaded_value.t2.two, 0x5);
    
        let array_to_store = [0x10u32; 20];
        let array_p: Ptr<[u32; 20]> = Ptr::new(0x750);
        array_p.store(&mut mem, &array_to_store)?;
    
        let loaded_array = array_p.load(&mut mem)?;
        assert_eq!(loaded_array.len(), 20);
        assert_eq!(loaded_array[10], 0x10u32);
    
        Ok(())
    }
}
