use blueflame::game;
use blueflame::linker;
use blueflame::processor::{self, Cpu2};
use skybook_parser::cir;

use crate::error::ErrorReport;
use crate::sim;

use super::{switch_to_overworld_or_stop, predrop_items, handle_predrop_result};

/// Add items to pouch by eventually calling itemGet or cookItemGet
pub fn get_items(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    items: &[cir::ItemSpec],
    pause_after: bool,
    accurate: bool,
) -> Result<(), processor::Error> {
    switch_to_overworld_or_stop!(ctx, sys, errors, "GET");
    let should_drop = predrop_items!(ctx, sys, errors, "GET");

    'outer: for item in items {
        if ctx.is_aborted() {
            break 'outer;
        }
        let amount = item.amount;
        let name = &item.name;
        let is_cook_item = name.starts_with("Item_Cook_");
        let meta = item.meta.as_ref();
        // TODO: cannotGetItem check?
        if is_cook_item {
            // cannot optimize cook items
            for _ in 0..amount {
                if ctx.is_aborted() {
                    break 'outer;
                }
                linker::get_cook_item(
                    ctx.inner,
                    name,
                    meta.map(|m| m.ingredients.as_slice()).unwrap_or(&[]),
                    meta.and_then(|m| m.life_recover_f32()),
                    meta.and_then(|m| m.effect_duration),
                    meta.and_then(|m| m.sell_price),
                    meta.and_then(|m| m.effect_id),
                    meta.and_then(|m| m.effect_level),
                )?;
            }
            continue;
        }
        let modifier = sim::util::modifier_from_meta(meta);
        let meta_value = meta.and_then(|m| m.value);

        let can_optimize = !accurate 
        && meta_value.is_none() 
        && game::can_stack(name) 
        // cannot optimize arrow, since it needs to be auto-equipped using the non value get call
        // (this can be improved if we manually check auto-equip)
        && game::get_pouch_item_type(name) != 2;
        if can_optimize {
            // optimize into one call with a value
            linker::get_item(ctx.cpu(), name, Some(amount as i32), modifier)?;
            continue;
        }
        for _ in 0..amount {
            if ctx.is_aborted() {
                break 'outer;
            }
            linker::get_item(ctx.cpu(), name, meta_value, modifier)?;
        }
    }

    handle_predrop_result(ctx, sys, errors, pause_after, should_drop, "GET")?;
    sys.overworld.despawn_items();

    Ok(())
}
