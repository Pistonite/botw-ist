use blueflame::game;
use blueflame::processor::{self, Cpu2};
use skybook_parser::cir;

use crate::sim;

/// Spawn items directly into the overworld
///
/// Actors will be spawned even when menu overloaded (think of it
/// as the item was there before menu overload)
pub fn spawn_items(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    items: &[cir::ItemSpec],
) -> Result<(), processor::Error> {
    for item in items {
        spawn_item_internal(ctx, sys, item)?;
        if ctx.is_aborted() {
            break;
        }
    }

    Ok(())
}

fn spawn_item_internal(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    item: &cir::ItemSpec,
) -> Result<(), processor::Error> {
    let is_weapon = sim::util::name_is_weapon(&item.name);
    let (value, modifier) = if is_weapon {
        let modifier = sim::util::modifier_from_meta(item.meta.as_ref());
        // use 10 as some dummy value, as it should always succeed
        let value = game::get_weapon_general_life(&item.name).unwrap_or(10) * 100;
        (value, modifier)
    }else {
        (1, None)
    };
    for _ in 0..item.amount {
        if ctx.is_aborted() {
            return Ok(());
        }
        let actor = sim::Actor {
            name: item.name.clone(),
            value,modifier
        };
        if is_weapon {
            sys.overworld.force_spawn_weapon(actor);
        } else {
            sys.overworld.force_spawn_material(actor);
        }
    }
    Ok(())
}
