use std::sync::Arc;

use rand_xoshiro::rand_core::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use sha2::{Digest, Sha256};

use super::error::Error;
use super::memory::Memory;
use super::region::RegionType;

/// The maximum number of proxy objects per type
pub const MAX_OBJECTS: u32 = 1024000;

/// Holds all proxy objects in memory
#[derive(Default, Clone)]
pub struct Proxies {
    // just as a placeholder
    string_proxies: Arc<ProxyList<String>>,
}

impl Proxies {
    /// (EXAMPLE) operate on a string proxy at the given address
    pub fn get_string(&self, mem: &Memory, address: u64) -> Result<&String, Error> {
        self.string_proxies.get_at_addr(mem, address)
    }

    /// (EXAMPLE) operate on a string proxy at the given address for mutation
    pub fn mut_string<'s>(&'s mut self, mem: &mut Memory, address: u64) -> Result<&'s mut String, Error> {
        Arc::make_mut(&mut self.string_proxies).mut_at_addr(mem, address)
    }

    /// (EXAMPLE) allocate a string proxy in memory and returns a pointer to it
    pub fn allocate_string(&mut self, mem: &mut Memory, s: String) -> Result<u64, Error> {
        Arc::make_mut(&mut self.string_proxies).allocate(mem, s)
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
        self.objects.push(Arc::new(Entry {
            obj: t,
            integrity
        }));
        Ok(handle)
    }

    /// Write a proxy object to memory
    fn write_proxy_object(rng: &mut Xoshiro256PlusPlus, mem: &mut Memory, pointer: u64, handle: u32, t: &T, hash_out: &mut [u8; 32]) -> Result<(), Error> {
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
            hash.update(&n.to_le_bytes());
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
    pub fn mut_at_addr<'s>(&'s mut self, mem: &mut Memory, pointer: u64) -> Result<&'s mut T, Error> {
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
            Self::write_proxy_object(&mut self.rng, mem, pointer, handle, &entry.obj, &mut entry.integrity)?;
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
