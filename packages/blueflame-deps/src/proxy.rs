/// Macro for accessing proxy from a process in a more readable way
#[macro_export]
macro_rules! proxy {
    (let [$guard_var:ident] $var:ident = * $pointer:ident as $field:ident in $proc:ident) => {
        let $guard_var = $proc.proxies().$field.read($proc.memory());
        let $var = $guard_var.get($pointer)?;
    };
    (let $var:ident = * $pointer:ident as $field:ident in $proc:ident) => {
        let guard = $proc.proxies().$field.read($proc.memory());
        let $var = guard.get($pointer)?;
    };
    (let mut [$guard_var:ident] $var:ident = * $pointer:ident as $field:ident in $proc:ident) => {
        let mut $guard_var = $proc.proxies_mut(|p| &mut p.$field);
        let mut $var = $guard_var.get_mut($pointer)?;
    };
    (let mut $var:ident = * $pointer:ident as $field:ident in $proc:ident) => {
        let mut guard = $proc.proxies_mut(|p| &mut p.$field);
        let mut $var = guard.get_mut($pointer)?;
    };
}
