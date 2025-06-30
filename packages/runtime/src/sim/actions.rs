use blueflame::game;
use blueflame::game::singleton_instance;
use blueflame::linker;
use blueflame::memory::Ptr;
use blueflame::memory::mem;
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
    if !sys
        .screen
        .transition_to_overworld(ctx, &mut sys.overworld, false, errors)?
    {
        log::warn!("failed to auto-switch to overworld for GET");
        return Ok(());
    }
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
            if ctx.is_aborted() {
                break 'outer;
            }
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
    if !sys
        .screen
        .transition_to_inventory(ctx, &mut sys.overworld, false, errors)?
    {
        log::warn!("failed to auto-switch to inventory for HOLD");
        return Ok(());
    }
    let inventory = sys.screen.current_screen_mut().as_inventory_mut().unwrap();
    'outer: for item in items {
        // TODO - check if item is material, prompt entanglement, etc
        let amount = if item.amount < 0 {
            i64::MAX
        } else {
            item.amount
        };

        let mut position = None;
        for _ in 0..amount {
            if ctx.is_aborted() {
                break 'outer;
            }
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

            linker::trash_item(ctx.cpu(), tab as i32, inventory.corrected_slot(tab, slot))?;

            let memory = ctx.cpu().proc.memory();
            // re-search the item if the item slot is used up
            if inventory.update(tab, slot, None, memory)?
                || inventory.value_at(tab, slot, memory)?.unwrap_or_default() < 1
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
    if !sys.screen.current_screen().is_inventory_or_overworld()
        && !sys
            .screen
            .transition_to_overworld(ctx, &mut sys.overworld, false, errors)?
    {
        log::warn!("failed to auto-switch to overworld for UNHOLD");
        return Ok(());
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
    if !sys
        .screen
        .transition_to_overworld(ctx, &mut sys.overworld, false, errors)?
    {
        log::warn!("failed to auto-switch to overworld for DROP");
        return Ok(());
    }
    if !sys.overworld.is_holding() {
        errors.push(sim_warning!(ctx.span, NotHolding));
        return Ok(());
    }
    linker::remove_held_items(ctx.cpu())?;
    sys.overworld.drop_held_items();

    Ok(())
}

pub fn sell_items(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    items: &[cir::ItemSelectSpec],
) -> Result<(), processor::Error> {
    // must be in shop to sell items
    if !sys
        .screen
        .transition_to_shop_selling(ctx, &mut sys.overworld, false, errors)?
    {
        log::warn!("failed to auto-switch to shop for SELL");
        return Ok(());
    }

    let shop = sys.screen.current_screen_mut().as_selling_mut().unwrap();
    'outer: for item in items {
        let mut remaining_amount = if item.amount < 0 {
            None
        } else {
            Some(item.amount as i32)
        };
        // if any item is sold - used to suppress warning when "sell all"
        let mut sold = false;
        loop {
            if ctx.is_aborted() {
                break 'outer;
            }
            if let Some(x) = remaining_amount
                && x <= 0
            {
                break; // done
            }

            let m = ctx.cpu().proc.memory();

            // find the item to sell
            let search_result = shop.select(&item.item, None, m, item.span, errors);

            let (tab, slot) = match search_result {
                Ok(Some(x)) => x,
                _ => {
                    // cannot find the item
                    if !sold || remaining_amount.is_some() {
                        errors.push(sim_error!(item.span, CannotFindSellableItem));
                    }
                    break;
                }
            };

            let Some(item_ptr) = shop.ptr_at(tab, slot) else {
                log::warn!("shop.select() succeeded but cannot get the item pointer");
                errors.push(sim_error!(item.span, CannotFindSellableItem));
                break;
            };

            let item_name = Ptr!(&item_ptr->mName).cstr(m)?.load_utf8_lossy(m)?;
            if !game::can_sell(&item_name) {
                log::debug!("{item_name} is not sellable, skipping selling this item");
                errors.push(sim_error!(item.span, CannotFindSellableItem));
                break;
            }

            let can_stack = game::can_stack(&item_name);
            mem! { m: let item_value = *(&item_ptr->mValue) };
            let (tab, slot, item_ptr, amount_to_sell) = if can_stack && item_value == 0 {
                // you cannot sell a stackable with 0 - so we can't sell this stack
                // we will try to find another stack with at least 1
                //
                // it's possible to sell corrupted 0 stack or Armor with value 0,
                // which is why we must find with no value filter above
                let search_result = shop.select(&item.item, Some(1), m, item.span, errors);
                let (tab, slot) = match search_result {
                    Ok(Some(x)) => x,
                    _ => {
                        // cannot find the item
                        if !sold || remaining_amount.is_some() {
                            errors.push(sim_error!(item.span, CannotFindSellableItem));
                        }
                        break;
                    }
                };
                // re-get the item pointer using the new position
                let Some(item_ptr) = shop.ptr_at(tab, slot) else {
                    log::warn!("shop.select() succeeded but cannot get the item pointer");
                    errors.push(sim_error!(item.span, CannotFindSellableItem));
                    break;
                };
                mem! { m: let value = *(&item_ptr->mValue) };
                let amount_to_sell = match remaining_amount {
                    Some(x) => x.min(value),
                    None => value, // sell all
                };

                // lower bound to 1 to prevent infinite loop just in case
                (tab, slot, item_ptr, amount_to_sell.max(1))
            } else if can_stack {
                let amount_to_sell = match remaining_amount {
                    Some(x) => x.min(item_value),
                    None => item_value, // sell all
                };
                // we know neither remaining_amount nor item_value is 0
                (tab, slot, item_ptr, amount_to_sell)
            } else {
                // not stackable, the amount to sell doesn't matter when calling PMDM,
                // for command handling, we assume user wants to sell X *slots* of this item,
                // so using 1 as the amount
                (tab, slot, item_ptr, 1)
            };

            if let Some(x) = &mut remaining_amount {
                *x -= amount_to_sell;
            }
            sold = true;
            linker::sell_item(ctx.cpu(), item_ptr, amount_to_sell)?;

            shop.update(tab, slot, None, ctx.cpu().proc.memory())?;
        }
    }

    Ok(())
}

