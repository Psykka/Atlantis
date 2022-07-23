use crate::types::Format;

pub struct Memory {
    ram: Box<[u8]>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self {
            ram: vec![0; size].into_boxed_slice(),
        }
    }

    pub fn size(&self) -> usize {
        self.ram.len()
    }

    pub fn read(&self, addr: u32, size: Format) -> Result<u32, ()> {
        match size {
            Format::Byte => Ok(self.read_byte(addr).unwrap() as u32),
            Format::Halfword => Ok(self.read_halfword(addr).unwrap() as u32),
            Format::Word => Ok(self.read_word(addr).unwrap() as u32),
        }
    }

    fn read_byte(&self, addr: u32) -> Result<u8, ()> {
        Ok(self.ram[addr as usize])
    }

    fn read_halfword(&self, addr: u32) -> Result<u16, ()> {
        Ok((self.ram[addr as usize] as u16)
        | ((self.ram[(addr + 1) as usize] as u16) << 8))
    }

    fn read_word(&self, addr: u32) -> Result<u32, ()> {
        Ok((self.ram[addr as usize] as u32)
        | ((self.ram[(addr + 1) as usize] as u32) << 8)
        | ((self.ram[(addr + 2) as usize] as u32) << 16)
        | ((self.ram[(addr + 3) as usize] as u32) << 24))
    }

    pub fn write(&mut self, addr: u32, size: Format, data: u32) {
        match size {
            Format::Byte => self.write_byte(addr, data as u8),
            Format::Halfword => self.write_halfword(addr, data as u16),
            Format::Word => self.write_word(addr, data as u32),
        }
    }

    fn write_byte(&mut self, addr: u32, data: u8) {
        self.ram[addr as usize] = data;
    }

    fn write_halfword(&mut self, addr: u32, data: u16) {
        self.ram[addr as usize] = (data & 0xFF) as u8;
        self.ram[(addr + 1) as usize] = ((data >> 8) & 0xFF) as u8;
    }

    fn write_word(&mut self, addr: u32, data: u32) {
        self.ram[addr as usize] = (data & 0xFF) as u8;
        self.ram[(addr + 1) as usize] = ((data >> 8) & 0xFF) as u8;
        self.ram[(addr + 2) as usize] = ((data >> 16) & 0xFF) as u8;
        self.ram[(addr + 3) as usize] = ((data >> 24) & 0xFF) as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte() {
        let mut memory = Memory::new(1024);
        memory.write_byte(0, 0xFF);
        assert_eq!(memory.read_byte(0).unwrap(), 0xFF);
    }

    #[test]
    fn test_halfword() {
        let mut memory = Memory::new(1024);
        memory.write_halfword(0, 0xFFFF);
        assert_eq!(memory.read_halfword(0).unwrap(), 0xFFFF);
    }

    #[test]
    fn test_word() {
        let mut memory = Memory::new(1024);
        memory.write_word(0, 0xFFFFFFFF);
        assert_eq!(memory.read_word(0).unwrap(), 0xFFFFFFFF);
    }
}