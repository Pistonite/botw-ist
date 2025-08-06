use blueflame::processor::{self, Cpu2};

use crate::error::ErrorReport;
use crate::sim;

/// Activate/deactivate menu overload
pub fn set_menu_overload(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    overload: bool
) -> Result<(), processor::Error> {
    if overload {
        sys.set_menu_overload(true);
        return Ok(());
    }
    // since you can't unoverload while in inventory,
    // we will close inventory automatically to avoid confusion
    super::switch_to_overworld_or_stop!(ctx, sys, errors, "OVERLOAD");
        sys.set_menu_overload(false);

    Ok(())
}
