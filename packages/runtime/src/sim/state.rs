use blueflame::processor::{CrashReport, Process};
use skybook_parser::cir;

use crate::error::{Report, sim_error};
use crate::{exec, sim};

/// The state of the simulator
#[derive(Clone, Default)]
pub struct State {
    /// Current game state
    pub game: Game,
    pub args: StateArgs,
    // /// named save data
    // saves: HashMap<String, Arc<gdt::TriggerParam>>,
    // /// The "manual" or "default" save (what is used if a name is not specified when saving)
    // manual_save: Option<gdt::TriggerParam>,
}

/// State args that could affect execution of the next state
#[derive(Clone, Default)]
pub struct StateArgs {
    /// Perform item smuggle for arrowless offset
    pub smug: bool,
    /// Assume next command has an item box, and open the pause menu during that text box
    pub item_box_pause: bool,
    /// Perform the next operation in the same dialog
    pub same_dialog: bool,
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

macro_rules! set_arg {
    ($state:ident, $args:ident, $arg:ident) => {{
        $args.$arg = true;
        $state.args = $args;
        return Ok(Report::new($state));
    }};
}

impl State {
    pub async fn execute_step(
        mut self,
        ctx: sim::Context<&sim::Runtime>,
        step: &cir::Step,
    ) -> Result<Report<Self>, exec::Error> {
        use cir::Command as X;
        let mut args = std::mem::take(&mut self.args);
        match step.command() {
            X::CoSmug => set_arg!(self, args, smug),
            X::CoItemBoxPause => set_arg!(self, args, item_box_pause),
            X::CoSameDialog => set_arg!(self, args, same_dialog),

            X::Get(items) => self.handle_get(ctx, items, &args).await,
            X::PickUp(items) => self.handle_pick_up(ctx, items, &args).await,
            X::OpenInv => self.handle_pause(ctx).await,
            X::CloseInv => self.handle_unpause(ctx).await,
            X::Hold(items) => self.handle_hold(ctx, items, &args).await,
            X::Unhold => self.handle_unhold(ctx).await,
            X::Drop(items) => self.handle_drop(ctx, items.as_deref()).await,
            X::SuBreak(count) => self.handle_su_break(ctx, *count).await,
            X::SuRemove(items) => self.handle_su_remove(ctx, items).await,

            X::OpenShop => self.handle_open_shop(ctx).await,
            X::CloseShop => self.handle_close_shop(ctx).await,
            X::Sell(items) => self.handle_sell(ctx, items).await,
            _ => Ok(Report::error(self, sim_error!(ctx.span, Unimplemented))),
        }
    }

    #[rustfmt::skip]
    async fn handle_get(self, rt: sim::Context<&sim::Runtime>,
        items: &[cir::ItemSpec], args: &StateArgs,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling GET command");
        let items = items.to_vec();
        let pause = args.item_box_pause;
        self.with_game(rt, async move |game, rt| { rt.execute(move |cpu| { cpu.execute_reporting(game, |mut cpu2, sys, errors| {

            sim::actions::get_items(&mut cpu2, sys, errors, &items, pause)?;
            sys.overworld.despawn_items();
            Ok(())

        }) }) .await }) .await
    }

    #[rustfmt::skip]
    async fn handle_pick_up(self, rt: sim::Context<&sim::Runtime>,
        items: &[cir::ItemSelectSpec], args: &StateArgs,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling PICKUP command");
        let items = items.to_vec();
        let pause = args.item_box_pause;
        self.with_game(rt, async move |game, rt| { rt.execute(move |cpu| { cpu.execute_reporting(game, |mut cpu2, sys, errors| {

            sim::actions::pick_up_items(&mut cpu2, sys, errors, &items, pause)?;
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
        items: &[cir::ItemSelectSpec], args: &StateArgs,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling HOLD command");
        let smug = args.smug;
        let items = items.to_vec();
        self.with_game(rt, async move |game, rt| { rt.execute(move |cpu| { cpu.execute_reporting(game, |mut cpu2, sys, errors| {
            sim::actions::hold_items(&mut cpu2, sys, errors, &items, smug)
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
