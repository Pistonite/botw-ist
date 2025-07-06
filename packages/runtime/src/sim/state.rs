use blueflame::processor::{CrashReport, Process};
use skybook_parser::cir;

use crate::error::{Report, sim_error};
use crate::{exec, sim};

/// The state of the simulator
#[derive(Clone, Default)]
pub struct State {
    /// Current game state
    pub game: Game,
    pub args: Option<Box<StateArgs>>,
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
    /// Open the pause menu during the next command (effect may differ depending on the command)
    pub pause_during: bool,
    /// Perform the next operation in the same dialog
    pub same_dialog: bool,
    /// Disable optimizations that may affect accuracy
    pub accurately_simulate: bool,
    /// Target item for PE (item that receives the prompt)
    pub entangle_target: Option<cir::ItemSelectSpec>,
    /// Specify the action should be done in overworld, if possible in both overworld and another
    /// screen
    pub overworld: bool,
    /// Specify the action should be done in dpad quick menu,
    /// if possible in both dpad and pause menu
    pub dpad: bool,
    /// Specify weapon throw to not break the weapon
    pub non_breaking: bool,
    /// Specify weapon throw to break the weapon
    pub breaking: bool,
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

impl GameSystems {
    /// Process weapon spawning if in overworld
    pub fn check_weapon_spawn(&mut self) {
        if self.screen.current_screen().is_overworld() {
            if !self.screen.menu_overload {
                self.overworld.spawn_ground_weapons()
            } else {
                self.overworld.clear_spawning_weapons()
            }
        }
    }
}

macro_rules! set_arg {
    ($state:ident, $args:ident, $arg:ident, $value:expr) => {{
        $state.args = Some(match $args {
            None => {
                let mut x = StateArgs::default();
                x.$arg = $value;
                Box::new(x)
            }
            Some(mut x) => {
                x.$arg = $value;
                x
            }
        });
        return Ok(Report::new($state));
    }};
}

macro_rules! in_game {
    ($self:ident, $rt:ident, $cpu:ident, $sys:ident, $errors:ident => $block:block) => {{
        $self
            .with_game($rt, async move |game, rt| {
                rt.execute(move |cpu| cpu.execute_reporting(game, |mut $cpu, $sys, $errors| $block))
                    .await
            })
            .await
    }};
}

impl State {
    pub async fn execute_step(
        mut self,
        ctx: sim::Context<&sim::Runtime>,
        step: &cir::Step,
    ) -> Result<Report<Self>, exec::Error> {
        use cir::Command as X;
        let args = std::mem::take(&mut self.args);
        match step.command() {
            X::CoSmug => set_arg!(self, args, smug, true),
            X::CoPauseDuring => set_arg!(self, args, pause_during, true),
            X::CoSameDialog => set_arg!(self, args, same_dialog, true),
            X::CoAccuratelySimulate => set_arg!(self, args, accurately_simulate, true),
            X::CoTargeting(spec) => {
                set_arg!(self, args, entangle_target, Some(spec.as_ref().clone()))
            }
            X::CoOverworld => set_arg!(self, args, overworld, true),
            X::CoDpad => set_arg!(self, args, dpad, true),
            X::CoNonBreaking => set_arg!(self, args, non_breaking, true),
            X::CoBreaking => set_arg!(self, args, breaking, true),

            X::Get(items) => self.handle_get(ctx, items, args.as_deref()).await,
            X::PickUp(items) => self.handle_pick_up(ctx, items, args.as_deref()).await,
            X::OpenInv => self.handle_pause(ctx).await,
            X::CloseInv => self.handle_unpause(ctx).await,
            X::Hold(items) => self.handle_hold(ctx, items, args.as_deref()).await,
            X::Unhold => self.handle_unhold(ctx).await,
            X::Drop(items) => self.handle_drop(ctx, items, args.as_deref(), false).await,
            X::SuBreak(count) => self.handle_su_break(ctx, *count).await,
            X::SuRemove(items) => self.handle_su_remove(ctx, items).await,

            X::OpenShop => self.handle_open_shop(ctx).await,
            X::CloseShop => self.handle_close_shop(ctx).await,
            X::Sell(items) => self.handle_sell(ctx, items).await,

            X::Entangle(item) => {
                self.args = Some(match args {
                    None => Box::new(StateArgs {
                        entangle_target: Some(item.as_ref().clone()),
                        ..Default::default()
                    }),
                    Some(mut x) => {
                        x.entangle_target = Some(item.as_ref().clone());
                        x
                    }
                });
                self.handle_entangle(ctx, item).await
            }

            _ => Ok(Report::error(self, sim_error!(ctx.span, Unimplemented))),
        }
    }

    async fn handle_get(
        self,
        rt: sim::Context<&sim::Runtime>,
        items: &[cir::ItemSpec],
        args: Option<&StateArgs>,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling GET command");
        let items = items.to_vec();
        let (pause, accurate) = args
            .map(|args| (args.pause_during, args.accurately_simulate))
            .unwrap_or_default();
        in_game!(self, rt, cpu, sys, errors => {
            sim::actions::get_items(&mut cpu, sys, errors, &items, pause, accurate)
        })
    }

