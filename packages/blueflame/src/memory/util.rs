use std::mem;

// somewhat hacky way to easily implement ByteAble for any of the primitive types that
// already implement from_le_bytes, to_le_bytes, etc.
macro_rules! impl_ByteAble {
    ($x:tt) => {
        impl ByteAble for $x {
            const SIZE: usize = mem::size_of::<$x>();

            fn from_bytes(val_as_bytes: &[u8]) -> Self {
                let mut val_as_bytes_resized: [u8; Self::SIZE] = [0; Self::SIZE];
                for i in 0..std::cmp::min(Self::SIZE, val_as_bytes.len()) {
                    val_as_bytes_resized[i] = val_as_bytes[i];
                }
                $x::from_le_bytes(val_as_bytes_resized)
            }

            fn to_bytes(&self) -> Vec<u8> {
                $x::to_le_bytes(*self).to_vec()
            }

            fn size(&self) -> usize {
                Self::SIZE
            }
        }
    };
}

pub(crate) trait ByteAble: Clone {
    // This constant is required for the macro above to work
    // Don't need to give this a meaningful value, it is just used for the above macro
    const SIZE: usize;
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(val: &[u8]) -> Self;
    fn size(&self) -> usize;
}

//implementation for primitive types can be done by macro, add primitive types here as needed for read/writing to memory,
//then proc.mem_write_val<T> will work

//unsigned ints
impl_ByteAble! {u64}
impl_ByteAble! {u32}
impl_ByteAble! {u16}
impl_ByteAble! {u8}
//signed ints
impl_ByteAble! {i64}
impl_ByteAble! {i32}
//floats
impl_ByteAble! {f32}
impl_ByteAble! {f64}

impl ByteAble for bool {
    const SIZE: usize = 1;

    fn to_bytes(&self) -> Vec<u8> {
        vec![if *self { 1 } else { 0 }]
    }

    fn from_bytes(val: &[u8]) -> Self {
        *val.first().unwrap() == 1
    }

    fn size(&self) -> usize {
        Self::SIZE
    }
}

impl<T: ByteAble, const N: usize> ByteAble for [T; N]
where
    T: Default,
    T: Copy,
{
    const SIZE: usize = mem::size_of::<T>() * N;

    fn to_bytes(&self) -> Vec<u8> {
        let mut out_vec: Vec<u8> = Vec::new();
        let mut index: usize = 0;
        while index < N {
            let mut temp_vec = self[index].to_bytes();
            out_vec.append(&mut temp_vec);
            index += 1;
        }
        out_vec
    }

    fn from_bytes(val: &[u8]) -> Self {
        let mut out: [T; N] = [T::default(); N];
        let mut index: usize = 0;
        while index < N {
            out[index] =
                T::from_bytes(&val[index * out[index].size()..(index + 1) * out[index].size()]);
            index += 1;
        }
        out
    }

    fn size(&self) -> usize {
        Self::SIZE
    }
}

impl<T> ByteAble for Vec<T>
where
    T: ByteAble,
{
    const SIZE: usize = 0; // not used

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::<u8>::new();
        for item in self {
            let val_in_bytes = item.to_bytes();
            for byte in val_in_bytes {
                bytes.push(byte);
            }
        }
        bytes
    }

    fn from_bytes(val: &[u8]) -> Self {
        let byte_size = size_of::<T>();
        let num_entries = val.len() / byte_size;
        let mut return_vec = Vec::new();
        for e in 0..num_entries {
            return_vec.push(T::from_bytes(&val[e..e + byte_size]));
        }
        return_vec
    }

    fn size(&self) -> usize {
        let mut sz: usize = 0;
        for item in self {
            sz += item.size();
        }
        sz
    }
}
