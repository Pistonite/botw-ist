use blueflame::linker;
use blueflame::memory::mem;
use blueflame::processor::{self, Cpu2};
use skybook_parser::cir;

use crate::error::{ErrorReport, sim_error, sim_warning};
use crate::sim;

use super::{
    ItemSelectCheck, check_not_holding_in_inventory, convert_amount, drop_held_items,
    switch_to_inventory_or_stop, switch_to_overworld_or_stop,
};

// /// Different modes for actions that involve holding
// pub enum HoldAction {
//     /// Just holding (`hold` command with items)
//     Hold,
//     /// Just holding, with the smuggle thing (`:smug hold`)
//     HoldAttach,
//     /// Hold items and keep dropping (`drop` command with items)
//     HoldDrop,
//     /// Figure out how many needs to be operated on, hold them, drop them, then pick them up,
//     /// repeat for all items (`dnp` command with items)
//     HoldDropPickup,
//     /// Hold items (up to 5), then cook (`cook` command with items)
//     HoldCook,
// }

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
    switch_to_inventory_or_stop!(ctx, sys, errors, "HOLD");
    // start holding in inventory
    sys.screen.holding_in_inventory = true;
    let inventory = sys.screen.current_screen_mut().as_inventory_mut().unwrap();
    'outer: for item in items {
        let name = &item.name;
        let meta = item.meta.as_ref();
        let memory = ctx.cpu().proc.memory();
        let mut remaining = convert_amount(item.amount, item.span, errors, false, || {
            // holding always decrease value instead of using slot
            inventory.get_amount(name, meta, sim::CountingMethod::Value, memory)
        })?;

        let mut check_for_extra_error = true;
        loop {
            if ctx.is_aborted() {
                break 'outer;
            }
            if remaining.is_done() {
                break;
            }
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
            let (tab, slot) = if let Some(target) = pe_target
                && inventory.is_pe_activated_slot(tab, slot)
            {
                // check if the target is allowed by current activated PE
                let Some((target_tab, target_slot)) = inventory.select(
                    &target.name,
                    target.meta.as_ref(),
                    None,
                    ctx.cpu().proc.memory(),
                    target.span,
                    errors,
                )?
                else {
                    errors.push(sim_error!(target.span, CannotFindPromptTarget));
                    check_for_extra_error = false;
                    break;
                };
                if !inventory.is_pe_activated_slot(target_tab, target_slot) {
                    errors.push(sim_error!(target.span, InvalidPromptTarget));
                    check_for_extra_error = false;
                    break;
                }
                let target_slot = inventory.get_pe_target_slot(target_tab, target_slot, false);
                (target_tab, target_slot)
            } else {
                // use the normal position if not PE
                (tab, slot)
            };

            if !linker::can_hold_another_item(ctx.cpu())? {
                errors.push(sim_error!(item.span, CannotHoldMore));
                break 'outer;
            }

            linker::trash_item(ctx.cpu(), tab as i32, inventory.corrected_slot(tab, slot))?;

            inventory.update(tab, slot, None, ctx.cpu().proc.memory())?;
            remaining.sub(1);
        }
        if check_for_extra_error {
            let memory = ctx.cpu().proc.memory();
            match remaining.check(item.span, errors, || {
                inventory.get_amount(name, meta, sim::CountingMethod::Value, memory)
            })? {
                ItemSelectCheck::NeverFound => {
                    errors.push(sim_error!(item.span, CannotFindItem));
                }
                ItemSelectCheck::NeedMore(n) => {
                    errors.push(sim_error!(item.span, CannotFindItemNeedMore(n)));
                }
                _ => {}
            }
        }
    }

    if attach {
        sys.screen
            .transition_to_overworld(ctx, &mut sys.overworld, false, errors)?;
        sys.overworld.set_held_attached(true);
    }

    Ok(())
}