    async fn handle_pick_up(
        self,
        rt: sim::Context<&sim::Runtime>,
        items: &[cir::ItemSelectSpec],
        args: Option<&StateArgs>,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling PICKUP command");
        let items = items.to_vec();
        let pause = args.map(|args| args.pause_during).unwrap_or_default();
        in_game!(self, rt, cpu, sys, errors => {
            sim::actions::pick_up_items(&mut cpu, sys, errors, &items, pause)
        })
    }

    async fn handle_pause(
        self,
        rt: sim::Context<&sim::Runtime>,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling PAUSE command");
        in_game!(self, rt, cpu, sys, errors => {
            sys.screen.transition_to_inventory(&mut cpu, &mut sys.overworld, true, errors)?;
            Ok(())
        })
    }

    async fn handle_unpause(
        self,
        rt: sim::Context<&sim::Runtime>,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling UNPAUSE command");
        in_game!(self, rt, cpu, sys, errors => {
            if !sys.screen.current_screen().is_inventory() {
                errors.push(sim_error!(cpu.span, NotRightScreen));
                return Ok(());
            }
            sys.screen.transition_to_overworld(
                &mut cpu,
                &mut sys.overworld,
                true,
                errors,
            )?;
            Ok(())
        })
    }

    async fn handle_hold(
        self,
        rt: sim::Context<&sim::Runtime>,
        items: &[cir::ItemSelectSpec],
        args: Option<&StateArgs>,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling HOLD command");
        let (smug, pe_target) = args
            .map(|x| (x.smug, x.entangle_target.as_ref().cloned()))
            .unwrap_or_default();
        let items = items.to_vec();
        in_game!(self, rt, cpu, sys, errors => {
            sim::actions::hold_items(&mut cpu, sys, errors, &items, pe_target.as_ref(), smug)
        })
    }

    async fn handle_unhold(
        self,
        rt: sim::Context<&sim::Runtime>,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling UNHOLD command");
        in_game!(self, rt, cpu, sys, errors => {
            if sys.screen.current_screen().is_overworld() {
                sys.overworld.despawn_items();
            }
            sim::actions::unhold(&mut cpu, sys, errors)
        })
    }

    async fn handle_drop(
        self,
        rt: sim::Context<&sim::Runtime>,
        items: &[cir::ItemSelectSpec],
        args: Option<&StateArgs>,
        pick_up: bool,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling DROP command");
        let (pe_target, overworld, pause_during) = args
            .map(|x| {
                (
                    x.entangle_target.as_ref().cloned(),
                    x.overworld,
                    x.pause_during,
                )
            })
            .unwrap_or_default();
        let items = items.to_vec();
        in_game!(self, rt, cpu, sys, errors => {
            sim::actions::drop_items(&mut cpu,
                sys, errors, &items, pe_target.as_ref(), pick_up, overworld, pause_during)
        })
    }

    async fn handle_open_shop(
        self,
        rt: sim::Context<&sim::Runtime>,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling OPEN SHOP command");
        in_game!(self, rt, cpu, sys, errors => {
            sys.screen.transition_to_shop_buying(&mut cpu, &mut sys.overworld, true, errors)?;
            Ok(())
        })
    }

    async fn handle_close_shop(
        self,
        rt: sim::Context<&sim::Runtime>,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling CLOSE SHOP command");
        in_game!(self, rt, cpu, sys, errors => {
            if !sys.screen.current_screen().is_shop() {
                errors.push(sim_error!(cpu.span, NotRightScreen));
                return Ok(());
            }
            sys.screen.transition_to_overworld(
                &mut cpu,
                &mut sys.overworld,
                true,
                errors,
            )?;
            sys.overworld.despawn_items();
            Ok(())
        })
    }

    async fn handle_sell(
        self,
        rt: sim::Context<&sim::Runtime>,
        items: &[cir::ItemSelectSpec],
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling SELL command");
        let items = items.to_vec();
        in_game!(self, rt, cpu, sys, errors => {
            sim::actions::sell_items(&mut cpu, sys, errors, &items)
        })
    }

    async fn handle_su_break(
        self,
        rt: sim::Context<&sim::Runtime>,
        count: i32,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling !BREAK command");
        in_game!(self, rt, cpu, _sys, _errors => {
            sim::actions::force_break_slot(&mut cpu, count)
        })
    }

    async fn handle_su_remove(
        self,
        rt: sim::Context<&sim::Runtime>,
        items: &[cir::ItemSelectSpec],
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling !REMOVE command");
        let items = items.to_vec();
        in_game!(self, rt, cpu, sys, errors => {
            sim::actions::force_remove_item(&mut cpu, sys, errors, &items)
        })
    }

    async fn handle_entangle(
        self,
        rt: sim::Context<&sim::Runtime>,
        item: &cir::ItemSelectSpec,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling ENTANGLE command");
        let item = item.clone();
        in_game!(self, rt, cpu, sys, errors => {
            sim::actions::entangle_item(&mut cpu, sys, errors, &item)
        })
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
