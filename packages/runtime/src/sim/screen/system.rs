use std::sync::Arc;

use blueflame::linker;
use blueflame::linker::events::GameEvent as _;
use blueflame::processor::{self, Cpu2};

use crate::error::{ErrorReport, sim_warning};
use crate::iv;
use crate::sim;

/// Simulation of different screens in the game and transitioning
/// between them
#[derive(Default, Clone)]
pub struct ScreenSystem {
    screen: Arc<Screen>,

    /// If the screen switch was performed manually
    is_manually_switched: bool,

    /// If Menu Overload Glitch is active
    pub menu_overload: bool,

    /// Flag for controlling whether removal of held items
    /// should happen after the dialog when transitioning
    /// from Overworld to a dialog.
    ///
    /// Normally, the game forces you to put away items before you
    /// can talk, but if you setup the smuggle glitch thingy,
    /// you can delay this until the dialog is finished to generate
    /// offsets (i.e broken slots)
    remove_held_item_after_dialog: bool,

    /// Stores removing equipped items from inventory until
    /// after it's closed. When removing, these will be called
    /// before deleteRemovedItems()
    ///
    /// This can be used to make translucent slots, by injecting
    /// item removal between inventory close and deleteRemovedItems(),
    /// so deleteRemovedItems() fails to remove the translucent slots.
    equipped_items_to_remove_after_dialog: Vec<String>,

    /// If pouch screen is in holding mode. This state persists
    /// even when menu is closed
    ///
    /// While holding, you can only hold and unhold in pouch
    pub holding_in_inventory: bool,
}

/// Type of the screen and the data they hold
#[derive(Default, Clone)]
pub enum Screen {
    /// In the overworld, no additional screens
    #[default]
    Overworld,
    /// In the inventory screen
    Inventory(sim::PouchScreen),
    /// In shop dialog (selling sellable items)
    Shop(sim::ShopScreen),
}

impl ScreenSystem {
    pub fn current_screen(&self) -> &Screen {
        &self.screen
    }

    pub fn current_screen_mut(&mut self) -> &mut Screen {
        Arc::make_mut(&mut self.screen)
    }

    pub fn set_remove_held_after_dialog(&mut self) {
        self.remove_held_item_after_dialog = true;
    }

    pub fn set_remove_equipment_after_dialog(&mut self, name: &str) {
        self.equipped_items_to_remove_after_dialog
            .push(name.to_string())
    }

    pub fn transition_to_inventory(
        &mut self,
        ctx: &mut sim::Context<&mut Cpu2>,
        overworld: &mut sim::OverworldSystem,
        manual: bool,
        errors: &mut Vec<ErrorReport>,
    ) -> Result<bool, processor::Error> {
        match self.screen.as_ref() {
            Screen::Inventory(_) => {
                if manual {
                    errors.push(sim_warning!(ctx.span, UselessScreenTransition));
                }
                return Ok(true);
            }
            Screen::Overworld => {}
            // if the screen cannot be transition directly to inventory
            // screen, close it first to go back to overworld
            Screen::Shop(_) => {
                if !self.transition_to_overworld(ctx, overworld, false, errors)? {
                    return Ok(false);
                }
            }
        }

        if manual {
            self.is_manually_switched = true;
        }

        *self.current_screen_mut() = Screen::Inventory(sim::PouchScreen::open(ctx.cpu(), false)?);
        log::debug!("inventory screen opened successfully");

        Ok(true)
    }

