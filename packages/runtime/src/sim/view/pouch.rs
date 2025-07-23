use std::collections::BTreeMap;

use blueflame::game::{
    GrabbedItemInfo, ListNode, PauseMenuDataMgr, PouchItem, PouchItemType, gdt, singleton_instance,
};
use blueflame::memory::{self, Memory, Ptr, proxy};
use blueflame::processor::Process;

use crate::iv;
use crate::sim;

use super::{Error, coherence_error, try_mem};

/// Temporary helper struct to help us reason the coherence of PMDM
struct ItemPtrData {
    node: Ptr![ListNode],
    item: Ptr![PouchItem],
    buffer_idx: i32,
}

/// Read the pouch view from the process memory.
///
/// Returns error if the inventory cannot be read as a normal inventory (i.e.
/// if ISU happened or the inventory list is corrupted in some way).
///
/// However, tab overflow is allowed and will be indicated in the returned inventory
pub fn extract_pouch_view(proc: &Process, sys: &sim::GameSystems) -> Result<iv::PouchList, Error> {
    log::debug!("extracting pouch view from process");

    // get any inventory temporary state
    let (visually_equipped_items, accessible_items, entangled_items, entangled_tab, entangled_slot) = {
        match sys.screen.current_screen().as_inventory() {
            None => {
                // open an inventory menu temporarily to get the status
                let inv = try_mem!(
                    sim::PouchScreen::open_no_exec(proc, false),
                    e,
                    "failed to open temporary inventory"
                );

                let accessible = inv.accessible_item_ptrs();
                (vec![], accessible, vec![], -1, -1)
            }
            Some(inventory) => {
                let equipped = inventory.equipped_item_ptrs();
                let accessible = inventory.accessible_item_ptrs();
                let entangled = inventory.pe_activated_items();
                let entangled_pos = inventory
                    .active_pe_slot()
                    .map(|(t, s)| ((t as i32) % 3, s as i32))
                    .unwrap_or((-1, -1));
                (
                    equipped,
                    accessible,
                    entangled,
                    entangled_pos.0,
                    entangled_pos.1,
                )
            }
        }
    };

    let memory = proc.memory();

    let pmdm = try_mem!(
        singleton_instance!(pmdm(memory)),
        e,
        "failed to read pmdm instance: {e}"
    );
    let count = try_mem!(
        Ptr!(&pmdm->mList1.mCount).load(memory),
        e,
        "failed to read pouch list1 count: {e}"
    );

    let dpad_menu_items = extract_dpad_accessible_items(pmdm, count, memory)?;
    log::debug!("{} dpad menu items", dpad_menu_items.len());

    let mut item_ptr_data = vec![];
    let mut list_index = 0;
    let mut list_iter = try_mem!(
        Ptr!(&pmdm->mList1).begin(memory),
        e,
        "failed to get pouch list1 begin: {e}"
    );
    let mut node_before = Ptr!(&pmdm->mList1.mStartEnd);
    let iter_end = try_mem!(
        Ptr!(&pmdm->mList1).end(memory),
        e,
        "failed to get pouch list1 end: {e}"
    );

    while list_iter != iter_end {
        let item_ptr: Ptr![PouchItem] = list_iter.get_tptr().into();
        if item_ptr.is_nullptr() {
            coherence_error!("item at list1 position {list_index} is null");
        }
        let Some(item_buffer_idx) = pmdm.get_item_buffer_idx(item_ptr) else {
            coherence_error!("item at list1 position {list_index} is not in item buffer");
        };
        let node = list_iter.curr;
        let prev = try_mem!(
            Ptr!(&node->mPrev).load(memory),
            e,
            "failed to read prev node for item at list1 position {list_index}: {e}"
        );
        if prev != node_before {
            coherence_error!(
                "item at list1 position {list_index} is not linked properly (prev not actually prev)"
            );
        }
        node_before = node;
        item_ptr_data.push(ItemPtrData {
            node,
            item: item_ptr,
            buffer_idx: item_buffer_idx,
        });
        list_index += 1;
        try_mem!(
            list_iter.next(memory),
            e,
            "failed to advance to list1 position {list_index}: {e}"
        );
    }

    let mut tabs = vec![];
    let (are_tabs_valid, num_tabs) =
        match extract_pouch_tabs(pmdm, memory, &item_ptr_data, &mut tabs) {
            None => (false, 0),
            Some(x) => (true, x),
        };

    let bow_slots: Result<i32, memory::Error> = (|| {
        let gdt = gdt::trigger_param_ptr(memory)?;
        proxy! { let trigger_param = *gdt as trigger_param in proc };
        let bow_slots = trigger_param
            .by_name::<gdt::fd!(s32)>("BowPorchStockNum")
            .map(|x| x.get())
            .copied()
            .unwrap_or(0);
        Ok(bow_slots)
    })();
    let bow_slots = try_mem!(bow_slots, e, "failed to read number of bow slots: {e}");

    let mut grabbed_items = [GrabbedItemInfo {
        mItem: 0u64.into(),
        mIsActorSpawned: false,
    }; 5];
    try_mem!(
        pmdm.grabbed_items().load_slice(&mut grabbed_items, memory),
        e,
        "failed to read grabbed_items: {e}"
    );

    // build the item list with the ptr data
    let mut tab_idx = get_next_tab_head(&tabs, 0).0;
    let (mut next_tab_idx, mut next_tab_item_idx) = get_next_tab_head(&tabs, tab_idx + 1);
    let mut tab_slot = 0;
    let mut items = Vec::with_capacity(item_ptr_data.len());
    let mut animated_icons_to_node = BTreeMap::new();
    let mut num_bows_for_curr_tab = 0;
    for (i, data) in item_ptr_data.into_iter().enumerate() {
        if i as i32 == next_tab_item_idx {
            tab_idx = next_tab_idx;
            tab_slot = 0;
            let next = get_next_tab_head(&tabs, tab_idx + 1);
            next_tab_idx = next.0;
            next_tab_item_idx = next.1;
            num_bows_for_curr_tab = 0;
        }
        let item_ptr = data.item;
        let item_ptr_raw = data.item.to_raw();
        let accessible = accessible_items.binary_search(&item_ptr_raw).is_ok();
        let dpad_accessible = dpad_menu_items.binary_search(&item_ptr_raw).is_ok();
        let item = extract_pouch_item(
            i as i32,
            data.node,
            data.buffer_idx,
            item_ptr,
            tab_idx as i32,
            tab_slot,
            bow_slots,
            &mut num_bows_for_curr_tab,
            &mut animated_icons_to_node,
            memory,
            &visually_equipped_items,
            &entangled_items,
            &grabbed_items,
            accessible,
            dpad_accessible,
        )?;
        items.push(item);
        tab_slot += 1;
    }

    // update the no_icon property
    for item in items.iter_mut() {
        if let Some(icon_node_ptr) = animated_icons_to_node.get(&item.common.actor_name) {
            if item.node_addr != (*icon_node_ptr).into() {
                item.is_no_icon = true;
            }
        }
    }

    log::debug!("pouch view extracted successfully");
    Ok(iv::PouchList {
        count,
        items,
        are_tabs_valid,
        num_tabs,
        tabs,
        entangled_tab,
        entangled_slot,
        screen: sys.screen.current_screen().iv_type(),
        is_holding_in_inventory: sys.screen.holding_in_inventory,
        is_arrowless_smuggle: sys.overworld.is_holding_arrowless_smuggled(),
    })
}

