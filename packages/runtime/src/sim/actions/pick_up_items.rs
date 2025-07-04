use blueflame::linker;
use blueflame::processor::{self, Cpu2};
use skybook_parser::cir;

use crate::error::{ErrorReport, sim_error};
use crate::sim;

use super::{
    ItemSelectCheck, convert_amount, handle_predrop_result, predrop_items,
    switch_to_overworld_or_stop,
};

/// Pick up items from the ground (overworld)
pub fn pick_up_items(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    items: &[cir::ItemSelectSpec],
    pause_after: bool,
) -> Result<(), processor::Error> {
    switch_to_overworld_or_stop!(ctx, sys, errors, "PICKUP");
    let should_drop = predrop_items!(ctx, sys, errors, "PICKUP");

    'outer: for item in items {
        let name = &item.name;
        let meta = item.meta.as_ref();
        let mut remaining = convert_amount(item.amount, item.span, errors, false, || {
            Ok(sys.overworld.get_ground_amount(name, meta))
        })?;
        loop {
            if ctx.is_aborted() {
                break 'outer;
            }
            if remaining.is_done() {
                break;
            }
            // find the item on the ground
            let Some(handle) = sys
                .overworld
                .ground_select_mut(name, meta, item.span, errors)
            else {
                break;
            };
            // TODO: cannotGetItem check?
            let actor = handle.remove();
            linker::get_item(ctx.cpu(), &actor.name, Some(actor.value), actor.modifier)?;
            remaining.sub(1);
        }
        match remaining.check(item.span, errors, || {
            Ok(sys.overworld.get_ground_amount(name, meta))
        })? {
            ItemSelectCheck::NeverFound => {
                errors.push(sim_error!(item.span, CannotFindGroundItem));
            }
            ItemSelectCheck::NeedMore(n) => {
                errors.push(sim_error!(item.span, CannotFindGroundItemNeedMore(n)));
            }
            _ => {}
        }
    }
    handle_predrop_result(ctx, sys, errors, pause_after, should_drop, "PICKUP")?;
    sys.overworld.despawn_items();

    Ok(())
}
