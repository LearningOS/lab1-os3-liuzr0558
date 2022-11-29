//! Types related to task management

use super::TaskContext;
use crate::config::SUPPORTED_SYSCALL_NUM;

#[derive(Copy, Clone)]
/// task control block structure
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    pub syscall_times: [(usize, usize); SUPPORTED_SYSCALL_NUM],
    pub start_time: Option<usize>,
    // LAB1: Add whatever you need about the Task.
}

pub struct KernelTaskInfo{
    pub status: TaskStatus,
    pub syscall_times: [(usize, usize); SUPPORTED_SYSCALL_NUM],
    pub running_time: usize,
}


#[derive(Copy, Clone, PartialEq)]
/// task status: UnInit, Ready, Running, Exited
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}
