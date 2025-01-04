
/// The region where the program segments are loaded into memory
///
/// This region is aligned to 0x10000, and contains the .text (RX)
/// .rodata (R) and .data (RW) segments of all modules loaded (rtld,
/// main, subsdk, sdk, in this order).
pub struct ProgramMemory {
}
