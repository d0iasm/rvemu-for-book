//! The clint module contains the core-local interruptor (CLINT). The CLINT
//! block holds memory-mapped control and status registers associated with
//! software and timer interrupts. It generates per-hart software interrupts and timer.

use crate::bus::*;
use crate::trap::*;

/// The address of a mtimecmp register starts. A mtimecmp is a dram mapped machine mode timer
/// compare register, used to trigger an interrupt when mtimecmp is greater than or equal to mtime.
pub const CLINT_MTIMECMP: u64 = CLINT_BASE + 0x4000;
/// The address of a timer register. A mtime is a machine mode timer register which runs at a
/// constant frequency.
pub const CLINT_MTIME: u64 = CLINT_BASE + 0xbff8;

/// The core-local interruptor (CLINT).
pub struct Clint {
    mtime: u64,
    mtimecmp: u64,
}

impl Device for Clint {
    fn load(&mut self, addr: u64, size: u64) -> Result<u64, Exception> {
        match size {
            64 => Ok(self.load64(addr)),
            _ => Err(Exception::LoadAccessFault),
        }
    }

    fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        match size {
            64 => Ok(self.store64(addr, value)),
            _ => Err(Exception::StoreAMOAccessFault),
        }
    }
}

impl Clint {
    /// Create a new `Clint` object.
    pub fn new() -> Self {
        Self {
            mtime: 0,
            mtimecmp: 0,
        }
    }

    fn load64(&self, addr: u64) -> u64 {
        match addr {
            CLINT_MTIMECMP => self.mtimecmp,
            CLINT_MTIME => self.mtime,
            _ => 0,
        }
    }

    fn store64(&mut self, addr: u64, value: u64) {
        match addr {
            CLINT_MTIMECMP => self.mtimecmp = value,
            CLINT_MTIME => self.mtime = value,
            _ => {}
        }
    }
}
