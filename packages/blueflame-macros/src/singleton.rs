/// Get the singleton info for the given path
#[macro_export]
macro_rules! singleton_info {
    ($($path:ident)::* ( $env:expr )) => {
        blueflame::game::singleton::SingletonInfo {
            name: $($path)::*::NAME,
            rel_start: $($path)::*::rel_start($env),
            size: $($path)::*::size($env),
            main_offset: $($path)::*::main_offset($env),
        }
    };
}

/// Load the single instance pointer for the given path
#[macro_export]
macro_rules! singleton_instance {
    ($($path:ident)::* ($proc:expr, $env:expr )) => {{
        let proc = { $proc };
        let ptr = proc.main_start() + $($path)::*::main_offset($env) as u64;
        let ptr = blueflame::memory::Ptr!(<blueflame::memory::Ptr![$($path)::*::Type]>(ptr));
        ptr.load(proc.memory())
    }};
}
