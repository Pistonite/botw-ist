use std::sync::Arc;

use blueflame::game::gdt;
use blueflame::memory::proxy;
use blueflame::processor::{self, Cpu2};

use crate::error::ErrorReport;
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
    let gdt_ptr = gdt::trigger_param_ptr(ctx.cpu().proc.memory())?;
    let proc = &ctx.cpu().proc;
    proxy! { let gdt = *gdt_ptr as trigger_param in proc };
    if send.send(Some(Arc::new(gdt.clone()))).is_err() {
        log::error!("failed to send save data to runtime main thread");
    }
    Ok(())
}
