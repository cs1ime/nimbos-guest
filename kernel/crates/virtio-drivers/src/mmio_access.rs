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

#[derive(Debug, Default)]
#[repr(transparent)]
pub struct Volatile<T: Copy>(T);
impl Volatile<u32> {
    pub fn read(&self) -> u32 {
        mmio_readword(&self.0 as *const _)
    }
    pub fn write(&mut self,value : u32){
        mmio_writeword(&mut self.0 as *mut _,value);
    }
}
impl Volatile<u8> {
    pub fn read(&self) -> u8 {
        mmio_readbyte(&self.0 as *const _)
    }
    pub fn write(&mut self,value : u8){
        mmio_writebyte(&mut self.0 as *mut _,value);
    }
}

#[derive(Debug, Default)]
pub struct ReadOnly<T: Copy>(Volatile<T>);
impl ReadOnly<u32> {
    pub fn read(&self) -> u32 {
        self.0.read()
    }
}
impl ReadOnly<u8> {
    pub fn read(&self) -> u8 {
        self.0.read()
    }
}



#[derive(Default)]
pub struct WriteOnly<T: Copy>(Volatile<T>);

impl WriteOnly<u32> {
    pub fn write(&mut self, value: u32) {
        self.0.write(value)
    }
}
impl WriteOnly<u8> {
    pub fn write(&mut self, value: u8) {
        self.0.write(value)
    }
}

