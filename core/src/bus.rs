pub const ADDRES_BASE: u32 = 0x2000000;

pub struct Bus {
    pub ewram: Box<[u8]>,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            ewram: vec![0; 1024 * 1024 * 256].into_boxed_slice(),
        }
    }

    pub fn read_8(self, addr: u32) -> u8 {
        let addr = addr - ADDRES_BASE;
        self.ewram[addr as usize]
    }

    pub fn read_16(self, addr: u32) -> u16 {
        let addr = (addr - ADDRES_BASE) as usize;
        let a = self.ewram[addr];
        let b = self.ewram[addr + 1];
        (a as u16) | ((b as u16) << 8)
    }

    pub fn read_32(self, addr: u32) -> u32 {
        let addr = (addr - ADDRES_BASE) as usize;
        let a = self.ewram[addr];
        let b = self.ewram[addr + 1];
        let c = self.ewram[addr + 2];
        let d = self.ewram[addr + 3];
        (a as u32) | ((b as u32) << 8) | ((c as u32) << 16) | ((d as u32) << 24)
    }
    
    pub fn write_8(&mut self, addr: u32, value: u8) {
        let addr = addr - ADDRES_BASE;
        self.ewram[addr as usize] = value;
    }

    pub fn write_16(&mut self, addr: u32, value: u16) {
        let addr = (addr - ADDRES_BASE) as usize;
        self.ewram[addr] = (value & 0xFF) as u8;
        self.ewram[addr + 1] = ((value >> 8) & 0xFF) as u8;
    }

    pub fn write_32(&mut self, addr: u32, value: u32) {
        let addr = (addr - ADDRES_BASE) as usize;
        self.ewram[addr] = (value & 0xFF) as u8;
        self.ewram[addr + 1] = ((value >> 8) & 0xFF) as u8;
        self.ewram[addr + 2] = ((value >> 16) & 0xFF) as u8;
        self.ewram[addr + 3] = ((value >> 24) & 0xFF) as u8;
    }
}

