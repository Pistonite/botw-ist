/// Environment to simulate
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "deku", derive(deku::DekuRead, deku::DekuWrite))]
#[cfg_attr(feature = "deku", deku(id_type = "u8"))]
pub struct Environment {
    /// Version of the game
    pub game_ver: GameVer,
    /// Version of the DLC
    pub dlc_ver: DlcVer,
}

impl Environment {
    #[inline]
    pub fn is150(self) -> bool {
        self.game_ver == GameVer::X150
    }
}

/// Version of the game
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "deku", derive(deku::DekuRead, deku::DekuWrite))]
#[cfg_attr(feature = "deku", deku(id_type = "u8"))]
#[repr(u8)]
pub enum GameVer {
    /// Switch 1.5.0
    X150,
    /// Switch 1.6.0
    X160
}

/// Version of the DLC
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "deku", derive(deku::DekuRead, deku::DekuWrite))]
#[cfg_attr(feature = "deku", deku(id_type = "u8"))]
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