/// Get the next tab index and the first item of that tab, skipping
/// empty tabs. Returns (0, -1) if no more items
fn get_next_tab_head(tabs: &[iv::PouchTab], next_tab_idx: usize) -> (usize, i32) {
    for (i, tab) in tabs.iter().skip(next_tab_idx).enumerate() {
        if tab.item_idx == -1 {
            continue;
        }
        return (next_tab_idx + i, tab.item_idx);
    }
    (0, -1)
}

/// Extract tab data into the tabs vec. Return mNumTabs if tabs are valid
fn extract_pouch_tabs(
    pmdm: Ptr![PauseMenuDataMgr],
    memory: &Memory,
    item_ptr_data: &[ItemPtrData],
    out_tabs: &mut Vec<iv::PouchTab>,
) -> Option<i32> {
    let num_tabs = match Ptr!(&pmdm->mNumTabs).load(memory) {
        Err(e) => {
            log::error!("failed to read num_tabs: {e}");
            return None;
        }
        Ok(x) => x,
    };
    if num_tabs < 0 || num_tabs > 50 {
        // FIXME: figure out how tabs are displayed when tabs are overflow
        return None;
    }
    let tabs = match Ptr!(&pmdm->mTabs).load(memory) {
        Err(e) => {
            log::error!("failed to read tabs: {e}");
            return None;
        }
        Ok(x) => x,
    };
    let tab_types = match Ptr!(&pmdm->mTabsType).load(memory) {
        Err(e) => {
            log::error!("failed to read tab types: {e}");
            return None;
        }
        Ok(x) => x,
    };
    for i in 0..50 {
        let tab_item = tabs[i];
        let tab_type = tab_types[i];
        if tab_item.is_nullptr() && tab_type == -1 {
            break;
        }
        let item_idx = if tab_item.is_nullptr() {
            -1
        } else {
            match item_ptr_data.iter().position(|x| x.item == tab_item) {
                None => {
                    log::error!("the tab data has an item that isn't in list1");
                    return None;
                }
                Some(x) => x as i32,
            }
        };
        out_tabs.push(iv::PouchTab { item_idx, tab_type })
    }

    Some(num_tabs)
}

