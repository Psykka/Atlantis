use crate::{
    memory::Memory,
    types::Format,
};

pub struct Bus {
    pub ram: Memory,
}

impl Bus {
    pub fn new(ram_size: usize) -> Self {
        Self {
            // gba 1024 * 1024 * 256 bytes
            ram: Memory::new(ram_size),
        }
    }

    pub fn read(&self, addr: u32, size: Format) -> Result<u32, ()> {
        self.ram.read(addr, size)
    }

    pub fn write(&mut self, addr: u32, size: Format, data: u32) {
        self.ram.write(addr, size, data)
    }

    pub fn tick(&mut self, cycles: usize) {
        for _ in 0..cycles {
            0;
        }
    }
}
