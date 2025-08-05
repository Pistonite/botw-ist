use blueflame::game::PouchItem;
use blueflame::linker;
use blueflame::memory::{self, Ptr};
use blueflame::processor::{self, Cpu2};
use skybook_parser::cir;

use crate::error::ErrorReport;
use crate::sim;

/// Use a "forced pouch screen" to find item in the pouch
pub fn find_single_item_target(
    ctx: &mut sim::Context<&mut Cpu2>,
    errors: &mut Vec<ErrorReport>,
    item: &cir::ItemSelectSpec,
) -> Result<Option<Ptr![PouchItem]>, memory::Error> {
    // open a temporary inventory that allows access to items
    let inventory = sim::PouchScreen::open_no_exec(ctx.cpu().proc, true)?;

    let matcher = &item.matcher;

    let mut new_errors = vec![];
    let Some((tab, slot)) = inventory.select(matcher, ctx.cpu().proc.memory(), &mut new_errors)?
    else {
        errors.extend(new_errors);
        return Ok(None);
    };
    // we eat all the error/warnings if the position is found successfully
    // and let the command add their own errors when they check the returned
    // Option here
    let item = inventory.get(tab, slot).as_ref().copied();
    Ok(item)
}

pub fn fix_inventory_state(ctx: &mut sim::Context<&mut Cpu2>) -> Result<(), processor::Error> {
    linker::update_inventory_info(ctx.cpu())?;
    linker::update_list_heads(ctx.cpu())?;
    Ok(())
}
