use blueflame::memory::Ptr;
use blueflame::processor::{self, Cpu2};
use blueflame::{game, linker};
use skybook_parser::cir;

use crate::error::{ErrorReport, sim_error};
use crate::sim;

/// Eat (use) items in pouch
pub fn eat_items(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    items: &[cir::ItemSelectSpec],
    pe_target: Option<&cir::ItemSelectSpec>,
) -> Result<(), processor::Error> {
    // must be in inventory to hold items
    super::switch_to_inventory_or_stop!(ctx, sys, errors, "EAT");
    super::check_not_holding_in_inventory!(ctx, sys, errors, "EAT");

    for item in items {
        if ctx.is_aborted() {
            break;
        }
        eat_item_internal(ctx, sys, errors, item, pe_target)?;
    }

    Ok(())
}

/// Internal helper for eating items
///
/// Must be in inventory to call this
pub fn eat_item_internal(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    item: &cir::ItemSelectSpec,
    pe_target: Option<&cir::ItemSelectSpec>,
) -> Result<(), processor::Error> {
    let name = &item.name;
    let meta = item.meta.as_ref();
    let memory = ctx.cpu().proc.memory();
    let inventory = sys.screen.current_screen_mut().as_inventory_mut().unwrap();
    let mut remaining = super::convert_amount(item.amount, item.span, errors, false, || {
        // eating always decrease value instead of using slot
        Ok(inventory.get_amount(name, meta, sim::CountingMethod::Value, memory)?)
    })?;

    let mut check_for_extra_error = true;
    loop {
        if ctx.is_aborted() {
            return Ok(());
        }
        // allowing more iterations if people want to "eat all" when they have millions
        // of items, like corrupted food
        // TODO --optimize: we could optimize by calling use_item
        // with a quantity instead of 1, but it won't be as accurate
        if remaining.is_done_allowing_iterations(item.span, errors, "EAT", 50000) {
            break;
        }
        let memory = ctx.cpu().proc.memory();
        let position = inventory.select(
            name,
            meta,
            Some(1), // must have at least 1 to eat
            memory,
            item.span,
            errors,
        )?;
        let Some((tab, slot)) = position else {
            break;
        };
        match inventory.get(tab, slot) {
            sim::ScreenItemState::Normal(item_ptr) => {
                // the item to eat must be eatable
                let name_ptr = Ptr!(&item_ptr->mName);
                let name = name_ptr.cstr(memory)?.load_utf8_lossy(memory)?;
                if !game::can_use(&name) {
                    errors.push(sim_error!(item.span, NotEatable));
                    check_for_extra_error = false;
                    break;
                }
            }
            _ => {
                // the item to eat must be non empty and non translucent
                errors.push(sim_error!(item.span, InvalidItemTarget));
                check_for_extra_error = false;
                break;
            }
        }
        // check if we are eating a PE activated slot
        let Some((tab, slot)) =
            super::change_to_pe_target_if_need(pe_target, inventory, memory, tab, slot, errors)?
        else {
            check_for_extra_error = false;
            break;
        };

        let correct_slot = inventory.corrected_slot(tab, slot);
        linker::use_item(ctx.cpu(), tab as i32, correct_slot, 1)?;

        inventory.update(tab, slot, None, ctx.cpu().proc.memory())?;
        remaining.sub(1);
    }
    if check_for_extra_error {
        let memory = ctx.cpu().proc.memory();
        let result = remaining.check(item.span, errors, || {
            inventory.get_amount(name, meta, sim::CountingMethod::Value, memory)
        })?;
        super::check_remaining!(result, errors, item.span);
    }

    Ok(())
}
