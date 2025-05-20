
extern crate self as blueflame;
use blueflame_utils::ProxyType;

use error::{Error, ExecutionError};
use memory::{Ptr, Memory, Proxies};
// use processor::instruction_registry::ExecutableInstruction;
// use processor::Processor;
// use proxy::trigger_param::GdtTriggerParam;
use structs::{CookEffectId, CookItem, FixedSafeString40, PouchItemType, SafeString};

pub struct Core< 'm, 'x> {
    // pub cpu: &'p mut Processor,
    pub mem: &'m mut Memory,
    pub proxies: &'x mut Proxies,
}

// const MAX_INSTRUCTIONS: u64 = 0x1000000000;
// const MAX_INSTRUCTIONS: u64 = 0x100000;
// const MAX_CALL_LEVEL: u64 = 64;
const ENTER_ADDR: u64 = 100;

/// Internal bindings to invoke functions
impl Core<'_, '_> {
    // TODO: hardcoded program start
    const FIXED_SAFE_STRING40_VTABLE_ADDR: u64 = 0x1234500000 + 0x2356A90;
    // these functions are called internally by the call
    // to execute commands
    //
    // these need to put the argument on the stack, set SP and PC
    // correctly, and then run the function using the Processor

    // pub fn setup(&mut self) -> Result<(), ExecutionError> {
    //     // Run any game code functions that must execute prior to running other functions
    //     self.init_common_flags()
    // }

    // pub fn load_from_game_data(&mut self) -> Result<(), ExecutionError> {
    //     self.cpu.write_arg(0, self.mem.get_pmdm_addr());
    //     self.call_func_at_addr(0x96BE24)
    // }
    //
    // pub fn save_to_game_data(&mut self) -> Result<(), ExecutionError> {
    //     self.cpu.write_arg(0, self.mem.get_pmdm_addr());
    //     // PouchItem Offset List
    //     self.cpu.write_arg(1, self.mem.get_pmdm_addr() + 0x68);
    //     self.call_func_at_addr(0x96F9BC)
    // }
    //
    // pub fn has_tag(&mut self, actor: &str, tag: u32) -> Result<bool, ExecutionError> {
    //     self.cpu.write_arg(0, 61472768);
    //     let addr: Ptr<FixedSafeString40> = Ptr::new(self.alloc_fixed_safe_string40(actor)?);
    //     let m_top = addr
    //         .deref(self.mem)
    //         .map_err(|e| self.to_execution_error(e))?;
    //     self.cpu.write_arg(1, m_top.safeString.mStringTop);
    //     self.cpu.write_arg(2, tag as u64);
    //     self.call_func_at_addr(0xD2F900)?;
    //     Ok(self.cpu.read_arg(0) != 0)
    // }

    // // 0x96efb8
    // pub fn pmdm_item_get(
    //     &mut self,
    //     actor: &str,
    //     value: i32,
    //     modifier_flags: u32,
    //     modifier_value: i32,
    // ) -> Result<(), ExecutionError> {
    //     let actor_name_addr = self.alloc_fixed_safe_string40(actor)?;
    //     let modifier_info_addr = self
    //         .alloc_weapon_modifier_info(modifier_flags, modifier_value)
    //         .map_err(|e| self.to_execution_error(e))?;
    //     self.cpu.write_arg(0, self.mem.get_pmdm_addr());
    //     self.cpu.write_arg(1, actor_name_addr);
    //     self.cpu.write_arg(2, value as u64);
    //     self.cpu.write_arg(3, modifier_info_addr);
    //     self.call_func_at_addr(0x96EFB8)
    // }

    // // 0x970060
    // #[allow(clippy::too_many_arguments)]
    // pub fn cook_item_get(
    //     &mut self,
    //     name: &str,
    //     life_recover: f32,
    //     effect_time: i32,
    //     sell_price: i32,
    //     effect_id: CookEffectId,
    //     vitality_boost: f32,
    //     is_crit: bool,
    // ) -> Result<(), ExecutionError> {
    //     let cook_item_addr = self
    //         .alloc_cook_item(
    //             name,
    //             life_recover,
    //             effect_time,
    //             sell_price,
    //             effect_id,
    //             vitality_boost,
    //             is_crit,
    //         )
    //         .map_err(|e| self.to_execution_error(e))?;
    //     self.cpu.write_arg(0, self.mem.get_pmdm_addr());
    //     self.cpu.write_arg(1, cook_item_addr);
    //     self.call_func_at_addr(0x970060)
    // }

