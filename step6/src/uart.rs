//! The uart module contains the implementation of a universal asynchronous receiver-transmitter
//! (UART). The device is 16550a UART, which is used in the QEMU virt machine.
//! See the spec: http://byterunner.com/16550.html

#![allow(dead_code)]

use crate::bus::*;
use crate::trap::*;

/// Receive holding register (for input bytes).
pub const UART_RHR: u64 = UART_BASE + 0;
/// Transmit holding register (for output bytes).
pub const UART_THR: u64 = UART_BASE + 0;
/// Interrupt enable register.
pub const UART_IER: u64 = UART_BASE + 1;
/// FIFO control register.
pub const UART_FCR: u64 = UART_BASE + 2;
/// Interrupt status register.
/// ISR BIT-0:
///     0 = an interrupt is pending and the ISR contents may be used as a pointer to the appropriate
/// interrupt service routine.
///     1 = no interrupt is pending.
pub const UART_ISR: u64 = UART_BASE + 2;
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

pub struct Uart {
    buffer: u8,
}

impl Device for Uart {
    fn load(&self, addr: u64, size: u64) -> Result<u64, Exception> {
        match size {
            8 => Ok(self.buffer as u64),
            _ => Err(Exception::LoadAccessFault),
        }
    }

    fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        match size {
            8 => {
                self.buffer = value as u8;
                Ok(())
            }
            _ => Err(Exception::StoreAMOAccessFault),
        }
    }
}

impl Uart {
    /// Create a new `Uart` object.
    pub fn new() -> Self {
        Self { buffer: 0 }
    }
}
