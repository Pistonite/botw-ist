use bitfield_struct::bitfield;
use disarm64::decoder::MOVEWIDE;

use crate::processor::{Cpu0, RegName};

#[bitfield(u32)]
struct XXX {
    #[bits(5)]
    pub rd: u8,
    #[bits(16)]
    pub imm16: u64,
    #[bits(2)]
    pub hw: u32,
    #[bits(8)]
    pub opc: u32,
    #[bits(1)]
    pub sf: u8,
}

impl super::OpExec for MOVEWIDE {
    #[inline(always)]
    fn exec_cpu(self, cpu: &mut Cpu0) {
        match self {
            MOVEWIDE::MOVK_Rd_HALF(x) => {
                // move keep
                let x: XXX = unsafe { std::mem::transmute(x) };
                let shift = x.hw() << 4;
                let reg = RegName::w_or_x(x.rd(), x.sf());
                let v: u64 = cpu.read(reg);
                let imm = x.imm16().wrapping_shl(shift);
                let mask = 0xffffu64.wrapping_shl(shift);
                let v = imm | (v & !mask);
                cpu.write(reg, v);
            }
            MOVEWIDE::MOVN_Rd_HALF(x) => {
                // move not
                let x: XXX = unsafe { std::mem::transmute(x) };
                let imm = x.imm16().wrapping_shl(x.hw() << 4);
                cpu.write(RegName::w_or_x(x.rd(), x.sf()), !imm);
            }
            MOVEWIDE::MOVZ_Rd_HALF(x) => {
                // move zeroed
                let x: XXX = unsafe { std::mem::transmute(x) };
                let imm = x.imm16().wrapping_shl(x.hw() << 4);
                cpu.write(RegName::w_or_x(x.rd(), x.sf()), imm);
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::processor::{Cpu0, Process, insn, reg};

    #[test]
    fn test_movk() {
        let mut cpu = Cpu0::default();
        cpu.write(reg!(x[8]), 0x7777_7777_7777_7777u64);
        let mut proc = Process::new_for_test();
        // movk x8, 0x1234
        insn::op::execute(&mut cpu, &mut proc, insn::decode!(88 46 82 f2));
        assert_eq!(0x7777_7777_7777_1234u64, cpu.read::<u64>(reg!(x[8])));
        // movk x8, 0x9999, lsl # 16
        insn::op::execute(&mut cpu, &mut proc, insn::decode!(28 33 b3 f2));
        assert_eq!(0x7777_7777_9999_1234u64, cpu.read::<u64>(reg!(x[8])));
        // movk x8, 0x8888, lsl # 32
        insn::op::execute(&mut cpu, &mut proc, insn::decode!(08 11 d1 f2));
        assert_eq!(0x7777_8888_9999_1234u64, cpu.read::<u64>(reg!(x[8])));
        // movk x8, 0x6666, lsl # 48
        insn::op::execute(&mut cpu, &mut proc, insn::decode!(c8 cc ec f2));
        assert_eq!(0x6666_8888_9999_1234u64, cpu.read::<u64>(reg!(x[8])));
        // movk w8, 0xabcd
        insn::op::execute(&mut cpu, &mut proc, insn::decode!(a8 79 95 72));
        assert_eq!(0x9999_abcd, cpu.read::<u64>(reg!(x[8])));
        // movk w8, 0xabcd, lsl # 16
        insn::op::execute(&mut cpu, &mut proc, insn::decode!(a8 79 b5 72));
        assert_eq!(0xabcd_abcd, cpu.read::<u64>(reg!(x[8])));
    }

    #[test]
    fn test_movz() {
        let mut cpu = Cpu0::default();
        cpu.write(reg!(x[8]), 0x7777_7777_7777_7777u64);
        let mut proc = Process::new_for_test();
        // movz x8, 0x1234
        insn::op::execute(&mut cpu, &mut proc, insn::decode!(88 46 82 d2));
        assert_eq!(0x1234u64, cpu.read::<u64>(reg!(x[8])));
        // movz x8, 0x9999, lsl # 16
        insn::op::execute(&mut cpu, &mut proc, insn::decode!(28 33 b3 d2));
        assert_eq!(0x9999_0000u64, cpu.read::<u64>(reg!(x[8])));
        // movz x8, 0x8888, lsl # 32
        insn::op::execute(&mut cpu, &mut proc, insn::decode!(08 11 d1 d2));
        assert_eq!(0x8888_0000_0000u64, cpu.read::<u64>(reg!(x[8])));
        // movz x8, 0x6666, lsl # 48
        insn::op::execute(&mut cpu, &mut proc, insn::decode!(c8 cc ec d2));
        assert_eq!(0x6666_0000_0000_0000u64, cpu.read::<u64>(reg!(x[8])));
        // movz w8, 0xabcd
        insn::op::execute(&mut cpu, &mut proc, insn::decode!(a8 79 95 52));
        assert_eq!(0xabcd, cpu.read::<u64>(reg!(x[8])));
        // movz w8, 0xabcd, lsl # 16
        insn::op::execute(&mut cpu, &mut proc, insn::decode!(a8 79 b5 52));
        assert_eq!(0xabcd_0000, cpu.read::<u64>(reg!(x[8])));
    }

    #[test]
    fn test_movn() {
        let mut cpu = Cpu0::default();
        cpu.write(reg!(x[8]), 0x7777_7777_7777_7777u64);
        let mut proc = Process::new_for_test();
        // movn x8, 0x1234
        insn::op::execute(&mut cpu, &mut proc, insn::decode!(88 46 82 92));
        assert_eq!(!0x1234u64, cpu.read::<u64>(reg!(x[8])));
        // movn x8, 0x9999, lsl # 16
        insn::op::execute(&mut cpu, &mut proc, insn::decode!(28 33 b3 92));
        assert_eq!(!0x9999_0000u64, cpu.read::<u64>(reg!(x[8])));
        // movn x8, 0x8888, lsl # 32
        insn::op::execute(&mut cpu, &mut proc, insn::decode!(08 11 d1 92));
        assert_eq!(!0x8888_0000_0000u64, cpu.read::<u64>(reg!(x[8])));
        // movn x8, 0x6666, lsl # 48
        insn::op::execute(&mut cpu, &mut proc, insn::decode!(c8 cc ec 92));
        assert_eq!(!0x6666_0000_0000_0000u64, cpu.read::<u64>(reg!(x[8])));
        // movn w8, 0xabcd
        insn::op::execute(&mut cpu, &mut proc, insn::decode!(a8 79 95 12));
        assert_eq!(!0xabcdu32 as u64, cpu.read::<u64>(reg!(x[8])));
        // movn w8, 0xabcd, lsl # 16
        insn::op::execute(&mut cpu, &mut proc, insn::decode!(a8 79 b5 12));
        assert_eq!(!0xabcd_0000u32 as u64, cpu.read::<u64>(reg!(x[8])));
    }
}
