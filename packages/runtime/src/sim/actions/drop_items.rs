use blueflame::linker;
use blueflame::memory::mem;
use blueflame::processor::{self, Cpu2};
use skybook_parser::cir;

use crate::error::{ErrorReport, sim_error, sim_warning};
use crate::sim;

/// Drop items
///
/// If no items are specified, it will only drop held items on the ground,
/// and requires overworld screen.
///
/// For equipments, it uses the drop prompt, and stays in inventory
/// For holdable items, it holds it first, then close inventory and drop
///
/// If `overworld`, equipments are dropped directly from overworld (i.e.
/// shocked or displayed)
#[allow(clippy::too_many_arguments)]
pub fn drop_items(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    items: &[cir::ItemSelectSpec],
    pe_target: Option<&cir::ItemSelectSpec>,
    pick_up: bool,
    overworld: bool,
    pause_during: bool,
) -> Result<(), processor::Error> {
    if items.is_empty() {
        log::debug!("no items specified for DROP, dropping held items ");
        super::switch_to_overworld_or_stop!(ctx, sys, errors, "DROP");
        if !sys.overworld.is_holding() {
            errors.push(sim_warning!(ctx.span, NotHolding));
            return Ok(());
        }
        super::drop_held_items(ctx, sys, "DROP")?;
        return Ok(());
    }
    // since the handling for different types of items are different,
    // we need to do that inside the loop
    for item in items {
        if ctx.is_aborted() {
            break;
        }
        if !sim::util::name_spec_can_drop(&item.name) {
            errors.push(sim_error!(item.span, NotDroppable));
            continue;
        }

        if sim::util::name_spec_is_weapon(&item.name) {
            // dropping a weapon
            if overworld {
                drop_overworld_weapon(ctx, sys, errors, item, pause_during)?;
            } else {
                // to drop items in the inventory, if you are holding,
                // then you are locked to holding items and cannot drop
                super::check_not_holding_in_inventory!(ctx, sys, errors, "DROP");
                drop_inventory_weapon(ctx, sys, errors, item, pe_target)?;
            }
        } else {
            // to drop items in the inventory, if you are holding,
            // then you are locked to holding items and cannot drop
            super::check_not_holding_in_inventory!(ctx, sys, errors, "DROP");
            drop_inventory_material(ctx, sys, errors, item, pe_target)?;
        }
    }

    if pick_up {
        super::pick_up_items(ctx, sys, errors, items, pause_during)?;
    }

    Ok(())
}

/// Internal handler for `drop` command specifically for materials - from inventory
///
/// may be in inventory or in overworld afterwards
fn drop_inventory_material(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    item: &cir::ItemSelectSpec,
    pe_target: Option<&cir::ItemSelectSpec>,
) -> Result<(), processor::Error> {
    // if holding in the overworld, drop those first
    if sys.overworld.is_holding() {
        log::debug!("dropping currently held items in DROP command before processing other items");
        super::switch_to_overworld_or_stop!(ctx, sys, errors, "DROP");
        super::drop_held_items(ctx, sys, "DROP")?;
    }
    // must be in inventory to hold materials
    super::switch_to_inventory_or_stop!(ctx, sys, errors, "DROP");
    let name = &item.name;
    let meta = item.meta.as_ref();
    // for items, hold 1 at a time and drop, and we must know
    // exactly how many there are
    let memory = ctx.cpu().proc.memory();
    let inventory = sys.screen.current_screen().as_inventory().unwrap();
    let count_fn = || inventory.get_amount(name, meta, sim::CountingMethod::Value, memory);
    let mut amount = super::convert_amount(item.amount, item.span, errors, true, count_fn)?
        .count()
        .unwrap_or_default();
    let mut hold_spec = item.clone();
    while amount > 0 {
        if ctx.is_aborted() {
            return Ok(());
        }
        super::switch_to_inventory_or_stop!(ctx, sys, errors, "DROP");
        let hold_amount = amount.min(5);
        hold_spec.amount = cir::AmountSpec::Num(hold_amount);
        super::hold_item_internal(ctx, sys, errors, &hold_spec, pe_target)?;
        // need to be in overworld to drop
        super::switch_to_overworld_or_stop!(ctx, sys, errors, "DROP");
        if !sys.overworld.is_holding() {
            log::debug!("failed to hold, stopping");
            errors.push(sim_error!(item.span, OperationNotComplete));
            break;
        }
        super::drop_held_items(ctx, sys, "DROP")?;
        amount -= hold_amount;
    }

    Ok(())
}

