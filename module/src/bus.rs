//! The bus module contains the system bus which can access the memroy or memory-mapped peripheral
//! devices.

use crate::clint::*;
use crate::dram::*;
use crate::plic::*;
use crate::trap::*;
use crate::uart::*;
use crate::virtio::*;

/// The address which the core-local interruptor (CLINT) starts. It contains the timer and
/// generates per-hart software interrupts and timer
/// interrupts.
pub const CLINT_BASE: u64 = 0x200_0000;
/// The size of CLINT.
pub const CLINT_SIZE: u64 = 0x10000;

/// The address which the platform-level interrupt controller (PLIC) starts. The PLIC connects all external interrupts in the
/// system to all hart contexts in the system, via the external interrupt source in each hart.
pub const PLIC_BASE: u64 = 0xc00_0000;
/// The size of PLIC.
pub const PLIC_SIZE: u64 = 0x4000000;

/// The address which UART starts, same as QEMU virt machine.
pub const UART_BASE: u64 = 0x1000_0000;
/// The size of UART.
pub const UART_SIZE: u64 = 0x100;

/// The address which virtio starts.
pub const VIRTIO_BASE: u64 = 0x1000_1000;
/// The size of virtio.
pub const VIRTIO_SIZE: u64 = 0x1000;

/// The address which dram starts, same as QEMU virt machine.
pub const DRAM_BASE: u64 = 0x8000_0000;

pub trait Device {
    fn load(&mut self, addr: u64, size: u64) -> Result<u64, Exception>;
    fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception>;
}

/// The system bus.
pub struct Bus {
    clint: Clint,
    plic: Plic,
    pub uart: Uart,
    pub virtio: Virtio,
    dram: Dram,
}

impl Bus {
    /// Create a new system bus object.
    pub fn new(binary: Vec<u8>, disk_image: Vec<u8>) -> Bus {
        Self {
            clint: Clint::new(),
            plic: Plic::new(),
            uart: Uart::new(),
            virtio: Virtio::new(disk_image),
            dram: Dram::new(binary),
        }
    }

    pub fn load(&mut self, addr: u64, size: u64) -> Result<u64, Exception> {
        if CLINT_BASE <= addr && addr < CLINT_BASE + CLINT_SIZE {
            return self.clint.load(addr, size);
        }
        if PLIC_BASE <= addr && addr < PLIC_BASE + PLIC_SIZE {
            return self.plic.load(addr, size);
        }
        if UART_BASE <= addr && addr < UART_BASE + UART_SIZE {
            return self.uart.load(addr, size);
        }
        if VIRTIO_BASE <= addr && addr < VIRTIO_BASE + VIRTIO_SIZE {
            return self.virtio.load(addr, size);
        }
        if DRAM_BASE <= addr {
            return self.dram.load(addr, size);
        }
        Err(Exception::LoadAccessFault)
    }

    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        if CLINT_BASE <= addr && addr < CLINT_BASE + CLINT_SIZE {
            return self.clint.store(addr, size, value);
        }
        if PLIC_BASE <= addr && addr < PLIC_BASE + PLIC_SIZE {
            return self.plic.store(addr, size, value);
        }
        if UART_BASE <= addr && addr < UART_BASE + UART_SIZE {
            return self.uart.store(addr, size, value);
        }
        if VIRTIO_BASE <= addr && addr < VIRTIO_BASE + VIRTIO_SIZE {
            return self.virtio.store(addr, size, value);
        }
        if DRAM_BASE <= addr {
            return self.dram.store(addr, size, value);
        }
        Err(Exception::StoreAMOAccessFault)
    }
}
