pub type FlagList<T> = Vec<Flag<T>>;

#[derive(Debug, Clone)]
pub struct Flag<T: Clone>
{
    value: T,
    initial_value: T,
    hash: i32,
    name: String, // Even if not required to run, useful for debugging purposes
    is_program_readable: bool,
    is_program_writeable: bool,
}

impl<T: Clone> Flag<T>
{
    pub fn new(
        initial_value: T,
        name: String,
        hash: i32,
        readable: bool,
        writeable: bool,
    ) -> Self {
        Flag {
            value: initial_value.clone(),
            initial_value,
            hash,
            name,
            is_program_readable: readable,
            is_program_writeable: writeable,
        }
    }

    pub fn from_name_value(name: &str, value: T) -> Self {
        let hash = get_hash(name);
        Flag {
            value: value.clone(),
            initial_value: value,
            hash,
            name: name.to_string(),
            is_program_readable: true,
            is_program_writeable: true,
        }
    }

    pub fn get(&self) -> &T {
        &self.value
    }
    pub fn set(&mut self, value: T) {
        self.value = value;
    }
    pub fn reset(&mut self) {
        self.value = self.initial_value.clone();
    }

    pub fn hash(&self) -> i32 {
        self.hash
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn readable(&self) -> bool {
        self.is_program_readable
    }

    pub fn writable(&self) -> bool {
        self.is_program_writeable
    }
}

impl<T: Clone> Flag<Box<[T]>>
{
    pub fn len(&self) -> usize {
        self.value.len()
    }
    pub fn get_at<I: FlagIndex>(&self, idx: I) -> Option<&T> {
        self.value.get(idx.to_index()?)
    }
    #[must_use]
    pub fn reset_at<I: FlagIndex>(&mut self, idx: I) -> bool {
        let Some(i) = idx.to_index() else {
            return false;
        };
        let Some(init_value) = self.initial_value.get(i) else {
            return false;
        };
        let init_value = init_value.clone();
        let Some(x) = self.value.get_mut(i) else {
            return false;
        };
        *x = init_value;
        true
    }
    #[must_use]
    pub fn set_at<I: FlagIndex>(&mut self, idx: I, value: T) -> bool {
        let Some(i) = idx.to_index() else {
            return false;
        };
        let Some(x) = self.value.get_mut(i) else {
            return false;
        };
        *x = value;
        true
    }
}

pub fn get_hash(name: &str) -> i32 {
    crc32fast::hash(name.as_bytes()) as i32
}

/// Helper trait for checking index for getter and setters
pub trait FlagIndex {
    fn to_index(self) -> Option<usize>;
}
#[rustfmt::skip]
const _: () = {
    impl FlagIndex for u8 { #[inline(always)] fn to_index(self) -> Option<usize> { Some(self as usize) } }
    impl FlagIndex for u16 { #[inline(always)] fn to_index(self) -> Option<usize> { Some(self as usize) } }
    impl FlagIndex for u32 { #[inline(always)] fn to_index(self) -> Option<usize> { Some(self as usize) } }
    impl FlagIndex for usize { #[inline(always)] fn to_index(self) -> Option<usize> { Some(self) } }
    impl FlagIndex for u64 { #[inline(always)] fn to_index(self) -> Option<usize> { 
        if self > usize::MAX as u64 {
            None
        } else {
            Some(self as usize)
        }
    }}
    impl FlagIndex for i8 { #[inline(always)] fn to_index(self) -> Option<usize> { (self as isize).to_index() } }
    impl FlagIndex for i16 { #[inline(always)] fn to_index(self) -> Option<usize> { (self as isize).to_index() } }
    impl FlagIndex for i32 { #[inline(always)] fn to_index(self) -> Option<usize> { (self as isize).to_index() } }
    impl FlagIndex for isize { #[inline(always)] fn to_index(self) -> Option<usize> { 
        if self < 0 {
            None
        } else {
            Some(self as usize)
        }
    } }
    impl FlagIndex for i64 { #[inline(always)] fn to_index(self) -> Option<usize> { 
        if self < 0 || self > usize::MAX as i64 {
            None
        } else {
            Some(self as usize)
        }
    } }
};
