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
}

impl<T: Clone> Flag<Box<[T]>>
{
    // TODO --cleanup: array range check
    pub fn reset_at(&mut self, idx: usize) {
        let init_value = self.initial_value[idx].clone();
        self.value[idx] = init_value;
    }
    pub fn set_at(&mut self, idx: usize, value: T) {
        self.value[idx] = value;
    }
}

pub fn get_hash(name: &str) -> i32 {
    crc32fast::hash(name.as_bytes()) as i32
}
