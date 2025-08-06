use blueflame::game::{PouchItem, PouchItemType};
use blueflame::memory::{Memory, Ptr, mem};
use blueflame::processor::{self, Cpu2};
use blueflame::{linker, memory};
use skybook_parser::cir;

use crate::error::{ErrorReport, sim_error, sim_warning};
use crate::sim;

/// Handle `equip` and `unequip` commands
pub fn change_equip(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    items: &[cir::ItemSelectSpec],
    pe_target: Option<&cir::ItemSelectSpec>,
    is_equip: bool,
    is_dpad: bool,
) -> Result<(), processor::Error> {
    if is_dpad {
        change_equip_dpad(ctx, sys, errors, items, is_equip)
    } else {
        change_equip_inventory(ctx, sys, errors, items, pe_target, is_equip)
    }
}

/// Handle equip/unequip items in the inventory
pub fn change_equip_inventory(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    items: &[cir::ItemSelectSpec],
    pe_target: Option<&cir::ItemSelectSpec>,
    is_equip: bool,
) -> Result<(), processor::Error> {
    super::switch_to_inventory_or_stop!(ctx, sys, errors, "CHGEQUIP");
    super::check_not_holding_in_inventory!(ctx, sys, errors, "CHGEQUIP");

    let inventory = sys.screen.current_screen_mut().as_inventory_mut().unwrap();

    'outer: for item in items {
        let mut matcher = item.matcher.clone();
        let span = matcher.span;
        let item_type = sim::util::name_spec_to_item_type(&matcher.name);
        if item_type > PouchItemType::ArmorLower as i32
            && item_type != PouchItemType::KeyItem as i32
        {
            errors.push(sim_error!(span, NotEquipment));
            continue;
        }
        if item_type == PouchItemType::Arrow as i32 && !is_equip {
            // unequipping arrow as the same effect as equipped,
            // but in the game, you won't even get the prompt normally,
            // so it's probably not what the user meant to do
            errors.push(sim_error!(span, CannotUnequipArrow));
            continue;
        }

        // only target equipped items if the command is unequip
        let equip_meta = if is_equip { None } else { Some(true) };
        match matcher.meta.as_mut() {
            None => {
                matcher.meta = Some(cir::ItemMeta {
                    equip: equip_meta,
                    ..Default::default()
                })
            }
            Some(meta) => meta.equip = equip_meta,
        }
        let memory = ctx.cpu().proc.memory();
        // we want to expand the "all" amount if it's equip, to avoid getting stuck
        // to try to equip everything (since equipping something would unequip the rest)
        let mut remaining = super::convert_amount(item.amount, span, errors, is_equip, |_| {
            Ok(inventory.get_amount(&matcher, sim::CountingMethod::Slot, memory)?)
        })?;
        let mut check_for_extra_error = true;

        loop {
            if ctx.is_aborted() {
                break 'outer;
            }
            if remaining.is_done(span, errors, "CHGEQUIP") {
                break;
            }
            // select the item
            let memory = ctx.cpu().proc.memory();
            let position = inventory.select(&matcher, memory, errors)?;
            let Some((tab, slot)) = position else {
                break;
            };
            // check if the item has the equip/unequip prompt
            let (original_item, is_arrow, is_weapon, item_type) = match inventory.get(tab, slot) {
                sim::ScreenItemState::Normal(item_ptr) => {
                    mem! { memory:
                        let t = *(&item_ptr->mType);
                        let equipped = *(&item_ptr->mEquipped);
                    };
                    if is_equip == equipped {
                        if is_equip {
                            errors.push(sim_warning!(span, ItemAlreadyEquipped));
                        } else {
                            errors.push(sim_warning!(span, ItemAlreadyUnequipped));
                        }
                        check_for_extra_error = false;
                        break;
                    }
                    let o_item_name = Ptr!(&item_ptr->mName)
                        .cstr(memory)?
                        .load_utf8_lossy(memory)?;
                    let is_herosoul = sim::util::is_hero_soul(&o_item_name);
                    if t > PouchItemType::ArmorLower as i32 && !is_herosoul {
                        errors.push(sim_error!(span, NotEquipment));
                        break;
                    }
                    let is_weapon = t <= PouchItemType::Shield as i32;
                    let is_arrow = sim::util::name_is_arrow(&o_item_name);
                    (item_ptr, is_arrow, is_weapon, t)
                }
                _ => {
                    // the item to equip must be non empty and non translucent
                    errors.push(sim_error!(span, InvalidItemTarget));
                    check_for_extra_error = false;
                    break;
                }
            };
            // check for PE
            let Some((tab, slot)) = super::change_to_pe_target_if_need(
                pe_target, inventory, memory, tab, slot, errors,
            )?
            else {
                check_for_extra_error = false;
                break;
            };
            let correct_slot = inventory.corrected_slot(tab, slot);

            // the exact equip/unequip behavior is checked based on original item
            // See 0x7100A3BA7C in 1.5.0
            if is_arrow {
                // only equip is possible for arrow
                // this should be checked already
                linker::equip_from_tab_slot(ctx.cpu(), tab as i32, correct_slot)?;
                // update visual equipped status
                inventory.update_all_items(ctx.cpu(), false)?;
                inventory.update(tab, slot, Some(true), ctx.cpu().proc.memory())?;
            } else {
                if is_equip {
                    linker::equip_from_tab_slot(ctx.cpu(), tab as i32, correct_slot)?;
                } else {
                    linker::unequip_from_tab_slot(ctx.cpu(), tab as i32, correct_slot)?;
                }
                // make sure to update items as unequipped
                inventory.update_all_items(ctx.cpu(), false)?;
                // update visual equipped status
                inventory.update(tab, slot, Some(is_equip), ctx.cpu().proc.memory())?;
                if is_weapon {
                    // update inventory equip status
                    let state = inventory.equipment_state_mut(item_type);
                    if is_equip {
                        state.set_equip(original_item);
                    } else {
                        state.set_unequip();
                    }
                }
            }

            remaining.sub(1)
        }
        if check_for_extra_error {
            let memory = ctx.cpu().proc.memory();
            let result = remaining.check(span, errors, |_| {
                inventory.get_amount(&matcher, sim::CountingMethod::Slot, memory)
            })?;
            super::check_remaining!(result, errors, span);
        }
    }

    Ok(())
}

