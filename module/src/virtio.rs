//! The virtio module contains a virtualization standard for network and disk device drivers.
//! This is the "legacy" virtio interface.
//!
//! The virtio spec:
//! https://docs.oasis-open.org/virtio/virtio/v1.1/virtio-v1.1.pdf

use crate::bus::*;
use crate::cpu::*;
use crate::trap::*;

/// The interrupt request of virtio.
pub const VIRTIO_IRQ: u64 = 1;

const VRING_DESC_SIZE: u64 = 16;
/// The number of virtio descriptors. It must be a power of two.
const DESC_NUM: u64 = 8;

/// Always return 0x74726976.
pub const VIRTIO_MAGIC: u64 = VIRTIO_BASE + 0x000;
/// The version. 1 is legacy.
pub const VIRTIO_VERSION: u64 = VIRTIO_BASE + 0x004;
/// device type; 1 is net, 2 is disk.
pub const VIRTIO_DEVICE_ID: u64 = VIRTIO_BASE + 0x008;
/// Always return 0x554d4551
pub const VIRTIO_VENDOR_ID: u64 = VIRTIO_BASE + 0x00c;
/// Device features.
pub const VIRTIO_DEVICE_FEATURES: u64 = VIRTIO_BASE + 0x010;
/// Driver features.
pub const VIRTIO_DRIVER_FEATURES: u64 = VIRTIO_BASE + 0x020;
/// Page size for PFN, write-only.
pub const VIRTIO_GUEST_PAGE_SIZE: u64 = VIRTIO_BASE + 0x028;
/// Select queue, write-only.
pub const VIRTIO_QUEUE_SEL: u64 = VIRTIO_BASE + 0x030;
/// Max size of current queue, read-only. In QEMU, `VIRTIO_COUNT = 8`.
pub const VIRTIO_QUEUE_NUM_MAX: u64 = VIRTIO_BASE + 0x034;
/// Size of current queue, write-only.
pub const VIRTIO_QUEUE_NUM: u64 = VIRTIO_BASE + 0x038;
/// Physical page number for queue, read and write.
pub const VIRTIO_QUEUE_PFN: u64 = VIRTIO_BASE + 0x040;
/// Notify the queue number, write-only.
pub const VIRTIO_QUEUE_NOTIFY: u64 = VIRTIO_BASE + 0x050;
/// Device status, read and write. Reading from this register returns the current device status flags.
/// Writing non-zero values to this register sets the status flags, indicating the OS/driver
/// progress. Writing zero (0x0) to this register triggers a device reset.
pub const VIRTIO_STATUS: u64 = VIRTIO_BASE + 0x070;

/// Paravirtualized drivers for IO virtualization.
pub struct Virtio {
    id: u64,
    driver_features: u32,
    page_size: u32,
    queue_sel: u32,
    queue_num: u32,
    queue_pfn: u32,
    queue_notify: u32,
    status: u32,
    disk: Vec<u8>,
}

impl Device for Virtio {
    fn load(&mut self, addr: u64, size: u64) -> Result<u64, Exception> {
        match size {
            32 => Ok(self.load32(addr)),
            _ => Err(Exception::LoadAccessFault),
        }
    }

    fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        match size {
            32 => Ok(self.store32(addr, value)),
            _ => Err(Exception::StoreAMOAccessFault),
        }
    }
}

impl Virtio {
    /// Create a new virtio object.
    pub fn new(disk_image: Vec<u8>) -> Self {
        let mut disk = Vec::new();
        disk.extend(disk_image.iter().cloned());

        Self {
            id: 0,
            driver_features: 0,
            page_size: 0,
            queue_sel: 0,
            queue_num: 0,
            queue_pfn: 0,
            queue_notify: 9999, // TODO: what is the correct initial value?
            status: 0,
            disk,
        }
    }

    /// Return true if an interrupt is pending.
    pub fn is_interrupting(&mut self) -> bool {
        if self.queue_notify != 9999 {
            self.queue_notify = 9999;
            return true;
        }
        false
    }

    /// Load 4 bytes from virtio only if the addr is valid. Otherwise, return 0.
    pub fn load32(&self, addr: u64) -> u64 {
        match addr {
            VIRTIO_MAGIC => 0x74726976,
            VIRTIO_VERSION => 0x1,
            VIRTIO_DEVICE_ID => 0x2,
            VIRTIO_VENDOR_ID => 0x554d4551,
            VIRTIO_DEVICE_FEATURES => 0, // TODO: what should it return?
            VIRTIO_DRIVER_FEATURES => self.driver_features as u64,
            VIRTIO_QUEUE_NUM_MAX => 8,
            VIRTIO_QUEUE_PFN => self.queue_pfn as u64,
            VIRTIO_STATUS => self.status as u64,
            _ => 0,
        }
    }