/// Internal handler for `drop` command specifically for weapons - from inventory
///
/// may be in inventory or in overworld afterwards
fn drop_inventory_weapon(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    item: &cir::ItemSelectSpec,
    pe_target: Option<&cir::ItemSelectSpec>,
) -> Result<(), processor::Error> {
    // must be in inventory to drop from inventory
    super::switch_to_inventory_or_stop!(ctx, sys, errors, "DROP");
    let name = &item.name;
    let meta = item.meta.as_ref();
    let memory = ctx.cpu().proc.memory();
    let inventory = sys.screen.current_screen().as_inventory().unwrap();
    let mut remaining = super::convert_amount(item.amount, item.span, errors, false, || {
        inventory.get_amount(name, meta, sim::CountingMethod::Slot, memory)
    })?;
    let mut check_for_extra_error = true;
    loop {
        if ctx.is_aborted() {
            return Ok(());
        }
        if remaining.is_done() {
            break;
        }
        let memory = ctx.cpu().proc.memory();
        // select the slot
        let inventory = sys.screen.current_screen().as_inventory().unwrap();
        let position = inventory.select(name, meta, None, memory, item.span, errors)?;
        let Some((tab, slot)) = position else {
            break;
        };
        log::debug!("dropping inventory weapon at tab={tab}, slot={slot}");
        let original_item = match inventory.get(tab, slot) {
            sim::ScreenItemState::Normal(item_ptr) => {
                // the item to drop must be a weapon/bow/shield
                mem! { memory: let t = *(&item_ptr->mType); };
                if !matches!(t, 0 | 1 | 3) {
                    errors.push(sim_error!(item.span, NotEquipment));
                    check_for_extra_error = false;
                    break;
                }
                item_ptr
            }
            _ => {
                // the item to drop must be non empty and non translucent
                errors.push(sim_error!(item.span, InvalidItemTarget));
                check_for_extra_error = false;
                break;
            }
        };
        let Some((tab, slot)) =
            super::change_to_pe_target_if_need(pe_target, inventory, memory, tab, slot, errors)?
        else {
            check_for_extra_error = false;
            break;
        };

        // first unequip the item based on if the original item
        // is equipped, see 0x7100A3AEE4 in 1.5
        mem! { memory:
            let is_equipped = *(&original_item->mEquipped);
            let original_type = *(&original_item->mType);
        }
        let correct_slot = inventory.corrected_slot(tab, slot);
        let inventory = sys.screen.current_screen_mut().as_inventory_mut().unwrap();
        if is_equipped {
            log::debug!("unequipped item from DROP");
            linker::unequip_from_tab_slot(ctx.cpu(), tab as i32, correct_slot)?;
            inventory.equipment_state_mut(original_type).set_unequip();
        }
        log::debug!("correct_slot is {correct_slot}");
        super::trash_item_wrapped(ctx.cpu(), sys, tab as i32, correct_slot)?;
        let inventory = sys.screen.current_screen_mut().as_inventory_mut().unwrap();
        inventory.update(tab, slot, Some(false), ctx.cpu().proc.memory())?;
        remaining.sub(1);
    }
    if check_for_extra_error {
        let memory = ctx.cpu().proc.memory();
        let inventory = sys.screen.current_screen().as_inventory().unwrap();
        let result = remaining.check(item.span, errors, || {
            inventory.get_amount(name, meta, sim::CountingMethod::Slot, memory)
        })?;
        super::check_remaining!(result, errors, item.span);
    }

    Ok(())
}

/// Internal handler for `drop` command specifically for weapons - from overworld
///
/// Might be in overworld or pause menu afterwards
fn drop_overworld_weapon(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    item: &cir::ItemSelectSpec,
    pause_during: bool,
) -> Result<(), processor::Error> {
    // must be in overworld to drop from inventory
    super::switch_to_overworld_or_stop!(ctx, sys, errors, "DROP");
    super::check_overworld_amount(item, errors);
    let name = &item.name;
    let meta = item.meta.as_ref();
    let Some(equipment) = sys
        .overworld
        .equipped_select_mut(name, meta, item.span, errors)
    else {
        return Ok(());
    };
    let actor = equipment.remove();
    if pause_during {
        log::debug!("pause-during drop_overworld_weapon");
        sys.screen.set_remove_equipment_after_dialog(&actor.name);
        sys.screen
            .transition_to_inventory(ctx, &mut sys.overworld, false, errors)?;
    } else {
        linker::remove_weapon_if_equipped(ctx.cpu(), &actor.name)?;
    }
    sys.overworld.spawn_weapon_later(actor);
    sys.check_weapon_spawn();

    Ok(())
}
