use blueflame::game::{self, WeaponModifierInfo};
use blueflame::linker::events::GameEvent as _;
use blueflame::memory::{self, Memory};
use blueflame::processor::{self, Cpu2};
use blueflame::linker;
use skybook_parser::{cir, Span};

use crate::error::{ErrorReport, sim_error, sim_warning};
use crate::sim;

macro_rules! switch_to_overworld_or_stop {
    ($ctx:ident, $sys:ident, $errors:ident, $command:literal) => {
        $crate::sim::actions::common::switch_to_overworld_or_stop!($ctx, $sys, $errors, $command, {})
    };
    ($ctx:ident, $sys:ident, $errors:ident, $command:literal, $block:block) => {
        if !$sys
            .screen
            .transition_to_overworld($ctx, &mut $sys.overworld, false, $errors)?
        {
            log::warn!(
                "failed to auto-switch to overworld for {} command",
                $command
            );
            $block
            return Ok(());
        }
    };
}
pub(crate) use switch_to_overworld_or_stop;

macro_rules! switch_to_inventory_or_stop {
    ($ctx:ident, $sys:ident, $errors:ident, $command:literal) => {
        $crate::sim::actions::common::switch_to_inventory_or_stop!($ctx, $sys, $errors, $command, {})
    };
    ($ctx:ident, $sys:ident, $errors:ident, $command:literal, $block:block) => {
        if !$sys
            .screen
            .transition_to_inventory($ctx, &mut $sys.overworld, false, $errors)?
        {
            log::warn!(
                "failed to auto-switch to inventory for {} command",
                $command
            );
            $block
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

macro_rules! check_remaining {
    ($result:ident, $errors:ident, $span:expr) => {
        $crate::sim::actions::common::check_remaining!(
            $result,
            $errors,
            $span,
            CannotFindItem,
            CannotFindItemNeedMore
        )
    };
    ($result:ident, $errors:ident, $span:expr, $NotFoundError:ident, $NeedMoreError:ident) => {
        match $result {
            $crate::sim::actions::common::ItemSelectCheck::NeverFound => {
                $errors.push($crate::error::sim_error!($span, $NotFoundError));
            }
            $crate::sim::actions::common::ItemSelectCheck::NeedMore(n) => {
                $errors.push($crate::error::sim_error!($span, $NeedMoreError(n)))
            }
            _ => {}
        }
    };
}
pub(crate) use check_remaining;

macro_rules! check_remaining_ground {
    ($result:ident, $errors:ident, $span:expr) => {
        $crate::sim::actions::common::check_remaining!(
            $result,
            $errors,
            $span,
            CannotFindGroundItem,
            CannotFindGroundItemNeedMore
        )
    };
}
pub(crate) use check_remaining_ground;

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

/// Add an error if the item amount is not 1
pub fn check_overworld_amount(item: &cir::ItemSelectSpec, errors: &mut Vec<ErrorReport>) {
    if item.amount != cir::AmountSpec::Num(1) {
        errors.push(sim_warning!(item.matcher.span, UselessAmountForOverworldEquipment))
    }
}

/// Convert `AllBut` variant from the "but" amount to real amount
pub fn convert_amount<F: FnOnce(&mut Vec<ErrorReport>) -> Result<usize, processor::Error>>(
    amount: cir::AmountSpec,
    span: Span,
    errors: &mut Vec<ErrorReport>,
    count_for_all: bool,
    count_fn: F,
) -> Result<OperationAmount, processor::Error> {
    match amount {
        cir::AmountSpec::AllBut(n) => {
            let count = count_fn(errors)?;
            if count < n {
                errors.push(sim_error!(span, NotEnoughForAllBut(n, count)));
                Ok(OperationAmount::all_but(0, n))
            } else {
                Ok(OperationAmount::all_but(count - n, n))
            }
        }
        cir::AmountSpec::All => {
            if count_for_all {
                let count = count_fn(errors)?;
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
    is_done_check_count: usize,
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
            is_done_check_count: 0,
        }
    }
    pub fn all() -> Self {
        Self {
            remaining_amount_or_all: None,
            all_but: None,
            was_found: false,
            is_done_check_count: 0,
        }
    }
    pub fn all_but(remaining: usize, all_but: usize) -> Self {
        Self {
            remaining_amount_or_all: Some(remaining),
            all_but: Some(all_but),
            was_found: false,
            is_done_check_count: 0,
        }
    }

    pub fn is_done(&mut self, span: Span, errors: &mut Vec<ErrorReport>, operation: &str) -> bool {
        self.is_done_allowing_iterations(span, errors, operation, 3000)
    }

    pub fn is_done_allowing_iterations(
        &mut self,
        span: Span,
        errors: &mut Vec<ErrorReport>,
        operation: &str,
        max: usize,
    ) -> bool {
        self.is_done_check_count += 1;
        if self.is_done_check_count > max {
            log::error!("iteration limit reached: {operation}, max is {max}");
            errors.push(sim_error!(span, TooManyIterations));
            return true;
        }
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
    pub fn check<F: FnOnce(&mut Vec<ErrorReport>) -> Result<usize, memory::Error>>(
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
                    if remaining != 0 || but != count_fn(errors)? {
                        log::warn!("inaccurate all-but detected");
                        errors.push(sim_warning!(span, InaccurateAllBut));
                    }
                    Ok(ItemSelectCheck::Done)
                }
                None => {
                    if remaining == 0 {
                        Ok(ItemSelectCheck::Done)
                    } else if self.was_found {
                        Ok(ItemSelectCheck::NeedMore(remaining))
                    } else {
                        Ok(ItemSelectCheck::NeverFound)
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

/// Change the currently targeted tab/slot to the PE target
///
/// Returns None if some error occurs
pub fn change_to_pe_target_if_need(
    pe_target: Option<&cir::ItemSelectSpec>,
    inventory: &sim::PouchScreen,
    memory: &Memory,
    tab: usize,
    slot: usize,
    errors: &mut Vec<ErrorReport>,
) -> Result<Option<(usize, usize)>, memory::Error> {
    if !inventory.is_pe_activated_slot(tab, slot, false) {
        // use position as-is if PE is not activated
        return Ok(Some((tab, slot)));
    }
    let Some(target) = pe_target else {
        // use position as-is if no PE target is set
        return Ok(Some((tab, slot)));
    };
    // find the target item
    let mut new_errors = vec![];
    let target_pos = inventory.select(
        &target.matcher,
        memory,
        &mut new_errors,
    )?;
    let Some((target_tab, target_slot)) = target_pos else {
        errors.extend(new_errors);
        errors.push(sim_error!(target.matcher.span, CannotFindPromptTarget));
        return Ok(None);
    };
    // eat the selection errors if the target was found

    // the target slot must be in a PE activate slot
    // to be able to use PE
    if !inventory.is_pe_activated_slot(target_tab, target_slot, true) {
        errors.push(sim_error!(target.matcher.span, InvalidPromptTarget));
        return Ok(None);
    }
    // adjust the slot index to target the actual
    // slot if it's translucent/empty
    let target_slot = inventory.get_pe_target_slot(target_tab, target_slot, false);

    Ok(Some((target_tab, target_slot)))
}

/// Call trash item with event wrapper
pub fn trash_item_wrapped(
    cpu: &mut Cpu2<'_, '_>,
    sys: &mut sim::GameSystems,
    tab: i32,
    slot: i32,
) -> Result<(), processor::Error> {
    let menu_overload = sys.screen.menu_overload;

    #[derive(Default)]
    struct State {
        pub drop_types: Vec<i32>, // TODO: smallvec?
        pub weapons_to_spawn: Vec<sim::OverworldActor>,
        menu_overload: bool,
    }

    let state = linker::events::TrashEquip::execute_subscribed(
        cpu,
        State {
            menu_overload,
            ..Default::default()
        },
        |state, arg| match arg {
            linker::events::TrashEquipArgs::Trash(name, value, modifier) => {
                if !state.menu_overload {
                    let actor = sim::OverworldActor {
                        name,
                        value,
                        modifier: if modifier.flags == 0 {
                            None
                        } else {
                            Some(modifier)
                        },
                    };
                    state.weapons_to_spawn.push(actor);
                }
            }
            linker::events::TrashEquipArgs::PlayerDrop(x) => state.drop_types.push(x),
            _ => {}
        },
        |cpu| linker::trash_item(cpu, tab, slot),
    )?;

    for x in state.drop_types {
        sys.overworld.drop_player_equipment(x);
    }

    for weapon in state.weapons_to_spawn {
        sys.overworld.spawn_weapon_later(weapon);
    }
    sys.check_weapon_spawn();

    Ok(())
}

pub fn get_item_with_auto_equip(
    cpu: &mut Cpu2<'_, '_>,
    sys: &mut sim::GameSystems,
    is_weapon: bool,
    name: &str,
    value: Option<i32>,
    modifier: Option<WeaponModifierInfo>,
) -> Result<(), processor::Error> {
    linker::get_item(cpu, name, value, modifier)?;
    if !is_weapon {
        return Ok(());
    }
    let value = match value {
        Some(x) => x,
        // since we use the game code to get the default value
        // if not present, it's not easy to get it here.
        // we could use some hook to get the result, but it's simpler
        // to include as data in the deps
        //
        // use 10 as some dummy value, as it should always succeed
        None => game::get_weapon_general_life(name).unwrap_or(10) * 100,
    };

    if sys.overworld.try_auto_equip(name, value, modifier.as_ref()) {
        log::debug!("auto-equipping {name}");
        linker::equip_last_added_item(cpu)?;
    }

    Ok(())
}
