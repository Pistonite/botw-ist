use enumset::{EnumSet, EnumSetType};

pub struct FlagStack<T: EnumSetType> {
    stack: Vec<EnumSet<T>>,
}
