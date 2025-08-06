use blueflame::linker;
use blueflame::processor::{self, Cpu2};
use skybook_parser::cir;

use crate::error::{ErrorReport, sim_error};
use crate::sim;

/// Pick up items from the ground (overworld)
pub fn pick_up_items(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    items: &[cir::ItemSelectSpec],
    pause_after: bool,
) -> Result<(), processor::Error> {
    super::switch_to_overworld_or_stop!(ctx, sys, errors, "PICKUP");
    let should_drop = super::predrop_items!(ctx, sys, errors, "PICKUP");

    'outer: for item in items {
        if ctx.is_aborted() {
            break 'outer;
        }
        pick_up_item_internal(ctx, sys, errors, item)?;
    }
    super::handle_predrop_result(ctx, sys, errors, pause_after, should_drop, "PICKUP")?;
    sys.overworld.despawn_items();

    Ok(())
}

/// Internal handler for picking up items, must be in overworld to call.
pub fn pick_up_item_internal(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    item: &cir::ItemSelectSpec,
) -> Result<(), processor::Error> {
    let matcher = &item.matcher;
    let span = matcher.span;
    let mut remaining = super::convert_amount(item.amount, span, errors, false, |_| {
        Ok(sys.overworld.get_ground_amount(matcher))
    })?;
    loop {
        if ctx.is_aborted() {
            return Ok(());
        }
        if remaining.is_done(span, errors, "PICKUP") {
            break;
        }
        // find the item on the ground
        let Some(handle) = sys.overworld.ground_select_mut(matcher, errors) else {
            break;
        };
        if linker::cannot_get_item(ctx.cpu(), &handle.actor().name, 1)? {
            errors.push(sim_error!(span, CannotGetMore));
            continue;
        }
        let actor = handle.remove();
        super::get_item_with_auto_equip(
            ctx.cpu(),
            sys,
            true,
            super::AutoEquipType::Spawned(actor)
            // &actor.name,
            // Some(actor.value),
            // actor.modifier,
        )?;
        remaining.sub(1);
    }
    let result = remaining.check(span, errors, |_| {
        Ok(sys.overworld.get_ground_amount(matcher))
    })?;
    super::check_remaining_ground!(result, errors, span);

    Ok(())
}
