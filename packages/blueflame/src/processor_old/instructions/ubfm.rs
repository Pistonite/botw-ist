use crate::processor::instruction_registry::RegisterType;

use crate::processor::Error;
use crate::Core;

impl Core<'_, '_, '_> {
    pub fn ubfm(
        &mut self,
        rd: RegisterType,
        rn: RegisterType,
        immr: i64,
        imms: i64,
    ) -> Result<(), Error> {
        let val = self.cpu.read_gen_reg(&rn)?;
        if imms >= immr {
            let start = immr;
            let end = imms + 1;
            let bits = select_bits(val, start, end);
            self.cpu.write_gen_reg(&rd, bits)?;
        } else {
            let start = 0;
            let end = imms + 1;
            let bits = select_bits(val, start, end);
            let regsz = match &rd {
                RegisterType::XReg(_) => 64,
                RegisterType::WReg(_) => 32,
                _ => return Err(Error::InvalidRegisterWrite("ubfm", rd)),
            };
            let bits = bits << (regsz - immr);
            self.cpu.write_gen_reg(&rd, bits)?;
        }
        Ok(())
    }
}

fn select_bits(value: i64, m: i64, n: i64) -> i64 {
    (value >> m) & ((1 << (n - m)) - 1)
}

#[cfg(test)]
#[test]
pub fn simple_ubfm_test() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.handle_string_command(&String::from("add w9, wzr, #11"))?; // b0...01011
    core.handle_string_command(&String::from("ubfm w8, w9, #1, #3"))?; // should end up with b101 (5)
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::WReg(8))?, 5);
    Ok(())
}
