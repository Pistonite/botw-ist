use memory::{Memory, Proxies};
use processor::Processor;

pub struct Core<'p, 'm, 'x> {
    pub cpu: &'p mut Processor,
    pub mem: &'m mut Memory,
    pub proxies: &'x mut Proxies,
}

/// Internal bindings to invoke functions
impl Core<'_, '_, '_> {

    // these functions are called internally by the call
    // to execute commands
    //
    // these need to put the argument on the stack, set SP and PC
    // correctly, and then run the function using the Processor


    // 0x96efb8
    pub fn pmdm_item_get(&self, actor: &str, value: i32, modifier_info: u64) -> 
    Result<(), error::Error> {
        todo!();
    }
}

/// Memory implementation
pub mod memory;


pub mod error;

mod loader;

pub mod processor;

/// Initialization for the memory
mod boot;

/// Proxy objects
mod proxy;
