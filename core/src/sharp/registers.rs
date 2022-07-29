pub enum WordReg {
    AF,
    BC,
    DE,
    HL,
}

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub f: u8,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            f: 0,
        }
    }

    pub fn get(&self, reg: WordReg) -> u16 {
        match reg {
            WordReg::AF => ((self.a as u16) << 8) | (self.f as u16),
            WordReg::BC => ((self.b as u16) << 8) | (self.c as u16),
            WordReg::DE => ((self.d as u16) << 8) | (self.e as u16),
            WordReg::HL => ((self.h as u16) << 8) | (self.l as u16),
        }
    }

    pub fn set(&mut self, reg: WordReg, value: u16) {
        match reg {
            WordReg::AF => {
                self.a = (value >> 8) as u8;
                self.f = value as u8;
            }
            WordReg::BC => {
                self.b = (value >> 8) as u8;
                self.c = value as u8;
            }
            WordReg::DE => {
                self.d = (value >> 8) as u8;
                self.e = value as u8;
            }
            WordReg::HL => {
                self.h = (value >> 8) as u8;
                self.l = value as u8;
            }
        }
    }
}
