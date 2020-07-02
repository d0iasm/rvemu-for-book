//! The bus module contains the system bus which can access the memroy or memory-mapped peripheral
//! devices.

use crate::memory::*;
use crate::trap::*;
use crate::uart::*;

/// The address which UART starts, same as QEMU virt machine.
pub const UART_BASE: u64 = 0x1000_0000;
/// The size of UART.
pub const UART_SIZE: u64 = 0x100;

/// The address which memory starts, same as QEMU virt machine.
pub const MEMORY_BASE: u64 = 0x8000_0000;

pub trait Device {
    fn load(&self, addr: u64, size: u64) -> Result<u64, Exception>;
    fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception>;
}

/// The system bus.
pub struct Bus {
    pub uart: Uart,
    pub memory: Memory,
}

impl Bus {
    /// Create a new system bus object.
    pub fn new(binary: Vec<u8>) -> Bus {
        Self {
            uart: Uart::new(),
            memory: Memory::new(binary),
        }
    }

    pub fn load(&self, addr: u64, size: u64) -> Result<u64, Exception> {
        if UART_BASE <= addr && addr < UART_BASE + UART_SIZE {
            return self.uart.load(addr, size);
        }
        if MEMORY_BASE <= addr {
            return self.memory.load(addr, size);
        }
        Err(Exception::LoadAccessFault)
    }

    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        if UART_BASE <= addr && addr < UART_BASE + UART_SIZE {
            return self.uart.store(addr, size, value);
        }
        if MEMORY_BASE <= addr {
            return self.memory.store(addr, size, value);
        }
        Err(Exception::StoreAMOAccessFault)
    }
}
