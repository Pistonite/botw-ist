use blueflame::game::PouchItem;
use blueflame::memory::{self, Ptr};
use blueflame::processor::{self, Cpu2};
use skybook_parser::cir;

use crate::error::{ErrorReport, sim_error};
use crate::sim;

/// Handle the `!swap ITEM1 and ITEM2` supercommand
pub fn swap_items(
    ctx: &mut sim::Context<&mut Cpu2>,
    errors: &mut Vec<ErrorReport>,
    item1: &cir::ItemSelectSpec,
    item2: &cir::ItemSelectSpec,
) -> Result<(), processor::Error> {
    // find the items
    let Some(item1_ptr) = super::find_single_item_target(ctx, errors, item1)? else {
        errors.push(sim_error!(item1.matcher.span, CannotFindItem));
        return Ok(());
    };
    let Some(item2_ptr) = super::find_single_item_target(ctx, errors, item2)? else {
        errors.push(sim_error!(item2.matcher.span, CannotFindItem));
        return Ok(());
    };
    swap_item_internal(ctx, item1_ptr, item2_ptr)?;
    super::fix_inventory_state(ctx)
}

/// Swap 2 item nodes
fn swap_item_internal(
    ctx: &mut sim::Context<&mut Cpu2>,
    item1: Ptr![PouchItem],
    item2: Ptr![PouchItem],
) -> Result<(), memory::Error> {
    let m = ctx.cpu().proc.memory_mut();
    let item1_node = Ptr!(&item1->mListNode);
    let item2_node = Ptr!(&item2->mListNode);
    item1_node.swap(item2_node, m)?;

    Ok(())
}
