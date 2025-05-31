use crate::game::gdt;
use crate::memory::ProxyList;

/// Holds all proxy objects in memory
#[derive(Debug, Default, Clone)]
pub struct Proxies {
    pub trigger_param: ProxyList<gdt::TriggerParam>,
}
