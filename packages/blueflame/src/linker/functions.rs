use crate::game::{CookItem, FixedSafeString40, WeaponModifierInfo, singleton_instance};
use crate::memory::{mem, Ptr};
use crate::processor::{self, Cpu2, reg};

/// Get one item with the default life. Calls `doGetItem_0x710073A464`
pub fn get_item(
    cpu: &mut Cpu2,
    actor: &str,
    modifier: Option<WeaponModifierInfo>,
) -> Result<(), processor::Error> {
    cpu.reset_stack();

    let actor_name_ptr = helper::stack_alloc_string40(cpu, actor)?;
    let modifier_ptr = helper::stack_alloc_weapon_modifier(cpu, modifier)?;

    reg! { cpu:
        x[0] = actor_name_ptr,
        x[1] = modifier_ptr,
    };

    if cpu.proc.is160() {
        panic!("1.6.0 not implemented yet");
        // cpu.native_jump_to_main_offset(0x0096f3d0)?;
    } else {
        cpu.native_jump_to_main_offset(0x0071a464)?;
    }

    cpu.stack_check::<FixedSafeString40>(actor_name_ptr.to_raw())?;
    cpu.stack_check::<WeaponModifierInfo>(modifier_ptr.to_raw())?;
    Ok(())
}

/// Get a cook item with the cook data. Calls `uking::ui::PauseMenuDataMgr::cookItemGet`
pub fn get_cook_item(
    cpu: &mut Cpu2,
    actor: &str,
    ingredients: &[impl AsRef<str>],
    life_recover: Option<f32>,
    effect_time: Option<i32>,
    sell_price: Option<i32>,
    effect_id: Option<i32>,
    vitality_boost: Option<f32>, // i.e effect level
    is_crit: bool,
) -> Result<(), processor::Error> {
    cpu.reset_stack();

    let this_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;

    let cook_item = cpu.stack_alloc::<CookItem>()?;
    let cook_item = Ptr!(<CookItem>(cook_item));
    let m = cpu.proc.memory_mut();

    cook_item.construct(m)?;
    mem! { m: safe_store(&cook_item->actor_name) = *actor; };
    for (i, ingredient) in ingredients.iter().take(5).enumerate() {
        let p = cook_item.ith_ingredient(i as u64);
        mem! { m: safe_store(p) = *ingredient; };
    }
    mem! { m:
        *(&cook_item->life_recover) = life_recover.unwrap_or(0.0);
        *(&cook_item->effect_time) = effect_time.unwrap_or(0);
        *(&cook_item->sell_price) = sell_price.unwrap_or(0);
        *(&cook_item->effect_id) = effect_id.unwrap_or(-1);
        *(&cook_item->vitality_boost) = vitality_boost.unwrap_or(0.0);
        *(&cook_item->is_crit) = is_crit;
    };
    reg! { cpu:
        x[0] = this_ptr,
        x[1] = cook_item,
    };

    if cpu.proc.is160() {
        cpu.native_jump_to_main_offset(0x010be740)?;
    } else {
        cpu.native_jump_to_main_offset(0x00970158)?;
    }

    cpu.stack_check::<CookItem>(cook_item.to_raw())?;
    Ok(())
}

pub fn call_pmdm_item_get(cpu: &mut Cpu2, actor: &str, value: i32) -> Result<(), processor::Error> {
    call_pmdm_item_get_with_modifier(cpu, actor, value, 0, 0)
}

