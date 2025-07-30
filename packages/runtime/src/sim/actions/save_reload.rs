use std::sync::Arc;

use blueflame::game::gdt;
use blueflame::linker;
use blueflame::linker::events::GameEvent;
use blueflame::memory::{self, proxy};
use blueflame::processor::{self, Cpu2, Process};

use crate::error::ErrorReport;
use crate::sim;

macro_rules! reload_gdt_or_stop {
    ($ctx:ident, $errors:ident, $gdt:ident) => {{
        let span = $ctx.span;
        let proc = &mut $ctx.cpu().proc;
        let gdt_ptr = blueflame::game::gdt::trigger_param_ptr(proc.memory())?;
        blueflame::memory::proxy! { let mut gdt = *gdt_ptr as trigger_param in proc };
        if !gdt.load_save($gdt) {
            log::error!("unexpected load_save fail");
            $errors.push($crate::error::sim_error!(span, ReloadFail));
            return Ok(());
        }
    }}
}

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
    regen_stage_internal(ctx, sys, errors, true, Some(load_gdt))
}


/// Reload savedata into GDT, but don't do anything else
pub fn reload_gdt(
    ctx: &mut sim::Context<&mut Cpu2>,
    errors: &mut Vec<ErrorReport>,
    load_gdt: &gdt::TriggerParam,
) -> Result<(), processor::Error> {
    reload_gdt_or_stop!(ctx, errors, load_gdt);
    Ok(())
}

/// Regenerate the game stage, and optionally load a save while doing that
pub fn regen_stage_internal(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    remove_translucent: bool,
    load_save: Option<&gdt::TriggerParam>,
) -> Result<(), processor::Error> {
    if remove_translucent {
        // 0. Translucent items are removed before reload
        // This is called as part of the loading screen, for example,
        // this is called when entering/exiting shrine as well
        linker::delete_removed_items(ctx.cpu())?;
    }

    // 1. BaseProcMgr deletes all actors
    sys.overworld.destroy_all();

    // X. Reset to overworld screen
    sys.screen.reset_to_overworld();

    // 2. SaveMgr/GdtMgr (?) loads the save into GDT
    if let Some(save_gdt) = load_save {
        reload_gdt_or_stop!(ctx, errors, save_gdt);
        // 3. PMDM loads from GDT
        linker::load_from_game_data(ctx.cpu())?;
    }

    // 4. Create player equipments
    recreate_overworld_equipments(ctx, sys)?;

    Ok(())
}

pub fn recreate_overworld_equipments(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
) -> Result<(), processor::Error> {
    let state = linker::events::CreateEquip::execute_subscribed(
        ctx.cpu(),
        CreateEquipState::default(),
        CreateEquipState::update,
        linker::create_player_equipment,
    )?;
    // update value to PMDM
    sys.overworld.reload_equipments(ctx.cpu(), state.weapon, state.bow, state.shield)?;
    Ok(())
}

/// Simulate starting a trial with empty inventory
pub fn trial_start(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
) -> Result<(), processor::Error> {
    // Init Pouch
    // Technically, we know the event subscription here is not needed
    // because weapons won't be created because we know the pouch is empty
    let state = linker::events::CreateEquip::execute_subscribed(
        ctx.cpu(),
        CreateEquipState::default(),
        CreateEquipState::update,
        linker::init_for_quest,
    )?;
    log::debug!("init_for_quest finished");

    // Equipments update their value
    sys.overworld.reload_equipments(ctx.cpu(), state.weapon, state.bow, state.shield)?;

    Ok(())
}

/// Simulate ending a trial and restoring the inventory
pub fn trial_end(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
) -> Result<(), processor::Error> {
    let state = linker::events::CreateEquip::execute_subscribed(
        ctx.cpu(),
        CreateEquipState::default(),
        CreateEquipState::update,
        linker::restore_from_quest,
    )?;
    sys.overworld.reload_equipments(ctx.cpu(), state.weapon, state.bow, state.shield)?;

    Ok(())
}

#[derive(Default)]
struct CreateEquipState {
    weapon: Option<sim::OverworldActor>,
    bow: Option<sim::OverworldActor>,
    shield: Option<sim::OverworldActor>,
}

impl CreateEquipState {
    fn update(&mut self, args: <linker::events::CreateEquip as GameEvent>::TArgs) {
        let (slot, name, value, modifier) = args;
        let actor = sim::OverworldActor {
            name,
            value,
            modifier,
        };
        match slot {
            0 => self.weapon = Some(actor),
            1 => self.shield = Some(actor),
            2 => self.bow = Some(actor),
            _ => {}
        }
    }
}
