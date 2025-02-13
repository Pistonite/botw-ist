use std::{collections::{HashMap, VecDeque}, future::Future, sync::{atomic::{AtomicBool, AtomicU64}, Arc}};

use skybook_parser::{search::QuotedItemResolver, ParseOutput};
use blueflame::{error::Error, memory::{Memory, Proxies}};

mod scheduler;
use scheduler::Scheduler;


pub struct Runtime<R: QuotedItemResolver, S: scheduler::Scheduler> {

    // === parsing stage ===
    resolver: R,
    is_parsing: bool,
    parse_script: Arc<str>,
    parse_serial: Arc<AtomicU64>,
    parse_output: Arc<ParseOutput>,

    // === running stage ===
    scheduler: S,
    is_running: bool,
    execute_script: Arc<str>,
    execute_serial: Arc<AtomicU64>,
    execute_states: Vec<State>,

    subscribers: Vec<Box<dyn Fn() + Send + Sync>>,
}

impl<R: QuotedItemResolver, S: scheduler::Scheduler> Runtime<R, S> {
    pub async fn parse_script(&mut self, script: &Arc<str>) -> Arc<ParseOutput> {
        // if the cache result is up-to-date, return it
        if !self.is_parsing && script.as_ref() == self.parse_script.as_ref() {
            return Arc::clone(&self.parse_output)
        }
        // start a new parsing job
        let serial_before = self.parse_serial.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.is_parsing = true;
        self.parse_script = Arc::clone(script);
        let parse_output = Arc::new(skybook_parser::parse(&self.resolver, script).await);

        // if no newer parsing job has started, update the cache
        let serial_after = self.parse_serial.load(std::sync::atomic::Ordering::SeqCst);
        if serial_before == serial_after {
            self.parse_output = Arc::clone(&parse_output);
            self.is_parsing = false;
        }

        parse_output
    }
    pub async fn execute_script(&mut self, script: &Arc<str>) -> bool {
        if !self.is_running && script.as_ref() == self.execute_script.as_ref() {
            // cached states are valid, do nothing
            return true;
        }
        let serial_before = self.execute_serial.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.is_running = true;
        self.execute_script = Arc::clone(script);

        let parse_output = self.parse_script(script).await;
        let serial_after = self.parse_serial.load(std::sync::atomic::Ordering::SeqCst);
        if serial_before != serial_after {
            return false;
        }

        let new_states = run_stuff(serial_before, &parse_output).await;
        if serial_before != serial_after {
            return false;
        }
        self.execute_states = new_states;
        self.is_running = false;

        for subscriber in &self.subscribers {
            subscriber();
        }

        true
    }
}

pub async fn run_stuff(serial: u64, parse_output: &ParseOutput) -> Vec<State> {
    todo!()
}

#[derive(PartialEq)]
pub enum Stage {
    Parsing,
    Running,
    Idle
}

#[derive(Clone)]
pub struct State {


    // named gamedata in saves
    saves: HashMap<String, u64>,

    // gamedata in manual save
    manual_save: u64,

    /// Current game state
    game: Game,

    /// Current screen, only valid if game is running
    screen: Screen,

    /// If inventory/dialog screen is activated manually,
    /// so auto-scoping will be disabled until returned to overworld screen
    is_manual_scope: bool,

    /// If auto scope is enabled at all
    enable_auto_scope: bool,
}

#[derive(Clone)]
pub enum Game {
    /// Game is not booted
    Off,
    /// Game is running
    Running(GameState),
    /// Game has crashed (must manually reboot)
    Crashed(Error) // TODO: more crash info (dump, stack trace, etc)
}

#[derive(Clone)]
pub enum Screen {
    /// In the overworld, no additional screens
    Overworld,
    /// In the inventory screen
    Inventory,
    /// In an unknown dialog (could be sell/statue, or other)
    Dialog,
    /// In sell dialog
    DialogSell,
    /// In statue dialog
    DialogStatue,
}

/// State available when the game is running
#[derive(Clone)]
pub struct GameState {
    // gamedata TriggerParam*
    gamedata: u64,
    // memory states
    //
    /// Full process memory
    memory: Memory,

    /// Proxy objects in memory
    proxies: Proxies,

    /// Current actors in the overworld
    /// TODO: make this copy on write and Arc
    ovwd_weapon: Option<ActorState>,
    ovwd_shield: Option<ActorState>,
    ovwd_bow: Option<ActorState>,
    ovwd_armor_head: Option<ActorState>,
    ovwd_armor_upper: Option<ActorState>,
    ovwd_armor_lower: Option<ActorState>,

    ovwd_dropped_materials: VecDeque<ActorState>,
    ovwd_dropped_equipments: VecDeque<ActorState>,

    ovwd_holding_materials: VecDeque<ActorState>,

    entangled_slots: Vec<u32>,
}

#[derive(Clone)]
pub struct ActorState {
    pub name: String,
    pub life: u32,
    pub modifier_bits: u32,
    pub modifier_value: i32,
}

impl GameState {

    // just a placeholder
    // probably some kind of macro to generate these
    pub async fn get_item(mut self, scheduler: impl Scheduler, item: &str) -> Result<GameState, Error> {
        scheduler.run_on_core(move |p| {
            let core = p.attach(&mut self.memory, &mut self.proxies);
            // todo: real function
            core.pmdm_item_get(item, 0, 0)?;

            Ok(self)
        }).await
    }
}
