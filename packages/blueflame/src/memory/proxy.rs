use std::sync::Arc;

use rand_xoshiro::rand_core::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use sha2::{Digest, Sha256};

use crate::processor::{Processor, Stub};
use crate::proxy::trigger_param::GdtTriggerParam;

use super::error::Error;
use super::memory::Memory;
use super::region::RegionType;

use paste::paste;

/// The maximum number of proxy objects per type
pub const MAX_OBJECTS: u32 = 1024000;

/// Holds all proxy objects in memory
#[derive(Default, Clone)]
pub struct Proxies {
    // just as a placeholder
    // do this and implement each function below for TriggerParam
    string_proxies: Arc<ProxyList<String>>,
    trigger_param_proxies: Arc<ProxyList<GdtTriggerParam>>,
}

macro_rules! proxy_funcs {
    ($name:ident, $typ:ty) => {
        paste::item! {
            pub fn [<get_ $name>](&self, mem: &Memory, address: u64) -> Result<&$typ, Error> {
                self.[<$name _proxies>].get_at_addr(mem, address)
            }
            pub fn [<mut_ $name>]<'s>(&'s mut self, mem: &mut Memory, address: u64) -> Result<&'s mut $typ, Error> {
                Arc::make_mut(&mut self.[<$name _proxies>]).mut_at_addr(mem, address)
            }
            pub fn [<allocate_ $name>](&mut self, mem: &mut Memory, v: $typ) -> Result<u64, Error> {
                Arc::make_mut(&mut self.[<$name _proxies>]).allocate(mem, v)
            }
        }
    }
}

impl Proxies {
    proxy_funcs!(string, String);
    proxy_funcs!(trigger_param, GdtTriggerParam);

    pub fn init_trigger_param_stubs(processor: &mut Processor) {
        macro_rules! reg_flag_stubs {
            ($name:ident, $get:expr, $get_idx_from_hash:expr, $reset:expr, $set:expr, $set_by_name:expr) => {
                paste! {
                    processor.register_stub_function($get, Stub::run_and_ret(Box::new(|c| GdtTriggerParam::[<get_ $name>](c))));
                    processor.register_stub_function($get_idx_from_hash, Stub::run_and_ret(Box::new(|c| GdtTriggerParam::[<get_ $name _index>](c))));
                    processor.register_stub_function($reset, Stub::run_and_ret(Box::new(|c| GdtTriggerParam::[<reset_ $name>](c))));
                    processor.register_stub_function($set, Stub::run_and_ret(Box::new(|c| GdtTriggerParam::[<set_ $name>](c))));
                    processor.register_stub_function($set_by_name, Stub::run_and_ret(Box::new(|c| GdtTriggerParam::[<set_ $name _safe_string>](c))));
                }
            }
        }
        macro_rules! reg_array_flag_stubs {
            ($name: ident, $get:expr, $get_idx_from_hash:expr, $reset:expr, $len:expr, $set:expr, $set_by_name:expr) => {
                paste! {
                    processor.register_stub_function($get, Stub::run_and_ret(Box::new(|c| GdtTriggerParam::[<get_ $name _array>](c))));
                    processor.register_stub_function($get_idx_from_hash, Stub::run_and_ret(Box::new(|c| GdtTriggerParam::[<get_ $name _index>](c))));
                    processor.register_stub_function($reset, Stub::run_and_ret(Box::new(|c| GdtTriggerParam::[<reset_ $name _array>](c))));
                    processor.register_stub_function($len, Stub::run_and_ret(Box::new(|c| GdtTriggerParam::[<get_ $name _array_size>](c))));
                    processor.register_stub_function($set, Stub::run_and_ret(Box::new(|c| GdtTriggerParam::[<set_ $name>](c))));
                    processor.register_stub_function($set_by_name, Stub::run_and_ret(Box::new(|c| GdtTriggerParam::[<set_ $name _safe_string>](c))));
                }
            }
        }

        reg_flag_stubs!(bool, 0xDDF0F8, 0xDF08B8, 0xDE6F1C, 0xDE1B64, 0xDE59E4);
        reg_flag_stubs!(s32, 0xDDF174, 0xDF0970, 0xDE7004, 0xDE22F8, 0xDE5B0C);
        reg_flag_stubs!(f32, 0xDDF1EC, 0xDF0A28, 0xDE70EC, 0xDE2908, 0xDE5C34);
        reg_flag_stubs!(string32, 0xDDF264, 0xDF0AE0, 0, 0xDE2F20, 0xDE5D64);
        reg_flag_stubs!(string64, 0xDDF2F0, 0xDF0B98, 0xDE71D4, 0xDE37B0, 0xDE5E8C);
        reg_flag_stubs!(string256, 0xDDF37C, 0xDF0C50, 0, 0xDE4040, 0);
        reg_flag_stubs!(vector3f, 0xDDF408, 0xDF0DC0, 0xDE72BC, 0xDE4EA0, 0xDE5FB4);

        reg_array_flag_stubs!(
            bool_array, 0xDE002C, 0xDF0E78, 0xDE77E4, 0xDE0D3C, 0xDE6170, 0xDE6B04
        );
        reg_array_flag_stubs!(
            s32_array, 0xDE00D0, 0xDF0F08, 0xDE7900, 0xDE0D74, 0xDE625C, 0xDE6C08
        );
        reg_array_flag_stubs!(
            f32_array, 0xDE0170, 0xDF0F98, 0xDE7A1C, 0xDE0DAC, 0xDE63E0, 0xDE6D0C
        );
        reg_array_flag_stubs!(
            string64_array,
            0xDE0210,
            0xDF1028,
            0xDE7C54,
            0xDE0E1C,
            0xDE656C,
            0xDE6E18
        );
        reg_array_flag_stubs!(
            string256_array,
            0xDE02C4,
            0xDF10B8,
            0xDE7D70,
            0xDE0E54,
            0xDE6758,
            0
        );
        reg_array_flag_stubs!(
            vector2f_array,
            0xDE0378,
            0xDF1148,
            0xDE7E8C,
            0xDE0E8C,
            0xDE6944,
            0
        );
        reg_array_flag_stubs!(
            vector3f_array,
            0xDE0418,
            0xDF11D8,
            0xDE7FA8,
            0xDE0EC4,
            0xDE6A24,
            0
        );

        processor.register_stub_function(
            0xDEEB8C,
            Stub::simple(Box::new(GdtTriggerParam::reset_all_flags_to_initial_values)),
        );
    }
}