    pub fn transition_to_shop_buying(
        &mut self,
        ctx: &mut sim::Context<&mut Cpu2>,
        overworld: &mut sim::OverworldSystem,
        manual: bool,
        errors: &mut Vec<ErrorReport>,
    ) -> Result<bool, processor::Error> {
        match self.screen.as_ref() {
            // close inventory first to return to overworld
            Screen::Inventory(_) => {
                if !self.transition_to_overworld(ctx, overworld, false, errors)? {
                    return Ok(false);
                }
            }
            Screen::Overworld => {}
            Screen::Shop(sim::ShopScreen::Buy) => {
                if manual {
                    errors.push(sim_warning!(ctx.span, UselessScreenTransition));
                }
                return Ok(true);
            }
            // shop screen can be switched without going back to overworld
            Screen::Shop(sim::ShopScreen::Sell(_)) => {
                if manual {
                    self.is_manually_switched = true;
                }
                match self.current_screen_mut() {
                    Screen::Shop(shop) => {
                        shop.transition_to_buy(ctx.cpu())?;
                    }
                    // unreachable: outer match
                    _ => unreachable!(),
                }
                return Ok(true);
            }
        }
        // open buying screen from overworld by talking to NPC

        // cannot talk to shop while holding items
        // ensure player is not holding items
        let should_drop = match overworld.predrop_for_action(ctx.span, errors) {
            sim::OverworldPreDropResult::Holding => {
                // cannot sell while holding, stop
                return Ok(false);
            }
            sim::OverworldPreDropResult::AutoDrop => true,
            sim::OverworldPreDropResult::Ok => false,
        };
        self.remove_held_item_after_dialog = should_drop;
        if manual {
            self.is_manually_switched = true;
        }
        *self.current_screen_mut() = Screen::Shop(sim::ShopScreen::Buy);

        Ok(true)
    }

    pub fn transition_to_shop_selling(
        &mut self,
        ctx: &mut sim::Context<&mut Cpu2>,
        overworld: &mut sim::OverworldSystem,
        manual: bool,
        errors: &mut Vec<ErrorReport>,
    ) -> Result<bool, processor::Error> {
        match self.screen.as_ref() {
            // close inventory first to return to overworld
            Screen::Inventory(_) => {
                if !self.transition_to_overworld(ctx, overworld, false, errors)? {
                    return Ok(false);
                }
            }
            Screen::Overworld => {}
            Screen::Shop(sim::ShopScreen::Sell(_)) => {
                if manual {
                    errors.push(sim_warning!(ctx.span, UselessScreenTransition));
                }
                return Ok(true);
            }
            // shop screen can be switched without going back to overworld
            Screen::Shop(sim::ShopScreen::Buy) => {
                if manual {
                    self.is_manually_switched = true;
                }
                *self.current_screen_mut() = Screen::Shop(sim::ShopScreen::open_sell(ctx.cpu())?);
                return Ok(true);
            }
        }
        // open selling screen from overworld by talking to NPC

        // cannot talk to shop while holding items
        // ensure player is not holding items
        let should_drop = match overworld.predrop_for_action(ctx.span, errors) {
            sim::OverworldPreDropResult::Holding => {
                // cannot sell while holding, stop
                return Ok(false);
            }
            sim::OverworldPreDropResult::AutoDrop => true,
            sim::OverworldPreDropResult::Ok => false,
        };
        self.remove_held_item_after_dialog = should_drop;
        if manual {
            self.is_manually_switched = true;
        }
        *self.current_screen_mut() = Screen::Shop(sim::ShopScreen::open_sell(ctx.cpu())?);

        Ok(true)
    }

    pub fn transition_to_overworld(
        &mut self,
        ctx: &mut sim::Context<&mut Cpu2>,
        overworld: &mut sim::OverworldSystem,
        manual: bool,
        errors: &mut Vec<ErrorReport>,
    ) -> Result<bool, processor::Error> {
        if matches!(self.screen.as_ref(), Screen::Overworld) {
            if manual {
                errors.push(sim_warning!(ctx.span, UselessScreenTransition));
            }
            return Ok(true);
        }
        // not allow auto switch to overworld if screen was manually switched
        if self.is_manually_switched && !manual {
            errors.push(sim_warning!(ctx.span, CannotAutoSwitchScreen));
            return Ok(false);
        }
        // after returning to overworld, allow automatic screen switch again
        self.is_manually_switched = false;
        let screen = Arc::make_mut(&mut self.screen);
        let drop_items = self.remove_held_item_after_dialog;
        self.remove_held_item_after_dialog = false;
        let remove_equipments = std::mem::take(&mut self.equipped_items_to_remove_after_dialog);
        screen.transition_to_overworld(
            ctx,
            overworld,
            self.menu_overload,
            drop_items,
            &remove_equipments,
        )?;
        if drop_items {
            self.holding_in_inventory = false;
        }

        Ok(true)
    }
}

impl Screen {
    /// Get the type for inventory view binding
    pub fn iv_type(&self) -> iv::Screen {
        match self {
            Screen::Overworld => iv::Screen::Overworld,
            Screen::Inventory(_) => iv::Screen::Inventory,
            Screen::Shop(_) => iv::Screen::Shop,
        }
    }

