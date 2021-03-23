use crate::cpu::Cpu;
use crate::trap::Exception;

/// The page size (4 KiB) for the virtual dram system.
pub const PAGE_SIZE: u64 = 4096;

/// Access type that is used in the virtual address translation process. It decides which exception
/// should raises (InstructionPageFault, LoadPageFault or StoreAMOPageFault).
#[derive(Debug, PartialEq, PartialOrd)]
pub enum AccessType {
    /// Raises the exception InstructionPageFault. It is used for an instruction fetch.
    Instruction,
    /// Raises the exception LoadPageFault.
    Load,
    /// Raises the exception StoreAMOPageFault.
    Store,
}

/// Translate a virtual address to a physical address for the paged virtual-dram system.
pub fn translate(cpu: &mut Cpu, addr: u64, access_type: AccessType) -> Result<u64, Exception> {
    if !cpu.enable_paging {
        return Ok(addr);
    }

    // The following comments are cited from 4.3.2 Virtual Address Translation Process
    // in "The RISC-V Instruction Set Manual Volume II-Privileged Architecture_20190608".

    // "A virtual address va is translated into a physical address pa as follows:"
    let levels = 3;
    let vpn = [
        (addr >> 12) & 0x1ff,
        (addr >> 21) & 0x1ff,
        (addr >> 30) & 0x1ff,
    ];

    // "1. Let a be satp.ppn × PAGESIZE, and let i = LEVELS − 1. (For Sv32, PAGESIZE=212
    //     and LEVELS=2.)"
    let mut a = cpu.page_table;
    let mut i: i64 = levels - 1;
    let mut pte;
    loop {
        // "2. Let pte be the value of the PTE at address a+va.vpn[i]×PTESIZE. (For Sv32,
        //     PTESIZE=4.) If accessing pte violates a PMA or PMP check, raise an access
        //     exception corresponding to the original access type."
        pte = cpu.bus.load(a + vpn[i as usize] * 8, 64)?;

        // "3. If pte.v = 0, or if pte.r = 0 and pte.w = 1, stop and raise a page-fault
        //     exception corresponding to the original access type."
        let v = pte & 1;
        let r = (pte >> 1) & 1;
        let w = (pte >> 2) & 1;
        let x = (pte >> 3) & 1;
        if v == 0 || (r == 0 && w == 1) {
            match access_type {
                AccessType::Instruction => return Err(Exception::InstructionPageFault),
                AccessType::Load => return Err(Exception::LoadPageFault),
                AccessType::Store => return Err(Exception::StoreAMOPageFault),
            }
        }

        // "4. Otherwise, the PTE is valid. If pte.r = 1 or pte.x = 1, go to step 5.
        //     Otherwise, this PTE is a pointer to the next level of the page table.
        //     Let i = i − 1. If i < 0, stop and raise a page-fault exception
        //     corresponding to the original access type. Otherwise,
        //     let a = pte.ppn × PAGESIZE and go to step 2."
        if r == 1 || x == 1 {
            break;
        }
        i -= 1;
        let ppn = (pte >> 10) & 0x0fff_ffff_ffff;
        a = ppn * PAGE_SIZE;
        if i < 0 {
            match access_type {
                AccessType::Instruction => return Err(Exception::InstructionPageFault),
                AccessType::Load => return Err(Exception::LoadPageFault),
                AccessType::Store => return Err(Exception::StoreAMOPageFault),
            }
        }
    }

    // A leaf PTE has been found.
    let ppn = [
        (pte >> 10) & 0x1ff,
        (pte >> 19) & 0x1ff,
        (pte >> 28) & 0x03ff_ffff,
    ];

    // We skip implementing from step 5 to 7.

    // "5. A leaf PTE has been found. Determine if the requested dram access is allowed by
    //     the pte.r, pte.w, pte.x, and pte.u bits, given the current privilege mode and the
    //     value of the SUM and MXR fields of the mstatus register. If not, stop and raise a
    //     page-fault exception corresponding to the original access type."

    // "6. If i > 0 and pte.ppn[i − 1 : 0] ̸= 0, this is a misaligned superpage; stop and
    //     raise a page-fault exception corresponding to the original access type."

    // "7. If pte.a = 0, or if the dram access is a store and pte.d = 0, either raise a
    //     page-fault exception corresponding to the original access type, or:
    //     • Set pte.a to 1 and, if the dram access is a store, also set pte.d to 1.
    //     • If this access violates a PMA or PMP check, raise an access exception
    //     corresponding to the original access type.
    //     • This update and the loading of pte in step 2 must be atomic; in particular, no
    //     intervening store to the PTE may be perceived to have occurred in-between."

    // "8. The translation is successful. The translated physical address is given as
    //     follows:
    //     • pa.pgoff = va.pgoff.
    //     • If i > 0, then this is a superpage translation and pa.ppn[i−1:0] =
    //     va.vpn[i−1:0].
    //     • pa.ppn[LEVELS−1:i] = pte.ppn[LEVELS−1:i]."
    let offset = addr & 0xfff;
    match i {
        0 => {
            let ppn = (pte >> 10) & 0x0fff_ffff_ffff;
            Ok((ppn << 12) | offset)
        }
        1 => {
            // Superpage translation. A superpage is a dram page of larger size than an
            // ordinary page (4 KiB). It reduces TLB misses and improves performance.
            Ok((ppn[2] << 30) | (ppn[1] << 21) | (vpn[0] << 12) | offset)
        }
        2 => {
            // Superpage translation. A superpage is a dram page of larger size than an
            // ordinary page (4 KiB). It reduces TLB misses and improves performance.
            Ok((ppn[2] << 30) | (vpn[1] << 21) | (vpn[0] << 12) | offset)
        }
        _ => match access_type {
            AccessType::Instruction => return Err(Exception::InstructionPageFault),
            AccessType::Load => return Err(Exception::LoadPageFault),
            AccessType::Store => return Err(Exception::StoreAMOPageFault),
        },
    }
}
