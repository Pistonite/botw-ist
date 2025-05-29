#[layered_crate::import]
use game::{
    super::memory::ProxyList,
    self::gdt
};

/// Holds all proxy objects in memory
#[derive(Debug, Default, Clone)]
pub struct Proxies {
    pub trigger_param: ProxyList<gdt::TriggerParam>,
}
