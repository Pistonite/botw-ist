/// Proxy type identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "deku", derive(deku::DekuRead, deku::DekuWrite))]
#[cfg_attr(feature = "deku", deku(id_type = "u8"))]
#[cfg_attr(feature = "enum-map", derive(enum_map::Enum))]
#[repr(u8)]
pub enum ProxyType {
    /// ksys::gdt::TriggerParam, the storage for game data flags
    #[cfg_attr(feature = "deku", deku(id = 0x01))]
    TriggerParam,
}

