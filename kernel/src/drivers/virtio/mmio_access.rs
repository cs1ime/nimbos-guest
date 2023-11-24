use core::arch::global_asm;

global_asm!(
    include_str!("mmio_access.S"),
);

extern "C" {
    fn __mmio_readbyte(ptr: *const u8) -> u8;
    fn __mmio_writebyte(ptr: *mut u8, value: u8);
    fn __mmio_readword(ptr: *const u32) -> u32;
    fn __mmio_writeword(ptr: *mut u32, value: u32);
}

pub fn mmio_readbyte(ptr: *const u8) -> u8 {
    unsafe{__mmio_readbyte(ptr)}
}
pub fn mmio_writebyte(ptr: *mut u8, value: u8) {
    unsafe{__mmio_writebyte(ptr,value)}
}
pub fn mmio_readword(ptr: *const u32) -> u32 {
    unsafe{__mmio_readword(ptr)}    
}
pub fn mmio_writeword(ptr: *mut u32, value: u32) {
    unsafe{__mmio_writeword(ptr,value)}
}