    pub fn is_overworld(&self) -> bool {
        matches!(self, Screen::Overworld)
    }

    pub fn is_inventory_or_overworld(&self) -> bool {
        matches!(self, Screen::Overworld | Screen::Inventory(_))
    }

    pub fn is_inventory(&self) -> bool {
        self.as_inventory().is_some()
    }

    pub fn is_shop(&self) -> bool {
        matches!(self, Screen::Shop(_))
    }

    pub fn as_inventory(&self) -> Option<&sim::PouchScreen> {
        match self {
            Screen::Inventory(inv) => Some(inv),
            _ => None,
        }
    }

    pub fn as_inventory_mut(&mut self) -> Option<&mut sim::PouchScreen> {
        match self {
            Screen::Inventory(inv) => Some(inv),
            _ => None,
        }
    }

    pub fn as_selling(&self) -> Option<&sim::ScreenItems> {
        match self {
            Screen::Shop(sim::ShopScreen::Sell(inv)) => Some(inv),
            _ => None,
        }
    }

    pub fn as_selling_mut(&mut self) -> Option<&mut sim::ScreenItems> {
        match self {
            Screen::Shop(sim::ShopScreen::Sell(inv)) => Some(inv),
            _ => None,
        }
    }

    fn transition_to_overworld(
        &mut self,
        ctx: &mut sim::Context<&mut Cpu2>,
        overworld: &mut sim::OverworldSystem,
        menu_overload: bool,
        drop_items: bool,
        remove_equipments: &[String],
    ) -> Result<(), processor::Error> {
        match self {
            Self::Overworld => {
                log::warn!("transition_to_overworld called but screen is already overworld");
                return Ok(());
            }
            Self::Inventory(inv_screen) => {
                if !menu_overload {
                    log::debug!("updating overworld equiments");
                    if inv_screen.weapon_state.to_delete {
                        overworld.weapon = None;
                    } else {
                        overworld.change_player_equipment(
                            inv_screen.weapon_state.item,
                            ctx.cpu().proc.memory(),
                        )?;
                    }
                    if inv_screen.bow_state.to_delete {
                        overworld.bow = None;
                    } else {
                        overworld.change_player_equipment(
                            inv_screen.bow_state.item,
                            ctx.cpu().proc.memory(),
                        )?;
                    }
                    if inv_screen.shield_state.to_delete {
                        overworld.shield = None;
                    } else {
                        overworld.change_player_equipment(
                            inv_screen.shield_state.item,
                            ctx.cpu().proc.memory(),
                        )?;
                    }
                } else {
                    log::debug!("not updating overworld equipments because of menu overload");
                }
                #[derive(Default)]
                struct State {
                    actors: Vec<String>,
                    menu_overload: bool,
                }
                let state = linker::events::CreateHoldingItem::execute_subscribed(
                    ctx.cpu(),
                    State {
                        menu_overload,
                        ..Default::default()
                    },
                    |state, name| {
                        if !state.menu_overload {
                            state.actors.push(name);
                        }
                    },
                    linker::create_holding_items,
                )?;
                log::debug!("spawning overworld holding items: {:?}", state.actors);
                overworld.spawn_held_items(state.actors);
                // TODO: there is a slight inaccuracy here. When closing inventory,
                // if player started holding inventory but didn't hold any item,
                // the holding state should end.
                // Could also be possible this check is done the next time inventory is opened,
                // but there's no way to know right now
            }
            Self::Shop(_) => {}
        }
        for x in remove_equipments {
            log::debug!("removing {x} on returning to overworld");
            linker::remove_weapon_if_equipped(ctx.cpu(), x)?;
        }
        log::debug!("removing translucent items on returning to overworld");
        linker::delete_removed_items(ctx.cpu())?;

        if !menu_overload {
            overworld.spawn_ground_weapons();
        } else {
            overworld.clear_spawning_weapons();
        }

        if drop_items {
            log::debug!("removing held items on returning to overworld");
            linker::remove_held_items(ctx.cpu())?;
            overworld.drop_held_items();
        }

        *self = Self::Overworld;
        Ok(())
    }
}
