use serde::{Deserialize, Serialize};
use bincode::{Encode, Decode};

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
    Encode, Decode
)]
// #[rkyv(compare(PartialEq), derive(Clone, Copy))]
#[repr(u8)]
pub enum DataId {
    /// Actor/ActorInfo.product.byml (decompressed version of the sbyml)
    ActorInfoByml,
}

// impl From<ArchivedDataId> for DataId {
//     fn from(archived: ArchivedDataId) -> Self {
//         match archived {
//             ArchivedDataId::ActorInfoByml => DataId::ActorInfoByml,
//         }
//     }
// }

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
    Encode, Decode,
)]
// #[rkyv(compare(PartialEq), derive(Clone, Copy))]
#[repr(u8)]
pub enum ProxyId {
    /// ksys::gdt::TriggerParam, the storage for game data flags
    TriggerParam,
}

// impl From<ArchivedProxyId> for ProxyId {
//     fn from(archived: ArchivedProxyId) -> Self {
//         match archived {
//             ArchivedProxyId::TriggerParam => ProxyId::TriggerParam,
//         }
//     }
// }