/// Handle equip/unequip items using dpad quick menu (MainShortCut)
pub fn change_equip_dpad(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    items: &[cir::ItemSelectSpec],
    is_equip: bool,
) -> Result<(), processor::Error> {
    // must be in overworld to use dpad quick menu
    super::switch_to_overworld_or_stop!(ctx, sys, errors, "CHGEQUIP-DPAD");
    // each dpad operation is separate, so we re-compute the quick menu
    // state inside the loop each time
    'outer: for item in items {
        if ctx.is_aborted() {
            break;
        }
        let matcher = &item.matcher;
        let span = matcher.span;

        let item_type = sim::util::name_spec_to_item_type(&matcher.name);
        if item_type > PouchItemType::Shield as i32 {
            errors.push(sim_error!(span, InvalidDpadType));
            continue;
        }
        if item_type == PouchItemType::Arrow as i32 && !is_equip {
            errors.push(sim_error!(span, CannotUnequipArrow));
            continue;
        }

        let cpu = ctx.cpu();
        let mut remaining = super::convert_amount(item.amount, span, errors, false, |_| {
            // open dpad menu just for counting first
            let dpad_items = linker::get_weapons_for_dpad(cpu, item_type)?;
            Ok(dpad_get_amount(
                &dpad_items,
                matcher,
                is_equip,
                cpu.proc.memory(),
            )?)
        })?;

        loop {
            if ctx.is_aborted() {
                break 'outer;
            }
            if remaining.is_done(span, errors, "CHGEQUIP-DPAD") {
                break;
            }
            // open the dpad menu for real
            let dpad_items = linker::get_weapons_for_dpad(ctx.cpu(), item_type)?;

            // select the item
            let selected_item = dpad_select(
                &dpad_items,
                matcher,
                is_equip,
                errors,
                ctx.cpu().proc.memory(),
            )?;
            let Some(selected_item) = selected_item else {
                if is_equip {
                    errors.push(sim_error!(span, CannotFindItemDpadEquip));
                } else {
                    errors.push(sim_error!(span, CannotFindItemDpadUnequip));
                }
                break;
            };
            let mut need_sync_to_pmdm = false;
            if is_equip {
                if selected_item != dpad_get_first_equipped(&dpad_items, ctx.cpu().proc.memory())? {
                    // mark equip in pmdm
                    linker::equip_weapon(ctx.cpu(), selected_item)?;
                        // update equipment in overworld
                        let memory = ctx.cpu().proc.memory();
                        need_sync_to_pmdm = 
                        sys.overworld
                            .change_player_equipment(selected_item, memory)?;
                }
            } else {
                linker::unequip(ctx.cpu(), selected_item)?;
                // get the item type, or fall back to the type specified in command
                let typ = if selected_item.is_nullptr() {
                    item_type
                } else {
                    mem! { (ctx.cpu().proc.memory()): let typ = *(&selected_item->mType); }
                    typ
                };
                sys.overworld.delete_player_equipment(typ);
            }
            // on close dpad
            linker::delete_removed_items(ctx.cpu())?;
            if need_sync_to_pmdm {
                sys.overworld
                    .update_equipment_value_to_pmdm(ctx.cpu(), item_type)?;
            }
            remaining.sub(1);
        }
        // currently we don't do remaining count checks for dpad equip/uneauip
    }

    Ok(())
}

