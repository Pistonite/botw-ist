use enum_map::Enum;
use serde::{Deserialize, Serialize};

/// Environment to simulate
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    rkyv::Serialize,
    rkyv::Deserialize,
    rkyv::Archive,
)]
#[rkyv(compare(PartialEq), derive(Clone, Copy))]
pub struct Environment {
    /// Version of the game
    pub game_ver: GameVer,
    /// Version of the DLC
    pub dlc_ver: DlcVer,
}

impl Environment {
    pub fn new(game_ver: GameVer, dlc_ver: DlcVer) -> Self {
        Self { game_ver, dlc_ver }
    }
    /// Get The offset of the main module compared to the start
    /// of the program region
    #[inline]
    pub const fn main_offset(self) -> u32 {
        0x4000
    }
    /// Get if the game version is 1.5.0
    #[inline]
    pub const fn is150(self) -> bool {
        matches!(self.game_ver, GameVer::X150)
    }

    /// Get if the game version is 1.6.0
    #[inline]
    pub const fn is160(self) -> bool {
        matches!(self.game_ver, GameVer::X160)
    }

    /// Get the DLC version number as a u32
    ///
    /// The upper 24 bits are the major version, and the lower 8 bits are the minor version
    #[inline]
    pub const fn dlc_version(self) -> u32 {
        self.dlc_ver.to_repr()
    }
}

/// Version of the game
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    Enum,
    rkyv::Serialize,
    rkyv::Deserialize,
    rkyv::Archive,
)]
#[rkyv(compare(PartialEq), derive(Clone, Copy))]
#[repr(u8)]
pub enum GameVer {
    /// Switch 1.5.0
    X150,
    /// Switch 1.6.0
    X160,
}

impl From<ArchivedGameVer> for GameVer {
    fn from(ver: ArchivedGameVer) -> Self {
        match ver {
            ArchivedGameVer::X150 => GameVer::X150,
            ArchivedGameVer::X160 => GameVer::X160,
        }
    }
}

/// Version of the DLC
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    rkyv::Serialize,
    rkyv::Deserialize,
    rkyv::Archive,
)]
#[rkyv(compare(PartialEq), derive(Clone, Copy))]
#[repr(u8)]
pub enum DlcVer {
    /// Not installed
    None,
    /// Version 1.0.0 (Day 1 stuff)
    V100,
    /// Version 2.0.0 (Master Trials)
    V200,
    /// Version 3.0.0 (Champions Ballad)
    V300,
}

impl DlcVer {
    /// Convert a number from 0-3 to a DLC version
    pub fn from_num(num: u32) -> Option<Self> {
        match num {
            0 => Some(DlcVer::None),
            1 => Some(DlcVer::V100),
            2 => Some(DlcVer::V200),
            3 => Some(DlcVer::V300),
            _ => None,
        }
    }

    /// Convert the version to in-game representation
    pub const fn to_repr(self) -> u32 {
        match self {
            DlcVer::None => 0,
            DlcVer::V100 => 0x100,
            DlcVer::V200 => 0x200,
            DlcVer::V300 => 0x300,
        }
    }
}
impl From<ArchivedDlcVer> for DlcVer {
    fn from(ver: ArchivedDlcVer) -> Self {
        match ver {
            ArchivedDlcVer::None => DlcVer::None,
            ArchivedDlcVer::V100 => DlcVer::V100,
            ArchivedDlcVer::V200 => DlcVer::V200,
            ArchivedDlcVer::V300 => DlcVer::V300,
        }
    }
}
