use std::sync::Arc;

use blueflame::game::gdt;
use blueflame::processor::{CrashReport, Process};
use skybook_parser::cir;
use teleparse::Span;

use crate::error::{Report, sim_error};
use crate::{exec, sim};

/// The state of one step in the simulation.
#[derive(Clone, Default)]
pub struct State {
    /// Current game state
    pub game: Game,
    /// Current args
    pub args: Option<Box<StateArgs>>,
    /// Named save data (clone on write)
    ///
    /// This needs to be kept in insertion order so the order
    /// doesn't feel random to users
    saves: Arc<Vec<(String, Arc<gdt::TriggerParam>)>>,
    /// The "manual" or "default" save (what is used if a name is not specified when saving)
    pub manual_save: Option<Arc<gdt::TriggerParam>>,
}

impl State {
    /// Get names of all saves
    pub fn save_names(&self) -> Vec<String> {
        self.saves
            .as_ref()
            .iter()
            .map(|(n, _)| n.to_string())
            .collect()
    }
    /// Get a manual save (if name is `None`) or a named save
    pub fn save_by_name(&self, name: Option<&str>) -> Option<Arc<gdt::TriggerParam>> {
        match name {
            None => self.manual_save.as_ref().map(Arc::clone),
            Some(name) => {
                for (save_name, save) in self.saves.as_ref() {
                    if save_name == name {
                        return Some(Arc::clone(save));
                    }
                }
                None
            }
        }
    }

    /// Set a manual save (if name is `None`) or a named save
    pub fn set_save_by_name(&mut self, name: Option<&str>, data: Arc<gdt::TriggerParam>) {
        match name {
            None => self.manual_save = Some(data),
            Some(name) => {
                let saves: &mut Vec<_> = Arc::make_mut(&mut self.saves);
                for (save_name, save) in saves.iter_mut() {
                    if save_name == name {
                        *save = data;
                        return;
                    }
                }
                saves.push((name.to_string(), data));
            }
        }
    }
}

/// State args that could affect execution of the next state
#[derive(Clone, Default)]
pub struct StateArgs {
    /// Perform item smuggle for arrowless offset
    pub smug: Option<Span>,
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
    /// Specify how much value is decreases per use
    pub per_use: Option<i32>,
}

