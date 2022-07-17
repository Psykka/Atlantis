mod arm;
pub mod bus;

use crate::arm::arm7tdmi::CPU;
use crate::bus::Bus;

pub struct Core {
    pub arm: CPU,
}

impl Core {
    pub fn new() -> Self {
        Self {
            arm: CPU::new(Bus::new())
        }
    }
}

impl Default for Core {
    fn default() -> Self {
        Self::new()
    }
}