impl ProxyObject for String {
    fn mem_size(&self) -> u32 {
        0x100
    }
}

#[derive(Clone)]
pub struct ProxyList<T: ProxyObject> {
    rng: Xoshiro256PlusPlus,
    objects: Vec<Arc<Entry<T>>>,
}

impl Default for ProxyList<String> {
    fn default() -> Self {
        Self {
            // same seed for every run
            rng: Xoshiro256PlusPlus::seed_from_u64(0),
            objects: Vec::new(),
        }
    }
}

impl Default for ProxyList<GdtTriggerParam> {
    fn default() -> Self {
        Self {
            rng: Xoshiro256PlusPlus::seed_from_u64(0),
            objects: Vec::new(),
        }
    }
}

#[derive(Clone)]
struct Entry<T: ProxyObject> {
    /// The proxy object (clone on write)
    obj: T,
    /// The hash of the object data in memory, initialized as random
    integrity: [u8; 32],
}

impl<T: ProxyObject> ProxyList<T> {
    /// Allocate a new proxy object in memory and return its address
    pub fn allocate(&mut self, mem: &mut Memory, t: T) -> Result<u64, Error> {
        // allocate the proxy object in memory
        let pointer = mem.heap_mut().alloc(t.mem_size())?;
        self.create_entry(mem, pointer, t)?;
        Ok(pointer)
    }

    /// Create a new proxy object at the given pointer.
    ///
    /// Return the handle to the proxy object
    ///
    /// # Error
    /// If error occurs, a potentially corrupted object will be left in memory,
    /// and the entry is not created
    fn create_entry(&mut self, mem: &mut Memory, pointer: u64, t: T) -> Result<u32, Error> {
        // allocate new entry and handle
        if self.objects.len() >= MAX_OBJECTS as usize {
            return Err(Error::ProxyOutOfMemory);
        }
        let mut integrity = [0; 32];
        let handle = self.objects.len() as u32;
        Self::write_proxy_object(&mut self.rng, mem, pointer, handle, &t, &mut integrity)?;
        // creating entry here to help elide copying
        self.objects.push(Arc::new(Entry { obj: t, integrity }));
        Ok(handle)
    }

