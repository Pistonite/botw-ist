use crate::{processor::Error, Core};

    fn parse_bl(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let label_offset = Self::get_label_val(args)?;
        Ok(Box::new(BlInstruction { label_offset }))
    }

#[derive(Clone)]
pub struct BlInstruction {
    label_offset: u64,
}

impl ExecutableInstruction for BlInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.bl(self.label_offset)
    }

    // fn instruction_type(&self) -> Option<InstructionType> {
    //     Some(InstructionType::Branch)
    // }
}

impl Core<'_, '_, '_> {
    /// Processes ARM64 command `bl label`
    ///
    /// The label address is pc relative
    pub fn bl(&mut self, label_offset: u64) -> Result<(), Error> {
        let lr = self.cpu.pc + 4; // Save to next instruction, 4 bytes past current instruction
                                  // let new_pc = self.cpu.pc + relative_addr - 4;
                                  // self.cpu.pc = new_pc;
        self.cpu.x[30] = lr as i64;
        let func_address = self.cpu.pc.wrapping_add_signed((label_offset - 4) as i64);
        self.cpu.stack_trace.push((
            self.compute_ida_addr(self.cpu.pc),
            self.compute_ida_addr(func_address + 4),
        ));
        self.cpu.pc = func_address;
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn simple_bl_test() -> anyhow::Result<()> {
    use crate::processor::instruction_registry::RegisterType;

    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.cpu.pc = 0x1000;
    core.cpu.write_gen_reg(&RegisterType::XReg(30), 5)?;
    core.handle_string_command(&String::from("bl 50"))?;
    assert_eq!(core.cpu.pc, 0x1050);
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::XReg(30))?, 0x1004);
    Ok(())
}
