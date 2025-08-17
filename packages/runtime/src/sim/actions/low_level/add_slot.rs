use blueflame::game::{
    self, ListNode, PouchCategory, PouchItem, PouchItemType, gdt, singleton_instance,
};
use blueflame::linker;
use blueflame::memory::{self, Ptr, mem, proxy};
use blueflame::processor::{self, Cpu2};
use skybook_parser::cir;

use crate::error::{ErrorReport, sim_error};
use crate::sim;

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
    super::fix_inventory_state(ctx)?;
    linker::save_to_game_data(ctx.cpu())
}

/// Perform `!add-slot X item` for one item spec
fn add_slots_internal(
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
            None => cu::warn!("add-slot could not find flag: {isget_flag_name}"),
        }

        if let Some(category) = PouchItemType::from_value(item_type).map(PouchItemType::to_category)
            && category != PouchCategory::Invalid
        {
            match gdt.by_name_mut::<gdt::fd!(bool[])>("IsOpenItemCategory") {
                Some(flag) => {
                    let _ = flag.set_at(category as i32, true);
                }
                None => cu::warn!("add-slot could not find IsOpenItemCategory flag"),
            }
        } else {
            cu::warn!("add-slot could not find category for item_type: {item_type}")
        }
    }

    Ok(())
}

/// Simulates `uking::ui::PauseMenuDataMgr::Lists::pushNewItem`
///
/// Returns nullptr if list2.mCount is 0
fn push_new_item(ctx: &mut sim::Context<&mut Cpu2>) -> Result<Ptr![PouchItem], memory::Error> {
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
