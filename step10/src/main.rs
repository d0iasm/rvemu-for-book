mod bus;
mod clint;
mod cpu;
mod dram;
mod plic;
mod trap;
mod uart;
mod virtio;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use crate::cpu::*;
use crate::trap::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if (args.len() != 2) && (args.len() != 3) {
        panic!("Usage: rvemu-for-book <filename> <(option) image>");
    }
    let mut file = File::open(&args[1])?;
    let mut binary = Vec::new();
    file.read_to_end(&mut binary)?;

    let mut disk_image = Vec::new();
    if args.len() == 3 {
        let mut file = File::open(&args[2])?;
        file.read_to_end(&mut disk_image)?;
    }

    let mut cpu = Cpu::new(binary, disk_image);

    loop {
        // 1. Fetch.
        let inst = match cpu.fetch() {
            // Break the loop if an error occurs.
            Ok(inst) => inst,
            Err(exception) => {
                exception.take_trap(&mut cpu);
                if exception.is_fatal() {
                    break;
                }
                0 // dummy instruction
            }
        };

        // 2. Add 4 to the program counter.
        cpu.pc += 4;

        // 3. Decode.
        // 4. Execute.
        match cpu.execute(inst) {
            // Break the loop if an error occurs.
            Ok(_) => {}
            Err(exception) => {
                exception.take_trap(&mut cpu);
                if exception.is_fatal() {
                    break;
                }
            }
        }

        match cpu.check_pending_interrupt() {
            Some(interrupt) => interrupt.take_trap(&mut cpu),
            None => {}
        }
    }
    cpu.dump_registers();
    println!("-----------------------------------------------------------------------------------------------------------");
    cpu.dump_csrs();

    Ok(())
}
