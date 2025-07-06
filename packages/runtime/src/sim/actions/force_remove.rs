use blueflame::game;
use blueflame::memory::{Ptr, mem};
use blueflame::processor::{self, Cpu2};
use skybook_parser::cir;

use crate::error::ErrorReport;
use crate::sim;

/// Forcefully remove items from the inventory
///
/// This is for backward compatibility with item removal in older versions
/// of simulator.
pub fn force_remove_item(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    items: &[cir::ItemSelectSpec],
) -> Result<(), processor::Error> {
    super::switch_to_inventory_or_stop!(ctx, sys, errors, "REMOVE");
    // open a temporary inventory that allows access to items
    // even when mCount = 0 (since it was allowed in the old version)
    let mut inventory = sim::PouchScreen::open(ctx.cpu(), true)?;

    'outer: for item in items {
        let name = &item.name;
        let meta = item.meta.as_ref();
        let memory = ctx.cpu().proc.memory();
        // remove food by stack value instead of slot (like old version)
        let mut remaining = super::convert_amount(item.amount, item.span, errors, false, || {
            inventory.get_amount(name, meta, sim::CountingMethod::CanStackOrFood, memory)
        })?;
        loop {
            if ctx.is_aborted() {
                break 'outer;
            }
            if remaining.is_done() {
                break;
            }
            let m = ctx.cpu().proc.memory();

            // find the item
            let Some((mut tab, mut slot)) =
                inventory.select(name, meta, None, m, item.span, errors)?
            else {
                break;
            };
            let sim::ScreenItemState::Normal(mut item_ptr) = inventory.get(tab, slot) else {
                // if found item is translucent or empty,
                // it must be found by slot, so there's no need to reselect
                // (it will just be the same result)
                break;
            };

            // if the found item is stackable and value is 0
            // we cannot remove from this stack, so find another stack
            // with at least 1 value
            mem! { m: let mut value = *(&item_ptr->mValue); }
            let item_name = Ptr!(&item_ptr->mName).cstr(m)?.load_utf8_lossy(m)?;
            let mut can_stack = game::can_stack(&item_name);

            if can_stack && value <= 0 {
                let position = inventory.select(name, meta, Some(1), m, item.span, errors)?;
                let Some((tab2, slot2)) = position else {
                    break;
                };
                tab = tab2;
                slot = slot2;
                let sim::ScreenItemState::Normal(item_ptr2) = inventory.get(tab, slot) else {
                    break;
                };
                item_ptr = item_ptr2;
                let item_name = Ptr!(&item_ptr->mName).cstr(m)?.load_utf8_lossy(m)?;
                can_stack = game::can_stack(&item_name);
                if can_stack {
                    mem! { m: let value2 = *(&item_ptr->mValue); }
                    value = value2;
                }
            }

            let mut should_decrease_stack_value = can_stack;
            // for corrupted food, we also delete by value
            if !should_decrease_stack_value {
                mem! { m: let item_type = *(&item_ptr->mType); };
                if item_type == 8 {
                    should_decrease_stack_value = true;
                }
            }

            if should_decrease_stack_value {
                let value = value.max(0) as usize;
                let amount_to_decrease = match remaining.count() {
                    Some(x) => x.min(value),
                    None => value, // remove all
                };
                remaining.sub(amount_to_decrease);
                // max(1) to ensure we are removing something
                let new_value = value.saturating_sub(amount_to_decrease.max(1));
                if new_value == 0 {
                    mem! { (ctx.cpu().proc.memory_mut()):
                        *(&item_ptr->mValue) = 0;
                        *(&item_ptr->mInInventory) = false;
                    }
                } else {
                    mem! { (ctx.cpu().proc.memory_mut()):
                        *(&item_ptr->mValue) = new_value as i32;
                    }
                }
            } else {
                // delete the slot
                mem! { (ctx.cpu().proc.memory_mut()):
                    *(&item_ptr->mValue) = 0;
                    *(&item_ptr->mInInventory) = false;
                }
                remaining.sub(1);
            }

            inventory.update(tab, slot, None, ctx.cpu().proc.memory())?;
        }
        let memory = ctx.cpu().proc.memory();
        let result = remaining.check(item.span, errors, || {
            inventory.get_amount(name, meta, sim::CountingMethod::CanStackOrFood, memory)
        })?;
        super::check_remaining!(result, errors, item.span);
    }

    // close and reopen inventory to ensure consistency
    super::switch_to_overworld_or_stop!(ctx, sys, errors, "REMOVE");
    super::switch_to_inventory_or_stop!(ctx, sys, errors, "REMOVE");

    Ok(())
}
