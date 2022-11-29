//! Process management syscalls

use crate::config::{MAX_SYSCALL_NUM};
use crate::task::{exit_current_and_run_next, get_current_task_info, suspend_current_and_run_next, TaskStatus};
use crate::timer::get_time_us;

pub use crate::task::increase_task_syscall_count;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub struct TaskInfo {
    pub status: TaskStatus,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    pub time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}


/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    let task_info = get_current_task_info();
    let mut syscall_times = [0u32; 500];

    for (call_id, call_times) in task_info.syscall_times{
        syscall_times[call_id] = call_times as u32;
    }

    unsafe {
        (*ti).time = task_info.running_time;
        (*ti).status = task_info.status;
        (*ti).syscall_times = syscall_times;
    }
    0
}
