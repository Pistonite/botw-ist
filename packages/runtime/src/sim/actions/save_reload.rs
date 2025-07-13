use std::sync::Arc;

use blueflame::game::gdt;
use blueflame::linker;
use blueflame::linker::events::GameEvent;
use blueflame::memory::{self, proxy};
use blueflame::processor::{self, Cpu2, Process};

use crate::error::{ErrorReport, sim_error};
use crate::sim;

/// Save the game
///
/// Since the save system extend beyond the game (you can access them
/// even when the game is closed), the easist way to handle
/// this is by using a channel to send the save back to the runtime thread
/// from the executor thread
pub fn save(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    allow_overworld: bool,
    send: oneshot::Sender<Option<Arc<gdt::TriggerParam>>>,
) -> Result<(), processor::Error> {
    if allow_overworld {
        if !sys.screen.current_screen().is_inventory_or_overworld() {
            super::switch_to_overworld_or_stop!(ctx, sys, errors, "SAVE", {
                if send.send(None).is_err() {
                    log::error!("failed to send save data to runtime main thread");
                }
            });
        }
    } else {
        super::switch_to_inventory_or_stop!(ctx, sys, errors, "SAVE", {
            if send.send(None).is_err() {
                log::error!("failed to send save data to runtime main thread");
            }
        });
    }
    // switching to system tab to save will automatically unhold items
    // (if in holding state. if in PE drop hold state then it would not unhold)
    if sys.screen.current_screen().is_inventory() && sys.screen.holding_in_inventory {
        super::hold_items::unhold_internal(ctx, sys)?;
    }
    let gdt = get_save(ctx.cpu().proc)?;
    if send.send(Some(Arc::new(gdt))).is_err() {
        log::error!("failed to send save data to runtime main thread");
    }
    Ok(())
}

/// Extract save GDT from process memory
pub fn get_save(proc: &Process) -> Result<gdt::TriggerParam, memory::Error> {
    let gdt_ptr = gdt::trigger_param_ptr(proc.memory())?;
    proxy! { let gdt = *gdt_ptr as trigger_param in proc };
    Ok(gdt.clone())
}

/// Reload a save
pub fn reload(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    load_gdt: &gdt::TriggerParam,
) -> Result<(), processor::Error> {
    // can only reload from inventory (System tab)
    super::switch_to_inventory_or_stop!(ctx, sys, errors, "RELOAD");
    // regenerate the stage
    regen_stage(ctx, sys, errors, Some(load_gdt))
}

/// Regenerate the game stage, and optionally load a save while doing that
pub fn regen_stage(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    load_gdt: Option<&gdt::TriggerParam>,
) -> Result<(), processor::Error> {
    // 1. BaseProcMgr deletes all actors
    sys.overworld.destroy_all();

    // 2. SaveMgr/GdtMgr (?) loads the save into GDT
    if let Some(save_gdt) = load_gdt {
        let span = ctx.span;
        let proc = &mut ctx.cpu().proc;
        let gdt_ptr = gdt::trigger_param_ptr(proc.memory())?;
        proxy! { let mut gdt = *gdt_ptr as trigger_param in proc };
        if !gdt.load_save(save_gdt) {
            log::error!("unexpected load_save fail");
            errors.push(sim_error!(span, ReloadFail));
            return Ok(());
        }
    }

    // X. Reset to overworld screen
    sys.screen.reset_to_overworld();

    // 3. PMDM loads GDT
    linker::load_from_game_data(ctx.cpu())?;

    // 4. Create player equipments
    #[derive(Default, Debug)]
    struct State {
        pub weapon: Option<sim::OverworldActor>,
        pub bow: Option<sim::OverworldActor>,
        pub shield: Option<sim::OverworldActor>,
    }

    let state = linker::events::CreateEquip::execute_subscribed(
        ctx.cpu(),
        State::default(),
        |state, (slot, name, value, modifier)| {
            log::debug!("create_equip called: {slot}, {name}, {value}, {modifier:?}");
            let actor = sim::OverworldActor {
                name,
                value,
                modifier,
            };
            match slot {
                0 => state.weapon = Some(actor),
                1 => state.shield = Some(actor),
                2 => state.bow = Some(actor),
                _ => {}
            }
        },
        linker::create_player_equipment,
    )?;
    log::debug!("create_equip state: {state:?}");

    // 4. Equipments update their value
    sys.overworld
        .reset_equipments_on_genstage(ctx.cpu(), state.weapon, state.bow, state.shield)?;

    Ok(())
}