pub fn call_pmdm_item_get_with_modifier(
    cpu: &mut Cpu2,
    actor: &str,
    value: i32,
    modifier_flags: u32,
    modifier_value: i32,
) -> Result<(), processor::Error> {
    cpu.reset_stack();

    // x0 - this
    let this_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    // x1 - actor name ptr
    let name_ptr = cpu.stack_alloc::<FixedSafeString40>()?;
    let name_ptr = Ptr!(<FixedSafeString40>(name_ptr));
    name_ptr.construct(cpu.proc.memory_mut())?;
    name_ptr.safe_store(actor, cpu.proc.memory_mut())?;
    // w2 - value (scalar)

    // x3 - modifier info ptr
    let modifier_info_ptr = if modifier_flags != 0 {
        let ptr = cpu.stack_alloc::<WeaponModifierInfo>()?;
        let ptr = Ptr!(<WeaponModifierInfo>(ptr));
        let info = WeaponModifierInfo {
            flags: modifier_flags,
            value: modifier_value,
        };
        ptr.store(&info, cpu.proc.memory_mut())?;
        ptr.to_raw()
    } else {
        0
    };

    reg! { cpu:
        x[0] = this_ptr,
        x[1] = name_ptr,
        w[2] = value,
        x[3] = modifier_info_ptr,
    };

    cpu.native_jump_to_main_offset(0x0096efb8)?;
    cpu.stack_check::<FixedSafeString40>(name_ptr.to_raw())?;
    cpu.stack_check::<WeaponModifierInfo>(modifier_info_ptr)?;
    Ok(())
}

pub fn call_load_from_game_data(cpu: &mut Cpu2) -> Result<(), processor::Error> {
    cpu.reset_stack();
    let this_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    reg! { cpu:
        x[0] = this_ptr,
    };
    // TODO --160
    cpu.native_jump_to_main_offset(0x0096be24)
}

pub fn is_weapon_profile(cpu: &mut Cpu2, actor: &str) -> Result<bool, processor::Error> {
    let profile = get_actor_profile(cpu, actor)?;
    Ok(profile.starts_with("Weapon"))
}

/// Call ksys::act::InfoData::getActorProfile
pub fn get_actor_profile(cpu: &mut Cpu2, actor: &str) -> Result<String, processor::Error> {
    cpu.reset_stack();
    let this_ptr = singleton_instance!(info_data(cpu.proc.memory()))?;

    // x1 - char** profile
    let out_profile = Ptr!(<Ptr![u8]>(cpu.stack_alloc::<Ptr![u8]>()?));

    // x2 - char* actor name ptr
    // FixedSafeString40*
    let name_ptr = cpu.stack_alloc::<FixedSafeString40>()?;
    let name_ptr = Ptr!(<FixedSafeString40>(name_ptr));
    name_ptr.construct(cpu.proc.memory_mut())?;
    name_ptr.safe_store(actor, cpu.proc.memory_mut())?;
    // char*
    let name_ptr_cstr = name_ptr.cstr(cpu.proc.memory())?;

    reg! { cpu:
        x[0] = this_ptr,
        x[1] = out_profile,
        x[2] = name_ptr_cstr,
    };

    if cpu.proc.is160() {
        cpu.native_jump_to_main_offset(0x01542270)?;
    } else {
        cpu.native_jump_to_main_offset(0x00d301fc)?;
    }
    cpu.stack_check::<FixedSafeString40>(name_ptr.to_raw())?;
    cpu.stack_check::<Ptr![u8]>(out_profile.to_raw())?;

    let profile = out_profile.load(cpu.proc.memory())?;
    let profile = profile.load_utf8_lossy(cpu.proc.memory())?;

    Ok(profile)
}

mod helper {
    use super::*;

    /// Allocate a FixedSafeString40 on the stack and store the value in it
    pub fn stack_alloc_string40(
        cpu: &mut Cpu2,
        value: &str,
    ) -> Result<Ptr![FixedSafeString40], processor::Error> {
        let ptr = cpu.stack_alloc::<FixedSafeString40>()?;
        let ptr = Ptr!(<FixedSafeString40>(ptr));
        ptr.construct(cpu.proc.memory_mut())?;
        ptr.safe_store(value, cpu.proc.memory_mut())?;
        Ok(ptr)
    }

    /// Allocate a WeaponModifierInfo on the stack and store the value in it
    pub fn stack_alloc_weapon_modifier(
        cpu: &mut Cpu2,
        value: Option<WeaponModifierInfo>,
    ) -> Result<Ptr![WeaponModifierInfo], processor::Error> {
        if let Some(modifier) = value {
            let ptr = cpu.stack_alloc::<WeaponModifierInfo>()?;
            mem! { (cpu.proc.memory_mut()):
                *(<WeaponModifierInfo>(ptr)) = modifier;
            };
            Ok(ptr.into())
        } else {
            Ok(Ptr!(<WeaponModifierInfo>(0)))
        }
    }
}

