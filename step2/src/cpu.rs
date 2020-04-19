// Default memory size (128MiB).
pub const MEMORY_SIZE: u64 = 1024 * 1024 * 128;

pub struct Cpu {
    pub regs: [u64; 32],
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

    /// Fetch an instruction from the memory.
    pub fn fetch(&self) -> u32 {
        return self.read32(self.pc) as u32;
    }

    /// Execute an instruction after decoding. Return true if an error happens, otherwise false.
    pub fn execute(&mut self, inst: u32) -> bool {
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
                // imm[11:0] = inst[31:20]
                let imm = ((inst & 0xfff00000) as i32 as i64 >> 20) as u64;
                // shamt size is 5 bits for RV32I and 6 bits for RV64I.
                let shamt = ((inst & 0x03f00000) >> 20) as u32;
                let funct6 = funct7 >> 1;
                match funct3 {
                    0x0 => {
                        // addi
                        self.regs[rd] = self.regs[rs1].wrapping_add(imm);
                    }
                    0x1 => {
                        // slli
                        self.regs[rd] = self.regs[rs1] << shamt;
                    }
                    0x2 => {
                        // slti
                        self.regs[rd] = if (self.regs[rs1] as i64) < (imm as i64) {
                            1
                        } else {
                            0
                        };
                    }
                    0x3 => {
                        // sltiu
                        self.regs[rd] = if self.regs[rs1] < imm { 1 } else { 0 };
                    }
                    0x4 => {
                        // xori
                        self.regs[rd] = self.regs[rs1] ^ imm;
                    }
                    0x5 => {
                        match funct6 {
                            // srli
                            0x00 => self.regs[rd] = self.regs[rs1].wrapping_shr(shamt),
                            // srai
                            0x10 => {
                                self.regs[rd] = (self.regs[rs1] as i64).wrapping_shr(shamt) as u64
                            }
                            _ => {}
                        }
                    }
                    0x6 => self.regs[rd] = self.regs[rs1] | imm, // ori
                    0x7 => self.regs[rd] = self.regs[rs1] & imm, // andi
                    _ => {}
                }
            }
            0x17 => {
                // auipc
                let imm = (inst & 0xfffff000) as i32 as i64 as u64;
                self.regs[rd] = self.pc.wrapping_add(imm).wrapping_sub(4);
            }
            0x1b => {
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
                                self.regs[rd] = (self.regs[rs1] as u32).wrapping_shr(shamt) as i32
                                    as i64 as u64;
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
                // S-type (RV32I)
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
                // R-type (RV64I and RV64M)
                // "SLL, SRL, and SRA perform logical left, logical right, and arithmetic right
                // shifts on the value in register rs1 by the shift amount held in register rs2.
                // In RV64I, only the low 6 bits of rs2 are considered for the shift amount."
                let shamt = ((self.regs[rs2] & 0x3f) as u64) as u32;
                match (funct3, funct7) {
                    (0x0, 0x00) => {
                        // add
                        self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]);
                    }
                    (0x0, 0x20) => {
                        // sub
                        self.regs[rd] = self.regs[rs1].wrapping_sub(self.regs[rs2]);
                    }
                    (0x1, 0x00) => {
                        // sll
                        self.regs[rd] = self.regs[rs1].wrapping_shl(shamt);
                    }
                    (0x2, 0x00) => {
                        // slt
                        self.regs[rd] = if (self.regs[rs1] as i64) < (self.regs[rs2] as i64) {
                            1
                        } else {
                            0
                        };
                    }
                    (0x3, 0x00) => {
                        // sltu
                        self.regs[rd] = if self.regs[rs1] < self.regs[rs2] {
                            1
                        } else {
                            0
                        };
                    }
                    (0x4, 0x00) => {
                        // xor
                        self.regs[rd] = self.regs[rs1] ^ self.regs[rs2];
                    }
                    (0x5, 0x00) => {
                        // srl
                        self.regs[rd] = self.regs[rs1].wrapping_shr(shamt);
                    }
                    (0x5, 0x20) => {
                        // sra
                        self.regs[rd] = (self.regs[rs1] as i64).wrapping_shr(shamt) as u64
                    } // sra
                    (0x6, 0x00) => {
                        // or
                        self.regs[rd] = self.regs[rs1] | self.regs[rs2];
                    }
                    (0x7, 0x00) => {
                        // and
                        self.regs[rd] = self.regs[rs1] & self.regs[rs2];
                    }
                    _ => {}
                }
            }
            0x37 => {
                // lui
                self.regs[rd] = (inst & 0xfffff000) as i32 as i64 as u64;
            }
            0x3b => {
                // The shift amount is given by rs2[4:0].
                let shamt = (self.regs[rs2] & 0x1f) as u32;
                match (funct3, funct7) {
                    (0x0, 0x00) => {
                        // addw
                        self.regs[rd] =
                            self.regs[rs1].wrapping_add(self.regs[rs2]) as i32 as i64 as u64;
                    }
                    (0x0, 0x20) => {
                        // subw
                        self.regs[rd] =
                            ((self.regs[rs1].wrapping_sub(self.regs[rs2])) as i32) as u64;
                    }
                    (0x1, 0x00) => {
                        // sllw
                        self.regs[rd] = ((self.regs[rs1] as u32) << shamt) as i32 as u64;
                    }
                    (0x5, 0x00) => {
                        // srlw
                        self.regs[rd] = ((self.regs[rs1] as u32) >> shamt) as i32 as u64;
                    }
                    (0x5, 0x20) => {
                        // sraw
                        self.regs[rd] = ((self.regs[rs1] as i32) >> (shamt as i32)) as u64;
                    }
                    _ => {}
                }
            }
            0x63 => {
                // imm[12|10:5|4:1|11] = inst[31|30:25|11:8|7]
                let imm = (((inst & 0x80000000) as i32 as i64 >> 19) as u64)
                    | ((inst & 0x80) << 4) // imm[11]
                    | ((inst >> 20) & 0x7e0) // imm[10:5]
                    | ((inst >> 7) & 0x1e); // imm[4:1]

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
            0x67 => {
                // jalr
                let t = self.pc;

                let imm = ((((inst & 0xfff00000) as i32) as i64) >> 20) as u64;
                self.pc = (self.regs[rs1].wrapping_add(imm)) & !1;

                self.regs[rd] = t;
            }
            0x6f => {
                // jal
                self.regs[rd] = self.pc;

                // imm[20|10:1|11|19:12] = inst[31|30:21|20|19:12]
                let imm = (((inst & 0x80000000) as i32 as i64 >> 11) as u64) // imm[20]
                    | (inst & 0xff000) // imm[19:12]
                    | ((inst >> 9) & 0x800) // imm[11]
                    | ((inst >> 20) & 0x7fe); // imm[10:1]

                self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
            }
            _ => {
                dbg!(format!("not implemented yet: opcode {:#x}", opcode));
                return true;
            }
        }
        return false;
    }
}