/// Get the first item with mEquipped = true, or nullptr
fn dpad_get_first_equipped(
    menu: &[Ptr![PouchItem]],
    memory: &Memory,
) -> Result<Ptr![PouchItem], memory::Error> {
    for item in menu {
        let item = *item;
        if item.is_nullptr() {
            continue;
        }
        mem! { memory: let equipped = *(&item->mEquipped); }
        if !equipped {
            continue;
        }
        return Ok(item);
    }

    Ok(0u64.into())
}

/// Select item from dpad
fn dpad_select(
    menu: &[Ptr![PouchItem]],
    matcher: &cir::ItemMatchSpec,
    // meta: Option<&cir::ItemMeta>,
    is_for_equip: bool,
    // span: Span,
    errors: &mut Vec<ErrorReport>,
    memory: &Memory,
) -> Result<Option<Ptr![PouchItem]>, memory::Error> {
    let span = matcher.span;
    let Some(meta) = &matcher.meta else {
        return dpad_select_without_position_nth(menu, matcher, is_for_equip, 0, errors, memory);
    };
    let from_slot = if is_for_equip {
        match &meta.position {
            None => 0, // match first slot
            Some(cir::ItemPosition::FromSlot(n)) => (*n as usize).saturating_sub(1), // match x-th slot, 1 indexed
            _ => {
                // cannot specify by tab for dpad items
                errors.push(sim_error!(span, PositionSpecNotAllowed));
                0
            }
        }
    } else {
        if meta.position.is_some() {
            // cannot specify by tab for unequipping (can only unequip first equipped)
            errors.push(sim_error!(span, PositionSpecNotAllowed));
        }
        0
    };

    dpad_select_without_position_nth(menu, matcher, is_for_equip, from_slot, errors, memory)
}

