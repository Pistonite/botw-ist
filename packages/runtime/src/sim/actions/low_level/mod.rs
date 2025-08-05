mod common;
use common::*;

mod add_slot;
pub use add_slot::*;
mod set_gdt;
pub use set_gdt::*;
mod swap_items;
pub use swap_items::*;
mod write_meta;
pub use write_meta::*;

mod break_slot {
    use blueflame::game::singleton_instance;
    use blueflame::memory::mem;
    use blueflame::processor::{self, Cpu2};

    use crate::sim;

    /// Forcefully decrease mCount of list 1 and increase mCount of list 2.
    ///
    /// This is for backward compatibility with slot breaking in older versions
    /// of simulator
    pub fn break_slot(
        ctx: &mut sim::Context<&mut Cpu2>,
        count: i32,
    ) -> Result<(), processor::Error> {
        let pmdm = singleton_instance!(pmdm(ctx.cpu().proc.memory()))?;
        mem! {(ctx.cpu().proc.memory_mut()):
            let count1 = *(&pmdm->mList1.mCount);
            let count2 = *(&pmdm->mList2.mCount);
            *(&pmdm->mList1.mCount) = count1 - count;
            *(&pmdm->mList2.mCount) = count2 + count;
        }
        Ok(())
    }
}
pub use break_slot::*;
