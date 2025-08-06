use blueflame::game;
use blueflame::linker;
use blueflame::processor::{self, Cpu2};
use skybook_parser::cir;

use crate::error::{ErrorReport, sim_error};
use crate::sim;

/// Add items to pouch by eventually calling itemGet or cookItemGet
pub fn get_items(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    items: &[cir::ItemSpec],
    pause_after: bool,
    accurate: bool,
) -> Result<(), processor::Error> {
    super::switch_to_overworld_or_stop!(ctx, sys, errors, "GET");
    let should_drop = super::predrop_items!(ctx, sys, errors, "GET");

    for item in items {
        get_item_internal(ctx, sys, item, errors, accurate)?;
        if ctx.is_aborted() {
            break;
        }
    }

    sys.overworld.despawn_items();
    super::handle_predrop_result(ctx, sys, errors, pause_after, should_drop, "GET")?;

    Ok(())
}

/// Buying items from shop
///
/// This is very similar to `get`, the only difference being
/// handling the screen.
///
/// Overworld screen is requred unless `:same-dialog` -
/// in which case it will try to auto switch to Shop Buying screen.
pub fn buy_items(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    items: &[cir::ItemSpec],
    pause_after: bool,
    accurate: bool,
    same_dialog: bool,
) -> Result<(), processor::Error> {
    // same-dialog option is only allowed if you are currently in
    // a shop screen
    if same_dialog {
        if !sys.screen.current_screen().is_shop() {
            errors.push(sim_error!(ctx.span, NotRightScreen));
            return Ok(());
        }
        if !sys
            .screen
            .transition_to_shop_buying(ctx, &mut sys.overworld, false, errors)?
        {
            log::error!("failed to transition to buying screen for BUY");
            return Ok(());
        }
    }
    // if we are not using same-dialog option and not in buying screen already
    // (by the switch above, assume it's overworld buying)
    let should_drop = if sys.screen.current_screen().is_shop_buying() {
        // already in the dialog, and drop is handled by the dialog
        false
    } else {
        super::switch_to_overworld_or_stop!(ctx, sys, errors, "BUY");
        // hold-attach state auto-drop
        super::predrop_items!(ctx, sys, errors, "BUY")
    };

    for item in items {
        get_item_internal(ctx, sys, item, errors, accurate)?;
        if ctx.is_aborted() {
            break;
        }
    }

    if sys.screen.current_screen().is_shop_buying() {
        if pause_after {
            // forcefully open inventory screen directly
            *sys.screen.current_screen_mut() =
                sim::Screen::Inventory(sim::PouchScreen::open(ctx.cpu(), false)?);
            log::debug!("inventory screen opened forcefully");
        }
        if should_drop {
            sys.screen.set_remove_held_after_dialog();
        }
    } else {
        // in overworld
        sys.overworld.despawn_items();
        super::handle_predrop_result(ctx, sys, errors, pause_after, should_drop, "BUY")?;
    }

    Ok(())
}

fn get_item_internal(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    item: &cir::ItemSpec,
    errors: &mut Vec<ErrorReport>,
    accurate: bool,
) -> Result<(), processor::Error> {
    let amount = item.amount;
    let name = &item.name;
    let is_cook_item = name.starts_with("Item_Cook_");
    let meta = item.meta.as_ref();
    if is_cook_item {
        // cannot optimize cook items
        for _ in 0..amount {
            if ctx.is_aborted() {
                return Ok(());
            }
            if linker::cannot_get_item(ctx.cpu(), name, 1)? {
                errors.push(sim_error!(item.span, CannotGetMore));
                return Ok(());
            }
            linker::get_cook_item(
                ctx.cpu(),
                name,
                meta.map(|m| m.ingredients.as_slice()).unwrap_or(&[]),
                meta.and_then(|m| m.life_recover_f32()),
                meta.and_then(|m| m.effect_duration),
                meta.and_then(|m| m.sell_price),
                meta.and_then(|m| m.effect_id),
                meta.and_then(|m| m.effect_level),
            )?;
        }
        return Ok(());
    }

    // getting non-cook item
    let modifier = sim::util::modifier_from_meta(meta);
    let meta_value = meta.and_then(|m| m.value);
    let is_weapon = sim::util::name_is_weapon(name);

    let can_optimize = !accurate
    && !is_weapon
    && meta_value.is_none()
    && game::can_stack(name)
    // cannot optimize arrow, since it needs to be auto-equipped using the non value get call
    // (this can be improved if we manually check auto-equip)
    && game::get_pouch_item_type(name) != 2;
    if can_optimize {
        if linker::cannot_get_item(ctx.cpu(), name, amount as i32)? {
            errors.push(sim_error!(item.span, CannotGetMore));
        } else {
            // optimize into one call with a value
            linker::get_item(ctx.cpu(), name, Some(amount as i32), modifier)?;
        }
        return Ok(());
    }
    for _ in 0..amount {
        if ctx.is_aborted() {
            return Ok(());
        }
        if linker::cannot_get_item(ctx.cpu(), name, 1)? {
            errors.push(sim_error!(item.span, CannotGetMore));
            return Ok(());
        }
        // need to generate new actor because it's not already
        // on the ground
        let auto_equip_type = super::AutoEquipType::NewItem {
            name,
            value: meta_value,
            modifier,
        };
        super::get_item_with_auto_equip(ctx.cpu(), sys, is_weapon, auto_equip_type)?;
    }
    Ok(())
}
