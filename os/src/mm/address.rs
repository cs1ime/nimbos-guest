#![allow(dead_code)]

use core::fmt;

use super::PAGE_SIZE;
use crate::config::KERNEL_ASPACE_BASE;

const PA_1TB_BITS: usize = 40;
const VA_MAX_BITS: usize = 48;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysAddr(usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtAddr(usize);

const fn phys_to_virt(paddr: PhysAddr) -> VirtAddr {
    VirtAddr::new(paddr.as_usize() + KERNEL_ASPACE_BASE)
}

impl fmt::Debug for PhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("PA:{:#x}", self.0))
    }
}

impl fmt::Debug for VirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("VA:{:#x}", self.0))
    }
}

impl PhysAddr {
    pub const MAX: usize = (1 << PA_1TB_BITS) - 1;

    pub const fn new(pa: usize) -> Self {
        Self(pa & Self::MAX)
    }
    pub const fn as_usize(&self) -> usize {
        self.0
    }
    pub const fn as_ptr(&self) -> *const u8 {
        phys_to_virt(*self).as_usize() as _
    }
    pub const fn as_mut_ptr(&self) -> *mut u8 {
        phys_to_virt(*self).as_usize() as _
    }
    pub const fn align_down(&self) -> Self {
        Self(align_down(self.0, PAGE_SIZE))
    }
    pub const fn align_up(&self) -> Self {
        Self(align_up(self.0, PAGE_SIZE))
    }
    pub const fn page_offset(&self) -> usize {
        page_offset(self.0, PAGE_SIZE)
    }
    pub const fn is_aligned(&self) -> bool {
        is_aligned(self.0, PAGE_SIZE)
    }
}

impl VirtAddr {
    pub const MAX: usize = (1 << VA_MAX_BITS) - 1;
    pub const TOP_BASE: usize = 0xffff_0000_0000_0000;

    pub const fn new(va: usize) -> Self {
        let top_bits = va >> VA_MAX_BITS;
        if top_bits != 0 && top_bits != 0xffff {
            panic!("invalid VA!")
        }
        Self(va)
    }
    pub const fn as_usize(&self) -> usize {
        self.0
    }
    pub const fn align_down(&self) -> Self {
        Self(align_down(self.0, PAGE_SIZE))
    }
    pub const fn align_up(&self) -> Self {
        Self(align_up(self.0, PAGE_SIZE))
    }
    pub const fn page_offset(&self) -> usize {
        page_offset(self.0, PAGE_SIZE)
    }
    pub const fn is_aligned(&self) -> bool {
        is_aligned(self.0, PAGE_SIZE)
    }
}

const fn align_down(addr: usize, page_size: usize) -> usize {
    addr & !(page_size - 1)
}

const fn align_up(addr: usize, page_size: usize) -> usize {
    (addr + page_size - 1) & !(page_size - 1)
}

const fn page_offset(addr: usize, page_size: usize) -> usize {
    addr & (page_size - 1)
}

const fn is_aligned(addr: usize, page_size: usize) -> bool {
    page_offset(addr, page_size) == 0
}