// impl Cpu2<'_, '_> {

// pub fn has_tag(&mut self, actor: &str, tag: u32) -> Result<bool, ExecutionError> {
//     self.cpu.write_arg(0, 61472768);
//     let addr: Ptr<FixedSafeString40> = Ptr::new(self.alloc_fixed_safe_string40(actor)?);
//     let m_top = addr
//         .deref(self.mem)
//         .map_err(|e| self.to_execution_error(e))?;
//     self.cpu.write_arg(1, m_top.safeString.mStringTop);
//     self.cpu.write_arg(2, tag as u64);
//     self.call_func_at_addr(0xD2F900)?;
//     Ok(self.cpu.read_arg(0) != 0)
// }

// // 0x970060
// #[allow(clippy::too_many_arguments)]
// pub fn cook_item_get(
//     &mut self,
//     name: &str,
//     life_recover: f32,
//     effect_time: i32,
//     sell_price: i32,
//     effect_id: CookEffectId,
//     vitality_boost: f32,
//     is_crit: bool,
// ) -> Result<(), ExecutionError> {
//     let cook_item_addr = self
//         .alloc_cook_item(
//             name,
//             life_recover,
//             effect_time,
//             sell_price,
//             effect_id,
//             vitality_boost,
//             is_crit,
//         )
//         .map_err(|e| self.to_execution_error(e))?;
//     self.cpu.write_arg(0, self.mem.get_pmdm_addr());
//     self.cpu.write_arg(1, cook_item_addr);
//     self.call_func_at_addr(0x970060)
// }

// // 0x9704BC
// pub fn remove_item(&mut self, actor: &str) -> Result<(), ExecutionError> {
//     let actor_name_addr = self.alloc_fixed_safe_string40(actor)?;
//     self.cpu.write_arg(0, self.mem.get_pmdm_addr());
//     self.cpu.write_arg(1, actor_name_addr);
//     self.call_func_at_addr(0x9704BC)
// }
//
// // 0x97A944
// pub fn equip_weapon(&mut self, item_addr: u64) -> Result<(), ExecutionError> {
//     self.cpu.write_arg(0, self.mem.get_pmdm_addr());
//     self.cpu.write_arg(1, item_addr);
//     self.call_func_at_addr(0x97A944)
// }

// // 0x97A9FC
// pub fn unequip(&mut self, item_addr: u64) -> Result<(), ExecutionError> {
//     self.cpu.write_arg(0, self.mem.get_pmdm_addr());
//     self.cpu.write_arg(1, item_addr);
//     self.call_func_at_addr(0x97A9FC)
// }

