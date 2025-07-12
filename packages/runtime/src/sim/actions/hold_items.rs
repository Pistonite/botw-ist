use blueflame::linker;
use blueflame::memory::mem;
use blueflame::processor::{self, Cpu2};
use skybook_parser::cir;

use crate::error::{ErrorReport, sim_error};
use crate::sim;

/// Hold items in pouch
pub fn hold_items(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    items: &[cir::ItemSelectSpec],
    pe_target: Option<&cir::ItemSelectSpec>,
    attach: bool,
) -> Result<(), processor::Error> {
    // must be in inventory to hold items
    super::switch_to_inventory_or_stop!(ctx, sys, errors, "HOLD");
    // start holding in inventory
    sys.screen.holding_in_inventory = true;
    for item in items {
        if ctx.is_aborted() {
            break;
        }
        hold_item_internal(ctx, sys, errors, item, pe_target)?;
    }

    if attach {
        sys.screen
            .transition_to_overworld(ctx, &mut sys.overworld, false, errors)?;
        sys.overworld.set_held_attached(true);
    }

    Ok(())
}

/// Internal helper for holding items
///
/// Must be in inventory to call this
pub fn hold_item_internal(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    item: &cir::ItemSelectSpec,
    pe_target: Option<&cir::ItemSelectSpec>,
) -> Result<(), processor::Error> {
    let name = &item.name;
    let meta = item.meta.as_ref();
    let memory = ctx.cpu().proc.memory();
    let inventory = sys.screen.current_screen().as_inventory().unwrap();
    let mut remaining = super::convert_amount(item.amount, item.span, errors, false, || {
        // holding always decrease value instead of using slot
        Ok(inventory.get_amount(name, meta, sim::CountingMethod::Value, memory)?)
    })?;

    let mut check_for_extra_error = true;
    loop {
        if ctx.is_aborted() {
            return Ok(());
        }
        if remaining.is_done(item.span, errors, "HOLD") {
            break;
        }
        let inventory = sys.screen.current_screen().as_inventory().unwrap();
        let memory = ctx.cpu().proc.memory();
        let position = inventory.select(
            name,
            meta,
            Some(1), // must have at least 1 to hold
            memory,
            item.span,
            errors,
        )?;
        let Some((tab, slot)) = position else {
            break;
        };
        match inventory.get(tab, slot) {
            sim::ScreenItemState::Normal(item_ptr) => {
                // the item to hold must be a material
                mem! { memory: let t = *(&item_ptr->mType); };
                if t != 7 {
                    errors.push(sim_error!(item.span, NotHoldable));
                    check_for_extra_error = false;
                    break;
                }
            }
            _ => {
                // the item to hold must be non empty and non translucent
                errors.push(sim_error!(item.span, InvalidItemTarget));
                check_for_extra_error = false;
                break;
            }
        }
        // check if we are holding a PE activated slot
        let Some((tab, slot)) =
            super::change_to_pe_target_if_need(pe_target, inventory, memory, tab, slot, errors)?
        else {
            check_for_extra_error = false;
            break;
        };

        if !linker::can_hold_another_item(ctx.cpu())? {
            errors.push(sim_error!(item.span, CannotHoldMore));
            return Ok(());
        }

        let correct_slot = inventory.corrected_slot(tab, slot);
        super::trash_item_wrapped(ctx.cpu(), sys, tab as i32, correct_slot)?;

        let inventory = sys.screen.current_screen_mut().as_inventory_mut().unwrap();
        inventory.update(tab, slot, None, ctx.cpu().proc.memory())?;
        remaining.sub(1);
    }
    if check_for_extra_error {
        let memory = ctx.cpu().proc.memory();
        let inventory = sys.screen.current_screen().as_inventory().unwrap();
        let result = remaining.check(item.span, errors, || {
            inventory.get_amount(name, meta, sim::CountingMethod::Value, memory)
        })?;
        super::check_remaining!(result, errors, item.span);
    }

    Ok(())
}

/// Unhold the items currently being held (i.e. put them back to pouch)
pub fn unhold(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
) -> Result<(), processor::Error> {
    // can only unhold while in the overworld or in the inventory
    if !sys.screen.current_screen().is_inventory_or_overworld() {
        super::switch_to_overworld_or_stop!(ctx, sys, errors, "UNHOLD");
    }
    unhold_internal(ctx, sys)
}

/// Unhold without screen checks
pub fn unhold_internal(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
) -> Result<(), processor::Error> {
    // we don't check if the player is currently holding anything,
    // as it requires reading PMDM to check if it's holding in the pause menu
    linker::unhold_items(ctx.cpu())?;
    sys.overworld.delete_held_items();
    sys.screen.holding_in_inventory = false;
    Ok(())
}
