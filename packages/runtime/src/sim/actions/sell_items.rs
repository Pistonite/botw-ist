use blueflame::game;
use blueflame::linker;
use blueflame::memory::{Ptr, mem};
use blueflame::processor::{self, Cpu2};
use skybook_parser::cir;

use crate::error::{ErrorReport, sim_error};
use crate::sim;

use super::{ItemSelectCheck, convert_amount};

/// Sell items to a shop
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
        let name = &item.name;
        let meta = item.meta.as_ref();
        let memory = ctx.cpu().proc.memory();
        let mut remaining = convert_amount(item.amount, item.span, errors, false, || {
            shop.get_amount(name, meta, sim::CountingMethod::CanStack, memory)
        })?;
        let mut check_for_extra_error = true;
        loop {
            if ctx.is_aborted() {
                break 'outer;
            }
            if remaining.is_done() {
                break;
            }

            let m = ctx.cpu().proc.memory();

            // find the item to sell
            let Some((mut tab, mut slot)) = shop.select(name, meta, None, m, item.span, errors)?
            else {
                break;
            };

            let sim::ScreenItemState::Normal(mut item_ptr) = shop.get(tab, slot) else {
                // if found item is translucent or empty,
                // it must be found by slot, so there's no need to reselect
                // (it will just be the same result)
                break;
            };

            // if the found item is stackable and value is 0, somehow,
            // we cannot sell this stack, so find another stack
            // with at least 1 value
            mem! { m: let mut value = *(&item_ptr->mValue); }
            let mut item_name = Ptr!(&item_ptr->mName).cstr(m)?.load_utf8_lossy(m)?;
            let mut can_stack = game::can_stack(&item_name);

            if can_stack && value <= 0 {
                let position = shop.select(name, meta, Some(1), m, item.span, errors)?;
                let Some((tab2, slot2)) = position else {
                    break;
                };
                tab = tab2;
                slot = slot2;
                let sim::ScreenItemState::Normal(item_ptr2) = shop.get(tab, slot) else {
                    break;
                };
                item_ptr = item_ptr2;
                item_name = Ptr!(&item_ptr->mName).cstr(m)?.load_utf8_lossy(m)?;
                can_stack = game::can_stack(&item_name);
                if can_stack {
                    mem! { m: let value2 = *(&item_ptr->mValue); }
                    value = value2;
                }
            }

            // check if the selected item is sellable
            // (needs to be after the reselect check above
            if !game::can_sell(&item_name) {
                errors.push(sim_error!(item.span, NotSellable(item_name)));
                check_for_extra_error = false;
                break;
            }

            // calculate the amount to sell
            let sell_amount = if can_stack {
                // if the item somehow has negative value, make it 0
                let value = value.max(0) as usize;
                match remaining.count() {
                    None => value, // sell all
                    Some(n) => n.min(value),
                }
            } else {
                1
            };

            remaining.sub(sell_amount);
            linker::sell_item(ctx.cpu(), item_ptr, sell_amount as i32)?;
            shop.update(tab, slot, None, ctx.cpu().proc.memory())?;
        }
        if check_for_extra_error {
            let memory = ctx.cpu().proc.memory();
            match remaining.check(item.span, errors, || {
                shop.get_amount(name, meta, sim::CountingMethod::CanStack, memory)
            })? {
                ItemSelectCheck::NeverFound => {
                    errors.push(sim_error!(item.span, CannotFindItem));
                }
                ItemSelectCheck::NeedMore(n) => {
                    errors.push(sim_error!(item.span, CannotFindItemNeedMore(n)));
                }
                _ => {}
            }
        }
    }

    Ok(())
}
