use crate::mm::address::phys_to_virt;


const VIRT_MMIO_BASE: usize = 0xfef0_0000;


pub fn init ()
{
    println!("virtio guest init!");
    unsafe
    {
        let mapped_mmio = phys_to_virt(VIRT_MMIO_BASE);
        println!("mapped_mmio = {:#x}",mapped_mmio);
        let test : *mut u8 = mapped_mmio as *mut _;
        *test = 1; 
    }
}
