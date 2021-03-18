use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use rvemu::cpu::*;
use rvemu::trap::*;

fn read_file(filename: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut binary = Vec::new();
    file.read_to_end(&mut binary)?;
    return Ok(binary);
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if (args.len() != 2) && (args.len() != 3) {
        panic!("Usage: rvemu-for-book <filename> <(option) image>");
    }
    let kernel = read_file(&args[1])?;

    let mut disk_image = Vec::new();
    if args.len() == 3 {
        disk_image = read_file(&args[2])?;
    }

    let mut cpu = Cpu::new(kernel, disk_image);

    loop {
        // 1. Fetch.
        let inst = match cpu.fetch() {
            Ok(inst) => inst,
            Err(_exception) => 0, // Place 0 if fetch() fails. It will break out of the loop.
        };

        // 2. Add 4 to the program counter.
        cpu.pc += 4;

        // 3. Decode.
        // 4. Execute.
        match cpu.execute(inst) {
            Ok(_) => {}
            Err(exception) => {
                exception.take_trap(&mut cpu);
                // Break the loop if a fatal error occurs.
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