/// Select item from dpad
// #[allow(clippy::too_many_arguments)]
fn dpad_select_without_position_nth(
    menu: &[Ptr![PouchItem]],
    matcher: &cir::ItemMatchSpec,
    // meta: Option<&cir::ItemMeta>,
    is_for_equip: bool,
    nth: usize,
    // span: Span,
    errors: &mut Vec<ErrorReport>,
    memory: &Memory,
) -> Result<Option<Ptr![PouchItem]>, memory::Error> {
    let meta = matcher.meta.as_ref();
    let span = matcher.span;
    // currently, we do not match these, as dpad is only supposed
    // to be for weapons. They may still be used in the future
    if let Some(meta) = meta {
        if meta.equip.is_some()
            || meta.effect_duration.is_some()
            || meta.effect_id.is_some()
            || meta.effect_level.is_some()
            || !meta.ingredients.is_empty()
            || meta.held.is_some()
        {
            errors.push(sim_warning!(span, UselessItemMatchProp));
        }
    }
    let mut count = nth;
    for item in menu {
        let item = *item;
        mem! {memory: let equipped = *(&item->mEquipped); }
        if !is_for_equip && !equipped {
            // skip the unequipped items if it's for unequip
            continue;
        }
        if !dpad_matches(item, matcher, is_for_equip, memory)? {
            if !is_for_equip {
                // for unequip, we only check the first
                return Ok(None);
            }
            continue;
        }

        if count > 0 {
            count -= 1;
            continue;
        }
        return Ok(Some(item));
    }

    Ok(None)
}

fn dpad_get_amount(
    menu: &[Ptr![PouchItem]],
    matcher: &cir::ItemMatchSpec,
    // meta: Option<&cir::ItemMeta>,
    is_for_equip: bool,
    memory: &Memory,
) -> Result<usize, memory::Error> {
    let Some(meta) = &matcher.meta else {
        return dpad_get_amount_without_position_nth(menu, matcher, is_for_equip, 0, memory);
    };
    let from_slot = if is_for_equip {
        match &meta.position {
            Some(cir::ItemPosition::FromSlot(n)) => (*n as usize).saturating_sub(1), // match x-th slot, 1 indexed
            _ => 0,
        }
    } else {
        0
    };

    dpad_get_amount_without_position_nth(menu, matcher, is_for_equip, from_slot, memory)
}

/// Get number of matching items in dpad menu
fn dpad_get_amount_without_position_nth(
    menu: &[Ptr![PouchItem]],
    matcher: &cir::ItemMatchSpec,
    // meta: Option<&cir::ItemMeta>,
    is_for_equip: bool,
    nth: usize,
    memory: &Memory,
) -> Result<usize, memory::Error> {
    let mut skip = nth;
    let mut count = 0;
    for item in menu {
        let item = *item;
        if !dpad_matches(item, matcher, is_for_equip, memory)? {
            continue;
        }
        if skip > 0 {
            skip -= 1;
            continue;
        }
        count += 1;
    }

    Ok(count)
}

fn dpad_matches(
    item: Ptr![PouchItem],
    matcher: &cir::ItemMatchSpec,
    // meta: Option<&cir::ItemMeta>,
    is_for_equip: bool, // false if for unequip
    memory: &Memory,
) -> Result<bool, memory::Error> {
    mem! {memory: let equipped = *(&item->mEquipped); }
    if is_for_equip {
        // only check unequipped
        if equipped {
            return Ok(false);
        }
    } else {
        // only check equipped
        if !equipped {
            return Ok(false);
        }
    }

    // try to match the item
    let name = Ptr!(&item->mName).cstr(memory)?.load_utf8_lossy(memory)?;
    if !sim::util::name_spec_matches(&matcher.name, &name) {
        return Ok(false);
    }
    let meta = matcher.meta.as_ref();

    if let Some(wanted_value) = meta.and_then(|x| x.value) {
        mem! { memory: let actual_value = *(&item->mValue); };
        if wanted_value != actual_value {
            return Ok(false);
        }
    }
    // modifier value
    if let Some(wanted_value) = meta.and_then(|x| x.life_recover) {
        mem! { memory: let actual_value = *(&item->mHealthRecover); };
        if wanted_value != actual_value {
            return Ok(false);
        }
    }
    // modifier flag
    if let Some(wanted) = meta.and_then(|x| x.sell_price) {
        mem! { memory: let actual = *(&item->mSellPrice); };
        if !sim::util::modifier_meta_matches(&matcher.name, wanted, actual) {
            return Ok(false);
        }
    }

    Ok(true)
}
