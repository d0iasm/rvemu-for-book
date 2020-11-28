//! The bus module contains the system bus which can access the memroy or memory-mapped peripheral
//! devices.

use crate::dram::*;

/// The address which dram starts, same as QEMU virt machine.
pub const DRAM_BASE: u64 = 0x8000_0000;

pub trait Device {
    fn load(&self, addr: u64, size: u64) -> Result<u64, ()>;
    fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), ()>;
}

/// The system bus.
pub struct Bus {
    dram: Dram,
}

impl Bus {
    /// Create a new system bus object.
    pub fn new(binary: Vec<u8>) -> Bus {
        Self {
            dram: Dram::new(binary),
        }
    }

    pub fn load(&self, addr: u64, size: u64) -> Result<u64, ()> {
        if DRAM_BASE <= addr {
            return self.dram.load(addr, size);
        }
        Err(())
    }
    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), ()> {
        if DRAM_BASE <= addr {
            return self.dram.store(addr, size, value);
        }
        Err(())
    }
}
