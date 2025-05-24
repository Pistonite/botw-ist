use crate::{processor::Error, Core};

    fn parse_b(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let label_offset = Self::get_label_val(args)?;
        Ok(Box::new(BInstruction { label_offset }))
    }

#[derive(Clone)]
pub struct BInstruction {
    label_offset: u64,
}

impl ExecutableInstruction for BInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.b(self.label_offset)
    }
}

impl Core<'_, '_, '_> {
    /// Processes the ARM64 command `b label`
    ///
    /// The label address is pc relative
    pub fn b(&mut self, label_offset: u64) -> Result<(), Error> {
        let new_pc = (self.cpu.pc as i64) + (label_offset as i64) - 4;
        self.cpu.pc = new_pc as u64;
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn simple_b_test() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.cpu.pc = 0x1000;
    core.handle_string_command(&String::from("b 50"))?;
    assert_eq!(core.cpu.pc, 0x1050);
    Ok(())
}