    /// Write a proxy object to memory
    fn write_proxy_object(
        rng: &mut Xoshiro256PlusPlus,
        mem: &mut Memory,
        pointer: u64,
        handle: u32,
        t: &T,
        hash_out: &mut [u8; 32],
    ) -> Result<(), Error> {
        let size = t.mem_size();
        if size < 4 {
            return Err(Error::InvalidProxyObjectSize(size));
        }
        let mut hash = Sha256::new();
        let mut w = mem.write(pointer, Some(RegionType::Heap.into()))?;
        w.write_u32(handle)?;
        hash.update(handle.to_le_bytes());

        // create garbage data
        let garbage_size = size - 4;
        let chunks = garbage_size / 8;
        for _ in 0..chunks {
            let n = rng.next_u64();
            w.write_u64(n)?;
            hash.update(n.to_le_bytes());
        }
        let remaining = garbage_size % 8;
        if remaining > 0 {
            let n = rng.next_u64();
            let bytes = n.to_le_bytes();
            for i in 0..remaining {
                w.write_u8(bytes[i as usize])?;
            }
            hash.update(&bytes[..remaining as usize]);
        }
        *hash_out = hash.finalize().into();
        Ok(())
    }

    /// Get the object at the given address in memory as a proxy object
    pub fn get_at_addr(&self, mem: &Memory, address: u64) -> Result<&T, Error> {
        let handle = self.get_checked_handle(mem, address)?;
        let e = &self.objects[handle as usize];
        Ok(&e.obj)
    }

    /// Get the object at the given address in memory as a proxy object
    /// for mutation.
    ///
    /// The proxy object is currently shared, it will be cloned,
    /// and the proxy will receive a new integrity hash. However,
    /// no cloning or updating will occur if the object is not shared.
    pub fn mut_at_addr<'s>(
        &'s mut self,
        mem: &mut Memory,
        pointer: u64,
    ) -> Result<&'s mut T, Error> {
        let handle = self.get_checked_handle(mem, pointer)?;
        // get mut object, clone on write
        // use pointer equality to check if it's cloned
        // note we cannot make multiple make_mut or get_mut calls,
        // because it's possible the object is changed in between
        let ptr_old = Arc::as_ptr(&self.objects[handle as usize]) as usize;
        let entry = Arc::make_mut(&mut self.objects[handle as usize]);
        let copied = (std::ptr::from_ref(entry) as usize) != ptr_old;

        // update the object in memory to a fresh copy
        if copied {
            Self::write_proxy_object(
                &mut self.rng,
                mem,
                pointer,
                handle,
                &entry.obj,
                &mut entry.integrity,
            )?;
        }
        Ok(&mut entry.obj)
    }

    /// Read the object at the given address and check its integrity.
    /// If OK, return the handle
    ///
    /// The entry is not returned to avoid borrowing
    fn get_checked_handle(&self, mem: &Memory, pointer: u64) -> Result<u32, Error> {
        let mut hash = Sha256::new();
        // read the handle
        let mut r = mem.read(pointer, Some(RegionType::Heap.into()), false)?;
        let handle: u32 = r.read_u32()?;
        hash.update(handle.to_le_bytes());
        let entry = match self.objects.get(handle as usize) {
            Some(obj) => obj,
            None => return Err(Error::InvalidProxyHandle(handle, pointer)),
        };
        let size = entry.obj.mem_size();
        let mut data = Vec::with_capacity(size as usize);
        for _ in 4..size {
            data.push(r.read_u8()?);
        }
        hash.update(&data);
        let integrity: [u8; 32] = hash.finalize().into();
        if integrity != entry.integrity {
            return Err(Error::CorruptedProxyObject(handle, pointer, size));
        }

        Ok(handle)
    }
}

pub trait ProxyObject: Clone + Send + Sync {
    /// Get the size of the object to mock in memory
    /// The size must be at least 4 bytes
    fn mem_size(&self) -> u32;
}
