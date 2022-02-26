use core::arch::{asm, global_asm};
use core::ops::Deref;
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::config::{APP_BASE_ADDRESS, KERNEL_STACK_SIZE, USER_STACK_SIZE};
use crate::loader;
use crate::trap::TrapContext;

global_asm!(include_str!("link_app.S"));

#[repr(align(4096))]
pub struct PageAligned<T>(T);

impl<T> Deref for PageAligned<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

static KERNEL_STACK: PageAligned<[u8; KERNEL_STACK_SIZE]> = PageAligned([0; KERNEL_STACK_SIZE]);
static USER_STACK: PageAligned<[u8; USER_STACK_SIZE]> = PageAligned([0; USER_STACK_SIZE]);

struct AppManager {
    current_app: AtomicUsize,
}

impl AppManager {
    pub const fn new() -> Self {
        Self {
            current_app: AtomicUsize::new(0),
        }
    }

    pub fn print_app_info(&self) {
        let app_count = loader::get_app_count();
        println!("[kernel] app_count = {}", app_count);
        for i in 0..app_count {
            let app_data = loader::get_app_data(i);
            println!(
                "[kernel] app_{} [{:#x}, {:#x})",
                i,
                app_data.as_ptr_range().start as usize,
                app_data.as_ptr_range().end as usize,
            );
        }
    }

    pub unsafe fn load_next_app(&self) {
        let app_id = self.current_app.fetch_add(1, Ordering::SeqCst);
        if app_id >= loader::get_app_count() {
            panic!("All applications completed!");
        }
        println!("[kernel] Loading app_{}", app_id);
        // clear app area
        let app_data = loader::get_app_data(app_id);
        let app_dst = core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_data.len());
        app_dst.copy_from_slice(app_data);
        // clear icache
        asm!("ic iallu; dsb sy; isb");
    }
}

static APP_MANAGER: AppManager = AppManager::new();

pub fn init() {
    APP_MANAGER.print_app_info();
}

pub fn run_next_app() -> ! {
    unsafe { APP_MANAGER.load_next_app() };
    let context =
        TrapContext::app_init_context(APP_BASE_ADDRESS, USER_STACK.as_ptr_range().end as _);
    unsafe { context.exec(KERNEL_STACK.as_ptr_range().end as _) };
}
