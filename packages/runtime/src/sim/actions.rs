use blueflame::linker;
use blueflame::processor::{self, Cpu2};
use skybook_parser::cir;

use crate::sim;

use super::util;

/// Add items to pouch by eventually calling itemGet or cookItemGet
pub fn get_items(
    ctx: &mut sim::Context<&mut Cpu2>,
    items: &[cir::ItemSpec],
) -> Result<(), processor::Error> {
    'outer: for item in items {
        let amount = item.amount;
        let item = &item.item;
        let is_cook_item = item.is_cook_item();
        let meta = item.meta.as_ref();
        for _ in 0..amount {
            if is_cook_item {
                linker::get_cook_item(
                    ctx.inner,
                    &item.actor,
                    meta.map(|m| m.ingredients.as_slice()).unwrap_or(&[]),
                    meta.and_then(|m| m.life_recover_f32()),
                    meta.and_then(|m| m.effect_duration),
                    meta.and_then(|m| m.sell_price),
                    meta.and_then(|m| m.effect_id),
                    meta.and_then(|m| m.effect_level),
                )?;
                continue;
            };
            let modifier = util::modifier_from_meta(meta);
            linker::get_item(ctx.inner, &item.actor, meta.and_then(|m| m.value), modifier)?;

            if ctx.is_aborted() {
                break 'outer;
            }
        }
    }
    Ok(())
}

/// Remove held items by calling removeGrabbedItems
pub fn remove_held_items(
    ctx: &mut sim::Context<&mut Cpu2>,
) -> Result<(), processor::Error> {
    todo!()
}
