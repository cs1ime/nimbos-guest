use crate::{mm::address::phys_to_virt, drivers::virtio::mmio_access::{self, mmio_readbyte}};

const VIRT_MMIO_BASE: usize = 0xfef0_0000;


pub fn init ()
{
    println!("virtio guest init!");
    unsafe
    {
        let mapped_mmio = phys_to_virt(VIRT_MMIO_BASE);
        println!("mapped_mmio = {:#x}",mapped_mmio);
        let test : *mut u8 = (mapped_mmio+4) as *mut _;
        mmio_readbyte(test);
    }
}
