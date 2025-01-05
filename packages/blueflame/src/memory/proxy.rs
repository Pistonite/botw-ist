use std::sync::{Arc, RwLock};

use rand_xoshiro::rand_core::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use sha2::{Digest, Sha256};

use super::error::Error;
use super::memory::Memory;
use super::RegionType;

/// The maximum number of proxy objects per type
pub const MAX_OBJECTS: u32 = 1024000;

/// Holds all proxy objects in memory
#[derive(Default)]
pub struct Proxies {
    // just as a placeholder
    string_proxies: Arc<RwLock<ProxyList<String>>>,
}

impl Clone for Proxies {
    fn clone(&self) -> Self {
        Self {
            string_proxies: self.string_proxies.clone(),
        }
    }
}

impl Proxies {
    /// (EXAMPLE) operate on a string proxy at the given address
    pub fn with_string<R, F: FnOnce(&String) -> R>(&self, mem: &Memory, address: u64, f: F) -> Result<R, Error> {
        Self::with_read(&self.string_proxies, mem, address, f)
    }

    /// (EXAMPLE) operate on a string proxy at the given address for mutation
    pub fn with_string_mut<R, F: FnOnce(&mut String) -> R>(&self, mem: &mut Memory, address: u64, f: F) -> Result<R, Error> {
        Self::with_write(&self.string_proxies, mem, address, f)
    }

    /// (EXAMPLE) allocate a string proxy in memory and returns a pointer to it
    pub fn allocate_string(&self, mem: &mut Memory, s: String) -> Result<u64, Error> {
        Self::allocate(&self.string_proxies, mem, s)
    }

    #[inline]
    fn with_read<R, T: ProxyObject, F:FnOnce(&T) -> R>(
        list: &Arc<RwLock<ProxyList<T>>>, mem: &Memory, address: u64, f:F) -> Result<R, Error> {
        let list = match list.read() {
            Ok(l) => l,
            Err(_) => return Err(Error::Unexpected("failed to lock proxy list, for reading".to_string())),
        };
        let t = list.get_at_addr(mem, address)?;
        Ok(f(t))
    }

    #[inline]
    fn with_write<R, T: ProxyObject, F:FnOnce(&mut T) -> R>(
        list: &Arc<RwLock<ProxyList<T>>>, mem: &mut Memory, address: u64, f:F) -> Result<R, Error> {
        let mut list = match list.write() {
            Ok(l) => l,
            Err(_) => return Err(Error::Unexpected("failed to lock proxy list for writing".to_string())),
        };
        let t = list.mut_at_addr(mem, address)?;
        Ok(f(t))
    }

    #[inline]
    fn allocate<T: ProxyObject>(
        list: &Arc<RwLock<ProxyList<T>>>, mem: &mut Memory, t: T) -> Result<u64, Error> {
        let mut list = match list.write() {
            Ok(l) => l,
            Err(_) => return Err(Error::Unexpected("failed to lock proxy list for writing".to_string())),
        };
        list.allocate(mem, t)
    }

}

impl ProxyObject for String {
    fn mem_size(&self) -> u32 {
        0x100
    }
}


pub struct ProxyList<T: ProxyObject> {
    rng: Xoshiro256PlusPlus,
    objects: Vec<Entry<T>>,
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

pub struct Entry<T: ProxyObject> {
    obj: T,
    integrity: [u8; 32],
}

impl<T: ProxyObject> ProxyList<T> {
    /// Allocate the proxy object in memory and return the address
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
        let mut entry = Entry {
            obj: t,
            integrity: [0; 32],
        };
        let handle = self.objects.len() as u32;
        self.write_proxy_object(mem, pointer, handle, &entry.obj, &mut entry.integrity)?;
        self.objects.push(entry);
        Ok(handle)
    }

    /// Write a proxy object to memory
    fn write_proxy_object(&mut self, mem: &mut Memory, pointer: u64, handle: u32, t: &T, hash_out: &mut [u8; 32]) -> Result<(), Error> {
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
            let n = self.rng.next_u64();
            w.write_u64(n)?;
            hash.update(&n.to_le_bytes());
        }
        let remaining = garbage_size % 8;
        if remaining > 0 {
            let n = self.rng.next_u64();
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
        let e = self.get_entry(mem, address)?;
        Ok(&e.obj)
    }

    /// Get the object at the given address in memory as a proxy object
    /// for mutation. The object will be cloned, and the memory will be 
    /// updated with the new handle
    pub fn mut_at_addr(&mut self, mem: &mut Memory, pointer: u64) -> Result<&mut T, Error> {
        let e = self.get_entry(mem, pointer)?;
        let cloned  = e.obj.clone();
        let handle = self.create_entry(mem, pointer, cloned)?;
        Ok(&mut self.objects[handle as usize].obj)
    }

    /// Get a proxy object from memory
    fn get_entry(&self, mem: &Memory, pointer: u64) -> Result<&Entry<T>, Error> {
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

        Ok(&entry)
    }

    
}

pub trait ProxyObject: Clone {
    /// Get the size of the object to mock in memory
    /// The size must be at least 4 bytes
    fn mem_size(&self) -> u32;
}
