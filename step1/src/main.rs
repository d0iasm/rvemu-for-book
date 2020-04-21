use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

/// The CPU to contain registers, a program coutner, and memory.
struct Cpu {
    /// 32 64-bit integer registers.
    regs: [u64; 32],
    /// Program counter to hold the the memory address of the next instruction that would be executed.
    pc: u64,
    /// Computer memory to store executable instructions.
    memory: Vec<u8>,
}

impl Cpu {
    /// Create a new `Cpu` object.
    fn new(binary: Vec<u8>) -> Self {
        Self {
            regs: [0; 32],
            pc: 0,
            memory: binary,
        }
    }

    /// Print values in all registers (x0-x31).
    fn dump_registers(&self) {
        let mut output = String::from("");
        for i in (0..32).step_by(4) {
            output = format!(
                "{}\n{}",
                output,
                format!(
                    "x{:02}={:>#18x} x{:02}={:>#18x} x{:02}={:>#18x} x{:02}={:>#18x}",
                    i,
                    self.regs[i],
                    i + 1,
                    self.regs[i + 1],
                    i + 2,
                    self.regs[i + 2],
                    i + 3,
                    self.regs[i + 3],
                )
            );
        }
        println!("{}", output);
    }

    /// Get an instruction from the memory.
    fn fetch(&self) -> u32 {
        let index = self.pc as usize;
        return (self.memory[index] as u32)
            | ((self.memory[index + 1] as u32) << 8)
            | ((self.memory[index + 2] as u32) << 16)
            | ((self.memory[index + 3] as u32) << 24);
    }

    /// Execute an instruction after decoding.
    fn execute(&mut self, inst: u32) {
        let opcode = inst & 0x0000007f;
        let rd = ((inst & 0x00000f80) >> 7) as usize;
        let rs1 = ((inst & 0x000f8000) >> 15) as usize;
        let rs2 = ((inst & 0x01f00000) >> 20) as usize;

        match opcode {
            0x13 => {
                // addi
                let imm = ((inst & 0xfff00000) as i32 as i64 >> 20) as u64;
                self.regs[rd] = self.regs[rs1] + imm;
            }
            0x33 => {
                // add
                self.regs[rd] = self.regs[rs1] + self.regs[rs2];
            }
            _ => {
                dbg!(format!("not implemented yet: opcode {:#x}", opcode));
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
    let mut binary = Vec::new();
    file.read_to_end(&mut binary)?;

    let mut cpu = Cpu::new(binary);

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
