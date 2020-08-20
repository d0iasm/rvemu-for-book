//! The memory module contains a memory structure and implementation for memory access.

use crate::bus::*;
use crate::trap::*;

/// Default memory size (128MiB).
pub const MEMORY_SIZE: u64 = 1024 * 1024 * 128;

/// The dynamic random access memory (DRAM).
#[derive(Debug)]
pub struct Memory {
    pub memory: Vec<u8>,
}

impl Device for Memory {
    fn load(&self, addr: u64, size: u64) -> Result<u64, Exception> {
        match size {
            8 => Ok(self.load8(addr)),
            16 => Ok(self.load16(addr)),
            32 => Ok(self.load32(addr)),
            64 => Ok(self.load64(addr)),
            _ => Err(Exception::LoadAccessFault),
        }
    }

    fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        match size {
            8 => Ok(self.store8(addr, value)),
            16 => Ok(self.store16(addr, value)),
            32 => Ok(self.store32(addr, value)),
            64 => Ok(self.store64(addr, value)),
            _ => Err(Exception::StoreAMOAccessFault),
        }
    }
}

impl Memory {
    /// Create a new `Memory` object with default memory size.
    pub fn new(binary: Vec<u8>) -> Memory {
        let mut memory = vec![0; MEMORY_SIZE as usize];
        memory.splice(..binary.len(), binary.iter().cloned());

        Self { memory }
    }

    /// Load a byte from the little-endian memory.
    fn load8(&self, addr: u64) -> u64 {
        let index = (addr - MEMORY_BASE) as usize;
        self.memory[index] as u64
    }

    /// Load 2 bytes from the little-endian memory.
    fn load16(&self, addr: u64) -> u64 {
        let index = (addr - MEMORY_BASE) as usize;
        return (self.memory[index] as u64) | ((self.memory[index + 1] as u64) << 8);
    }

    /// Load 4 bytes from the little-endian memory.
    fn load32(&self, addr: u64) -> u64 {
        let index = (addr - MEMORY_BASE) as usize;
        return (self.memory[index] as u64)
            | ((self.memory[index + 1] as u64) << 8)
            | ((self.memory[index + 2] as u64) << 16)
            | ((self.memory[index + 3] as u64) << 24);
    }

    /// Load 8 bytes from the little-endian memory.
    fn load64(&self, addr: u64) -> u64 {
        let index = (addr - MEMORY_BASE) as usize;
        return (self.memory[index] as u64)
            | ((self.memory[index + 1] as u64) << 8)
            | ((self.memory[index + 2] as u64) << 16)
            | ((self.memory[index + 3] as u64) << 24)
            | ((self.memory[index + 4] as u64) << 32)
            | ((self.memory[index + 5] as u64) << 40)
            | ((self.memory[index + 6] as u64) << 48)
            | ((self.memory[index + 7] as u64) << 56);
    }

    /// Store a byte to the little-endian memory.
    fn store8(&mut self, addr: u64, value: u64) {
        let index = (addr - MEMORY_BASE) as usize;
        self.memory[index] = value as u8
    }

    /// Store 2 bytes to the little-endian memory.
    fn store16(&mut self, addr: u64, value: u64) {
        let index = (addr - MEMORY_BASE) as usize;
        self.memory[index] = (value & 0xff) as u8;
        self.memory[index + 1] = ((value >> 8) & 0xff) as u8;
    }

    /// Store 4 bytes to the little-endian memory.
    fn store32(&mut self, addr: u64, value: u64) {
        let index = (addr - MEMORY_BASE) as usize;
        self.memory[index] = (value & 0xff) as u8;
        self.memory[index + 1] = ((value >> 8) & 0xff) as u8;
        self.memory[index + 2] = ((value >> 16) & 0xff) as u8;
        self.memory[index + 3] = ((value >> 24) & 0xff) as u8;
    }

    /// Store 8 bytes to the little-endian memory.
    fn store64(&mut self, addr: u64, value: u64) {
        let index = (addr - MEMORY_BASE) as usize;
        self.memory[index] = (value & 0xff) as u8;
        self.memory[index + 1] = ((value >> 8) & 0xff) as u8;
        self.memory[index + 2] = ((value >> 16) & 0xff) as u8;
        self.memory[index + 3] = ((value >> 24) & 0xff) as u8;
        self.memory[index + 4] = ((value >> 32) & 0xff) as u8;
        self.memory[index + 5] = ((value >> 40) & 0xff) as u8;
        self.memory[index + 6] = ((value >> 48) & 0xff) as u8;
        self.memory[index + 7] = ((value >> 56) & 0xff) as u8;
    }
}
