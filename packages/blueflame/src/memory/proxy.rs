use std::panic::UnwindSafe;
use std::sync::Arc;

use rand_xoshiro::rand_core::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use sha2::{Digest, Sha256};

use crate::memory::{perm, region, Error, Memory};

pub use blueflame_deps::proxy;

/// The maximum number of proxy objects per type
pub const MAX_OBJECTS: u32 = 1024000;

/// Clone-on-Write wrapper for inner ProxyList
#[derive(Debug, Clone)]
pub struct ProxyList<T: ProxyObject>(Arc<ProxyListInner<T>>);
impl<T: ProxyObject> Default for ProxyList<T> {
    fn default() -> Self {
        Self(Arc::new(Default::default()))
    }
}
impl<T: ProxyObject> ProxyList<T> {
    /// Acquire a read-only guard for accessing objects in memory.
    ///
    /// This ensures that memory cannot be mutated while a proxy object
    /// is being accessed
    pub fn read<'a, 'b>(&'a self, mem: &'b Memory) -> ProxyGuard<'a, 'b, T> {
        ProxyGuard {
            list: self,
            memory: mem,
        }
    }

    /// Acquire a mutable guard for accessing objects in memory.
    ///
    /// This ensures that we keep exclusive (mutable) access to the memory
    /// while the proxy object is being mutated
    pub fn write<'a, 'b>(&'a mut self, mem: &'b mut Memory) -> ProxyGuardMut<'a, 'b, T> {
        ProxyGuardMut {
            list: self,
            memory: mem,
        }
    }
}

/// Read-only guard for accessing proxy objects in memory.
///
/// This ensures that memory cannot be mutated while a proxy object
/// is being accessed
pub struct ProxyGuard<'a, 'b, T: ProxyObject> {
    list: &'a ProxyList<T>,
    memory: &'b Memory,
}

/// Mutable guard for accessing proxy objects in memory.
///
/// This ensures that we keep exclusive (mutable) access to the memory
/// while proxy objects are being mutated
pub struct ProxyGuardMut<'a, 'b, T: ProxyObject> {
    list: &'a mut ProxyList<T>,
    memory: &'b mut Memory,
}

/// Mutable guard for an acquired proxy object
///
/// This can be used to access and mutate the proxy object.
/// While this guard is alive, no other proxy object can be accessed
/// or mutated.
///
/// When this guard is dropped, the corresponding memory location
/// will be updated with a new integrity hash to signify that the
/// object has been mutated.
pub struct ProxyObjectGuardMut<'g, T: ProxyObject> {
    // both reference live as long as the list guard ('g)
    obj: &'g mut Entry<T>,
    handle: u32,
    address: u64,
    // treat memory and list as being borrowed mutably by the proxy
    memory: &'g mut Memory,
    list_rng: &'g mut Xoshiro256PlusPlus,
}

impl<T> std::ops::Deref for ProxyObjectGuardMut<'_, T>
where
    T: ProxyObject,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.obj.obj
    }
}

impl<T> std::ops::DerefMut for ProxyObjectGuardMut<'_, T>
where
    T: ProxyObject,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.obj.obj
    }
}

/// The inner list that holds all the proxy objects
#[derive(Debug, Clone)]
struct ProxyListInner<T: ProxyObject> {
    rng: Xoshiro256PlusPlus,
    objects: Vec<Arc<Entry<T>>>,
}

#[derive(Debug, Clone)]
struct Entry<T: ProxyObject> {
    /// The proxy object (clone on write)
    obj: T,
    /// The hash of the object data in memory, initialized as random
    integrity: [u8; 32],
}

impl<T: ProxyObject> Default for ProxyListInner<T> {
    fn default() -> Self {
        Self {
            // same seed for every run is fine, as
            // we are just using it to prevent accidental
            // corruption of the proxy object
            rng: Xoshiro256PlusPlus::seed_from_u64(0),
            objects: Vec::new(),
        }
    }
}

impl<T: ProxyObject> ProxyGuard<'_, '_, T> {
    /// Get the proxy object at the given address in memory
    ///
    /// Returns [`Error::InvalidProxyHandle`] if the corresponding memory location has been changed,
    /// as determined by the integrity hash
    pub fn get(&self, address: u64) -> Result<&T, Error> {
        self.list.0.get_at_addr(self.memory, address)
    }
}

impl<T: ProxyObject> ProxyGuardMut<'_, '_, T> {
    /// Get the proxy object at the given address in memory for mutation
    ///
    /// Returns [`Error::InvalidProxyHandle`] if the corresponding memory location has been changed,
    /// as determined by the integrity hash.
    pub fn get_mut<'g>(&'g mut self, address: u64) -> Result<ProxyObjectGuardMut<'g, T>, Error> {
        let handle = self.list.0.get_checked_handle(self.memory, address)?;
        // clone the proxy list on write if needed
        let mut_list = Arc::make_mut(&mut self.list.0);
        // clone the entry on write if needed
        let entry = Arc::make_mut(&mut mut_list.objects[handle as usize]);
        return Ok(ProxyObjectGuardMut {
            obj: entry,
            memory: self.memory,
            handle,
            address,
            list_rng: &mut mut_list.rng,
        });
    }

    /// Allocate space in memory for the new proxy object T and return its address
    pub fn alloc<'g>(&'g mut self, t: T) -> Result<u64, Error> {
        let mut_list = Arc::make_mut(&mut self.list.0);
        let pointer = mut_list.allocate(self.memory, t)?;
        Ok(pointer)
    }
}

impl<T: ProxyObject> Drop for ProxyObjectGuardMut<'_, T> {
    fn drop(&mut self) {
        let _ = ProxyListInner::<T>::write_proxy_object(
            self.list_rng,
            self.memory,
            self.address,
            self.handle,
            &self.obj.obj,
            &mut self.obj.integrity,
        );
    }
}

impl<T: ProxyObject> ProxyListInner<T> {
    /// Allocate a new proxy object in memory and return its address
    fn allocate(&mut self, mem: &mut Memory, t: T) -> Result<u64, Error> {
        // allocate the proxy object in memory
        let pointer = mem.alloc(t.mem_size())?;
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
        let mut w = mem.write(pointer, perm!(w) | region!(heap))?;
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
    fn get_at_addr(&self, mem: &Memory, address: u64) -> Result<&T, Error> {
        let handle = self.get_checked_handle(mem, address)?;
        let e = &self.objects[handle as usize];
        Ok(&e.obj)
    }

    /// Read the object at the given address and check its integrity.
    /// If OK, return the handle
    ///
    /// The entry is not returned to avoid borrowing
    fn get_checked_handle(&self, mem: &Memory, pointer: u64) -> Result<u32, Error> {
        let mut hash = Sha256::new();
        // read the handle
        // TODO --cleanup: macro
        let mut r = mem.read(pointer, perm!(r) | region!(heap))?;
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

pub trait ProxyObject: Clone + Send + Sync + UnwindSafe + 'static {
    /// Get the size of the object to mock in memory
    /// The size must be at least 4 bytes
    fn mem_size(&self) -> u32;
}