// pub fn info_data_get_type(&mut self, name: &str) -> Result<PouchItemType, ExecutionError> {
//     let actor_name_addr = self.alloc_fixed_safe_string40(name)?;
//     self.cpu.write_arg(0, actor_name_addr);
//     self.cpu.write_arg(1, 0);
//     self.call_func_at_addr(0x96DC34)?;
//     Ok(PouchItemType::from(self.cpu.read_arg(0) as i32))
// }
//
// pub fn create_player_equipment(&mut self) -> Result<(), ExecutionError> {
//     self.cpu.write_arg(0, self.mem.get_pmdm_addr());
//     self.call_func_at_addr(0x971504)
// }
//
//
// #[allow(clippy::too_many_arguments)]
// fn alloc_cook_item(
//     &mut self,
//     name: &str,
//     life_recover: f32,
//     effect_time: i32,
//     sell_price: i32,
//     effect_id: CookEffectId,
//     vitality_boost: f32,
//     is_crit: bool,
// ) -> Result<u64, error::Error> {
//     self.mem.heap_mut().alloc(0x64)?;
//     let base_address = self.mem.heap_mut().alloc(0x228)?;
//
//     let mut buffer = [0u8; 64];
//
//     let bytes = name.as_bytes();
//     let len = bytes.len().min(buffer.len());
//
//     buffer[..len].copy_from_slice(&bytes[..len]);
//     // actor_name is offset 0 from base
//     let actor_name = FixedSafeString40 {
//         safeString: SafeString {
//             vtable: Self::FIXED_SAFE_STRING40_VTABLE_ADDR + (self.mem.get_main_offset() as u64),
//             mStringTop: base_address + 0x14,
//         },
//         mBufferSize: 64,
//         mBuffer: buffer,
//     };
//
//     let buffer = [b'A'; 64];
//
//     let ingredient1 = FixedSafeString40 {
//         safeString: SafeString {
//             vtable: Self::FIXED_SAFE_STRING40_VTABLE_ADDR + (self.mem.get_main_offset() as u64),
//             mStringTop: base_address + 0x14 + 0x58,
//         },
//         mBufferSize: 64,
//         mBuffer: buffer,
//     };
//
//     let buffer = [b'B'; 64];
//     let ingredient2 = FixedSafeString40 {
//         safeString: SafeString {
//             vtable: Self::FIXED_SAFE_STRING40_VTABLE_ADDR + (self.mem.get_main_offset() as u64),
//             mStringTop: base_address + 0x14 + 0x58 * 2,
//         },
//         mBufferSize: 64,
//         mBuffer: buffer,
//     };
//
//     let buffer = [b'C'; 64];
//     let ingredient3 = FixedSafeString40 {
//         safeString: SafeString {
//             vtable: Self::FIXED_SAFE_STRING40_VTABLE_ADDR + (self.mem.get_main_offset() as u64),
//             mStringTop: base_address + 0x14 + 0x58 * 3,
//         },
//         mBufferSize: 64,
//         mBuffer: buffer,
//     };
//
//     let buffer = [b'D'; 64];
//     let ingredient4 = FixedSafeString40 {
//         safeString: SafeString {
//             vtable: Self::FIXED_SAFE_STRING40_VTABLE_ADDR + (self.mem.get_main_offset() as u64),
//             mStringTop: base_address + 0x14 + 0x58 * 4,
//         },
//         mBufferSize: 64,
//         mBuffer: buffer,
//     };
//
//     let buffer = [b'E'; 64];
//     let ingredient5 = FixedSafeString40 {
//         safeString: SafeString {
//             vtable: Self::FIXED_SAFE_STRING40_VTABLE_ADDR + (self.mem.get_main_offset() as u64),
//             mStringTop: base_address + 0x14 + 0x58 * 5,
//         },
//         mBufferSize: 64,
//         mBuffer: buffer,
//     };
//
//     let ingredients = [
//         ingredient1,
//         ingredient2,
//         ingredient3,
//         ingredient4,
//         ingredient5,
//     ];
//
//     let cook_item = CookItem::new(
//         actor_name,
//         ingredients,
//         life_recover,
//         effect_time,
//         sell_price,
//         effect_id as i32,
//         vitality_boost,
//         is_crit,
//     );
//
//     let mut writer = self.mem.write(base_address, None)?;
//     cook_item.write_to_mem(&mut writer)?;
//     Ok(base_address)
// }
//
// pub fn get_hash_for_actor(&mut self, name: &str) -> Result<u32, ExecutionError> {
//     let actor_name_addr = self.alloc_fixed_safe_string40(name)?;
//     let actor_name: FixedSafeString40 = Ptr::new(actor_name_addr)
//         .deref(self.mem)
//         .map_err(|e| self.to_execution_error(e))?;
//     self.cpu.write_arg(0, actor_name.safeString.mStringTop);
//     self.call_func_at_addr(0xB2170C)?;
//     Ok(self.cpu.read_arg(0) as u32)
// }
//
// fn alloc_weapon_modifier_info(&mut self, flags: u32, value: i32) -> Result<u64, error::Error> {
//     let base_address = self.mem.heap_mut().alloc(0x8)?;
//     self.mem.mem_write_val::<u32>(base_address, flags)?;
//     self.mem.mem_write_i32(base_address + 0x4, value)?;
//     Ok(base_address)
// }
// pub fn allocate_data(&mut self, data: Vec<u8>) -> Result<u64, crate::memory::Error> {
//     let start = self.mem.heap_mut().alloc(data.len() as u32)?;
//     self.mem.mem_write_bytes(start, data)?;
//     Ok(start)
// }
// }
