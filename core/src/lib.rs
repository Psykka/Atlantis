mod arm;
pub mod bus;

pub mod types;

use crate::arm::CPU;
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

    pub fn start(self) {
        self.arm.run();
    }
}

impl Default for Core {
    fn default() -> Self {
        Self::new()
    }
}
