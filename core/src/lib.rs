pub mod arm;
pub mod bus;
pub mod memory;
pub mod sharp;
pub mod types;

use sharp::cpu::SharpCpu;

pub struct Core {
    pub cpu: SharpCpu,
}

impl Default for Core {
    fn default() -> Self {
        Self {
            cpu: SharpCpu::new(),
        }
    }
}

impl Core {
    pub fn new() -> Self {
        Self {
            cpu: SharpCpu::new(),
        }
    }
}
