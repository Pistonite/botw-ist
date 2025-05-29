use serde::{Deserialize, Serialize};
// use deku::{DekuRead, DekuWrite};

/// Type for static data files used by BlueFlame
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
#[rkyv(compare(PartialEq))]
// #[deku(id_type = "u8")]
#[repr(u8)]
pub enum DataId {
    /// Actor/ActorInfo.product.byml (decompressed version of the sbyml)
    // #[deku(id = 0x01)]
    ActorInfoByml,
}

/// Proxy type identifiers
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
#[rkyv(compare(PartialEq))]
// #[deku(id_type = "u8")]
#[repr(u8)]
pub enum ProxyId {
    /// ksys::gdt::TriggerParam, the storage for game data flags
    // #[deku(id = 0x01)]
    TriggerParam,
}
