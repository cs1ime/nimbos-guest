use super::time::TimeSpec;
use crate::arch::TrapFrame;
use crate::mm::{UserInPtr, UserOutPtr};
use crate::task::{current, spawn_task};

const MAX_STR_LEN: usize = 256;

pub fn sys_exit(exit_code: i32) -> ! {
    current().exit(exit_code);
}

pub fn sys_yield() -> isize {
    current().yield_now();
    0
}

pub fn sys_getpid() -> isize {
    current().pid().as_usize() as isize
}

pub fn sys_clone(newsp: usize, tf: &TrapFrame) -> isize {
    let new_task = current().new_clone(newsp, tf);
    let pid = new_task.pid().as_usize() as isize;
    spawn_task(new_task);
    pid
}

pub fn sys_fork(tf: &TrapFrame) -> isize {
    let new_task = current().new_fork(tf);
    let pid = new_task.pid().as_usize() as isize;
    spawn_task(new_task);
    pid
}

pub fn sys_exec(path: UserInPtr<u8>, tf: &mut TrapFrame) -> isize {
    let (path_buf, len) = path.read_str::<MAX_STR_LEN>();
    let path = core::str::from_utf8(&path_buf[..len]).unwrap();
    current().exec(path, tf)
}

/// If there is no child process has the same pid as the given, return -1.
/// Else if there is a child process but it is still running, return -2.
pub fn sys_waitpid(pid: isize, mut exit_code_ptr: UserOutPtr<i32>) -> isize {
    let mut exit_code = 0;
    let ret = current().waitpid(pid, &mut exit_code);
    exit_code_ptr.write(exit_code);
    ret
}

pub fn sys_nanosleep(req: UserInPtr<TimeSpec>) -> isize {
    let deadline = crate::timer::current_time() + req.read().into();
    current().sleep(deadline);
    0
}
