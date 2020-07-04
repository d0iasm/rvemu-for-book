//! The plic module contains the platform-level interrupt controller (PLIC).
//! The plic connects all external interrupts in the system to all hart
//! contexts in the system, via the external interrupt source in each hart.
//! It's the global interrupt controller in a RISC-V system.

use crate::bus::*;
use crate::trap::*;

/// The address of interrupt pending bits.
pub const PLIC_PENDING: u64 = PLIC_BASE + 0x1000;
/// The address of the regsiters to enable interrupts for S-mode.
pub const PLIC_SENABLE: u64 = PLIC_BASE + 0x2080;
/// The address of the registers to set a priority for S-mode.
pub const PLIC_SPRIORITY: u64 = PLIC_BASE + 0x201000;
/// The address of the claim/complete registers for S-mode.
pub const PLIC_SCLAIM: u64 = PLIC_BASE + 0x201004;

/// The platform-level-interrupt controller (PLIC).
pub struct Plic {
    pending: u64,
    senable: u64,
    spriority: u64,
    sclaim: u64,
}

impl Device for Plic {
    fn load(&mut self, addr: u64, size: u64) -> Result<u64, Exception> {
        match size {
            32 => Ok(self.load32(addr)),
            _ => Err(Exception::LoadAccessFault),
        }
    }

    fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        match size {
            32 => Ok(self.store32(addr, value)),
            _ => Err(Exception::StoreAMOAccessFault),
        }
    }
}

impl Plic {
    /// Create a new `Plic` object.
    pub fn new() -> Self {
        Self {
            pending: 0,
            senable: 0,
            spriority: 0,
            sclaim: 0,
        }
    }

    fn load32(&self, addr: u64) -> u64 {
        match addr {
            PLIC_PENDING => self.pending,
            PLIC_SENABLE => self.senable,
            PLIC_SPRIORITY => self.spriority,
            PLIC_SCLAIM => self.sclaim,
            _ => 0,
        }
    }

    fn store32(&mut self, addr: u64, value: u64) {
        match addr {
            PLIC_PENDING => self.pending = value,
            PLIC_SENABLE => self.senable = value,
            PLIC_SPRIORITY => self.spriority = value,
            PLIC_SCLAIM => self.sclaim = value,
            _ => {}
        }
    }
}
