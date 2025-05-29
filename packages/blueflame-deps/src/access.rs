/// Macro to define flags for memory access
///
/// Use [`perm`] or [`region`] to specify just permission or region bits
///
/// # Example
/// ```rust,ignore
/// // short hands
/// access!(default) // All bits off - let Memory decide the default bits
/// access!(execute) // Perm: execute | read, Region: text
/// access!(read)    // Perm: read          , Region: all
/// access!(write)   // Perm: write         , Region: writable
/// access!(force)   // Perm: ALL + Force   , Region: all
/// ```
///
#[macro_export]
macro_rules! access {
    ($single:ident) => {{ blueflame::memory::AccessFlags::$single() }};
}

/// Macro to define permission bits.
///
/// These always return only the permission bits, or with
/// the force bit if `force` is specified.
///
/// # Example
/// ```rust,ignore
/// perm!(r) // Read
/// perm!(w) // Write
/// perm!(x) // Execute
/// perm!(rx) // Read + Execute
/// perm!(rw) // Read + Write
/// perm!(rwx) // All permissions
///
/// perm!(r, force) // Read + Force bit
/// ```
#[macro_export]
macro_rules! perm {
    ($single:ident) => {{ blueflame::memory::__Perm::$single() }};
    ($single:ident, force) => {{ blueflame::memory::__Perm::$single() | blueflame::memory::AccessFlag::Force }};
}

/// Macro to define region/section bits.
///
/// These always return only the region/section bits
///
/// # Example
/// ```rust,ignore
/// region!(stack)   // stack
/// region!(program) // text | rodata | data
/// region!(heap)    // heap
/// region!(text)    // text
/// region!(data)    // data
/// region!(rodata)  // rodata
/// region!(all)     // text | rodata | data | stack | heap
/// region!(writable) // data | stack | heap
/// ```
#[macro_export]
macro_rules! region {
    ($single:ident) => {{ blueflame::memory::__Region::$single() }};
}
