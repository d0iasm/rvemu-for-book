use crate::cpu::Cpu;
use crate::trap::Exception;

pub fn execute(cpu: &mut Cpu, inst: u64) -> Result<(), Exception> {
    let opcode = inst & 0x7f;
    let rd = ((inst >> 7) & 0x1f) as usize;
    let rs1 = ((inst >> 15) & 0x1f) as usize;
    let rs2 = ((inst >> 20) & 0x1f) as usize;
    let funct3 = (inst >> 12) & 0x7;
    let funct7 = (inst >> 25) & 0x7f;

    Ok(())
}
