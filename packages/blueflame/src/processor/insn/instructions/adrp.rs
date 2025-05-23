use crate::processor::Error;
use crate::Core;

use crate::processor::instruction_registry::RegisterType;

impl Core<'_, '_, '_> {
    /// Processes ARM64 command adrp xd, label
    ///
    /// This instruction adds an immediate value that is shifted left by 12 bits, to the PC value to form a PC-relative address, with the bottom 12 bits masked out, and writes the result to the destination register.
    pub fn adrp(&mut self, xd: RegisterType, label_offset: u64) -> Result<(), Error> {
        // zero out bottom 12 bits to get to a page offset
        let label_page = label_offset & 0xFFFF_FFFF_FFFF_F000; // Align label address to 4 KB boundary
        let pc_page = self.cpu.pc & 0xFFFF_FFFF_FFFF_F000;
        let offset = label_page.wrapping_sub(pc_page);
        let page_val = pc_page.wrapping_add(offset);

        let result = pc_page + page_val;
        self.cpu.write_gen_reg(&xd, result as i64)?;
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn adrp_simple() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    // Check that other flags are unaffected
    core.cpu.pc = 0x4050;
    core.handle_string_command(&String::from("adrp x0, 0x1000"))?;
    assert_eq!(core.cpu.x[0], 0x5000);
    Ok(())
}
