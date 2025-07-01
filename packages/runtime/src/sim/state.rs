use blueflame::processor::{CrashReport, Process};
use skybook_parser::cir;

use crate::error::{Report, sim_error};
use crate::{exec, sim};

/// The state of the simulator
#[derive(Clone, Default)]
pub struct State {
    /// Current game state
    pub game: Game,
    // /// named save data
    // saves: HashMap<String, Arc<gdt::TriggerParam>>,
    // /// The "manual" or "default" save (what is used if a name is not specified when saving)
    // manual_save: Option<gdt::TriggerParam>,
    /// If the screen was manually changed by a command
    ///
    /// If so, the simulator will not automatically change screen
    /// until the screen returns to overworld
    pub is_screen_manually_changed: bool,
}

#[derive(Clone, Default)]
pub enum Game {
    /// Game is never started
    #[default]
    Uninit,
    /// Game is running
    Running(Box<GameState>),
    /// Game has crashed in the last step (must manually reboot)
    Crashed(CrashReport),
    /// Game has crashed in a previous step
    PreviousCrash,
}

/// The state of the running game in the simulator
#[derive(Clone)]
pub struct GameState {
    /// Running game's process
    pub process: Process,
    /// Simulated systems in the game
    pub systems: GameSystems,
}

#[derive(Default, Clone)]
pub struct GameSystems {
    /// Simulation of screens in the game
    pub screen: sim::ScreenSystem,
    /// Simulation of the overworld
    pub overworld: sim::OverworldSystem,
}

impl State {
    pub async fn execute_step(
        self,
        ctx: sim::Context<&sim::Runtime>,
        step: &cir::Step,
    ) -> Result<Report<Self>, exec::Error> {
        match step.command() {
            cir::Command::Get(items) => self.handle_get(ctx, items, false).await,
            cir::Command::GetPause(items) => self.handle_get(ctx, items, true).await,
            cir::Command::PickUp(items) => self.handle_pick_up(ctx, items, false).await,
            // TODO: pickup
            cir::Command::OpenInv => self.handle_pause(ctx).await,
            cir::Command::CloseInv => self.handle_unpause(ctx).await,
            cir::Command::Hold(items) => self.handle_hold(ctx, items, false).await,
            cir::Command::HoldAttach(items) => self.handle_hold(ctx, items, true).await,
            cir::Command::Unhold => self.handle_unhold(ctx).await,
            cir::Command::Drop(items) => self.handle_drop(ctx, items.as_deref()).await,
            cir::Command::SuBreak(count) => self.handle_su_break(ctx, *count).await,
            cir::Command::SuRemove(items) => self.handle_su_remove(ctx, items).await,

            cir::Command::OpenShop => self.handle_open_shop(ctx).await,
            cir::Command::CloseShop => self.handle_close_shop(ctx).await,
            cir::Command::Sell(items) => self.handle_sell(ctx, items).await,
            _ => Ok(Report::error(self, sim_error!(ctx.span, Unimplemented))),
        }
    }

    #[rustfmt::skip]
    async fn handle_get(self, rt: sim::Context<&sim::Runtime>,
        items: &[cir::ItemSpec], pause_after: bool,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling GET command");
        let items = items.to_vec();
        self.with_game(rt, async move |game, rt| { rt.execute(move |cpu| { cpu.execute_reporting(game, |mut cpu2, sys, errors| {

            sim::actions::get_items(&mut cpu2, sys, errors, &items, pause_after)?;
            sys.overworld.despawn_items();
            Ok(())

        }) }) .await }) .await
    }

    #[rustfmt::skip]
    async fn handle_pick_up(self, rt: sim::Context<&sim::Runtime>,
        items: &[cir::ItemSelectSpec], pause_after: bool,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling PICKUP command");
        let items = items.to_vec();
        self.with_game(rt, async move |game, rt| { rt.execute(move |cpu| { cpu.execute_reporting(game, |mut cpu2, sys, errors| {

            sim::actions::pick_up_items(&mut cpu2, sys, errors, &items, pause_after)?;
            sys.overworld.despawn_items();
            Ok(())

        }) }) .await }) .await
    }

