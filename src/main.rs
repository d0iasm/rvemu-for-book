use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

struct Cpu {
    xregs: [u64; 32],
    pc: u64,
    memory: Vec<u8>,
}

impl Cpu {
    fn new(memory: Vec<u8>) -> Self {
        Self {
            xregs: [0; 32],
            pc: 0,
            memory: memory,
        }
    }

    fn dump_registers(&self) {
        let mut output = String::from("");
        for i in (0..32).step_by(4) {
            output = format!(
                "{}\n{}",
                output,
                format!(
                    "x{:02}={:>#18x} x{:02}={:>#18x} x{:02}={:>#18x} x{:02}={:>#18x}",
                    i,
                    self.xregs[i],
                    i + 1,
                    self.xregs[i + 1],
                    i + 2,
                    self.xregs[i + 2],
                    i + 3,
                    self.xregs[i + 3],
                )
            );
        }
        println!("{}", output);
    }

    fn fetch(&self) -> u32 {
        let index = self.pc as usize;
        return (self.memory[index] as u32)
            | ((self.memory[index + 1] as u32) << 8)
            | ((self.memory[index + 2] as u32) << 16)
            | ((self.memory[index + 3] as u32) << 24);
    }

    fn execute(&mut self, inst: u32) {
        let opcode = inst & 0x0000007f;
        let rd = ((inst & 0x00000f80) >> 7) as usize;
        let rs1 = ((inst & 0x000f8000) >> 15) as usize;
        let rs2 = ((inst & 0x01f00000) >> 20) as usize;

        match opcode {
            0x13 => {
                // addi
                let imm = ((inst & 0xfff00000) as i32 as i64 >> 20) as u64;
                self.xregs[rd] = self.xregs[rs1] + imm;
            }
            0x33 => {
                // add
                self.xregs[rd] = self.xregs[rs1] + self.xregs[rs2];
            }
            _ => {
                dbg!("not implemented yet");
            }
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Usage: rvemu-simple <filename>");
    }
    let mut file = File::open(&args[1])?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut cpu = Cpu::new(buffer);

    while cpu.pc < cpu.memory.len() as u64 {
        // 1. Fetch.
        let inst = cpu.fetch();

        // 2. Add 4 to the program counter.
        cpu.pc += 4;

        // 3. Decode.
        // 4. Execute.
        cpu.execute(inst);
    }
    cpu.dump_registers();

    Ok(())
}