    // // 0x9704BC
    // pub fn remove_item(&mut self, actor: &str) -> Result<(), ExecutionError> {
    //     let actor_name_addr = self.alloc_fixed_safe_string40(actor)?;
    //     self.cpu.write_arg(0, self.mem.get_pmdm_addr());
    //     self.cpu.write_arg(1, actor_name_addr);
    //     self.call_func_at_addr(0x9704BC)
    // }
    //
    // // 0x97A944
    // pub fn equip_weapon(&mut self, item_addr: u64) -> Result<(), ExecutionError> {
    //     self.cpu.write_arg(0, self.mem.get_pmdm_addr());
    //     self.cpu.write_arg(1, item_addr);
    //     self.call_func_at_addr(0x97A944)
    // }

    // // 0x97A9FC
    // pub fn unequip(&mut self, item_addr: u64) -> Result<(), ExecutionError> {
    //     self.cpu.write_arg(0, self.mem.get_pmdm_addr());
    //     self.cpu.write_arg(1, item_addr);
    //     self.call_func_at_addr(0x97A9FC)
    // }

    // pub fn info_data_get_type(&mut self, name: &str) -> Result<PouchItemType, ExecutionError> {
    //     let actor_name_addr = self.alloc_fixed_safe_string40(name)?;
    //     self.cpu.write_arg(0, actor_name_addr);
    //     self.cpu.write_arg(1, 0);
    //     self.call_func_at_addr(0x96DC34)?;
    //     Ok(PouchItemType::from(self.cpu.read_arg(0) as i32))
    // }