    /// Store 4 bytes to virtio only if the addr is valid. Otherwise, does nothing.
    pub fn store32(&mut self, addr: u64, value: u64) {
        let val = value as u32;
        match addr {
            VIRTIO_DEVICE_FEATURES => self.driver_features = val,
            VIRTIO_GUEST_PAGE_SIZE => self.page_size = val,
            VIRTIO_QUEUE_SEL => self.queue_sel = val,
            VIRTIO_QUEUE_NUM => self.queue_num = val,
            VIRTIO_QUEUE_PFN => self.queue_pfn = val,
            VIRTIO_QUEUE_NOTIFY => self.queue_notify = val,
            VIRTIO_STATUS => self.status = val,
            _ => {}
        }
    }

    fn get_new_id(&mut self) -> u64 {
        self.id = self.id.wrapping_add(1);
        self.id
    }

    fn desc_addr(&self) -> u64 {
        self.queue_pfn as u64 * self.page_size as u64
    }

    fn read_disk(&self, addr: u64) -> u64 {
        self.disk[addr as usize] as u64
    }

    fn write_disk(&mut self, addr: u64, value: u64) {
        self.disk[addr as usize] = value as u8
    }

    /// Access the disk via virtio. This is an associated function which takes a `cpu` object to
    /// read and write with a dram directly (DMA).
    pub fn disk_access(cpu: &mut Cpu) {
        // See more information in
        // https://github.com/mit-pdos/xv6-riscv/blob/riscv/kernel/virtio_disk.c

        // the spec says that legacy block operations use three
        // descriptors: one for type/reserved/sector, one for
        // the data, one for a 1-byte status result.

        // desc = pages -- num * VRingDesc
        // avail = pages + 0x40 -- 2 * uint16, then num * uint16
        // used = pages + 4096 -- 2 * uint16, then num * vRingUsedElem
        let desc_addr = cpu.bus.virtio.desc_addr();
        let avail_addr = cpu.bus.virtio.desc_addr() + 0x40;
        let used_addr = cpu.bus.virtio.desc_addr() + 4096;

        // avail[0] is flags
        // avail[1] tells the device how far to look in avail[2...].
        let offset = cpu
            .bus
            .load(avail_addr.wrapping_add(1), 16)
            .expect("failed to read offset");
        // avail[2...] are desc[] indices the device should process.
        // we only tell device the first index in our chain of descriptors.
        let index = cpu
            .bus
            .load(
                avail_addr.wrapping_add(offset % DESC_NUM).wrapping_add(2),
                16,
            )
            .expect("failed to read index");

        // Read `VRingDesc`, virtio descriptors.
        let desc_addr0 = desc_addr + VRING_DESC_SIZE * index;
        let addr0 = cpu
            .bus
            .load(desc_addr0, 64)
            .expect("failed to read an address field in a descriptor");
        // Add 14 because of `VRingDesc` structure.
        // struct VRingDesc {
        //   uint64 addr;
        //   uint32 len;
        //   uint16 flags;
        //   uint16 next
        // };
        // The `next` field can be accessed by offset 14 (8 + 4 + 2) bytes.
        let next0 = cpu
            .bus
            .load(desc_addr0.wrapping_add(14), 16)
            .expect("failed to read a next field in a descripor");

        // Read `VRingDesc` again, virtio descriptors.
        let desc_addr1 = desc_addr + VRING_DESC_SIZE * next0;
        let addr1 = cpu
            .bus
            .load(desc_addr1, 64)
            .expect("failed to read an address field in a descriptor");
        let len1 = cpu
            .bus
            .load(desc_addr1.wrapping_add(8), 32)
            .expect("failed to read a length field in a descriptor");
        let flags1 = cpu
            .bus
            .load(desc_addr1.wrapping_add(12), 16)
            .expect("failed to read a flags field in a descriptor");

        // Read `virtio_blk_outhdr`. Add 8 because of its structure.
        // struct virtio_blk_outhdr {
        //   uint32 type;
        //   uint32 reserved;
        //   uint64 sector;
        // } buf0;
        let blk_sector = cpu
            .bus
            .load(addr0.wrapping_add(8), 64)
            .expect("failed to read a sector field in a virtio_blk_outhdr");

        // Write to a device if the second bit `flag1` is set.
        match (flags1 & 2) == 0 {
            true => {
                // Read dram data and write it to a disk directly (DMA).
                for i in 0..len1 as u64 {
                    let data = cpu
                        .bus
                        .load(addr1 + i, 8)
                        .expect("failed to read from dram");
                    cpu.bus.virtio.write_disk(blk_sector * 512 + i, data);
                }
            }
            false => {
                // Read disk data and write it to dram directly (DMA).
                for i in 0..len1 as u64 {
                    let data = cpu.bus.virtio.read_disk(blk_sector * 512 + i);
                    cpu.bus
                        .store(addr1 + i, 8, data)
                        .expect("failed to write to dram");
                }
            }
        };

        // Write id to `UsedArea`. Add 2 because of its structure.
        // struct UsedArea {
        //   uint16 flags;
        //   uint16 id;
        //   struct VRingUsedElem elems[NUM];
        // };
        let new_id = cpu.bus.virtio.get_new_id();
        cpu.bus
            .store(used_addr.wrapping_add(2), 16, new_id % 8)
            .expect("failed to write to dram");
    }
}
