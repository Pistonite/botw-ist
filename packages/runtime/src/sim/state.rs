use blueflame::linker;
use blueflame::processor::{Cpu1, CrashReport, Process};
use skybook_parser::cir;

use super::util;
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

    // /// If inventory/dialog screen is activated manually,
    // /// so auto-scoping will be disabled until returned to overworld screen
    // is_manual_scope: bool,
    //
    // /// If auto scope is enabled at all
    // enable_auto_scope: bool,
}

#[derive(Clone, Default)]
pub enum Game {
    /// Game is never started
    #[default]
    Uninit,
    /// Game is running
    Running(GameState),
    /// Game has crashed (must manually reboot)
    Crashed(CrashReport),
}

/// The state of the running game in the simulator
#[derive(Clone)]
pub struct GameState {
    // /// Current screen, only valid if game is running
    // screen: Screen,
    /// Running game's process
    pub process: Process,
    //
    // /// Current actors in the overworld
    // /// TODO: make this copy on write and Arc
    // ovwd_weapon: Option<ActorState>,
    // ovwd_shield: Option<ActorState>,
    // ovwd_bow: Option<ActorState>,
    // ovwd_armor_head: Option<ActorState>,
    // ovwd_armor_upper: Option<ActorState>,
    // ovwd_armor_lower: Option<ActorState>,
    //
    // ovwd_dropped_materials: VecDeque<ActorState>,
    // ovwd_dropped_equipments: VecDeque<ActorState>,
    //
    // ovwd_holding_materials: VecDeque<ActorState>,
    //
    // entangled_slots: Vec<u32>,
}

#[derive(Clone)]
pub enum Screen {
    /// In the overworld, no additional screens
    Overworld,
    /// In the inventory screen
    Inventory,
    /// In an unknown dialog (could be sell/statue, or other)
    Dialog,
    // /// In sell dialog
    // DialogSell,
    // /// In statue dialog
    // DialogStatue,
}

impl State {
    pub async fn execute_step(
        self,
        ctx: sim::Context<&sim::Runtime>,
        step: &cir::Step,
    ) -> Result<Report<Self>, exec::Error> {
        match &step.command {
            cir::Command::Get(items) => self.handle_get(ctx, items).await,
            _ => Ok(Report::error(self, sim_error!(&ctx.span, Unimplemented))),
        }
    }

    async fn handle_get(
        self,
        ctx: sim::Context<&sim::Runtime>,
        items: &[cir::ItemSpec],
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling GET command");
        self.with_game(ctx, async move |game, ctx| {
            let items = items.to_vec();
            ctx.execute(move |ctx| game.cmd_get(ctx, &items)).await
        })
        .await
    }
}

impl GameState {
    pub fn new(process: Process) -> Self {
        Self {
            // screen: Screen::Overworld,
            process,
        }
    }

    pub fn cmd_get(
        self,
        ctx: sim::Context<&mut Cpu1>,
        items: &[cir::ItemSpec],
    ) -> Result<Self, CrashReport> {
        ctx.execute(self, |ctx| {
            'outer: for item in items {
                let amount = item.amount;
                let item = &item.item;
                let is_cook_item = item.is_cook_item();
                let meta = item.meta.as_ref();
                for _ in 0..amount {
                    if is_cook_item {
                        linker::get_cook_item(
                            ctx.inner,
                            &item.actor,
                            meta.map(|m| m.ingredients.as_slice()).unwrap_or(&[]),
                            meta.and_then(|m| m.life_recover_f32()),
                            meta.and_then(|m| m.effect_duration),
                            meta.and_then(|m| m.sell_price),
                            meta.and_then(|m| m.effect_id),
                            meta.and_then(|m| m.effect_level),
                        )?;
                        continue;
                    };
                    let modifier = util::modifier_from_meta(meta);
                    linker::get_item(ctx.inner, &item.actor, meta.and_then(|m| m.value), modifier)?;

                    if ctx.is_aborted() {
                        break 'outer;
                    }
                }
            }
            Ok(())
        })
    }
}
