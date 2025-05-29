// use deku::{DekuRead, DekuWrite};
use enum_map::Enum;
use serde::{Serialize, Deserialize};

/// Environment to simulate
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(rkyv::Serialize, rkyv::Deserialize, rkyv::Archive)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(Enum)]
#[derive(rkyv::Serialize, rkyv::Deserialize, rkyv::Archive)]
// #[deku(id_type = "u8")]
#[repr(u8)]
pub enum GameVer {
    /// Switch 1.5.0
    // #[deku(id = 0x01)]
    X150,
    /// Switch 1.6.0
    // #[deku(id = 0x02)]
    X160
}

/// Version of the DLC
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(rkyv::Serialize, rkyv::Deserialize, rkyv::Archive)]
// #[deku(id_type = "u8")]
#[repr(u8)]
pub enum DlcVer {
    /// Not installed
    // #[deku(id = 0x00)]
    None,
    /// Version 1.0.0 (Day 1 stuff)
    // #[deku(id = 0x01)]
    V100,
    /// Version 2.0.0 (Master Trials)
    // #[deku(id = 0x02)]
    V200,
    /// Version 3.0.0 (Champions Ballad)
    // #[deku(id = 0x03)]
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