    #[rustfmt::skip]
    async fn handle_pause(self, rt: sim::Context<&sim::Runtime>) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling PAUSE command");
        self.with_game(rt, async move |game, rt| { rt.execute(move |cpu| { cpu.execute_reporting(game, |mut cpu2, sys, errors| {

            let _ = sys.screen
                .transition_to_inventory(&mut cpu2, &mut sys.overworld, true, errors)?;
            Ok(())

        }) }) .await }) .await
    }

    #[rustfmt::skip]
    async fn handle_unpause(self, rt: sim::Context<&sim::Runtime>,) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling UNPAUSE command");
        self.with_game(rt, async move |game, rt| { rt.execute(move |cpu| { cpu.execute_reporting(game, |mut cpu2, sys, errors| {

            if !sys.screen.current_screen().is_inventory() {
                errors.push(sim_error!(cpu2.span, NotRightScreen));
                return Ok(());
            }
            sys.screen.transition_to_overworld(
                &mut cpu2,
                &mut sys.overworld,
                true,
                errors,
            )?;
            sys.overworld.despawn_items();
            Ok(())

        }) }) .await }) .await
    }

    #[rustfmt::skip]
    async fn handle_hold(self, rt: sim::Context<&sim::Runtime>,
        items: &[cir::ItemSelectSpec], attached: bool,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling HOLD command");
        let items = items.to_vec();
        self.with_game(rt, async move |game, rt| { rt.execute(move |cpu| { cpu.execute_reporting(game, |mut cpu2, sys, errors| {
            sim::actions::hold_items(&mut cpu2, sys, errors, &items, attached)
        }) }) .await }) .await
    }

    #[rustfmt::skip]
    async fn handle_unhold(self, rt: sim::Context<&sim::Runtime>) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling HOLD command");
        self.with_game(rt, async move |game, rt| { rt.execute(move |cpu| { cpu.execute_reporting(game, |mut cpu2, sys, errors| {
            sim::actions::unhold(&mut cpu2, sys, errors)
        }) }) .await }) .await
    }

    #[rustfmt::skip]
    async fn handle_drop(self, rt: sim::Context<&sim::Runtime>,
        items: Option<&[cir::ItemSelectSpec]>,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling DROP command");
        let items = items.map(|x| x.to_vec());
        self.with_game(rt, async move |game, rt| { rt.execute(move |cpu| { cpu.execute_reporting(game, |mut cpu2, sys, errors| {
            if let Some(items) = items {
                sim::actions::hold_items(&mut cpu2, sys, errors, &items, false)?;
            }
            sys.overworld.despawn_items();
            sim::actions::drop_held(&mut cpu2, sys, errors)
        }) }) .await }) .await
    }

    #[rustfmt::skip]
    async fn handle_open_shop(self, rt: sim::Context<&sim::Runtime>,) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling OPEN SHOP command");
        self.with_game(rt, async move |game, rt| { rt.execute(move |cpu| { cpu.execute_reporting(game, |mut cpu2, sys, errors| {

            let _ = sys.screen
                .transition_to_shop_buying(&mut cpu2, &mut sys.overworld, true, errors)?;
            Ok(())

        }) }) .await }) .await
    }

    #[rustfmt::skip]
    async fn handle_close_shop(self, rt: sim::Context<&sim::Runtime>,) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling CLOSE SHOP command");
        self.with_game(rt, async move |game, rt| { rt.execute(move |cpu| { cpu.execute_reporting(game, |mut cpu2, sys, errors| {

            if !sys.screen.current_screen().is_shop() {
                errors.push(sim_error!(cpu2.span, NotRightScreen));
                return Ok(());
            }
            sys.screen.transition_to_overworld(
                &mut cpu2,
                &mut sys.overworld,
                true,
                errors,
            )?;
            sys.overworld.despawn_items();
            Ok(())

        }) }) .await }) .await
    }

    #[rustfmt::skip]
    async fn handle_sell(self, rt: sim::Context<&sim::Runtime>,
        items: &[cir::ItemSelectSpec],
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling SELL command");
        let items = items.to_vec();
        self.with_game(rt, async move |game, rt| { rt.execute(move |cpu| { cpu.execute_reporting(game, |mut cpu2, sys, errors| {
            sim::actions::sell_items(&mut cpu2, sys, errors, &items)
        }) }) .await }) .await
    }

    #[rustfmt::skip]
    async fn handle_su_break(self, rt: sim::Context<&sim::Runtime>,
        count: i32
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling !BREAK command");
        self.with_game(rt, async move |game, rt| { rt.execute(move |cpu| { cpu.execute_reporting(game, |mut cpu2, _, _| {
            sim::actions::force_break_slot(&mut cpu2, count)
        }) }) .await }) .await
    }

    #[rustfmt::skip]
    async fn handle_su_remove(self, rt: sim::Context<&sim::Runtime>,
        items: &[cir::ItemSelectSpec]
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling !REMOVE command");
        let items = items.to_vec();
        self.with_game(rt, async move |game, rt| { rt.execute(move |cpu| { cpu.execute_reporting(game, |mut cpu2, sys, errors| {
            sim::actions::force_remove_item(&mut cpu2, sys, errors, &items)
        }) }) .await }) .await
    }
}

impl GameState {
    pub fn new(process: Process) -> Self {
        Self {
            process,
            systems: GameSystems::default(),
        }
    }
}