/// Forcefully remove items from the inventory, regardless of screen.
///
/// This is for backward compatibility with item removal in older versions
/// of simulator, and for quickly remove items (like `!remove 999 apples`)
///
/// This is very similar to `sell_items`, with the exceptions that:
/// - Items from the entire pouch are removable, including non-sellable items
/// - decrease value / remove slot depends on the type of the item
///   instead of CanStack tag. Arrow, Material, Food and KeyItem will be
///   removed based on value
/// - translucent slots are deleted and game data is synced
pub fn force_remove_item(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    items: &[cir::ItemSelectSpec],
) -> Result<(), processor::Error> {
    let mut temp_inv = sim::PouchScreen::open(ctx.cpu(), true)?;

    'outer: for item in items {
        let mut remaining_amount = if item.amount < 0 {
            None
        } else {
            Some(item.amount as i32)
        };

        loop {
            if ctx.is_aborted() {
                break 'outer;
            }
            if let Some(x) = remaining_amount
                && x <= 0
            {
                break; // done
            }
            let m = ctx.cpu().proc.memory();

            // find the item
            let search_result = temp_inv.select(&item.item, None, m, item.span, errors);

            let (tab, slot) = match search_result {
                Ok(Some(x)) => x,
                _ => {
                    // cannot find the item
                    if remaining_amount.is_some() {
                        errors.push(sim_error!(item.span, CannotFindItem));
                    }
                    break;
                }
            };

            let Some(item_ptr) = temp_inv.ptr_at(tab, slot) else {
                log::warn!(
                    "temporary_inventory.select() succeeded but cannot get the item pointer"
                );
                errors.push(sim_error!(item.span, CannotFindItem));
                break;
            };

            mem! { m:
                let item_type = *(&item_ptr->mType);
                let item_value = *(&item_ptr->mValue);
            };
            let should_decrease_stack_value = matches!(item_type, 7..=9);

            let (tab, slot, item_ptr, item_value) =
                if should_decrease_stack_value && item_value == 0 {
                    // cannot decrease this stack, find another stack with at least 1 value
                    let search_result = temp_inv.select(&item.item, Some(1), m, item.span, errors);
                    let (tab, slot) = match search_result {
                        Ok(Some(x)) => x,
                        _ => {
                            // cannot find the item
                            if remaining_amount.is_some() {
                                errors.push(sim_error!(item.span, CannotFindItem));
                            }
                            break;
                        }
                    };
                    // re-get the item pointer using the new position
                    let Some(item_ptr) = temp_inv.ptr_at(tab, slot) else {
                        log::warn!(
                            "temporary_inventory.select() succeeded but cannot get the item pointer"
                        );
                        errors.push(sim_error!(item.span, CannotFindItem));
                        break;
                    };
                    mem! { m: let value = *(&item_ptr->mValue) };

                    (tab, slot, item_ptr, value)
                } else {
                    (tab, slot, item_ptr, item_value)
                };

            if should_decrease_stack_value {
                let amount_to_decrease = match remaining_amount {
                    Some(x) => x.min(item_value),
                    None => item_value, // remove all
                };

                let new_value = item_value - amount_to_decrease.max(1);
                if new_value <= 0 {
                    mem! { (ctx.cpu().proc.memory_mut()):
                        *(&item_ptr->mValue) = 0;
                        *(&item_ptr->mInInventory) = false;
                    }
                } else {
                    mem! { (ctx.cpu().proc.memory_mut()):
                        *(&item_ptr->mValue) = new_value;
                    }
                }
                if let Some(x) = &mut remaining_amount {
                    *x -= amount_to_decrease;
                }
            } else {
                // delete the slot
                mem! { (ctx.cpu().proc.memory_mut()):
                    *(&item_ptr->mValue) = 0;
                    *(&item_ptr->mInInventory) = false;
                }
                if let Some(x) = &mut remaining_amount {
                    *x -= 1;
                }
            }

            linker::update_inventory_info(ctx.cpu())?;
            linker::update_list_heads(ctx.cpu())?;
            linker::save_to_game_data(ctx.cpu())?;

            temp_inv.update(tab, slot, None, ctx.cpu().proc.memory())?;
        }
    }

    linker::delete_removed_items(ctx.cpu())?;

    // since the inventory is changed, if the inventory screen is open,
    // force a full update
    if let Some(inv) = sys.screen.current_screen_mut().as_inventory_mut() {
        inv.update_all_items(ctx.cpu(), false)?;
    }

    Ok(())
}

/// Forcefully decrease mCount of list 1 and increase mCount of list 2.
///
/// This is for backward compatibility with slot breaking in older versions
/// of simulator
pub fn force_break_slot(
    ctx: &mut sim::Context<&mut Cpu2>,
    count: i32,
) -> Result<(), processor::Error> {
    let pmdm = singleton_instance!(pmdm(ctx.cpu().proc.memory()))?;
    mem! {(ctx.cpu().proc.memory_mut()):
        let count1 = *(&pmdm->mList1.mCount);
        let count2 = *(&pmdm->mList2.mCount);
        *(&pmdm->mList1.mCount) = count1 - count;
        *(&pmdm->mList2.mCount) = count2 + count;
    }
    Ok(())
}