#[derive(Clone, Default)]
pub enum Game {
    /// Game is never started
    ///
    /// This is the only state that the simulator will allow
    /// auto-starting a new game
    #[default]
    Uninit,
    /// Game is running
    Running(Box<GameState>),
    /// Game has crashed in the last step
    Crashed(CrashReport),
    /// Game has crashed in a previous step
    PreviousCrash,
    /// Game was manually closed in the last step
    Closed,
    /// Game was manually closed in a previous step
    PreviousClosed,
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

macro_rules! execute_command {
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
            X::Multi(cmds) => {
                let mut errors = vec![];
                let mut state = self;
                for x in cmds {
                    // the args handling is a bit wacky
                    let result = state.handle_command(ctx.clone(), args.clone(), x).await?;
                    errors.extend(result.errors);
                    state = result.value;
                }
                Ok(Report::with_errors(state, errors))
            }
            X::CoSmug => set_arg!(self, args, smug, Some(ctx.span)),
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
            X::CoPerUse(x) => set_arg!(self, args, per_use, Some(*x)),

            command => self.handle_command(ctx, args, command).await,
        }
    }
    async fn handle_command(
        mut self,
        ctx: sim::Context<&sim::Runtime>,
        args: Option<Box<StateArgs>>,
        command: &cir::Command,
    ) -> Result<Report<Self>, exec::Error> {
        use cir::Command as X;
        match command {
            X::Get(items) => self.handle_get(ctx, items, args.as_deref()).await,
            X::PickUp(items) => self.handle_pick_up(ctx, items, args.as_deref()).await,
            X::OpenInv => self.handle_pause(ctx).await,
            X::CloseInv => self.handle_unpause(ctx).await,
            X::Hold(items) => self.handle_hold(ctx, items, args.as_deref()).await,
            X::Unhold => self.handle_unhold(ctx).await,
            X::Drop(items) => self.handle_drop(ctx, items, args.as_deref(), false).await,
            X::Dnp(items) => self.handle_drop(ctx, items, args.as_deref(), true).await,
            X::Eat(items) => self.handle_eat(ctx, items, args.as_deref()).await,
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
            X::Sort(spec) => {
                self.handle_sort(ctx, spec.category, spec.amount as usize, args.as_deref())
                    .await
            }

            X::Equip(items) => {
                self.handle_change_equip(ctx, items, args.as_deref(), true)
                    .await
            }
            X::Unequip(items) => {
                self.handle_change_equip(ctx, items, args.as_deref(), false)
                    .await
            }
            X::Use(item, times) => self.handle_use(ctx, item, *times, args.as_deref()).await,

            X::OpenShop => self.handle_open_shop(ctx).await,
            X::CloseShop => self.handle_close_shop(ctx).await,
            X::Sell(items) => self.handle_sell(ctx, items).await,
            X::Buy(items) => self.handle_buy(ctx, items, args.as_deref()).await,

            X::Save(name) => self.handle_save(ctx, name.as_deref()).await,
            X::Reload(name) => self.handle_reload(ctx, name.as_deref(), false).await,
            X::CloseGame => {
                log::debug!("handling CLOSE-GAME");
                self.game = Game::Closed;
                Ok(Report::new(self))
            }
            X::NewGame => self.handle_reload(ctx, None, true).await,

            X::SuBreak(count) => self.handle_su_break(ctx, *count).await,
            X::SuInit(items) => self.handle_su_add_slot(ctx, items, true).await,
            X::SuAddSlot(items) => self.handle_su_add_slot(ctx, items, false).await,
            X::SuRemove(items) => self.handle_su_remove(ctx, items).await,
            X::SuSwap(item1, item2) => self.handle_su_swap(ctx, item1, item2).await,
            X::SuWrite(meta, item) => self.handle_su_write(ctx, meta, item).await,
            X::SuSetGdt(name, meta) => self.handle_su_set_gdt(ctx, name, meta).await,
            X::SuArrowlessSmuggle => self.handle_su_arrowless_smuggle(ctx).await,

            _ => Ok(Report::error(self, sim_error!(ctx.span, Unimplemented))),
        }
    }

    async fn handle_get(
        self,
        rt: sim::Context<&sim::Runtime>,
        items: &[cir::ItemSpec],
        args: Option<&StateArgs>,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("handling GET");
        let items = items.to_vec();
        let (pause, accurate) = args
            .map(|args| (args.pause_during, args.accurately_simulate))
            .unwrap_or_default();
        execute_command!(self, rt, cpu, sys, errors => {
            sim::actions::get_items(&mut cpu, sys, errors, &items, pause, accurate)
        })
    }

    async fn handle_pick_up(
        self,
        rt: sim::Context<&sim::Runtime>,
        items: &[cir::ItemSelectSpec],
        args: Option<&StateArgs>,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("handling PICKUP");
        let items = items.to_vec();
        let pause = args.map(|args| args.pause_during).unwrap_or_default();
        execute_command!(self, rt, cpu, sys, errors => {
            sim::actions::pick_up_items(&mut cpu, sys, errors, &items, pause)
        })
    }

    async fn handle_pause(
        self,
        rt: sim::Context<&sim::Runtime>,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("handling PAUSE");
        execute_command!(self, rt, cpu, sys, errors => {
            sys.screen.transition_to_inventory(&mut cpu, &mut sys.overworld, true, errors)?;
            Ok(())
        })
    }

    async fn handle_unpause(
        self,
        rt: sim::Context<&sim::Runtime>,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("handling UNPAUSE");
        execute_command!(self, rt, cpu, sys, errors => {
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
        log::debug!("handling HOLD");
        let (smug, pe_target) = args
            .map(|x| (x.smug, x.entangle_target.as_ref().cloned()))
            .unwrap_or_default();
        let items = items.to_vec();
        execute_command!(self, rt, cpu, sys, errors => {
            sim::actions::hold_items(&mut cpu, sys, errors, &items, pe_target.as_ref(), smug)
        })
    }

    async fn handle_unhold(
        self,
        rt: sim::Context<&sim::Runtime>,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("handling UNHOLD");
        execute_command!(self, rt, cpu, sys, errors => {
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
        log::debug!("handling DROP");
        let (smug, pe_target, overworld, pause_during) = args
            .map(|x| {
                (
                    x.smug,
                    x.entangle_target.as_ref().cloned(),
                    x.overworld,
                    x.pause_during,
                )
            })
            .unwrap_or_default();
        let items = items.to_vec();
        execute_command!(self, rt, cpu, sys, errors => {
            sim::actions::drop_items(&mut cpu,
                sys, errors, &items, pe_target.as_ref(), smug, pick_up, overworld, pause_during)
        })
    }

    async fn handle_eat(
        self,
        rt: sim::Context<&sim::Runtime>,
        items: &[cir::ItemSelectSpec],
        args: Option<&StateArgs>,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("handling EAT");
        let pe_target = args
            .map(|x| x.entangle_target.as_ref().cloned())
            .unwrap_or_default();
        let items = items.to_vec();
        execute_command!(self, rt, cpu, sys, errors => {
            sim::actions::eat_items(&mut cpu, sys, errors, &items, pe_target.as_ref())
        })
    }

    async fn handle_entangle(
        self,
        rt: sim::Context<&sim::Runtime>,
        item: &cir::ItemSelectSpec,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("handling ENTANGLE");
        let item = item.clone();
        execute_command!(self, rt, cpu, sys, errors => {
            sim::actions::entangle_item(&mut cpu, sys, errors, &item)
        })
    }

    async fn handle_sort(
        self,
        rt: sim::Context<&sim::Runtime>,
        category: cir::Category,
        times: usize,
        args: Option<&StateArgs>,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("handling SORT");
        let (accurate, same_dialog) = args
            .map(|x| (x.accurately_simulate, x.same_dialog))
            .unwrap_or_default();
        execute_command!(self, rt, cpu, sys, errors => {
            sim::actions::sort_items(&mut cpu, sys, errors, category, times, accurate, same_dialog)
        })
    }

    async fn handle_change_equip(
        self,
        rt: sim::Context<&sim::Runtime>,
        items: &[cir::ItemSelectSpec],
        args: Option<&StateArgs>,
        is_equip: bool,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("handling CHGEQUIP");
        let (pe_target, is_dpad) = args
            .map(|x| (x.entangle_target.as_ref().cloned(), x.dpad))
            .unwrap_or_default();
        let items = items.to_vec();
        execute_command!(self, rt, cpu, sys, errors => {
            sim::actions::change_equip(&mut cpu,
                sys, errors, &items, pe_target.as_ref(), is_equip, is_dpad)
        })
    }

    async fn handle_use(
        self,
        rt: sim::Context<&sim::Runtime>,
        item: &cir::ItemNameSpec,
        times: usize,
        args: Option<&StateArgs>,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("handling USE");
        let per_use = args.and_then(|x| x.per_use);
        let item = item.clone();
        execute_command!(self, rt, cpu, sys, errors => {
            sim::actions::use_items(&mut cpu, sys, errors, &item, times, per_use)
        })
    }

    async fn handle_open_shop(
        self,
        rt: sim::Context<&sim::Runtime>,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("handling OPEN-SHOP");
        execute_command!(self, rt, cpu, sys, errors => {
            sys.screen.transition_to_shop_buying(&mut cpu, &mut sys.overworld, true, errors)?;
            Ok(())
        })
    }

    async fn handle_close_shop(
        self,
        rt: sim::Context<&sim::Runtime>,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("handling CLOSE-SHOP");
        execute_command!(self, rt, cpu, sys, errors => {
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
        log::debug!("handling SELL");
        let items = items.to_vec();
        execute_command!(self, rt, cpu, sys, errors => {
            sim::actions::sell_items(&mut cpu, sys, errors, &items)
        })
    }

    async fn handle_buy(
        self,
        rt: sim::Context<&sim::Runtime>,
        items: &[cir::ItemSpec],
        args: Option<&StateArgs>,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("handling BUY");
        let items = items.to_vec();
        let (pause, accurate, same_dialog) = args
            .map(|args| {
                (
                    args.pause_during,
                    args.accurately_simulate,
                    args.same_dialog,
                )
            })
            .unwrap_or_default();
        execute_command!(self, rt, cpu, sys, errors => {
            sim::actions::buy_items(&mut cpu, sys, errors, &items, pause, accurate, same_dialog)
        })
    }

    async fn handle_save(
        self,
        rt: sim::Context<&sim::Runtime>,
        name: Option<&str>,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling SAVE command");

        let (send, recv) = oneshot::channel();
        // allow overworld for auto saves
        let allow_overworld = name.is_some();

        let new_state = execute_command!(self, rt, cpu, sys, errors => {
            sim::actions::save(&mut cpu, sys, errors, allow_overworld, send)
        })?;
        let data = recv
            .recv()
            .map_err(|x| exec::Error::RecvResult(x.to_string()));
        let data = match data {
            Ok(x) => x,
            Err(e) => {
                log::error!("fail to receive save data: {e}");
                return Err(e);
            }
        };
        Ok(new_state.map(|mut state| {
            match data {
                Some(data) => state.set_save_by_name(name, data),
                // if data is not sent, that means save cannot be executed,
                // which is fine, and the error should be contained
                // in the report
                None => log::warn!("did not get save data from executor thread"),
            }
            state
        }))
    }

    async fn handle_reload(
        self,
        rt: sim::Context<&sim::Runtime>,
        name: Option<&str>,
        new_game: bool,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("handling RELOAD command");
        // find the save
        let save = if new_game {
            // use the GDT from "new game"
            let proc = match rt.runtime().initial_process() {
                Ok(x) => x,
                Err(e) => {
                    return Ok(Report::spanned(self, rt.span, e));
                }
            };
            let gdt = match sim::actions::get_save(&proc) {
                Ok(x) => x,
                Err(e) => {
                    log::error!("failed to load new-game save: {e}");
                    return Ok(Report::error(self, sim_error!(rt.span, Executor)));
                }
            };
            Arc::new(gdt)
        } else {
            let Some(save) = self.save_by_name(name) else {
                let error = match name {
                    Some(name) => sim_error!(rt.span, SaveNotFound(name.to_string())),
                    None => sim_error!(rt.span, NoManualSave),
                };
                return Ok(Report::error(self, error));
            };
            save
        };
        self.with_game_or_start(rt, async move |game, rt| {
            rt.execute(move |cpu| {
                cpu.execute_reporting(game, |mut cpu, sys, errors| {
                    sim::actions::reload(&mut cpu, sys, errors, save.as_ref())
                })
            })
            .await
        })
        .await
    }

    async fn handle_su_break(
        self,
        rt: sim::Context<&sim::Runtime>,
        count: i32,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("handling !BREAK");
        execute_command!(self, rt, cpu, _sys, _errors => {
            sim::actions::low_level::break_slot(&mut cpu, count)
        })
    }

    async fn handle_su_add_slot(
        self,
        rt: sim::Context<&sim::Runtime>,
        items: &[cir::ItemSpec],
        init: bool,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("handling !ADDSLOT");
        let items = items.to_vec();
        execute_command!(self, rt, cpu, _sys, errors => {
            sim::actions::low_level::add_slots(&mut cpu, errors, &items, init)
        })
    }

    async fn handle_su_remove(
        self,
        rt: sim::Context<&sim::Runtime>,
        items: &[cir::ItemSelectSpec],
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("handling !REMOVE");
        let items = items.to_vec();
        execute_command!(self, rt, cpu, sys, errors => {
            sim::actions::force_remove_item(&mut cpu, sys, errors, &items)
        })
    }

    async fn handle_su_swap(
        self,
        rt: sim::Context<&sim::Runtime>,
        item1: &cir::ItemSelectSpec,
        item2: &cir::ItemSelectSpec,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("handling !SWAP");
        let item1 = item1.clone();
        let item2 = item2.clone();
        execute_command!(self, rt, cpu, _sys, errors => {
            sim::actions::low_level::swap_items(&mut cpu, errors, &item1, &item2)
        })
    }

    async fn handle_su_write(
        self,
        rt: sim::Context<&sim::Runtime>,
        write_meta: &cir::ItemMeta,
        item: &cir::ItemSelectSpec,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("handling !WRITE");
        let write_meta = write_meta.clone();
        let item = item.clone();
        execute_command!(self, rt, cpu, _sys, errors => {
            sim::actions::low_level::write_meta(&mut cpu, errors, &write_meta, &item)
        })
    }

    async fn handle_su_set_gdt(
        self,
        rt: sim::Context<&sim::Runtime>,
        name: &str,
        meta: &cir::GdtMeta,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("handling !SETGDT");
        let name = name.to_string();
        let meta = meta.clone();
        execute_command!(self, rt, cpu, _sys, errors => {
            Ok(sim::actions::low_level::set_gdt(&mut cpu, &name, &meta, errors)?)
        })
    }

    async fn handle_su_arrowless_smuggle(
        self,
        rt: sim::Context<&sim::Runtime>,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("handling !SMUGARROWLESS");
        execute_command!(self, rt, cpu, sys, errors => {
            sim::actions::trigger_arrowless_smuggle(&mut cpu, sys, errors)
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
