use crate::cpu::Cpu;
use crate::trap::Exception;

pub fn execute(_cpu: &mut Cpu, _inst: u64) -> Result<(), Exception> {
    /*
    let opcode = inst & 0x7f;
    let rd = ((inst >> 7) & 0x1f) as usize;
    let rs1 = ((inst >> 15) & 0x1f) as usize;
    let rs2 = ((inst >> 20) & 0x1f) as usize;
    let funct3 = (inst >> 12) & 0x7;
    let funct7 = (inst >> 25) & 0x7f;

    if opcode == 0x03 {
        // imm[11:0] = inst[31:20]
        let imm = ((inst as i32 as i64) >> 20) as u64;
        let addr = cpu.regs[rs1].wrapping_add(imm);
        match funct3 {
            0x0 => {
                // lb
                let val = cpu.load(addr, 8)?;
                cpu.regs[rd] = val as i8 as i64 as u64;
            }
            0x1 => {
                // lh
                let val = cpu.load(addr, 16)?;
                cpu.regs[rd] = val as i16 as i64 as u64;
            }
            0x2 => {
                // lw
                let val = cpu.load(addr, 32)?;
                cpu.regs[rd] = val as i32 as i64 as u64;
            }
            0x3 => {
                // ld
                let val = cpu.load(addr, 64)?;
                cpu.regs[rd] = val;
            }
            0x4 => {
                // lbu
                let val = cpu.load(addr, 8)?;
                cpu.regs[rd] = val;
            }
            0x5 => {
                // lhu
                let val = cpu.load(addr, 16)?;
                cpu.regs[rd] = val;
            }
            0x6 => {
                // lwu
                let val = cpu.load(addr, 32)?;
                cpu.regs[rd] = val;
            }
            _ => {
                println!("not implemented: opcode {:#x} funct3 {:#x}", opcode, funct3);
                return Err(Exception::IllegalInstruction);
            }
        }
    }

    if opcode == 0x0f {
        // A fence instruction does nothing because this emulator executes an instruction
        // sequentially on a single thread.
        match funct3 {
            0x0 => {} // fence
            _ => {
                println!("not implemented: opcode {:#x} funct3 {:#x}", opcode, funct3);
                return Err(Exception::IllegalInstruction);
            }
        }
    }

    */
    Ok(())
}
