/// Macro for making register names
///
/// # Naming a register
/// - `reg!(x[0])` for x0 register
/// - `reg!(w[1])` for w1 register
/// - `reg!(s[2])` for s2 register
/// - `reg!(xzr)` for XZR register
/// - `reg!(sp)` for SP register
/// - `reg!(lr)` for LR register (x30)
///
/// # Binding register values to local variables
/// ```rust,ignore
/// // cpu is the identifier
/// reg! {
/// cpu:
///     x[0] => local_var: u64,
///     w[1] => another_var: u32,
/// };
/// ```
///
/// # Writing values to registers and returning
/// ```rust,ignore
/// // cpu is the identifier
/// return reg! { cpu: x[0] = 42 };
/// ```
#[macro_export]
#[rustfmt::skip]
macro_rules! reg {
    // names
    (x[$idx:expr]) => { blueflame::processor::RegName::x($idx as u8) };
    (w[$idx:expr]) => { blueflame::processor::RegName::w($idx as u8) };
    (s[$idx:expr]) => { blueflame::processor::RegName::s($idx as u8) };
    (d[$idx:expr]) => { blueflame::processor::RegName::d($idx as u8) };
    (q[$idx:expr]) => { blueflame::processor::RegName::q($idx as u8) };
    (sp) => { blueflame::processor::RegName::sp() };
    (lr) => { blueflame::processor::RegName::x(30) };
    (xzr) => { blueflame::processor::RegName::xzr() };
    (wzr) => { blueflame::processor::RegName::wzr() };

    // reads
    ($cpu:ident : $(,)? $regn:ident [ $regi:expr ] => let $var:ident : $t:ty $(,)? ) => {
         let $var: $t = $cpu.read($crate::reg!($regn[$regi]));
    };
    ($cpu:ident : $(,)? $regn:ident [ $regi:expr ] => let mut $var:ident : $t:ty $(,)? ) => {
         let mut $var: $t = $cpu.read($crate::reg!($regn[$regi]));
    };
    ($cpu:ident : $(,)? $regn:ident => let $var:ident : $t:ty $(,)? ) => {
         let $var: $t = $cpu.read($crate::reg!($regn));
    };
    ($cpu:ident : $(,)? $regn:ident => let mut $var:ident : $t:ty $(,)? ) => {
         let mut $var: $t = $cpu.read($crate::reg!($regn));
    };
    ($cpu:ident : $(,)? $regn:ident [ $regi:expr ] => let $var:ident : $t:ty $( , $($rest:tt)* )? ) => {
         let $var: $t = $cpu.read($crate::reg!($regn[$regi]));
         $( $crate::reg!($cpu : $($rest)*) )?
    };
    ($cpu:ident : $(,)? $regn:ident [ $regi:expr ] => let mut $var:ident : $t:ty $( , $($rest:tt)* )? ) => {
         let mut $var: $t = $cpu.read($crate::reg!($regn[$regi]));
         $( $crate::reg!($cpu : $($rest)*) )?
    };
    ($cpu:ident : $(,)? $regn:ident => let $var:ident : $t:ty $( , $($rest:tt)* )? ) => {
         let $var: $t = $cpu.read($crate::reg!($regn));
         $( $crate::reg!($cpu : $($rest)*) )?
    };
    ($cpu:ident : $(,)? $regn:ident => let mut $var:ident : $t:ty $( , $($rest:tt)* )? ) => {
         let mut $var: $t = $cpu.read($crate::reg!($regn));
         $( $crate::reg!($cpu : $($rest)*) )?
    };

    // writes
    ($cpu:ident : $(,)? $regn:ident [ $regi:expr ] = $value:expr $(,)?) => {
        $cpu.write($crate::reg!($regn [ $regi ]), $value);
    };
    ($cpu:ident : $(,)? $regn:ident [ $regi:expr ] = $value:expr $( , $($rest:tt)* )?) => {
        $cpu.write($crate::reg!($regn [ $regi ]), $value);
         $( $crate::reg!($cpu : $($rest)*) )?
    };
    ($cpu:ident : $(,)? $regn:ident = $value:expr $(,)?) => {
        $cpu.write($crate::reg!($regn), $value);
    };
    ($cpu:ident : $(,)? $regn:ident = $value:expr $( , $($rest:tt)* )?) => {
        $cpu.write($crate::reg!($regn), $value);
         $( $crate::reg!($cpu : $($rest)*) )?
    };
    ($cpu:ident : $(,)? return) => {{
        $cpu.return_to_lr()?;
        return Ok(());
    }};
}