/// Drop items
///
/// If no items are specified, the action becomes dropping held items in the overworld
///
/// For equipments, it uses the drop prompt, and stays in inventory
/// For holdable items, it holds it first, then close inventory and drop
///
/// If `overworld`, equipments are dropped directly from overworld (i.e.
/// shocked or displayed)
pub fn drop_items(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    items: &[cir::ItemSelectSpec],
    pe_target: Option<&cir::ItemSelectSpec>,
    pick_up: bool,
    _overworld: bool,
) -> Result<(), processor::Error> {
    if items.is_empty() {
        log::debug!("no items specified for DROP, dropping held items ");
        switch_to_overworld_or_stop!(ctx, sys, errors, "DROP");
        if !sys.overworld.is_holding() {
            errors.push(sim_warning!(ctx.span, NotHolding));
            return Ok(());
        }
        drop_held_items(ctx, sys, "DROP")?;
        return Ok(());
    }
    // to drop items in the inventory, if you are holding,
    // then you are locked to holding items and cannot drop
    check_not_holding_in_inventory!(ctx, sys, errors, "DROP");
    // since the handling for different types of items are different,
    // we need to do that inside the loop
    for item in items {
        if sim::util::name_spec_is_weapon(&item.name) {
            log::error!("TODO: drop weapon");
            errors.push(sim_error!(item.span, Unimplemented));
            continue;
        }
        if !sim::util::name_spec_can_drop(&item.name) {
            errors.push(sim_error!(item.span, NotDroppable));
            continue;
        }
        // if holding in the overworld, drop those first
        if sys.overworld.is_holding() {
            log::debug!(
                "dropping currently held items in DROP command before processing other items"
            );
            switch_to_overworld_or_stop!(ctx, sys, errors, "DROP");
            drop_held_items(ctx, sys, "DROP")?;
        }
        // must be in inventory to hold materials
        switch_to_inventory_or_stop!(ctx, sys, errors, "DROP");
        // for items, hold 1 at a time and drop, and we must know
        // how many there are
        let memory = ctx.cpu().proc.memory();
        let inventory = sys.screen.current_screen().as_inventory().unwrap();
        let count_fn = || {
            inventory.get_amount(
                &item.name,
                item.meta.as_ref(),
                sim::CountingMethod::Value,
                memory,
            )
        };
        let amount = convert_amount(item.amount, item.span, errors, true, count_fn)?
            .count()
            .unwrap_or_default();
        log::debug!("need to drop {amount}");
        let mut hold_spec = item.clone();
        hold_spec.amount = cir::AmountSpec::Num(1);
        let hold_item = &[hold_spec];
        // remove the position spec when picking up
        let pick_up_spec = if pick_up {
            let mut x = item.clone();
            if let Some(x) = x.meta.as_mut() {
                x.position = None;
            }
            Some(x)
        } else {
            None
        };
        for _ in 0..amount {
            hold_items(ctx, sys, errors, hold_item, pe_target, false)?;
            // need to be in overworld to drop
            switch_to_overworld_or_stop!(ctx, sys, errors, "DROP");
            if !sys.overworld.is_holding() {
                log::debug!("failed to hold, stopping");
                errors.push(sim_error!(item.span, OperationNotComplete));
                break;
            }
            drop_held_items(ctx, sys, "DROP")?;
            if let Some(spec) = &pick_up_spec {
                let Some(handle) = sys.overworld.ground_select_mut(
                    &spec.name,
                    spec.meta.as_ref(),
                    item.span,
                    errors,
                ) else {
                    errors.push(sim_error!(item.span, CannotFindGroundItem));
                    continue;
                };
                // TODO: cannotGetItem check?
                let actor = handle.remove();
                linker::get_item(ctx.cpu(), &actor.name, Some(actor.value), actor.modifier)?;
            }
        }
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
        switch_to_overworld_or_stop!(ctx, sys, errors, "UNHOLD");
    }
    // we don't check if the player is currently holding anything,
    // as it requires reading PMDM to check if it's holding in the pause menu
    linker::unhold_items(ctx.cpu())?;
    sys.overworld.delete_held_items();
    sys.screen.holding_in_inventory = false;
    Ok(())
}
