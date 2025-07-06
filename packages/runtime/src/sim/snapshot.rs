use blueflame::{
    game::{PouchItemType, PouchItemUse},
    processor::CrashReport,
};

use crate::{iv, sim};

#[derive(PartialEq)]
pub struct StateSnapshot {
    pub game: GameSnapshot,
}

impl std::fmt::Display for StateSnapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.game.fmt(f)
    }
}

#[derive(PartialEq)]
pub enum GameSnapshot {
    Uninit,
    Running(GameSnapshotRunning),
    Crashed(CrashReport),
    PreviousCrash,
}

#[derive(PartialEq)]
pub struct GameSnapshotRunning {
    pub pouch: Result<iv::PouchList, sim::view::Error>,
    pub overworld: iv::Overworld,
}

macro_rules! write_snapshot_ln {
    ($fmt:ident, $prefix:literal, $($variable:expr),* $(,)?) => {
        writeln!(
            $fmt,
            concat!($prefix, ": (", $( stringify!($variable),"={}, ",)* ")"),
            $($variable),*
        )
    }
}

impl std::fmt::Display for GameSnapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Uninit => {
                writeln!(f, "game: (Uninit)")
            }
            Self::Running(state) => {
                writeln!(f, "game: (Running)")?;
                state.fmt(f)
            }
            Self::Crashed(report) => {
                writeln!(f, "game: (Crashed)")?;
                writeln!(f, "{report:?}")
            }
            Self::PreviousCrash => {
                writeln!(f, "game: (PreviousCrash)")
            }
        }
    }
}

impl std::fmt::Display for GameSnapshotRunning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_iv_pouch(&self.pouch, f)?;
        fmt_iv_overworld(&self.overworld, f)?;

        Ok(())
    }
}

fn fmt_iv_pouch(
    pouch: &Result<iv::PouchList, sim::view::Error>,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    let pouch = match pouch {
        Err(e) => {
            writeln!(f, "  pouch_error: ({e})")?;
            return Ok(());
        }
        Ok(x) => x,
    };
    {
        let screen = pouch.screen;
        writeln!(f, "  screen: ({screen:?})")?;
        let count = pouch.count;
        let are_tabs_valid = pouch.are_tabs_valid;
        let num_tabs = pouch.num_tabs;
        let holding_in_inventory = pouch.is_holding_in_inventory;
        write_snapshot_ln!(
            f,
            "  pouch",
            count,
            are_tabs_valid,
            num_tabs,
            holding_in_inventory
        )?;
    }
    {
        let len = pouch.items.len();
        write_snapshot_ln!(f, "    items", len)?;
    }

    for (i, item) in pouch.items.iter().enumerate() {
        let actor_name = &item.common.actor_name;
        let value = item.common.value;
        let is_equipped = item.common.is_equipped;
        let item_type = PouchItemType::describe(item.item_type);
        let item_use = PouchItemUse::describe(item.item_use);
        let tab_idx = item.tab_idx;
        let tab_slot = item.tab_slot;
        write!(f, "      [{i:03}]")?;
        write_snapshot_ln!(
            f,
            "",
            actor_name,
            value,
            is_equipped,
            item_type,
            item_use,
            tab_idx,
            tab_slot
        )?;
        {
            let valid = item.node_valid;
            let addr: u64 = item.node_addr.into();
            let addr = format!("0x{addr:016x}");
            let pos = if item.node_pos < 0 {
                format!("-0x{:016x}", item.node_pos)
            } else if valid {
                format!("{}", item.node_pos)
            } else {
                format!("0x{:016x}", item.node_pos)
            };
            let prev: u64 = item.node_prev.into();
            let prev = format!("0x{prev:016x}");
            let next: u64 = item.node_next.into();
            let next = format!("0x{next:016x}");
            write_snapshot_ln!(f, "        node", valid, pos, addr, prev, next)?;
        }
        if item.data != Default::default() {
            write!(f, "        ")?;
            fmt_item_data(&item.data, f)?;
        }

        // optional values
        // these are not always logged since they are
        // mostly likely the same value every time. we only
        // log them if it's not the most common case to save some
        // space
        if !item.is_in_inventory {
            writeln!(f, "        in_inventory: false")?;
        }
        if item.is_no_icon {
            writeln!(f, "        no_icon: true")?;
        }
        if item.ingredients.iter().any(|x| !x.is_empty()) {
            let x = &item.ingredients;
            write_snapshot_ln!(f, "        ingredients", x[0], x[1], x[2], x[3], x[4])?;
        }
        if item.holding_count != 0 {
            writeln!(f, "        holding: {}", item.holding_count)?;
        }
        if item.prompt_entangled {
            writeln!(f, "        entangled: true")?;
        }
        if !item.accessible {
            writeln!(f, "        accessible: false")?;
        }
        if !item.dpad_accessible {
            writeln!(f, "        dpad_accessible: false")?;
        }
        if item.allocated_idx != i as i32 {
            writeln!(f, "        allocated_idx: {}", item.allocated_idx)?;
        }
        if item.unallocated_idx != -1 {
            writeln!(f, "        unallocated_idx: {}", item.unallocated_idx)?;
        }
    }
    {
        let len = pouch.tabs.len();
        write_snapshot_ln!(f, "    tabs", len)?;
    }
    for (i, tab) in pouch.tabs.iter().enumerate() {
        let item_idx = tab.item_idx;
        let tab_type = PouchItemType::describe(tab.tab_type);
        write!(f, "      [{i:02}]")?;
        write_snapshot_ln!(f, "", item_idx, tab_type)?;
    }

    Ok(())
}

