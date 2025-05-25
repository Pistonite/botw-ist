/// Get the singleton info for the given path
#[macro_export]
macro_rules! singleton_info {
    ($path:ident ( $env:expr )) => {
        blueflame::game::singleton::SingletonInfo {
            name: blueflame::game::singleton::$path::NAME,
            rel_start: blueflame::game::singleton::$path::rel_start($env),
            size: blueflame::game::singleton::$path::size($env),
            main_offset: blueflame::game::singleton::$path::main_offset($env),
        }
    };
}

/// Load the single instance pointer for the given path
#[macro_export]
macro_rules! singleton_instance {
    ($path:ident ($memory:expr)) => {{
        let mem = { $memory };
        let ptr = mem.main_start() + blueflame::game::singleton::$path::main_offset(mem.env()) as u64;
        let ptr = blueflame::memory::Ptr!(<blueflame::memory::Ptr![blueflame::game::singleton::$path::Type]>(ptr));
        ptr.load(mem)
    }};
}
