use std::sync::Arc;

use blueflame::game::{
    self, ListNode, PouchCategory, PouchItem, PouchItemType, gdt, singleton_instance,
};
use blueflame::linker;
use blueflame::memory::{self, Ptr, mem, proxy};
use blueflame::processor::{self, Cpu2};
use skybook_parser::cir;

use crate::error::{ErrorReport, sim_error, sim_warning};
use crate::sim;

/// Forcefully decrease mCount of list 1 and increase mCount of list 2.
///
/// This is for backward compatibility with slot breaking in older versions
/// of simulator
pub fn break_slot(ctx: &mut sim::Context<&mut Cpu2>, count: i32) -> Result<(), processor::Error> {
    let pmdm = singleton_instance!(pmdm(ctx.cpu().proc.memory()))?;
    mem! {(ctx.cpu().proc.memory_mut()):
        let count1 = *(&pmdm->mList1.mCount);
        let count2 = *(&pmdm->mList2.mCount);
        *(&pmdm->mList1.mCount) = count1 - count;
        *(&pmdm->mList2.mCount) = count2 + count;
    }
    Ok(())
}

/// Add items as slots directly. If init, PMDM lists will be reset to initial state
pub fn add_slots(
    ctx: &mut sim::Context<&mut Cpu2>,
    errors: &mut Vec<ErrorReport>,
    items: &[cir::ItemSpec],
    init: bool,
) -> Result<(), processor::Error> {
    if init {
        // reset list1 and list2
        let m = ctx.cpu().proc.memory();
        let pmdm = singleton_instance!(pmdm(m))?;
        let m = ctx.cpu().proc.memory_mut();
        Ptr!(&pmdm->mList1).construct_with_offset(8, m)?;
        Ptr!(&pmdm->mList2).construct_with_offset(8, m)?;

        let list2 = Ptr!(&pmdm->mList2);
        // re-construct each item, and push it to list2
        for i in 0..420 {
            let item_ptr = pmdm.item_buffer().ith(i);
            item_ptr.construct(m)?;
            list2.push_front(Ptr!(&item_ptr->mListNode), m)?;
        }
    }
    for item in items {
        add_slots_internal(ctx, errors, item)?;
    }
    fix_inventory_state_and_gamedata(ctx)
}

/// Perform `!add-slot X item` for one item spec
pub fn add_slots_internal(
    ctx: &mut sim::Context<&mut Cpu2>,
    errors: &mut Vec<ErrorReport>,
    item: &cir::ItemSpec,
) -> Result<(), processor::Error> {
    // get type and use from item name
    let item_type = game::get_pouch_item_type(&item.name);
    let item_use = game::get_pouch_item_use(&item.name);

    let can_stack = game::can_stack(&item.name);
    let (amount, value) = match (
        item.amount,
        item.meta.as_ref().and_then(|x| x.value),
        can_stack,
    ) {
        (amount, Some(value), _) => {
            // adding stack with explicit value will always set value on the new stack
            (amount, value)
        }
        (amount, None, true) => {
            // adding stack with no explicit value, for stackable
            // the amount becomes the value
            (1, amount as i32)
        }
        (amount, None, false) => {
            // adding stack with no explicit value, for non-stackable
            // if it's a weapon, use the default life
            match game::get_weapon_general_life(&item.name) {
                None => (amount, 1),
                Some(life) => (amount, life * 100),
            }
        }
    };
    let equipped = item.meta.as_ref().and_then(|x| x.equip).unwrap_or_default();
    let life_recover = item
        .meta
        .as_ref()
        .and_then(|x| x.life_recover)
        .unwrap_or_default();
    let effect_duration = item
        .meta
        .as_ref()
        .and_then(|x| x.effect_duration)
        .unwrap_or_default();
    let sell_price = item
        .meta
        .as_ref()
        .and_then(|x| x.sell_price)
        .unwrap_or_default();
    let effect_id = item
        .meta
        .as_ref()
        .and_then(|x| x.effect_id_f32())
        .unwrap_or(-1f32);
    let effect_level = item
        .meta
        .as_ref()
        .and_then(|x| x.effect_level)
        .unwrap_or_default();

    let mut added = false;

    for _ in 0..amount {
        let item_ptr = push_new_item(ctx)?;
        if item_ptr.is_nullptr() {
            errors.push(sim_error!(item.span, CannotGetMore));
            break;
        }
        added = true;
        let m = ctx.cpu().proc.memory_mut();
        mem! { m:
            *(&item_ptr->mType) = item_type;
            *(&item_ptr->mItemUse) = item_use;
            *(&item_ptr->mValue) = value;
            *(&item_ptr->mEquipped) = equipped;
            *(&item_ptr->mInInventory) = true;
            *(&item_ptr->mHealthRecover) = life_recover;
            *(&item_ptr->mEffectDuration) = effect_duration;
            *(&item_ptr->mSellPrice) = sell_price;
            *(&item_ptr->mEffectId) = effect_id;
            *(&item_ptr->mEffectLevel) = effect_level;
        }
        let name_ptr = Ptr!(&item_ptr->mName);
        name_ptr.construct(m)?;
        name_ptr.safe_store(&item.name, m)?;
        if let Some(ingrs) = item.meta.as_ref().map(|x| &x.ingredients) {
            // the item ingredients array always have 5 elements for some reason
            // so this should be fine
            for (i, ingr) in ingrs.iter().enumerate() {
                let ingr_ptr = item_ptr.ith_ingredient(i as u64, m)?;
                ingr_ptr.construct(m)?;
                ingr_ptr.safe_store(ingr, m)?;
            }
        }
    }

    if added {
        let m = ctx.cpu().proc.memory_mut();
        let gdt_ptr = gdt::trigger_param_ptr(m)?;
        let proc = &mut ctx.cpu().proc;
        proxy! { let mut gdt = *gdt_ptr as trigger_param in proc }

        let isget_flag_name = format!("IsGet_{}", item.name);
        match gdt.by_name_mut::<gdt::fd!(bool)>(&isget_flag_name) {
            Some(flag) => flag.set(true),
            None => log::warn!("add-slot could not find flag: {isget_flag_name}"),
        }

        if let Some(category) = PouchItemType::from_value(item_type).map(PouchItemType::to_category)
            && category != PouchCategory::Invalid
        {
            match gdt.by_name_mut::<gdt::fd!(bool[])>("IsOpenItemCategory") {
                Some(flag) => {
                    let _ = flag.set_at(category as i32, true);
                }
                None => log::warn!("add-slot could not find IsOpenItemCategory flag"),
            }
        } else {
            log::warn!("add-slot could not find category for item_type: {item_type}")
        }
    }

    Ok(())
}

