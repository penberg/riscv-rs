use crate::Memory;

use log::trace;

const INSN_LEN: u32 = 4;

/// A RISC-V instruction.
pub struct Insn(u32);

impl Insn {
    fn get_opcode(&self) -> u8 {
        (self.0 & 0b1111111) as u8
    }
    fn get_rd(&self) -> usize {
        ((self.0 & 0b111110000000) >> 6) as usize
    }
    fn get_j_imm(&self) -> u32 {
        let imm = self.0 >> 12;
        let imm_20 = (imm & 0b10000000000000000000) >> 20;
        let imm_10_1 = (imm & 0b01111111111000000000) >> 9;
        let imm_11 = (imm & 0b00000000000100000000) >> 8;
        let imm_19_12 = (imm & 0b00000000000011111111) >> 0;
        (imm_20 << 20) | (imm_19_12 << 12) | (imm_11 << 11) | (imm_10_1 << 1)
    }
}

/// A RISC-V CPU.
pub struct CPU {
    /// The memory interface.
    mem: Memory,
    /// The general purpose registers.
    regs: [u32; 32],
    /// The program counter (PC) register.
    pc: u32,
}

impl CPU {
    pub fn new(mem: Memory, pc: u32) -> Self {
        CPU {
            mem,
            pc: pc,
            regs: [0; 32],
        }
    }
    pub fn exec(&mut self) {
        let raw_insn = self.mem.read_u32(self.pc as u64);
        let insn = Insn(raw_insn);
        let opc = insn.get_opcode();
        let mut pc = self.pc + INSN_LEN;
        match opc {
            0b1101111 => {
                let rd = insn.get_rd();
                let offset = insn.get_j_imm();
                self.regs[rd] = pc;
                pc = self.pc + offset;
                trace!("jal x{}, {:x}", rd, offset);
            }
            _ => {
                println!(
                    "invalid opcode: {:02x} (pc = {:08x}, insn = {:08x})",
                    opc, self.pc, raw_insn
                );
            }
        }
        self.pc = pc;
    }
    pub fn halted(&self) -> bool {
        false
    }
}
