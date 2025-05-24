use crate::processor::instruction_registry::RegisterType;

use crate::processor::Error;
use crate::Core;

    fn parse_bfxil(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let lsb = Self::get_imm_val(&collected_args[2])?;
        let width = Self::get_imm_val(&collected_args[3])?;

        Ok(Box::new(BfxilInstruction { rd, rn, lsb, width }))
    }


#[derive(Clone)]
pub struct BfxilInstruction {
    rd: RegisterType,
    rn: RegisterType,
    lsb: i64,
    width: i64,
}

impl ExecutableInstruction for BfxilInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.bfxil(self.rd, self.rn, self.lsb, self.width)
    }
}

impl Core<'_, '_, '_> {
    /// Processes ARM64 command `bfxil xd, xn, lsb, width`
    ///
    /// Bitfield extract and insert at low end copies any number of low-order bits from a source register into the same number of adjacent bits at the low end in the destination register, leaving other bits unchanged.
    pub fn bfxil(
        &mut self,
        xd: RegisterType,
        xn: RegisterType,
        lsb: i64,
        width: i64,
    ) -> Result<(), Error> {
        let xd_val = self.cpu.read_gen_reg(&xd)? as u64;
        let xn_val = self.cpu.read_gen_reg(&xn)? as u64;
        let lsb_val = lsb as u64;
        let width_val = width as u64;

        let mask = (1u64 << width_val) - 1;

        let extracted = (xn_val & mask) << lsb_val;

        let cleared_dst = xd_val & !(mask << lsb_val);

        self.cpu
            .write_gen_reg(&xd, (cleared_dst | extracted) as i64)?;
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn simple_bxfil_test() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.handle_string_command(&String::from("mov x1, #0x00000FFF"))?;
    core.handle_string_command(&String::from("mov x2, #0xFFFF0000"))?;
    core.handle_string_command(&String::from("bfxil x2, x1, #4, #8"))?;
    //Makes this into 0xFFFF0FF0
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::XReg(2))?, 4294905840);
    Ok(())
}