/// Simulates `uking::ui::PauseMenuDataMgr::Lists::pushNewItem`
///
/// Returns nullptr if list2.mCount is 0
pub fn push_new_item(ctx: &mut sim::Context<&mut Cpu2>) -> Result<Ptr![PouchItem], memory::Error> {
    let m = ctx.cpu().proc.memory();
    let pmdm = singleton_instance!(pmdm(m))?;

    let list2 = Ptr!(&pmdm->mList2);

    let m = ctx.cpu().proc.memory_mut();
    let item_node = list2.pop_front(m)?;
    if item_node.is_nullptr() {
        return Ok(0u64.into());
    }
    mem! { m:
        let offset1 = *(&pmdm->mList1.mOffset);
        let offset2 = *(&pmdm->mList2.mOffset);
    }
    let item_ptr = Ptr!(<PouchItem>(item_node.to_raw() - offset2 as u64));
    let item_node = Ptr!(<ListNode>(item_ptr.to_raw() + offset1 as u64));
    let list1 = Ptr!(&pmdm->mList1);
    list1.push_back(item_node, m)?;
    Ok(item_ptr)
}

pub fn fix_inventory_state_and_gamedata(
    ctx: &mut sim::Context<&mut Cpu2>,
) -> Result<(), processor::Error> {
    linker::update_inventory_info(ctx.cpu())?;
    linker::update_list_heads(ctx.cpu())?;
    linker::save_to_game_data(ctx.cpu())?;
    Ok(())
}

