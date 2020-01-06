use byteorder::{ByteOrder, LittleEndian};
use memmap::Mmap;
use rangemap::RangeMap;

/// Memory interface.
pub struct Memory {
    // The mapping between virtual addresses to offsets in the mmap region.
    mapping: RangeMap<u64, (u64, u64)>,
    // The backing mmap region for memory.
    mmap: Mmap,
}

impl Memory {
    pub fn new(mapping: RangeMap<u64, (u64, u64)>, mmap: Mmap) -> Self {
        Memory { mapping, mmap }
    }
    pub fn read_u32(&self, vaddr: u64) -> u32 {
        if let Some((vaddr_offset, mmap_offset)) = self.mapping.get(&vaddr) {
            let start = *mmap_offset as usize + (vaddr as usize - *vaddr_offset as usize);
            let end = start + std::mem::size_of::<u32>();
            LittleEndian::read_u32(&self.mmap[start..end])
        } else {
            panic!("out-of-bounds memory read")
        }
    }
}
