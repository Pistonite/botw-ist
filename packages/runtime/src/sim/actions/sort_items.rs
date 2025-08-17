use blueflame::game::PouchCategory;
use blueflame::linker;
use blueflame::processor::{self, Cpu2};
use skybook_parser::cir;

use crate::error::{ErrorReport, sim_error};
use crate::sim;

/// Sort an item category
///
/// Inventory screen is required unless `:same-dialog`,
/// in which case it will error unless already in buying or selling screen,
/// and will auto switch to selling
pub fn sort_items(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    category: cir::Category,
    mut times: usize,
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
            .transition_to_shop_selling(ctx, &mut sys.overworld, false, errors)?
        {
            cu::error!("failed to transition to selling screen for SORT");
            return Ok(());
        }
    } else {
        super::switch_to_inventory_or_stop!(ctx, sys, errors, "SORT");
    }

    let pouch_category = sim::util::category_to_pouch_category(category);
    if sys.screen.current_screen().is_shop() {
        if !matches!(
            pouch_category,
            PouchCategory::Armor | PouchCategory::Material | PouchCategory::Food
        ) {
            errors.push(sim_error!(ctx.span, CannotSortCategory));
            return Ok(());
        }
    }

    if !accurate {
        times = cap_times(times);
    }

    for _ in 0..times {
        linker::delete_removed_items(ctx.cpu())?;
        linker::sort_items(ctx.cpu(), pouch_category)?;
        // update inventory display
        if let Some(inventory) = sys.screen.current_screen_mut().as_inventory_mut() {
            inventory.update_all_items(ctx.cpu(), false)?;
        }
    }

    Ok(())
}

fn cap_times(times: usize) -> usize {
    if times.is_multiple_of(2) {
        times.min(4)
    } else {
        times.min(5)
    }
}
