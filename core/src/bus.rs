use crate::types::Format;
pub const ADDRES_BASE: u32 = 0x2000000;

#[derive(Clone)]
pub struct Bus {
    pub ewram: Box<[u8]>,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            ewram: vec![0; 1024 * 1024 * 256].into_boxed_slice(),
        }
    }

    pub fn read(self, addr: u32, size: Format) -> u32 {
        match size {
            Format::Byte => self.read_byte(addr) as u32,
            Format::Halfword => self.read_halfword(addr) as u32,
            Format::Word => self.read_word(addr) as u32,
        }
    }

    pub fn write(mut self, addr: u32, size: Format, data: u64) {
        match size {
            Format::Byte => self.write_byte(addr, data as u8),
            Format::Halfword => self.write_halfword(addr, data as u16),
            Format::Word => self.write_word(addr, data as u32),
        }
    }

    fn read_byte(self, addr: u32) -> u8 {
        let addr = addr - ADDRES_BASE;
        self.ewram[addr as usize]
    }

    fn read_halfword(self, addr: u32) -> u16 {
        let addr = (addr - ADDRES_BASE) as usize;
        let a = self.ewram[addr];
        let b = self.ewram[addr + 1];
        (a as u16) | ((b as u16) << 8)
    }

    fn read_word(self, addr: u32) -> u32 {
        let addr = (addr - ADDRES_BASE) as usize;
        let a = self.ewram[addr];
        let b = self.ewram[addr + 1];
        let c = self.ewram[addr + 2];
        let d = self.ewram[addr + 3];
        (a as u32) | ((b as u32) << 8) | ((c as u32) << 16) | ((d as u32) << 24)
    }
    
    fn write_byte(&mut self, addr: u32, value: u8) {
        let addr = addr - ADDRES_BASE;
        self.ewram[addr as usize] = value;
    }

    fn write_halfword(&mut self, addr: u32, value: u16) {
        let addr = (addr - ADDRES_BASE) as usize;
        self.ewram[addr] = (value & 0xFF) as u8;
        self.ewram[addr + 1] = ((value >> 8) & 0xFF) as u8;
    }

    fn write_word(&mut self, addr: u32, value: u32) {
        let addr = (addr - ADDRES_BASE) as usize;
        self.ewram[addr] = (value & 0xFF) as u8;
        self.ewram[addr + 1] = ((value >> 8) & 0xFF) as u8;
        self.ewram[addr + 2] = ((value >> 16) & 0xFF) as u8;
        self.ewram[addr + 3] = ((value >> 24) & 0xFF) as u8;
    }
}