fn fmt_item_data(data: &iv::ItemData, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let value = data.effect_value;
    let duration = data.effect_duration;
    let price = data.sell_price;
    let id = data.effect_id;
    let level = data.effect_level;
    write_snapshot_ln!(f, "data", value, duration, price, id, level)
}

fn fmt_iv_overworld(
    overworld: &iv::Overworld,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    let len = overworld.items.len();
    if len == 0 {
        // don't bloat the snapshot if overworld is empty
        return Ok(());
    }
    write_snapshot_ln!(f, "  overworld", len)?;
    for (i, item) in overworld.items.iter().enumerate() {
        write!(f, "      [{i:03}]")?;
        fmt_iv_overworld_item(item, f)?;
    }
    Ok(())
}

fn fmt_iv_overworld_item(
    item: &iv::OverworldItem,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    match item {
        iv::OverworldItem::Equipped {
            actor,
            value,
            modifier,
        } => {
            let typ = "Equipped";
            let modifier = if modifier.flag == 0 && modifier.value == 0 {
                "none".to_string()
            } else {
                format!("(flag={:08x}, value={})", modifier.flag, modifier.value)
            };
            write_snapshot_ln!(f, "", typ, actor, value, modifier)?;
        }
        iv::OverworldItem::Held { actor } => {
            let typ = "Held";
            write_snapshot_ln!(f, "", typ, actor,)?;
        }
        iv::OverworldItem::GroundEquipment {
            actor,
            value,
            modifier,
        } => {
            let typ = "GroundEquipment";
            let modifier = if modifier.flag == 0 && modifier.value == 0 {
                "none".to_string()
            } else {
                format!("(flag={:08x}, value={})", modifier.flag, modifier.value)
            };
            write_snapshot_ln!(f, "", typ, actor, value, modifier)?;
        }
        iv::OverworldItem::GroundItem { actor, despawning } => {
            let typ = "GroundItem";
            write_snapshot_ln!(f, "", typ, actor, despawning)?;
        }
    }
    Ok(())
}

impl sim::State {
    pub fn to_snapshot(&self) -> StateSnapshot {
        StateSnapshot {
            game: self.game.to_snapshot(),
        }
    }
}

impl sim::Game {
    pub fn to_snapshot(&self) -> GameSnapshot {
        match self {
            sim::Game::Uninit => GameSnapshot::Uninit,
            sim::Game::Running(game_state) => GameSnapshot::Running(game_state.to_snapshot()),
            sim::Game::Crashed(crash_report) => GameSnapshot::Crashed(crash_report.clone()),
            sim::Game::PreviousCrash => GameSnapshot::PreviousCrash,
        }
    }
}

impl sim::GameState {
    pub fn to_snapshot(&self) -> GameSnapshotRunning {
        let pouch = sim::view::extract_pouch_view(&self.process, &self.systems);
        let overworld = self.systems.overworld.to_iv();
        GameSnapshotRunning { pouch, overworld }
    }
}
