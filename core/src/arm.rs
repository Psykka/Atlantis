use crate::bus::Bus;

pub mod arm7tdmi;

pub struct CPU {
    // iwram: [u8; 1024 * 1024 * 32],
    bus: Bus,
    cprs: u32,
    flags: u32,
    r: [u32; 16],
    // R[13] (SP) is the stack pointer.
    // R[14] (LR) is link register.
    // R[15] (PC) is the program counter.
}