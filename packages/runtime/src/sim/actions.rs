use blueflame::linker;
use blueflame::processor::{self, Cpu2};
use skybook_parser::cir;

use crate::error::{ErrorReport, sim_error};
use crate::sim;

use super::util;

/// Add items to pouch by eventually calling itemGet or cookItemGet
pub fn get_items(
    ctx: &mut sim::Context<&mut Cpu2>,
    items: &[cir::ItemSpec],
) -> Result<(), processor::Error> {
    // TODO: cannot get while holding items
    'outer: for item in items {
        let amount = item.amount;
        let item = &item.item;
        let is_cook_item = item.is_cook_item();
        let meta = item.meta.as_ref();
        for _ in 0..amount {
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

    Ok(())
}

/// Hold items in pouch
pub fn hold_items(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    items: &[cir::ItemSelectSpec],
) -> Result<(), processor::Error> {
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
                        errors.push(sim_error!(&item.span, CannotFindItem));
                        None
                    }
                };
            }
            let Some((tab, slot)) = position else {
                // can no longer find the item, stop further attempts
                break;
            };

            if !linker::can_hold_another_item(ctx.cpu())? {
                errors.push(sim_error!(&item.span, CannotHoldMore));
                break 'outer;
            }

            linker::trash_item(ctx.cpu(), tab as i32, slot as i32)?;

            let memory = ctx.cpu().proc.memory();
            // re-search the item if the item slot is used up
            if inventory.update(tab, slot, None, memory)?
                || inventory.get_value(tab, slot, memory)?.unwrap_or_default() < 1
            {
                position = None;
            }
        }
    }

    Ok(())
}
