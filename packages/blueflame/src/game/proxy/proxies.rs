use crate::game::{self as self_, crate_};

use crate_::memory::ProxyList;
use self_::gdt;

/// Holds all proxy objects in memory
#[derive(Debug, Default, Clone)]
pub struct Proxies {
    pub trigger_param: ProxyList<gdt::TriggerParam>,
}