#[allow(clippy::too_many_arguments)]
fn extract_pouch_item(
    list_index: i32,
    node_ptr: Ptr![ListNode],
    buffer_idx: i32,
    item: Ptr![PouchItem],
    tab_idx: i32,
    tab_slot: i32,
    bow_slots: i32,
    num_bows_for_curr_tab: &mut i32,
    animated_icons_to_node: &mut BTreeMap<String, u64>,
    memory: &Memory,
    visually_equipped_items: &[u64],
    entangled_items: &[u64],
    grabbed_items: &[GrabbedItemInfo],
    accessible: bool,
    dpad_accessible: bool,
) -> Result<iv::PouchItem, Error> {
    let name = try_mem!(
        Ptr!(&item->mName).utf8_lossy(memory),
        e,
        "failed to load item.name at list1 position {list_index}: {e}"
    );
    // only the last icon is animated
    if sim::util::is_animated_icon_actor(&name) {
        animated_icons_to_node.insert(name.to_string(), node_ptr.to_raw());
    }

    let value = try_mem!(
        Ptr!(&item->mValue).load(memory),
        e,
        "failed to load item.value at list1 position {list_index}: {e}"
    );
    let mut is_equipped = try_mem!(
        Ptr!(&item->mEquipped).load(memory),
        e,
        "failed to load item.is_equipped at list1 position {list_index}: {e}"
    );
    if !is_equipped {
        // check if the item is visually equipped
        is_equipped = visually_equipped_items
            .binary_search(&item.to_raw())
            .is_ok();
    }
    let common = iv::CommonItem {
        actor_name: name,
        value,
        is_equipped,
    };
    let item_type = try_mem!(
        Ptr!(&item->mType).load(memory),
        e,
        "failed to load item.type at list1 position {list_index}: {e}"
    );
    let item_use = try_mem!(
        Ptr!(&item->mItemUse).load(memory),
        e,
        "failed to load item.use at list1 position {list_index}: {e}"
    );
    let is_in_inventory = try_mem!(
        Ptr!(&item->mInInventory).load(memory),
        e,
        "failed to load item.is_in_inventory at list1 position {list_index}: {e}"
    );

    let effect_value = try_mem!(
        Ptr!(&item->mHealthRecover).load(memory),
        e,
        "failed to load item.effect_value at list1 position {list_index}: {e}"
    );
    let effect_duration = try_mem!(
        Ptr!(&item->mEffectDuration).load(memory),
        e,
        "failed to load item.effect_duration at list1 position {list_index}: {e}"
    );
    let sell_price = try_mem!(
        Ptr!(&item->mSellPrice).load(memory),
        e,
        "failed to load item.sell_price at list1 position {list_index}: {e}"
    );
    let effect_id = try_mem!(
        Ptr!(&item->mEffectId).load(memory),
        e,
        "failed to load item.effect_id at list1 position {list_index}: {e}"
    );
    let effect_level = try_mem!(
        Ptr!(&item->mEffectLevel).load(memory),
        e,
        "failed to load item.effect_level at list1 position {list_index}: {e}"
    );
    let data = iv::ItemData {
        effect_value,
        effect_duration,
        sell_price,
        effect_id,
        effect_level,
    };

    let ingredients_ptr = try_mem!(
        Ptr!(&item->mIngredients.mPtrs).load(memory),
        e,
        "failed to load item.ingredients_ptr at list1 position {list_index}: {e}"
    );
    let ingredients_num = try_mem!(
        Ptr!(&item->mIngredients.mPtrNum).load(memory),
        e,
        "failed to load item.ingredients.ptr_num at list1 position {list_index}: {e}"
    );
    if ingredients_num < 0 {
        coherence_error!(
            "item ingredients array at list1 position {list_index} has negative length"
        );
    }
    let ingredients_num = ingredients_num as usize;
    let mut ingredients: [String; 5] = std::array::from_fn(|_| String::default());

    #[allow(clippy::needless_range_loop)]
    for i in 0..ingredients_num {
        let ingr_ptr = try_mem!(
            ingredients_ptr.ith(i as u64).load(memory),
            e,
            "failed to load {i}-th ingredient for item at list1 position {list_index}: {e}"
        );
        let ingr_name = try_mem!(
            ingr_ptr.utf8_lossy(memory),
            e,
            "failed to load name of {i}-th ingredient for item at list1 position {list_index}: {e}"
        );
        ingredients[i] = ingr_name;
    }

    let prev = try_mem!(
        Ptr!(&node_ptr->mPrev).load(memory),
        e,
        "failed to load item.prev at list1 position {list_index}: {e}"
    );
    let next = try_mem!(
        Ptr!(&node_ptr->mNext).load(memory),
        e,
        "failed to load item.prev at list1 position {list_index}: {e}"
    );

    // adjust the slot for arrows using the type
    if item_type == 1 {
        *num_bows_for_curr_tab += 1;
    }
    let tab_slot = if item_type == 2 {
        tab_slot + bow_slots - *num_bows_for_curr_tab
    } else {
        tab_slot
    };

    let mut holding_count = 0;
    for g in grabbed_items {
        if g.mItem == item {
            holding_count += 1;
        }
    }

    let prompt_entangled = entangled_items.binary_search(&item.to_raw()).is_ok();

    let dpad_accessible = if item_type > PouchItemType::Shield as i32 {
        accessible
    } else {
        dpad_accessible
    };

    Ok(iv::PouchItem {
        common,
        item_type,
        item_use,
        is_in_inventory,
        is_no_icon: false, // will update in a later step
        data,
        ingredients,
        holding_count,
        prompt_entangled,
        node_addr: node_ptr.to_raw().into(),
        node_valid: true,
        node_pos: buffer_idx as i128,
        node_prev: prev.to_raw().into(),
        node_next: next.to_raw().into(),
        allocated_idx: list_index,
        unallocated_idx: -1, // TODO
        tab_idx,
        tab_slot,
        accessible,
        dpad_accessible,
    })
}

