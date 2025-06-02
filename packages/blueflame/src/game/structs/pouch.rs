use crate::game::FixedSafeString40;
use crate::memory::{self, MemObject, Memory, Ptr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, MemObject)]
#[size(0x8)]
pub struct WeaponModifierInfo {
    #[offset(0x0)]
    pub flags: u32,
    #[offset(0x4)]
    pub value: i32,
}

#[derive(MemObject, Clone)]
#[size(0x228)]
pub struct CookItem {
    #[offset(0x0)]
    actor_name: FixedSafeString40,
    #[offset(0x58)]
    ingredients: [FixedSafeString40; 5],
    #[offset(0x210)]
    life_recover: f32,
    #[offset(0x214)]
    effect_time: i32,
    #[offset(0x218)]
    sell_price: i32,
    #[offset(0x21c)]
    effect_id: i32,
    #[offset(0x220)]
    vitality_boost: f32, // i.e effect level
    #[offset(0x224)]
    is_crit: bool,
}

impl Ptr![CookItem] {
    pub fn construct(self, m: &mut Memory) -> Result<(), memory::Error> {
        Ptr!(&self->actor_name).construct(m)?;
        for i in 0..5 {
            self.ith_ingredient(i).construct(m)?;
        }
        Ok(())
    }

    pub fn ith_ingredient(self, i: u64) -> Ptr![FixedSafeString40] {
        // TODO --fix-array-macro
        let ingredients = Ptr!(&self->ingredients)
            .reinterpret_array::<FixedSafeString40, { FixedSafeString40::SIZE }, 5>();
        ingredients.ith(i)
    }
}
