use core::arch::global_asm;

global_asm!(
    include_str!("mmio_access.S"),
);

extern "C" {
    fn __mmio_read8(ptr: *const u8) -> u8;
    fn __mmio_write8(ptr: *mut u8, value: u8);
    fn __mmio_read16(ptr: *const u16) -> u16;
    fn __mmio_write16(ptr: *mut u16, value: u16);
    fn __mmio_read32(ptr: *const u32) -> u32;
    fn __mmio_write32(ptr: *mut u32, value: u32);
    fn __mmio_read64(ptr: *const u64) -> u64;
    fn __mmio_write64(ptr: *mut u64, value: u64);
}

pub fn mmio_read8(ptr: *const u8) -> u8 {
    unsafe{__mmio_read8(ptr)}
}
pub fn mmio_write8(ptr: *mut u8, value: u8) {
    unsafe{__mmio_write8(ptr,value)}
}
pub fn mmio_read16(ptr: *const u16) -> u16 {
    unsafe{__mmio_read16(ptr)}    
}
pub fn mmio_write16(ptr: *mut u16, value: u16) {
    unsafe{__mmio_write16(ptr,value)}
}
pub fn mmio_read32(ptr: *const u32) -> u32 {
    unsafe{__mmio_read32(ptr)}    
}
pub fn mmio_write32(ptr: *mut u32, value: u32) {
    unsafe{__mmio_write32(ptr,value)}
}
pub fn mmio_read64(ptr: *const u64) -> u64 {
    unsafe{__mmio_read64(ptr)}    
}
pub fn mmio_write64(ptr: *mut u64, value: u64) {
    unsafe{__mmio_write64(ptr,value)}
}

#[derive(Debug, Default)]
#[repr(transparent)]
pub struct Volatile<T: Copy>(T);

impl Volatile<u8> {
    pub fn read(&self) -> u8 {
        mmio_read8(&self.0 as *const _)
    }
    pub fn write(&mut self,value : u8){
        mmio_write8(&mut self.0 as *mut _,value);
    }
}
impl Volatile<u16> {
    pub fn read(&self) -> u16 {
        mmio_read16(&self.0 as *const _)
    }
    pub fn write(&mut self,value : u16){
        mmio_write16(&mut self.0 as *mut _,value);
    }
}
impl Volatile<u32> {
    pub fn read(&self) -> u32 {
        mmio_read32(&self.0 as *const _)
    }
    pub fn write(&mut self,value : u32){
        mmio_write32(&mut self.0 as *mut _,value);
    }
}
impl Volatile<u64> {
    pub fn read(&self) -> u64 {
        mmio_read64(&self.0 as *const _)
    }
    pub fn write(&mut self,value : u64){
        mmio_write64(&mut self.0 as *mut _,value);
    }
}


#[derive(Debug, Default)]
pub struct ReadOnly<T: Copy>(Volatile<T>);

impl ReadOnly<u8> {
    pub fn read(&self) -> u8 {
        self.0.read()
    }
}
impl ReadOnly<u16> {
    pub fn read(&self) -> u16 {
        self.0.read()
    }
}
impl ReadOnly<u32> {
    pub fn read(&self) -> u32 {
        self.0.read()
    }
}
impl ReadOnly<u64> {
    pub fn read(&self) -> u64 {
        self.0.read()
    }
}



#[derive(Default)]
pub struct WriteOnly<T: Copy>(Volatile<T>);

impl WriteOnly<u8> {
    pub fn write(&mut self, value: u8) {
        self.0.write(value)
    }
}
impl WriteOnly<u16> {
    pub fn write(&mut self, value: u16) {
        self.0.write(value)
    }
}
impl WriteOnly<u32> {
    pub fn write(&mut self, value: u32) {
        self.0.write(value)
    }
}
impl WriteOnly<u64> {
    pub fn write(&mut self, value: u64) {
        self.0.write(value)
    }
}

