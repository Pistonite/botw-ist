use blueflame::game::PouchItemType;
use blueflame::linker;
use blueflame::memory::{Ptr, mem};
use blueflame::processor::{self, Cpu2};
use skybook_parser::cir;

use crate::error::{ErrorReport, sim_error, sim_warning};
use crate::sim;

/// Use items
///
/// The behavior depends on the item type:
/// - Weapon/Bow/Shield:
///   - Can only target equipped weapon, uses the weapon (loses dura)
/// - Material: The only legitmate use is using fairy
/// - Arrow/Armor/Food/KeyItem: will call removeItem(), but the behavior
///   doesn't correspond to anything in game
///
/// Non-equipment must specify the name of the item to remove
pub fn use_items(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    item: &cir::ItemNameSpec,
    times: usize,
    per_use: Option<i32>,
) -> Result<(), processor::Error> {
    // must be in the overworld to use anything
    super::switch_to_overworld_or_stop!(ctx, sys, errors, "USE");
    let item_type = sim::util::name_spec_to_item_type(item);

    const SWORD: i32 = PouchItemType::Sword as i32;
    const SHIELD: i32 = PouchItemType::Shield as i32;
    const BOW: i32 = PouchItemType::Bow as i32;

    // using overworld equipment
    if matches!(item_type, SWORD | SHIELD | BOW) {
        let step_by = per_use.unwrap_or(100);
        if !sys.overworld.equipped_item_matches(item_type, item) {
            errors.push(sim_error!(ctx.span, NotEquippedInOverworld));
            return Ok(());
        }
        for i in 0..times {
            if ctx.is_aborted() {
                break;
            }
            let can_continue =
                use_overworld_equipment_once_internal(ctx, sys, errors, item, item_type, step_by)?;
            if !can_continue {
                if i != times - 1 {
                    errors.push(sim_warning!(ctx.span, CannotUseMore(times - 1 - i)));
                }
                break;
            }
        }
        return Ok(());
    }

    if per_use.is_some() {
        errors.push(sim_warning!(ctx.span, UselessPerUseAmount));
    }

    let cir::ItemNameSpec::Actor(item_name) = item else {
        errors.push(sim_error!(ctx.span, CannotUseCategory));
        return Ok(());
    };

    const MAX_ITERATIONS: usize = 3000;

    for _ in 0..times.min(MAX_ITERATIONS) {
        if ctx.is_aborted() {
            break;
        }
        linker::remove_item_by_name(ctx.cpu(), item_name)?;
    }
    if times > MAX_ITERATIONS {
        errors.push(sim_error!(ctx.span, TooManyIterations));
    }
    Ok(())
}

// returns the equipment can continue to be used
fn use_overworld_equipment_once_internal(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    item: &cir::ItemNameSpec,
    item_type: i32,
    step_by: i32,
) -> Result<bool, processor::Error> {
    // cannot use anymore
    if !sys.overworld.equipped_item_matches(item_type, item) {
        return Ok(false);
    }
    let item_name = sys
        .overworld
        .get_equiped_item(item_type)
        .map(|x| x.name.to_string())
        .unwrap_or_default();
    let is_damagable = !matches!(
        item_name.as_str(),
        "Weapon_Bow_071" | "Weapon_Sword_502" | "Weapon_Sword_503"
    );

    // TODO: to confirm, does hold smuggle items get unheld? (probably not)
    if item_type == PouchItemType::Bow as i32 {
        // to use bow, first drop/unhold items
        if sys.overworld.is_holding_arrowless_smuggled() {
            cu::debug!("automatically dropping for USE command");
            super::drop_held_items(ctx, sys, "USE")?;
        } else if sys.overworld.is_holding() {
            cu::debug!("automatically unholding for USE command");
            super::unhold_internal(ctx, sys)?;
        }

        // get the arrow to spawn
        let arrow_name = match item_name.as_str() {
            "Weapon_Bow_072" | "Weapon_Bow_071" => None,
            _ => {
                // get the equipped arrow item first
                let equipped_arrow =
                    linker::get_equipped_item(ctx.cpu(), PouchItemType::Arrow as i32)?;
                if equipped_arrow.is_nullptr() {
                    cu::warn!("no arrows equipped");
                    errors.push(sim_error!(ctx.span, NoArrowsToShoot));
                    return Ok(false);
                }
                let m = ctx.cpu().proc.memory();
                mem! { m: let arrow_count = *(&equipped_arrow->mValue) }
                if arrow_count < 1 {
                    cu::warn!("equipped arrow has count < 1: {arrow_count}");
                    errors.push(sim_error!(ctx.span, NoArrowsToShoot));
                    return Ok(false);
                }

                let arrow_name = Ptr!(&equipped_arrow->mName).cstr(m)?.load_utf8_lossy(m)?;
                Some(arrow_name)
            }
        };

        // damage the bow
        let broken = is_damagable
            && sys
                .overworld
                .damage_equipment(ctx.cpu(), item_type, step_by)?;

        // decrease the arrow count
        if let Some(arrow) = arrow_name {
            cu::trace!("removing arrow: {arrow}");
            linker::remove_arrow(ctx.cpu(), &arrow, 1)?;
        }

        return Ok(!broken);
    }

    // using weapon/shield
    if sys.overworld.is_holding() {
        cu::debug!("automatically unholding for USE command");
        super::unhold_internal(ctx, sys)?;
    }
    // damage the equipment
    let broken = is_damagable
        && sys
            .overworld
            .damage_equipment(ctx.cpu(), item_type, step_by)?;

    Ok(!broken)
}
