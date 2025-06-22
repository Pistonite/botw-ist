#![allow(non_snake_case)]
use crate::memory::{self, MemObject, Memory, Ptr};

#[derive(MemObject, Default, Clone, Copy)]
#[size(0x10)]
pub struct SafeString {
    #[offset(0x0)]
    pub vtable: u64,
    #[offset(0x8)]
    pub mStringTop: Ptr![u8],
}

impl SafeString {
    const VTABLE_OFFSET: u32 = 0x023556C0;
    pub fn cstr(&self) -> Ptr![u8] {
        self.mStringTop
    }
}

impl Ptr![SafeString] {
    /// Call SafeString constructor
    pub fn construct(self, m: &mut Memory) -> Result<(), memory::Error> {
        let vptr = m.main_start() + SafeString::VTABLE_OFFSET as u64;
        Ptr!(&self->vtable).store(&vptr, m)?;
        // set to empty
        self.cstr(m)?.store(&0, m)?;
        Ok(())
    }

    pub fn utf8_lossy(self, memory: &Memory) -> Result<String, memory::Error> {
        let cstr_ptr = self.cstr(memory)?;
        let bytes = cstr_ptr.load_zero_terminated(memory)?;
        // FIXME: unstable
        // Ok(String::from_utf8_lossy_owned(bytes))
        Ok(String::from_utf8_lossy(&bytes).into_owned())
    }

    /// Get the cstr pointer (does not ensure termination, like the safe string in game)
    pub fn cstr(self, memory: &Memory) -> Result<Ptr![u8], memory::Error> {
        Ptr!(&self->mStringTop).load(memory)
    }
}

#[derive(MemObject, Clone)]
#[size(0x58)]
pub struct FixedSafeString40 {
    #[offset(0x0)]
    pub base: SafeString,
    #[offset(0x10)]
    pub mBufferSize: i32,
    #[offset(0x14)]
    pub mBuffer: [u8; 64],
}

impl FixedSafeString40 {
    const VTABLE_OFFSET: u32 = 0x2356A90;
    /// Get the cstr pointer (does not ensure termination, like the safe string in game)
    pub fn cstr(&self) -> Ptr![u8] {
        self.base.mStringTop
    }
}

impl Ptr![FixedSafeString40] {
    pub fn construct(self, m: &mut Memory) -> Result<(), memory::Error> {
        let vptr = m.main_start() + FixedSafeString40::VTABLE_OFFSET as u64;
        Ptr!(&self->base.vtable).store(&vptr, m)?;
        Ptr!(&self->mBufferSize).store(&0x40, m)?;
        let buffer_ptr = Ptr!(&self->mBuffer).reinterpret::<u8, 1>();
        Ptr!(&self->base.mStringTop).store(&buffer_ptr, m)?;
        buffer_ptr.store(&0, m)?;
        Ok(())
    }

    /// Get the cstr pointer (does not ensure termination, like the safe string in game)
    pub fn cstr(self, memory: &Memory) -> Result<Ptr![u8], memory::Error> {
        Ptr!(&self->base.mStringTop).load(memory)
    }

    pub fn safe_store(
        self,
        value: impl AsRef<str>,
        memory: &mut Memory,
    ) -> Result<(), memory::Error> {
        let string = value.as_ref();
        if string.len() < 64 {
            // safe to fit the whole string
            return self.cstr(memory)?.store_string(string, memory);
        }
        // truncate to 63 bytes + null
        let bytes = string.as_bytes();
        self.cstr(memory)?
            .store_bytes_plus_nul(&bytes[..63], memory)
    }

    pub fn utf8_lossy(self, memory: &Memory) -> Result<String, memory::Error> {
        let cstr_ptr = self.cstr(memory)?;
        let bytes = cstr_ptr.load_zero_terminated(memory)?;

        // FIXME: unstable
        // Ok(String::from_utf8_lossy_owned(bytes))
        Ok(String::from_utf8_lossy(&bytes).into_owned())
    }
}

impl std::fmt::Display for FixedSafeString40 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            String::from_utf8_lossy(
                &self.mBuffer[..self
                    .mBuffer
                    .iter()
                    .position(|&x| x == 0)
                    .unwrap_or(self.mBuffer.len())],
            )
        )
    }
}

impl Default for FixedSafeString40 {
    // note this does NOT create a valid FixedSafeString40 in emulated memory
    // this is only to construct temporary default instance in Rust
    fn default() -> Self {
        Self {
            base: SafeString::default(),
            mBufferSize: 0x40,
            mBuffer: [0; 64],
        }
    }
}
