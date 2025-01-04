use memory::Memory;
use processor::Processor;

pub struct Core<'p, 'm> {
    pub cpu: &'p mut Processor,
    pub mem: &'m mut Memory,
}

/// Internal bindings to invoke functions
trait CoreInternal {

    // these functions are called internally by the call
    // to execute commands
    //
    // these need to put the argument on the stack, set SP and PC
    // correctly, and then run the function using the Processor


    // 0x96efb8
    fn pmdm_item_get(&self, actor: &str, value: i32, modifier_info: u64) -> 
    Result<(), ()>;
}

/// Memory implementation
mod memory;

mod error;

mod loader;

mod processor;

/// Initialization for the memory
mod boot;
