use blueflame::linker;
use blueflame::processor::{self, Cpu2};
use skybook_parser::cir;

use crate::error::{ErrorReport, sim_error, sim_warning};
use crate::sim;

use super::util;

/// Add items to pouch by eventually calling itemGet or cookItemGet
pub fn get_items(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    items: &[cir::ItemSpec],
    pause_after: bool,
) -> Result<(), processor::Error> {
    // first, must be in the overworld to get items
    sys.screen
        .transition_to_overworld(ctx, &mut sys.overworld, false, errors)?;
    // ensure player is not holding items
    let should_drop = match sys.overworld.predrop_for_action(ctx.span, errors) {
        sim::OverworldPreDropResult::Holding => {
            // cannot get while holding, stop
            return Ok(());
        }
        sim::OverworldPreDropResult::AutoDrop => true,
        sim::OverworldPreDropResult::Ok => false,
    };

    'outer: for item in items {
        let amount = item.amount;
        let item = &item.item;
        let is_cook_item = item.is_cook_item();
        let meta = item.meta.as_ref();
        for _ in 0..amount {
            // TODO: cannotGetItem check?
            if is_cook_item {
                linker::get_cook_item(
                    ctx.inner,
                    &item.actor,
                    meta.map(|m| m.ingredients.as_slice()).unwrap_or(&[]),
                    meta.and_then(|m| m.life_recover_f32()),
                    meta.and_then(|m| m.effect_duration),
                    meta.and_then(|m| m.sell_price),
                    meta.and_then(|m| m.effect_id),
                    meta.and_then(|m| m.effect_level),
                )?;
                continue;
            };
            let modifier = util::modifier_from_meta(meta);
            linker::get_item(ctx.inner, &item.actor, meta.and_then(|m| m.value), modifier)?;

            if ctx.is_aborted() {
                break 'outer;
            }
        }
    }

    if pause_after {
        // open pause menu and delay drop
        sys.screen
            .transition_to_inventory(ctx, &mut sys.overworld, false, errors)?;
        if should_drop {
            sys.screen.set_remove_held_after_dialog();
        }
    } else if should_drop {
        log::debug!("removing held items on auto-drop cleanup in get command");
        linker::remove_held_items(ctx.cpu())?;
        sys.overworld.drop_held_items();
    }

    Ok(())
}

/// Hold items in pouch
pub fn hold_items(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    items: &[cir::ItemSelectSpec],
    attached: bool,
) -> Result<(), processor::Error> {
    // must be in inventory to hold items
    sys.screen
        .transition_to_inventory(ctx, &mut sys.overworld, false, errors)?;
    let inventory = sys.screen.current_screen_mut().as_inventory_mut().unwrap();
    'outer: for item in items {
        // TODO - check if item is material, prompt entanglement, etc
        let amount = item.amount;

        let mut position = None;
        for _ in 0..amount {
            if position.is_none() {
                // try to find this item
                log::debug!("finding in {:#?}", inventory.tabs);
                let search_result = inventory.select(
                    &item.item,
                    Some(1),
                    ctx.cpu().proc.memory(),
                    item.span,
                    errors,
                );
                log::debug!("result {search_result:?}");
                position = match search_result {
                    Ok(Some(x)) => Some(x),
                    _ => {
                        errors.push(sim_error!(item.span, CannotFindItem));
                        None
                    }
                };
            }
            let Some((tab, slot)) = position else {
                // can no longer find the item, stop further attempts
                break;
            };

            if !linker::can_hold_another_item(ctx.cpu())? {
                errors.push(sim_error!(item.span, CannotHoldMore));
                break 'outer;
            }

            linker::trash_item(
                ctx.cpu(),
                tab as i32,
                inventory.get_corrected_slot(tab, slot),
            )?;

            let memory = ctx.cpu().proc.memory();
            // re-search the item if the item slot is used up
            if inventory.update(tab, slot, None, memory)?
                || inventory.get_value(tab, slot, memory)?.unwrap_or_default() < 1
            {
                position = None;
            }
        }
    }

    if attached {
        sys.screen
            .transition_to_overworld(ctx, &mut sys.overworld, false, errors)?;
        sys.overworld.set_held_attached(true);
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
        sys.screen
            .transition_to_overworld(ctx, &mut sys.overworld, false, errors)?;
    }
    // we don't check if the player is currently holding anything,
    // as it requires reading PMDM to check if it's holding in the pause menu
    linker::unhold_items(ctx.cpu())?;
    sys.overworld.delete_held_items();
    Ok(())
}

/// Drop items currently held to the ground
pub fn drop_held(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
) -> Result<(), processor::Error> {
    sys.screen
        .transition_to_overworld(ctx, &mut sys.overworld, false, errors)?;
    if !sys.overworld.is_holding() {
        errors.push(sim_warning!(ctx.span, NotHolding));
        return Ok(());
    }
    linker::remove_held_items(ctx.cpu())?;
    sys.overworld.drop_held_items();

    Ok(())
}
