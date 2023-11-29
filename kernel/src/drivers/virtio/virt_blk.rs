use alloc::sync::Arc;
use virtio_drivers::{VirtIOHeader, VirtIOBlk, Hal};

use crate::{mm::{address::{phys_to_virt, virt_to_phys}, PhysFrame, PAGE_SIZE, PhysAddr}};
use core::{any::Any, cell::RefCell, borrow::BorrowMut};
use crate::sync::Mutex;

pub trait BlockDevice: Send + Sync + Any {
    ///Read data form block to buffer
    fn read_block(&self, block_id: usize, buf: &mut [u8]);
    ///Write data from buffer to block
    fn write_block(&self, block_id: usize, buf: &[u8]);
}

pub struct VirtIOBlock(Mutex<VirtIOBlk<'static, VirtioHal>>);


impl BlockDevice for VirtIOBlock {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) {
        self.0
            .lock()
            .read_block(block_id, buf)
            .expect("Error when reading VirtIOBlk");
    }
    fn write_block(&self, block_id: usize, buf: &[u8]) {
        self.0
            .lock()
            .write_block(block_id, buf)
            .expect("Error when writing VirtIOBlk");
    }
}

impl VirtIOBlock {
    #[allow(unused)]
    /// Create a new VirtIOBlock driver with VIRTIO0 base_addr for virtio_blk device
    pub fn new() -> Self {
        let mapped_mmio = phys_to_virt(VIRT_MMIO_BASE);
        println!("mapped_mmio2 = {:#x}",mapped_mmio);
        unsafe {
            Self(Mutex::new(
                VirtIOBlk::<VirtioHal>::new(&mut *(mapped_mmio as *mut VirtIOHeader)).unwrap(),
            ))
        }
    }
}

pub struct VirtioHal;

impl Hal for VirtioHal {
    fn dma_alloc(pages: usize) -> usize {
        // 申请一段连续的物理内存空间
        println!("alloc pages = {}",pages);
        if pages == 0 {panic!("pages == 0");}
        let start = PhysFrame::alloc().unwrap();
        let start_physaddr = start.start_paddr().as_usize();

        let mut prev_physaddr = start_physaddr;
        for _ in 1..pages {
            let current = PhysFrame::alloc().unwrap();
            let current_physaddr = current.start_paddr().as_usize();
            assert!(current_physaddr - prev_physaddr == PAGE_SIZE);
            prev_physaddr = current_physaddr;
        }
        start_physaddr

    }

    fn dma_dealloc(pa: usize, pages: usize) -> i32 {
        let start_physaddr = pa;
        for i in 0..pages {
            let frame = PhysFrame::from(PhysAddr::new(start_physaddr+i*PAGE_SIZE));
            
        }
        0
    }

    fn phys_to_virt(addr: usize) -> usize {
        phys_to_virt(addr)
    }

    fn virt_to_phys(vaddr: usize) -> usize {
        virt_to_phys(vaddr)
    }
}


const VIRT_MMIO_BASE: usize = 0xfef0_0000;


pub fn init ()
{
    println!("virtio guest init!");
    let mapped_mmio = phys_to_virt(VIRT_MMIO_BASE);
    println!("mapped_mmio = {:#x}",mapped_mmio);
    // let v = unsafe{&mut *(mapped_mmio as *mut VirtIOHeader)};

    // v.verify();
    // v.begin_init(|f| f )

    let dev = Arc::new(VirtIOBlock::new());

    let buf = &mut [0; 512];
    dev.read_block(1, buf);
    println!("buf = {:#x}",buf[0]);
    buf[0] = 2;
    dev.write_block(1, buf);

    println!("read ok!");

}
