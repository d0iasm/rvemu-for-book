// Default memory size (128MiB).
pub const MEMORY_SIZE: u64 = 1024 * 1024 * 128;

pub struct Cpu {
    regs: [u64; 32],
    pub pc: u64,
    pub memory: Vec<u8>,
}

impl Cpu {
    pub fn new(binary: Vec<u8>) -> Self {
        let mut memory = vec![0; MEMORY_SIZE as usize];
        memory.splice(..binary.len(), binary.iter().cloned());

        // The stack pointer (SP) must be set up at first.
        let mut regs = [0; 32];
        regs[2] = MEMORY_SIZE;

        Self {
            regs,
            pc: 0,
            memory,
        }
    }

    pub fn dump_registers(&self) {
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

    /// Fetch an instruction from the memory.
    pub fn fetch(&self) -> u32 {
        let index = self.pc as usize;
        return (self.memory[index] as u32)
            | ((self.memory[index + 1] as u32) << 8)
            | ((self.memory[index + 2] as u32) << 16)
            | ((self.memory[index + 3] as u32) << 24);
    }

    /// Execute an instruction after decoding.
    pub fn execute(&mut self, inst: u32) {
        // Let `inst` u64 for the sake of simplicity.
        let inst = inst as u64;

        let opcode = inst & 0x0000007f;
        let rd = ((inst & 0x00000f80) >> 7) as usize;
        let rs1 = ((inst & 0x000f8000) >> 15) as usize;
        let rs2 = ((inst & 0x01f00000) >> 20) as usize;
        let funct3 = (inst & 0x00007000) >> 12;
        let funct7 = (inst & 0xfe000000) >> 25;

        match opcode {
            0x03 => {
                // imm[11:0] = inst[31:20]
                let imm = ((inst as i32 as i64) >> 20) as u64;
                let addr = self.regs[rs1].wrapping_add(imm);
                match funct3 {
                    0x0 => {
                        // lb
                        let val = self.read8(addr);
                        self.regs[rd] = val as i8 as i64 as u64;
                    }
                    0x1 => {
                        // lh
                        let val = self.read16(addr);
                        self.regs[rd] = val as i16 as i64 as u64;
                    }
                    0x2 => {
                        // lw
                        let val = self.read32(addr);
                        self.regs[rd] = val as i32 as i64 as u64;
                    }
                    0x3 => {
                        // ld
                        let val = self.read64(addr);
                        self.regs[rd] = val;
                    }
                    0x4 => {
                        // lbu
                        let val = self.read8(addr);
                        self.regs[rd] = val;
                    }
                    0x5 => {
                        // lhu
                        let val = self.read16(addr);
                        self.regs[rd] = val;
                    }
                    0x6 => {
                        // lwu
                        let val = self.read32(addr);
                        self.regs[rd] = val;
                    }
                    _ => {}
                }
            }
            0x13 => {
                // addi
                let imm = ((inst & 0xfff00000) as i32 as i64 >> 20) as u64;
                self.regs[rd] = self.regs[rs1].wrapping_add(imm);
            }
            0x17 => {
                // auipc
                let imm = (inst & 0xfffff000) as i32 as i64 as u64;
                self.regs[rd] = self.pc.wrapping_add(imm).wrapping_sub(4);
            }
            0x1b => {
                // imm[11:0] = inst[31:20]
                let imm = ((inst as i32 as i64) >> 20) as u64;
                // "SLLIW, SRLIW, and SRAIW encodings with imm[5] ̸= 0 are reserved."
                let shamt = (imm & 0x1f) as u32;
                match funct3 {
                    0x0 => {
                        // addiw
                        self.regs[rd] = self.regs[rs1].wrapping_add(imm) as i32 as i64 as u64;
                    }
                    0x1 => {
                        // slliw
                        self.regs[rd] = self.regs[rs1].wrapping_shl(shamt) as i32 as i64 as u64;
                    }
                    0x5 => {
                        match funct7 {
                            0x00 => {
                                // srliw
                                self.regs[rd] =
                                    self.regs[rs1].wrapping_shr(shamt) as i32 as i64 as u64;
                            }
                            0x20 => {
                                // sraiw
                                self.regs[rd] =
                                    (self.regs[rs1] as i32).wrapping_shr(shamt) as i64 as u64;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            0x23 => {
                // imm[11:5|4:0] = inst[31:25|11:7]
                let imm = (((inst & 0xfe000000) as i32 as i64 >> 20) as u64) | ((inst >> 7) & 0x1f);
                let addr = self.regs[rs1].wrapping_add(imm);
                match funct3 {
                    0x0 => self.write8(addr, self.regs[rs2]),  // sb
                    0x1 => self.write16(addr, self.regs[rs2]), // sh
                    0x2 => self.write32(addr, self.regs[rs2]), // sw
                    0x3 => self.write64(addr, self.regs[rs2]), // sd
                    _ => {}
                }
            }
            0x33 => {
                // add
                self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]);
            }
            0x37 => {
                // lui
                self.regs[rd] = (inst & 0xfffff000) as i32 as i64 as u64;
            }
            0x63 => {
                // imm[12|10:5|4:1|11] = inst[31|30:25|11:8|7]
                let imm = (((inst & 0x80000000) as i32 as i64 >> 31) as u64)
                    | ((inst >> 20) & 0x7e0)
                    | ((inst >> 7) & 0x1e)
                    | ((inst & 0x8) << 4);

                let imm12 = (((inst & 0x80000000) as i32) as i64) >> 31;
                let imm10_5 = (inst & 0x7e000000) >> 25;
                //let imm10_5 = (inst >> 25) & 0x3f;
                let imm4_1 = (inst & 0x00000f00) >> 8;
                //let imm4_1 = (inst >> 8) & 0xf;
                let imm11 = (inst & 0x00000080) >> 7;
                let offset =
                    ((imm12 << 12) as u64 | (imm11 << 11) | (imm10_5 << 5) | (imm4_1 << 1)) as i64;

                println!(
                    "imm10_5: {:#x} {:#x} imm4_1: {:#x} {:#x} imm11: {:#x} {:#x}",
                    imm10_5 << 5,
                    ((inst >> 20) & 0x3f),
                    imm4_1 << 1,
                    ((inst >> 7) & 0xf),
                    imm11 << 11,
                    ((inst & 0x8) << 4)
                );
                println!("inst {:#b} imm: {:#x} offset: {:#x}", inst, imm, offset);
                match funct3 {
                    0x0 => {
                        // beq
                        if self.regs[rs1] == self.regs[rs2] {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }
                    0x1 => {
                        // bne
                        if self.regs[rs1] != self.regs[rs2] {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }
                    0x4 => {
                        // blt
                        if (self.regs[rs1] as i64) < (self.regs[rs2] as i64) {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }
                    0x5 => {
                        // bge
                        if (self.regs[rs1] as i64) >= (self.regs[rs2] as i64) {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }
                    0x6 => {
                        // bltu
                        if self.regs[rs1] < self.regs[rs2] {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }
                    0x7 => {
                        // bgeu
                        if self.regs[rs1] >= self.regs[rs2] {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }
                    _ => {}
                }
            }
            0x6f => {
                // jal
                self.regs[rd] = self.pc;

                // imm[20|10:1|11|19:12] = inst[31|30:21|20|19:12]
                let imm = (((inst & 0x80000000) as i32 as i64 >> 11) as u64)
                    | ((inst >> 20) & 0x3ff)
                    | ((inst >> 9) & 0x1)
                    | (inst & 0xff000);
                self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
            }
            _ => {
                dbg!(format!("not implemented yet: opcode {:#x}", opcode));
                std::process::exit(1);
            }
        }
    }

    /// Read a byte from the little-endian memory.
    fn read8(&self, addr: u64) -> u64 {
        self.memory[addr as usize] as u64
    }

    /// Read 2 bytes from the little-endian memory.
    fn read16(&self, addr: u64) -> u64 {
        let index = addr as usize;
        return (self.memory[index] as u64) | ((self.memory[index + 1] as u64) << 8);
    }

    /// Read 4 bytes from the little-endian memory.
    fn read32(&self, addr: u64) -> u64 {
        let index = addr as usize;
        return (self.memory[index] as u64)
            | ((self.memory[index + 1] as u64) << 8)
            | ((self.memory[index + 2] as u64) << 16)
            | ((self.memory[index + 3] as u64) << 24);
    }

    /// Read 8 bytes from the little-endian memory.
    fn read64(&self, addr: u64) -> u64 {
        let index = addr as usize;
        return (self.memory[index] as u64)
            | ((self.memory[index + 1] as u64) << 8)
            | ((self.memory[index + 2] as u64) << 16)
            | ((self.memory[index + 3] as u64) << 24)
            | ((self.memory[index + 4] as u64) << 32)
            | ((self.memory[index + 5] as u64) << 40)
            | ((self.memory[index + 6] as u64) << 48)
            | ((self.memory[index + 7] as u64) << 56);
    }

    /// Write a byte to the little-endian memory.
    fn write8(&mut self, addr: u64, val: u64) {
        let index = addr as usize;
        self.memory[index] = val as u8
    }

    /// Write 2 bytes to the little-endian memory.
    fn write16(&mut self, addr: u64, val: u64) {
        let index = addr as usize;
        self.memory[index] = (val & 0xff) as u8;
        self.memory[index + 1] = ((val >> 8) & 0xff) as u8;
    }

    /// Write 4 bytes to the little-endian memory.
    fn write32(&mut self, addr: u64, val: u64) {
        let index = addr as usize;
        self.memory[index] = (val & 0xff) as u8;
        self.memory[index + 1] = ((val >> 8) & 0xff) as u8;
        self.memory[index + 2] = ((val >> 16) & 0xff) as u8;
        self.memory[index + 3] = ((val >> 24) & 0xff) as u8;
    }

    /// Write 8 bytes to the little-endian memory.
    fn write64(&mut self, addr: u64, val: u64) {
        let index = addr as usize;
        self.memory[index] = (val & 0xff) as u8;
        self.memory[index + 1] = ((val >> 8) & 0xff) as u8;
        self.memory[index + 2] = ((val >> 16) & 0xff) as u8;
        self.memory[index + 3] = ((val >> 24) & 0xff) as u8;
        self.memory[index + 4] = ((val >> 32) & 0xff) as u8;
        self.memory[index + 5] = ((val >> 40) & 0xff) as u8;
        self.memory[index + 6] = ((val >> 48) & 0xff) as u8;
        self.memory[index + 7] = ((val >> 56) & 0xff) as u8;
    }
}
