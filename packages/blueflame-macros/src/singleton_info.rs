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
