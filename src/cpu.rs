use crate::Memory;

const INSN_LEN: u32 = 4;

/// A RISC-V instruction.
pub struct Insn(u32);

impl Insn {
    fn get_opcode(&self) -> u8 {
        (self.0 & 0x7f) as u8
    }
}

/// A RISC-V CPU.
pub struct CPU {
    /// The memory interface.
    mem: Memory,
    /// The general purpose registers.
    _regs: [u32; 32],
    /// The program counter (PC) register.
    pc: u32,
}

impl CPU {
    pub fn new(mem: Memory, pc: u32) -> Self {
        CPU {
            mem,
            pc: pc,
            _regs: [0; 32],
        }
    }
    pub fn exec(&mut self) {
        let raw_insn = self.mem.read_u32(self.pc as u64);
        let insn = Insn(raw_insn);
        let opc = insn.get_opcode();
        match opc {
            _ => {
                println!(
                    "invalid opcode: {:02x} (pc = {:08x}, insn = {:08x})",
                    opc, self.pc, raw_insn
                );
            }
        }
        self.pc += INSN_LEN;
    }
    pub fn halted(&self) -> bool {
        false
    }
}
