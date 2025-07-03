use blueflame::linker;
use blueflame::processor::{self, Cpu2};
use skybook_parser::cir;
use teleparse::Span;

use crate::error::{ErrorReport, sim_error, sim_warning};
use crate::sim;

macro_rules! switch_to_overworld_or_stop {
    ($ctx:ident, $sys:ident, $errors:ident, $command:literal) => {
        if !$sys.screen.transition_to_overworld($ctx, &mut $sys.overworld, false, $errors)? {
            log::warn!("failed to auto-switch to overworld for {} command", $command);
            return Ok(());
        }
    };
}
pub(crate) use switch_to_overworld_or_stop;

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
    command: &'static str
) -> Result<(), processor::Error> {
    if open_inventory {
        log::debug!("auto-opening inventory after {} command", command);
        // open pause menu and delay drop
        sys.screen
            .transition_to_inventory(ctx, &mut sys.overworld, false, errors)?;
        if should_drop {
            log::debug!("setting remove_held_after_dialog after {} command", command);
            sys.screen.set_remove_held_after_dialog();
        }
    } else if should_drop {
        log::debug!("removing held items on auto-drop cleanup after {} command", command);
        linker::remove_held_items(ctx.cpu())?;
        sys.overworld.drop_held_items();
    }

    Ok(())
}

/// Convert `AllBut` variant from the "but" amount to real amount
pub fn convert_amount<F: Fn() -> usize>(
    amount: cir::AmountSpec, 
    span: Span,
    errors: &mut Vec<ErrorReport>,
    count_fn: F,
) -> ItemSelectTracker {
    match amount {
        cir::AmountSpec::AllBut(n) => {
            let count = count_fn();
            if count < n {
                errors.push(sim_error!(span, NotEnoughForAllBut(n, count)));
                ItemSelectTracker::all_but(0, n)
            } else {
                ItemSelectTracker::all_but(count - n, n)
            }
        },
        cir::AmountSpec::All => ItemSelectTracker::all(),
        cir::AmountSpec::Num(n) => ItemSelectTracker::num(n)
    }
}

pub struct ItemSelectTracker {
    remaining_amount_or_all: Option<usize>,
    all_but: Option<usize>,
    was_found: bool
}

impl ItemSelectTracker {
    pub fn num(n: usize) -> Self {
        Self {
            remaining_amount_or_all: Some(n),all_but: None, was_found: false
        }
    }
    pub fn all() -> Self {
        Self {
            remaining_amount_or_all: None,all_but: None, was_found: false
        }
    }
    pub fn all_but(remaining: usize, all_but: usize) -> Self {
        Self {
            remaining_amount_or_all: Some(remaining),all_but: Some(all_but), was_found: false
        }
    }
    pub fn is_done(&self) -> bool {
        matches!(self.remaining_amount_or_all, Some(0))
    }
    pub fn decrement(&mut self) {
        match &mut self.remaining_amount_or_all {
            Some(n) => *n -= 1,
            _ => {}
        }
    }

    /// Check the remaining amount
    ///
    /// If `None` is returned, the execution can continue with no additional error.
    /// If `Some(X)` is returned, it means the command is expecting `X` more items, which can
    /// no longer be found
    #[must_use = "result of checking if error should be emitted"]
    pub fn check<F: Fn() -> usize>(&self, span: Span, errors: &mut Vec<ErrorReport>, count_fn: F) -> NoLongerFound {
        match self.remaining_amount_or_all {
            None => {
                if self.was_found {
                    NoLongerFound::Done
                } else {
                    NoLongerFound::NeverFound
                }
            }
            Some(remaining) => {
                match self.all_but {
                    Some(but) => {
                        if remaining != but || but != count_fn() {
                            log::warn!("inaccurate all-but detected");
                            errors.push(sim_warning!(span, InaccurateAllBut));
                        }
                        NoLongerFound::Done
                    }
                    None => {
                        if remaining == 0 {
                            NoLongerFound::Done
                        } else {
                            NoLongerFound::NeedMore(remaining)
                        }
                    }
                }
            }
        }
    }
}

/// Control flow when item is no longer found
pub enum NoLongerFound {
    /// No error, just continue
    Done,
    /// Emit error because this item is never found
    NeverFound,
    /// Emit error because we need more of this item
    NeedMore(usize),
}
