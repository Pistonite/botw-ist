use blueflame::game::{ self, PouchItem };
use blueflame::memory::{self, Ptr, mem};
use blueflame::processor::{self, Cpu2};
use skybook_parser::cir;

use crate::error::{ErrorReport, sim_error};
use crate::sim;

/// Handle the `!write [META] to item[TARGET_META]` supercommand
pub fn write_meta(
    ctx: &mut sim::Context<&mut Cpu2>,
    errors: &mut Vec<ErrorReport>,
    write_meta: &cir::ItemMeta,
    item: &cir::ItemSelectSpec,
) -> Result<(), processor::Error> {
    let matcher = &item.matcher;
    // find the item
    let Some(item_ptr) = super::find_single_item_target(ctx, errors, item)? else {
        errors.push(sim_error!(matcher.span, CannotFindItem));
        return Ok(());
    };

    // if the item spec has a name, also write the name
    let name = match &matcher.name {
        cir::ItemNameSpec::Actor(x) => Some(x.as_str()),
        cir::ItemNameSpec::Category(_) => None,
    };

    write_meta_internal(ctx, write_meta, name, item_ptr)?;
    super::fix_inventory_state(ctx)
}

/// Write meta data and optionally name to the item
fn write_meta_internal(
    ctx: &mut sim::Context<&mut Cpu2>,
    meta: &cir::ItemMeta,
    name: Option<&str>,
    item_ptr: Ptr![PouchItem],
) -> Result<(), memory::Error> {
    let m = ctx.cpu().proc.memory_mut();
    if let Some(x) = meta.value {
        mem! { m: *(&item_ptr->mValue) = x }
    }
    if let Some(x) = meta.equip {
        mem! { m: *(&item_ptr->mEquipped) = x }
    }
    if let Some(x) = meta.life_recover {
        mem! { m: *(&item_ptr->mHealthRecover) = x }
    }
    if let Some(x) = meta.effect_duration {
        mem! { m: *(&item_ptr->mEffectDuration) = x }
    }
    if let Some(x) = meta.sell_price {
        mem! { m: *(&item_ptr->mSellPrice) = x }
    }
    if let Some(x) = meta.effect_id_f32() {
        mem! { m: *(&item_ptr->mEffectId) = x }
    }
    // currently don't support ingredients (as we can't tell
    // ingredients not specified vs specifying no ingredients yet)

    if let Some(x) = name {
        // get type and use from item name
        let item_type = game::get_pouch_item_type(x);
        let item_use = game::get_pouch_item_use(x);
        Ptr!(&item_ptr->mName).safe_store(x, m)?;
        mem! { m:
            *(&item_ptr->mType) = item_type;
            *(&item_ptr->mItemUse) = item_use;
        }
    }

    Ok(())
}
