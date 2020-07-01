mod cpu;
mod trap;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use crate::cpu::*;
use crate::trap::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Usage: rvemu-simple <filename>");
    }
    let mut file = File::open(&args[1])?;
    let mut binary = Vec::new();
    file.read_to_end(&mut binary)?;

    let mut cpu = Cpu::new(binary);

    loop {
        // 1. Fetch.
        let inst = cpu.fetch();

        // 2. Add 4 to the program counter.
        cpu.pc += 4;

        // 3. Decode.
        // 4. Execute.
        match cpu.execute(inst) {
            // True if an error happens.
            Ok(_) => {}
            Err(exception) => {
                exception.take_trap(&mut cpu);
            }
        };

        // This is a workaround for avoiding an infinite loop.
        if cpu.pc == 0 {
            break;
        }
    }
    cpu.dump_registers();
    println!("-----------------------------------------------------------------------------------------------------------");
    cpu.dump_csrs();

    Ok(())
}