    // pub fn init_common_flags(&mut self) -> Result<(), ExecutionError> {
    //     self.call_func_at_addr(0x8BF8A0)
    // }
    //
    // pub fn create_player_equipment(&mut self) -> Result<(), ExecutionError> {
    //     self.cpu.write_arg(0, self.mem.get_pmdm_addr());
    //     self.call_func_at_addr(0x971504)
    // }
    //
    // fn alloc_fixed_safe_string40(&mut self, value: &str) -> Result<u64, ExecutionError> {
    //     let base_address = self
    //         .mem
    //         .heap_mut()
    //         .alloc(0x58)
    //         .map_err(|e| self.to_execution_error(Error::Mem(e)))?;
    //     let mut buffer = [0u8; 64];
    //
    //     let bytes = value.as_bytes();
    //     let len = bytes.len().min(buffer.len());
    //
    //     buffer[..len].copy_from_slice(&bytes[..len]);
    //     let fixed_safe_string = FixedSafeString40 {
    //         safeString: SafeString {
    //             vtable: Self::FIXED_SAFE_STRING40_VTABLE_ADDR + (self.mem.get_main_offset() as u64),
    //             mStringTop: base_address + 0x14,
    //         },
    //         mBufferSize: 64,
    //         mBuffer: buffer,
    //     };
    //
    //     let ida_addr = self.compute_ida_addr(self.cpu.pc);
    //     let stack_trace = self.cpu.stack_trace.clone();
    //     let mut writer = self
    //         .mem
    //         .write(base_address, None)
    //         .map_err(|e| ExecutionError::new(Error::Mem(e), ida_addr, stack_trace))?;
    //     fixed_safe_string
    //         .write_to_mem(&mut writer)
    //         .map_err(|e| self.to_execution_error(e))?;
    //     Ok(base_address)
    // }
    //
    // #[allow(clippy::too_many_arguments)]
    // fn alloc_cook_item(
    //     &mut self,
    //     name: &str,
    //     life_recover: f32,
    //     effect_time: i32,
    //     sell_price: i32,
    //     effect_id: CookEffectId,
    //     vitality_boost: f32,
    //     is_crit: bool,
    // ) -> Result<u64, error::Error> {
    //     self.mem.heap_mut().alloc(0x64)?;
    //     let base_address = self.mem.heap_mut().alloc(0x228)?;
    //
    //     let mut buffer = [0u8; 64];
    //
    //     let bytes = name.as_bytes();
    //     let len = bytes.len().min(buffer.len());
    //
    //     buffer[..len].copy_from_slice(&bytes[..len]);
    //     // actor_name is offset 0 from base
    //     let actor_name = FixedSafeString40 {
    //         safeString: SafeString {
    //             vtable: Self::FIXED_SAFE_STRING40_VTABLE_ADDR + (self.mem.get_main_offset() as u64),
    //             mStringTop: base_address + 0x14,
    //         },
    //         mBufferSize: 64,
    //         mBuffer: buffer,
    //     };
    //
    //     let buffer = [b'A'; 64];
    //
    //     let ingredient1 = FixedSafeString40 {
    //         safeString: SafeString {
    //             vtable: Self::FIXED_SAFE_STRING40_VTABLE_ADDR + (self.mem.get_main_offset() as u64),
    //             mStringTop: base_address + 0x14 + 0x58,
    //         },
    //         mBufferSize: 64,
    //         mBuffer: buffer,
    //     };
    //
    //     let buffer = [b'B'; 64];
    //     let ingredient2 = FixedSafeString40 {
    //         safeString: SafeString {
    //             vtable: Self::FIXED_SAFE_STRING40_VTABLE_ADDR + (self.mem.get_main_offset() as u64),
    //             mStringTop: base_address + 0x14 + 0x58 * 2,
    //         },
    //         mBufferSize: 64,
    //         mBuffer: buffer,
    //     };
    //
    //     let buffer = [b'C'; 64];
    //     let ingredient3 = FixedSafeString40 {
    //         safeString: SafeString {
    //             vtable: Self::FIXED_SAFE_STRING40_VTABLE_ADDR + (self.mem.get_main_offset() as u64),
    //             mStringTop: base_address + 0x14 + 0x58 * 3,
    //         },
    //         mBufferSize: 64,
    //         mBuffer: buffer,
    //     };
    //
    //     let buffer = [b'D'; 64];
    //     let ingredient4 = FixedSafeString40 {
    //         safeString: SafeString {
    //             vtable: Self::FIXED_SAFE_STRING40_VTABLE_ADDR + (self.mem.get_main_offset() as u64),
    //             mStringTop: base_address + 0x14 + 0x58 * 4,
    //         },
    //         mBufferSize: 64,
    //         mBuffer: buffer,
    //     };
    //
    //     let buffer = [b'E'; 64];
    //     let ingredient5 = FixedSafeString40 {
    //         safeString: SafeString {
    //             vtable: Self::FIXED_SAFE_STRING40_VTABLE_ADDR + (self.mem.get_main_offset() as u64),
    //             mStringTop: base_address + 0x14 + 0x58 * 5,
    //         },
    //         mBufferSize: 64,
    //         mBuffer: buffer,
    //     };
    //
    //     let ingredients = [
    //         ingredient1,
    //         ingredient2,
    //         ingredient3,
    //         ingredient4,
    //         ingredient5,
    //     ];
    //
    //     let cook_item = CookItem::new(
    //         actor_name,
    //         ingredients,
    //         life_recover,
    //         effect_time,
    //         sell_price,
    //         effect_id as i32,
    //         vitality_boost,
    //         is_crit,
    //     );
    //
    //     let mut writer = self.mem.write(base_address, None)?;
    //     cook_item.write_to_mem(&mut writer)?;
    //     Ok(base_address)
    // }
    //
    // pub fn get_hash_for_actor(&mut self, name: &str) -> Result<u32, ExecutionError> {
    //     let actor_name_addr = self.alloc_fixed_safe_string40(name)?;
    //     let actor_name: FixedSafeString40 = Ptr::new(actor_name_addr)
    //         .deref(self.mem)
    //         .map_err(|e| self.to_execution_error(e))?;
    //     self.cpu.write_arg(0, actor_name.safeString.mStringTop);
    //     self.call_func_at_addr(0xB2170C)?;
    //     Ok(self.cpu.read_arg(0) as u32)
    // }
    //
    // fn alloc_weapon_modifier_info(&mut self, flags: u32, value: i32) -> Result<u64, error::Error> {
    //     let base_address = self.mem.heap_mut().alloc(0x8)?;
    //     self.mem.mem_write_val::<u32>(base_address, flags)?;
    //     self.mem.mem_write_i32(base_address + 0x4, value)?;
    //     Ok(base_address)
    // }
    //
    // fn set_sp(&mut self) {
    //     let new_sp = self.mem.get_region(memory::RegionType::Stack).start + 0x4000;
    //     self.cpu.sp_el0 = new_sp;
    // }
    //
    // pub fn call_func_at_addr(&mut self, addr: u64) -> Result<(), ExecutionError> {
    //     //Set pc to start of function
    //     // TODO: hardcoded program start
    //     self.cpu.set_pc(0x1234500000 + addr + (self.mem.get_main_offset() as u64));
    //     self.set_sp();
    //
    //     //Call level representing if the function is still inside of the original function
    //     // let mut call_level: u64 = 0;
    //     // let mut instructions_ran: u64 = 0;
    //     self.cpu.write_arg(30, ENTER_ADDR);
    //     while self.cpu.pc != ENTER_ADDR {
    //         let ida_addr = self.compute_ida_addr(self.cpu.pc);
    //         let result = self.execute_at_pc();
    //         match result {
    //             Ok(_) => {}
    //             Err(e) => {
    //                 return Err(ExecutionError::new(
    //                     e,
    //                     ida_addr,
    //                     self.cpu.stack_trace.clone(),
    //                 ));
    //             }
    //         };
    //         // instructions_ran += 1;
    //     }
    //     Ok(())
    // }

