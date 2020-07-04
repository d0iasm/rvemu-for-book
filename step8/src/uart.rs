//! The uart module contains the implementation of a universal asynchronous receiver-transmitter
//! (UART). The device is 16550a UART, which is used in the QEMU virt machine.
//! See the spec: http://byterunner.com/16550.html

#![allow(dead_code)]

use std::io;
use std::io::prelude::*;

use crate::bus::*;
use crate::trap::*;

/// Receive holding register (for input bytes).
pub const UART_RHR: u64 = UART_BASE + 0;
/// Transmit holding register (for output bytes).
pub const UART_THR: u64 = UART_BASE + 0;
/// Line control register.
pub const UART_LCR: u64 = UART_BASE + 3;
/// Line status register.
/// LSR BIT 0:
///     0 = no data in receive holding register or FIFO.
///     1 = data has been receive and saved in the receive holding register or FIFO.
/// LSR BIT 5:
///     0 = transmit holding register is full. 16550 will not accept any data for transmission.
///     1 = transmitter hold register (or FIFO) is empty. CPU can load the next character.
pub const UART_LSR: u64 = UART_BASE + 5;

/// The receiver (RX) bit.
pub const UART_LSR_RX: u8 = 1;
/// The transmitter (TX) bit.
pub const UART_LSR_TX: u8 = 1 << 5;

pub struct Uart {
    /// 8-bit buffer for input and output.
    buffer: u8,
    /// Line status register.
    lsr: u8,
}

impl Device for Uart {
    fn load(&mut self, addr: u64, size: u64) -> Result<u64, Exception> {
        match size {
            8 => Ok(self.load8(addr)),
            _ => Err(Exception::LoadAccessFault),
        }
    }

    fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        match size {
            8 => Ok(self.store8(addr, value)),
            _ => Err(Exception::StoreAMOAccessFault),
        }
    }
}

impl Uart {
    /// Create a new `Uart` object.
    pub fn new() -> Self {
        Self { buffer: 0, lsr: 0 }
    }

    fn load8(&mut self, addr: u64) -> u64 {
        match addr {
            UART_RHR => {
                self.lsr &= !UART_LSR_RX;
                self.buffer as u64
            }
            _ => 0,
        }
    }

    fn store8(&mut self, addr: u64, value: u64) {
        match addr {
            UART_THR => {
                print!("{}", value as u8 as char);
                io::stdout().flush().expect("failed to flush stdout");
            }
            _ => {}
        }
    }
}
