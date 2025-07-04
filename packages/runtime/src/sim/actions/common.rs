use blueflame::processor::{self, Cpu2};
use blueflame::{linker, memory};
use skybook_parser::cir;
use teleparse::Span;

use crate::error::{ErrorReport, sim_error, sim_warning};
use crate::sim;

macro_rules! switch_to_overworld_or_stop {
    ($ctx:ident, $sys:ident, $errors:ident, $command:literal) => {
        if !$sys
            .screen
            .transition_to_overworld($ctx, &mut $sys.overworld, false, $errors)?
        {
            log::warn!(
                "failed to auto-switch to overworld for {} command",
                $command
            );
            return Ok(());
        }
    };
}
pub(crate) use switch_to_overworld_or_stop;

macro_rules! switch_to_inventory_or_stop {
    ($ctx:ident, $sys:ident, $errors:ident, $command:literal) => {
        if !$sys
            .screen
            .transition_to_inventory($ctx, &mut $sys.overworld, false, $errors)?
        {
            log::warn!(
                "failed to auto-switch to inventory for {} command",
                $command
            );
            return Ok(());
        }
    };
}
pub(crate) use switch_to_inventory_or_stop;

macro_rules! check_not_holding_in_inventory {
    ($ctx:ident, $sys:ident, $errors:ident, $command:literal) => {
        if $sys.screen.holding_in_inventory {
            log::warn!(
                "cannot perform {} command while holding in inventory",
                $command
            );
            $errors.push($crate::error::sim_error!(
                $ctx.span,
                CannotDoWhileHoldingInInventory
            ));
            return Ok(());
        }
    };
}
pub(crate) use check_not_holding_in_inventory;

macro_rules! predrop_items {
    ($ctx:ident, $sys:ident, $errors:ident, $command:literal) => {
        match $sys.overworld.predrop_for_action($ctx.span, $errors) {
            $crate::sim::OverworldPreDropResult::Holding => {
                log::warn!("cannot execute {} command while holding items", $command);
                return Ok(());
            }
            $crate::sim::OverworldPreDropResult::AutoDrop => true,
            $crate::sim::OverworldPreDropResult::Ok => false,
        }
    };
}
pub(crate) use predrop_items;

#[inline]
pub fn handle_predrop_result(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    open_inventory: bool,
    should_drop: bool,
    command: &'static str,
) -> Result<(), processor::Error> {
    if open_inventory {
        log::debug!("auto-opening inventory after {command} command");
        // open pause menu and delay drop
        sys.screen
            .transition_to_inventory(ctx, &mut sys.overworld, false, errors)?;
        if should_drop {
            log::debug!("setting remove_held_after_dialog after {command} command");
            sys.screen.set_remove_held_after_dialog();
        }
    } else if should_drop {
        log::debug!("removing held items on auto-drop cleanup after {command} command");
        drop_held_items(ctx, sys, command)?;
    }

    Ok(())
}

pub fn drop_held_items(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    command: &str,
) -> Result<(), processor::Error> {
    log::debug!("dropping held items for {command} command");
    linker::remove_held_items(ctx.cpu())?;
    sys.overworld.drop_held_items();
    sys.screen.holding_in_inventory = false;
    Ok(())
}

/// Convert `AllBut` variant from the "but" amount to real amount
pub fn convert_amount<F: Fn() -> Result<usize, memory::Error>>(
    amount: cir::AmountSpec,
    span: Span,
    errors: &mut Vec<ErrorReport>,
    count_for_all: bool,
    count_fn: F,
) -> Result<OperationAmount, memory::Error> {
    match amount {
        cir::AmountSpec::AllBut(n) => {
            let count = count_fn()?;
            if count < n {
                errors.push(sim_error!(span, NotEnoughForAllBut(n, count)));
                Ok(OperationAmount::all_but(0, n))
            } else {
                Ok(OperationAmount::all_but(count - n, n))
            }
        }
        cir::AmountSpec::All => {
            if count_for_all {
                let count = count_fn()?;
                Ok(OperationAmount::num(count))
            } else {
                Ok(OperationAmount::all())
            }
        }
        cir::AmountSpec::Num(n) => Ok(OperationAmount::num(n)),
    }
}

pub struct OperationAmount {
    remaining_amount_or_all: Option<usize>,
    all_but: Option<usize>,
    was_found: bool,
}

impl OperationAmount {
    pub fn count(&self) -> Option<usize> {
        self.remaining_amount_or_all
    }
    pub fn num(n: usize) -> Self {
        Self {
            remaining_amount_or_all: Some(n),
            all_but: None,
            was_found: false,
        }
    }
    pub fn all() -> Self {
        Self {
            remaining_amount_or_all: None,
            all_but: None,
            was_found: false,
        }
    }
    pub fn all_but(remaining: usize, all_but: usize) -> Self {
        Self {
            remaining_amount_or_all: Some(remaining),
            all_but: Some(all_but),
            was_found: false,
        }
    }
    pub fn is_done(&self) -> bool {
        matches!(self.remaining_amount_or_all, Some(0))
    }

    pub fn sub(&mut self, amount: usize) {
        self.was_found = true;
        if let Some(n) = &mut self.remaining_amount_or_all {
            *n = n.saturating_sub(amount)
        }
    }

    /// Check the remaining amount
    ///
    /// If `None` is returned, the execution can continue with no additional error.
    /// If `Some(X)` is returned, it means the command is expecting `X` more items, which can
    /// no longer be found
    #[must_use = "result of checking if error should be emitted"]
    pub fn check<F: Fn() -> Result<usize, memory::Error>>(
        &self,
        span: Span,
        errors: &mut Vec<ErrorReport>,
        count_fn: F,
    ) -> Result<ItemSelectCheck, memory::Error> {
        match self.remaining_amount_or_all {
            None => {
                if self.was_found {
                    Ok(ItemSelectCheck::Done)
                } else {
                    Ok(ItemSelectCheck::NeverFound)
                }
            }
            Some(remaining) => match self.all_but {
                Some(but) => {
                    if remaining != 0 || but != count_fn()? {
                        log::warn!("inaccurate all-but detected");
                        errors.push(sim_warning!(span, InaccurateAllBut));
                    }
                    Ok(ItemSelectCheck::Done)
                }
                None => {
                    if remaining == 0 {
                        Ok(ItemSelectCheck::Done)
                    } else {
                        Ok(ItemSelectCheck::NeedMore(remaining))
                    }
                }
            },
        }
    }
}

/// Control flow when item is no longer found
pub enum ItemSelectCheck {
    /// No error, just continue
    Done,
    /// Emit error because this item is never found
    NeverFound,
    /// Emit error because we need more of this item
    NeedMore(usize),
}
