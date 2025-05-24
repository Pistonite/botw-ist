/// Macro for making register names
#[macro_export]
#[rustfmt::skip]
macro_rules! reg {
    (x[$idx:expr]) => { blueflame::processor::RegName::x($idx as u8) };
    (w[$idx:expr]) => { blueflame::processor::RegName::w($idx as u8) };
    (s[$idx:expr]) => { blueflame::processor::RegName::s($idx as u8) };
    (d[$idx:expr]) => { blueflame::processor::RegName::d($idx as u8) };
    (q[$idx:expr]) => { blueflame::processor::RegName::q($idx as u8) };
    (sp) => { blueflame::processor::RegName::sp() };
    (lr) => { blueflame::processor::RegName::x(30) };
    (xzr) => { blueflame::processor::RegName::xzr() };
    (wzr) => { blueflame::processor::RegName::wzr() };
}
