use blueflame::processor::{self, Cpu2};
use skybook_parser::cir;

use crate::error::sim_error;
use crate::error::ErrorReport;
use crate::sim;

use super::{switch_to_inventory_or_stop};

/// Activate PE
pub fn entangle_item(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    item: &cir::ItemSelectSpec,
) -> Result<(), processor::Error> {
    // note: normally it might not be super useful to activate
    // PE while already holding items (since you are locked to material)
    // however, we don't check for that here

    switch_to_inventory_or_stop!(ctx, sys, errors, "ENTANGLE");

    let inventory = sys.screen.current_screen_mut().as_inventory_mut().unwrap();
    // find the slot to activate
    let position = inventory.select(
        &item.name, item.meta.as_ref(), None, ctx.cpu().proc.memory(),
        item.span, errors)?;

    let Some((tab, slot)) = position else {
        errors.push(sim_error!(item.span, CannotFindItem));
        return Ok(());
    };

    // you can target empty slots, so we don't check at all
    // if the tab/slot is valid
    inventory.activate_pe(tab, slot);

    Ok(())
}
