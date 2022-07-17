use crate::{bus::Bus, types::Format};

pub struct CPU {
    // iwram: [u8; 1024 * 1024 * 32],
    pub bus: Bus,
    r: [u32; 16],
    // R[13] (SP) is the stack pointer.
    // R[14] (LR) is link register.
    // R[15] (PC) is the program counter.
    cprs: u32
}

impl CPU {
    pub fn new(bus: Bus) -> Self {
        let mut r = [0; 16];
        r[15] = 0x2000000; // PC is set before BIOS.

        Self {
            // iwram: [0; 1024 * 1024 * 32],
            bus,
            r,
            cprs: 0,
        }
    }

    fn _fetch(self) -> u32 {
        let opcode = self.bus.read(self.r[15], Format::Word);
        opcode as u32
    }
}