fn extract_dpad_accessible_items(
    pmdm: Ptr![PauseMenuDataMgr],
    count: i32,
    memory: &Memory,
) -> Result<Vec<u64>, Error> {
    log::debug!("extracting dpad menus");
    let mut out = vec![];
    for t in [
        PouchItemType::Sword,
        PouchItemType::Bow,
        PouchItemType::Arrow,
        PouchItemType::Shield,
    ] {
        out.extend(
            extract_dpad_menu(pmdm, count, t, memory)?
                .into_iter()
                .map(|x| x.to_raw()),
        );
    }
    out.sort();
    Ok(out)
}

/// Reimplementation of getWeaponsForDpad, for reading dpad view,
/// which is used to determine if an equipment is accessible via dpad menu
fn extract_dpad_menu(
    pmdm: Ptr![PauseMenuDataMgr],
    count: i32,
    pouch_type: PouchItemType,
    memory: &Memory,
) -> Result<Vec<Ptr![PouchItem]>, Error> {
    if count == 0 {
        return Ok(vec![]);
    }
    let mut list_iter = try_mem!(
        Ptr!(&pmdm->mList1).begin(memory),
        e,
        "failed to get pouch list1 begin: {e}"
    );
    let iter_end = try_mem!(
        Ptr!(&pmdm->mList1).end(memory),
        e,
        "failed to get pouch list1 end: {e}"
    );
    let mut out = Vec::with_capacity(20); // TODO: optimize: can avoid heap alloc here
    let mut i = 0;
    while list_iter != iter_end && out.len() < 20 {
        let item_ptr: Ptr![PouchItem] = list_iter.get_tptr().into();
        if item_ptr.is_nullptr() {
            break;
        }
        let item_type = try_mem!(
            Ptr!(&item_ptr->mType).load(memory),
            e,
            "failed to get pouch item type: {e}"
        );
        if item_type > PouchItemType::Shield as i32 {
            break;
        }
        let should_skip = if item_type != pouch_type as i32 {
            true
        } else if pouch_type != PouchItemType::Arrow {
            // do not include MS with 0 dura (but do include other 0 dura)
            let value = try_mem!(
                Ptr!(&item_ptr->mValue).load(memory),
                e,
                "failed to get pouch item value: {e}"
            );
            if value <= 0 && item_type == PouchItemType::Sword as i32 {
                let name = try_mem!(
                    Ptr!(&item_ptr->mName).cstr(memory),
                    e,
                    "failed to get pouch item name: {e}"
                );
                let name = try_mem!(
                    name.load_utf8_lossy(memory),
                    e,
                    "failed to get pouch item name: {e}"
                );
                name == "Weapon_Sword_070"
            } else {
                false
            }
        } else {
            false
        };
        if !should_skip {
            out.push(item_ptr);
        }
        i += 1;
        try_mem!(
            list_iter.next(memory),
            e,
            "failed to advance to list1 position {i}: {e}"
        );
    }

    Ok(out)
}