    // fn fetch_instruction(&mut self, pc: u64) -> Result<Box<dyn ExecutableInstruction>, Error> {
    //     const BLOCK_SIZE: u64 = 64;
    //     const INST_SIZE: u64 = 4;
    //
    //     let block_base = pc & !(BLOCK_SIZE - 1);
    //     let offset = ((pc - block_base) / INST_SIZE) as usize;
    //
    //     // log::debug!("fetching instruction at pc: {:#x}", pc);
    //     let inst = self.mem.mem_read_inst(pc)?;
    //     let block = self
    //         .cpu
    //         .inst_cache
    //         .entry(block_base)
    //         .or_insert_with(|| vec![None; (BLOCK_SIZE / INST_SIZE) as usize]);
    //    
    //     if block[offset].is_none() {
    //         block[offset] = Some(inst);
    //     }
    //    
    //     block
    //         .get(offset)
    //         .and_then(|slot| slot.as_ref())
    //         .ok_or(Error::Cpu(
    //             crate::processor::Error::InstructionCouldNotBeRead(pc),
    //         ))
    //         .cloned()
    // }

    // pub fn execute_at_pc(&mut self) -> Result<(), Error> {
    //     let s = self.cpu.check_pc(self.mem.get_main_offset()).cloned();
    //     // let s = self.cpu.check_pc(self.mem.get_main_offset()).cloned();
    //
    //     if let Some(stub) = s {
    //         let condition_met = {
    //             let lock = stub.lock().unwrap();
    //             let condition = &lock.condition; // Immutable borrow ends here
    //             if let Some(cond) = condition {
    //                 (cond)(self.cpu).map_err(|e| {
    //                     Error::Cpu(crate::processor::Error::Unexpected(format!("{:?}", e)))
    //                 })?
    //             } else {
    //                 true
    //             }
    //         };
    //         if condition_met {
    //             // TODO: fix the mutability check - Stub needs to be moved out of CPU
    //             let lock = stub.lock().unwrap();
    //             (lock.func)(self).unwrap();
    //         }
    //     } else {
    //         let inst = self.fetch_instruction(self.cpu.pc)?;
    //         inst.exec_on(self)?;
    //     }
    //     self.cpu.pc += 4;
    //     Ok(())
    // }

    // pub fn allocate_proxy(&mut self, proxy: ProxyType) -> Result<u64, crate::memory::Error> {
    //     match proxy {
    //         ProxyType::TriggerParam => {
    //             let mut trig = GdtTriggerParam::default();
    //             trig.load_yaml_files()
    //                 .map_err(|e| crate::memory::Error::Unexpected(format!("{}", e)))?;
    //             let tp_addr = self.proxies.allocate_trigger_param(self.mem, trig)?;
    //             self.mem.set_trigger_param_addr(tp_addr);
    //             Ok(tp_addr)
    //         }
    //     }
    // }

    pub fn allocate_data(&mut self, data: Vec<u8>) -> Result<u64, crate::memory::Error> {
        let start = self.mem.heap_mut().alloc(data.len() as u32)?;
        self.mem.mem_write_bytes(start, data)?;
        Ok(start)
    }

    // fn to_execution_error(&self, error: Error) -> ExecutionError {
    //     let ida_addr = self.compute_ida_addr(self.cpu.pc);
    //     ExecutionError::new(error, ida_addr, self.cpu.stack_trace.clone())
    // }
    pub(crate) fn compute_ida_addr(&self, addr: u64) -> u64 {
        0x7100000000 + addr - self.mem.main_start()
    }
}

// The library is divided into layers and modules,
// lower layer should not include from higher layers.
// There is not enforcement at the moment since it's not worth
// the effort trying to set it up (because shared dependency
// management will be a pain)

/// Memory implementation
pub mod memory;

pub mod error;

pub mod processor;

/// Initialization for the memory
// pub mod boot;

/// Proxy objects
// pub mod proxy;

/// Struct Types
pub mod structs;

pub mod singleton;



pub mod process;

/////// --- LAYER --- ///////

/// Utilities for handling program images
pub mod program;

/////// --- LAYER --- ///////

/// Other utils and shared types
pub mod util;
/// Handle core feature flags (core-* flags in the script)
pub mod features;
