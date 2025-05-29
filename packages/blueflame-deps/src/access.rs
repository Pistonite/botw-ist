/// Make an instance of an `AccessFlags`
///
/// # Example
/// ```rust,ignore
/// // short hands
/// access!(default) // All bits off
/// access!(execute) // Perm: execute | read, Region: text
/// access!(read)    // Perm: read          , Region: all
/// access!(write)   // Perm: write         , Region: data | stack | heap
///
/// // long hands, form: access!(<region>, <perm>)
/// access!(text, rx)   // Perm: execute | read, Region: text
/// access!(rodata, rw) // Perm: read    | write,   Region: rodata
///
/// // ^Region can be a shorthand:
/// // program = text | rodata | data
/// // all = text | rodata | data | stack | heap
/// // writable = data | stack | heap
/// // executable = text
///
/// ```
///
/// Use [`perm`] or [`region`] to specify single permission or region.
#[macro_export]
macro_rules! access {
    ($single:ident) => {{ blueflame::memory::AccessFlags::$single() }};
}
