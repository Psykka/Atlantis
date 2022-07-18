use crate::{bus::Bus, types::Format};

use super::CPU;

impl CPU {
    pub fn new(bus: Bus) -> Self {
        let mut r = [0; 16];
        r[15] = 0x2000000; // PC is set before BIOS.

        Self {
            bus,
            r,
            cprs: 0,
            flags: 0,
        }
    }

    pub fn fetch(&self) -> u32{
        todo!()
    }

    pub fn decode(&self, inst: u32) -> u32 {
        // data processing and FSR transfer instructions
        todo!()
    }

    pub fn execute(&self, inst: u32) {
        todo!()
    }

    pub fn run(&self) {
        loop {
            // Fetch instruction.
            let inst = self.fetch();
            // Decode instruction.
            self.decode(inst);
            // Execute instruction.
            self.execute(inst);
        }
    }
}