pub fn set_gdt(
    ctx: &mut sim::Context<&mut Cpu2>,
    name: &str,
    meta: &cir::GdtMeta,
    errors: &mut Vec<ErrorReport>,
) -> Result<(), memory::Error> {
    let span = ctx.span;

    macro_rules! cannot_find {
        ($desc:literal) => {{
            errors.push(sim_error!(
                span,
                CannotFindGdtFlag(name.to_string(), $desc.to_string())
            ));
            return Ok(());
        }};
    }
    macro_rules! invalid_index {
        ($i:ident, $desc:literal) => {{
            errors.push(sim_warning!(
                span,
                InvalidGdtArrayIndex(name.to_string(), $desc.to_string(), $i)
            ));
            return Ok(());
        }};
    }
    let m = ctx.cpu().proc.memory();
    let gdt_ptr = gdt::trigger_param_ptr(m)?;
    let proc = &mut ctx.cpu().proc;
    proxy! { let mut gdt = *gdt_ptr as trigger_param in proc};

    match &meta.value {
        cir::GdtValueSpec::Bool(v) => match meta.array_idx {
            Some(i) => match gdt.by_name_mut::<gdt::fd!(bool[])>(name) {
                None => cannot_find!("bool[]"),
                Some(flag) => {
                    if !flag.set_at(i, *v) {
                        invalid_index!(i, "bool[]");
                    }
                }
            },
            None => match gdt.by_name_mut::<gdt::fd!(bool)>(name) {
                None => cannot_find!("bool[]"),
                Some(flag) => flag.set(*v),
            },
        },
        cir::GdtValueSpec::S32(v) => match meta.array_idx {
            Some(i) => match gdt.by_name_mut::<gdt::fd!(s32[])>(name) {
                None => cannot_find!("s32[]"),
                Some(flag) => {
                    if !flag.set_at(i, *v) {
                        invalid_index!(i, "s32[]");
                    }
                }
            },
            None => match gdt.by_name_mut::<gdt::fd!(s32)>(name) {
                None => cannot_find!("s32"),
                Some(flag) => flag.set(*v),
            },
        },
        cir::GdtValueSpec::F32(v) => match meta.array_idx {
            Some(i) => match gdt.by_name_mut::<gdt::fd!(f32[])>(name) {
                None => cannot_find!("f32[]"),
                Some(flag) => {
                    if !flag.set_at(i, *v) {
                        invalid_index!(i, "f32[]");
                    }
                }
            },
            None => match gdt.by_name_mut::<gdt::fd!(f32)>(name) {
                None => cannot_find!("f32"),
                Some(flag) => flag.set(*v),
            },
        },
        cir::GdtValueSpec::String32(v) => match meta.array_idx {
            // there are no str32[] flags in the game
            Some(_) => cannot_find!("str32[]"),
            None => match gdt.by_name_mut::<gdt::fd!(str32)>(name) {
                None => cannot_find!("str32"),
                Some(flag) => flag.set(Arc::from(v.as_str())),
            },
        },
        cir::GdtValueSpec::String64(v) => match meta.array_idx {
            Some(i) => match gdt.by_name_mut::<gdt::fd!(str64[])>(name) {
                None => cannot_find!("str64[]"),
                Some(flag) => {
                    if !flag.set_at(i, Arc::from(v.as_str())) {
                        invalid_index!(i, "str64[]")
                    }
                }
            },
            None => match gdt.by_name_mut::<gdt::fd!(str64)>(name) {
                None => cannot_find!("str64"),
                Some(flag) => flag.set(Arc::from(v.as_str())),
            },
        },
        cir::GdtValueSpec::String256(v) => match meta.array_idx {
            Some(i) => match gdt.by_name_mut::<gdt::fd!(str256[])>(name) {
                None => cannot_find!("str256[]"),
                Some(flag) => {
                    if !flag.set_at(i, Arc::from(v.as_str())) {
                        invalid_index!(i, "str256[]")
                    }
                }
            },
            None => match gdt.by_name_mut::<gdt::fd!(str256)>(name) {
                None => cannot_find!("str256"),
                Some(flag) => flag.set(Arc::from(v.as_str())),
            },
        },
        cir::GdtValueSpec::Vec2f(x, y) => match meta.array_idx {
            Some(i) => match gdt.by_name_mut::<gdt::fd!(vec2f[])>(name) {
                None => cannot_find!("vec2f[]"),
                Some(flag) => {
                    let (x, y) = match (x, y) {
                        (Some(x), Some(y)) => (*x, *y),
                        (x, y) => {
                            let Some(v) = flag.get_at(i) else {
                                invalid_index!(i, "vec2f[]");
                            };
                            ((*x).unwrap_or(v.0), (*y).unwrap_or(v.1))
                        }
                    };
                    if !flag.set_at(i, (x, y)) {
                        invalid_index!(i, "vec2f[]");
                    }
                }
            },
            None => match gdt.by_name_mut::<gdt::fd!(vec2f)>(name) {
                None => cannot_find!("vec2f"),
                Some(flag) => {
                    let (x, y) = match (x, y) {
                        (Some(x), Some(y)) => (*x, *y),
                        (x, y) => {
                            let v = flag.get();
                            ((*x).unwrap_or(v.0), (*y).unwrap_or(v.1))
                        }
                    };
                    flag.set((x, y));
                }
            },
        },
        cir::GdtValueSpec::Vec3f(x, y, z) => match meta.array_idx {
            Some(i) => match gdt.by_name_mut::<gdt::fd!(vec3f[])>(name) {
                None => cannot_find!("vec3f[]"),
                Some(flag) => {
                    let (x, y, z) = match (x, y, z) {
                        (Some(x), Some(y), Some(z)) => (*x, *y, *z),
                        (x, y, z) => {
                            let Some(v) = flag.get_at(i) else {
                                invalid_index!(i, "vec3f[]");
                            };
                            (
                                (*x).unwrap_or(v.0),
                                (*y).unwrap_or(v.1),
                                (*z).unwrap_or(v.2),
                            )
                        }
                    };
                    if !flag.set_at(i, (x, y, z)) {
                        invalid_index!(i, "vec3f[]");
                    }
                }
            },
            None => match gdt.by_name_mut::<gdt::fd!(vec3f)>(name) {
                None => cannot_find!("vec3f"),
                Some(flag) => {
                    let (x, y, z) = match (x, y, z) {
                        (Some(x), Some(y), Some(z)) => (*x, *y, *z),
                        (x, y, z) => {
                            let v = flag.get();
                            (
                                (*x).unwrap_or(v.0),
                                (*y).unwrap_or(v.1),
                                (*z).unwrap_or(v.2),
                            )
                        }
                    };
                    flag.set((x, y, z));
                }
            },
        },
    }

    Ok(())
}
