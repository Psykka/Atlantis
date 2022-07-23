use crate::{bus::Bus, types::Format};

use super::registers::{Registers, WordReg};

// enum Mode {
//     D8,
//     D16,
//     A8,
//     A16,
//     R8
// }

pub struct SharpCpu {
    reg: Registers,
    bus: Bus,
    pc: u16,
    sp: u16,
}

impl SharpCpu {
    pub fn new() -> Self {
        Self {
            reg: Registers::new(),
            bus: Bus::new(0xFFFF),
            pc: 0,
            sp: 0,
        }
    }

    fn read_byte(&mut self, addr: u16) -> u8 {
        self.bus.read((addr as u16).into(), Format::Byte).unwrap() as u8
    }

    fn step(&mut self) {
        let opcode = self.read_byte(self.pc);
        self.execute(opcode);
        self.pc += 1;
    }

    pub fn run(&mut self) {
        while self.pc < self.bus.ram.size() as u16 {
            self.step();
        }
    }

    fn execute(&mut self, opcode: u8) {
        match opcode {
            // 8-bit load instructions
            0x41 => self.ld_r(1),
            0x06 => self.ld_n(2),
            0x46 => self.ld(WordReg::HL, 2),

            _ => panic!("Unimplemented opcode: {:02x}", opcode),
        }
    }

    fn ld_r(&mut self, cycles: usize) {
        self.bus.tick(cycles);
        self.reg.b = self.reg.c;
    }

    fn ld_n(&mut self, cycles: usize) {
        self.bus.tick(cycles);
        self.reg.b = self.read_byte(self.pc + 1)
    }

    fn ld(&mut self, reg: WordReg, cycles: usize) {
        self.bus.tick(cycles);
        self.reg.b = self.read_byte(self.reg.get(reg));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ld_r() {
        let mut cpu = SharpCpu::new();

        cpu.bus.write(0x00, Format::Byte, 0x41);
        cpu.reg.c = 0xFF;
        cpu.step();

        assert_eq!(cpu.reg.b, cpu.reg.c);
        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn test_ld_n() {
        let mut cpu = SharpCpu::new();

        cpu.bus.write(0x00, Format::Byte, 0x06);
        cpu.bus.write(0x01, Format::Byte, 0xFF);
        cpu.step();

        assert_eq!(cpu.reg.b, 0xFF);
        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn test_ld_hl() {
        let mut cpu = SharpCpu::new();

        cpu.bus.write(0x00, Format::Byte, 0x46);
        cpu.bus.write(0xFF00, Format::Byte, 0xFF);
        cpu.reg.set(WordReg::HL, 0xFF00);
        cpu.step();

        assert_eq!(cpu.reg.b, 0xFF);
        assert_eq!(cpu.pc, 1);
    }
}
