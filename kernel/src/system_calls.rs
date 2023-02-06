use crate::hardware::{
    memory_mapping::MemoryMapping,
    uart::{self},
};
use crate::ipc::*;
use crate::scheduler::{self, get_process, Prog};
use crate::{ipc::set_sending_ipc_block, scheduler::*};
pub use core::arch::asm;
use core::ops::Add;
use riscv_utils::*;

fn syscall_from(number: usize) -> SysCall {
    crate::enum_matching!(
        number: SysCall::GetChar,
        SysCall::Print,
        SysCall::Yield,
        SysCall::Exit,
        SysCall::TaskNew,
        SysCall::LthreadExRegs,
        SysCall::IpcSend,
        SysCall::IpcReceiver
    );
    panic!("Illegal syscall: {}", number);
}

pub unsafe fn syscall(number: usize, _param_0: usize, _param_1: usize) -> Option<usize> {
    match syscall_from(number) {
        SysCall::GetChar => {
            let char = sys_get_char();
            scheduler::cur().increment_mepc();
            return char;
        }
        SysCall::Print => {
            sys_print_string(_param_0, _param_1);
            scheduler::cur().increment_mepc();
            return None;
        }
        SysCall::Exit => {
            exit();
            return None;
        }
        SysCall::Yield => {
            scheduler::cur().increment_mepc();
            sys_yield();
            return None;
        }
        SysCall::TaskNew => {
            scheduler::cur().increment_mepc();
            return task_new(_param_0);
        }
        SysCall::LthreadExRegs => {
            //TODO syscall for creating and switching between threads, needs valid ip and sp, returns current thread id
            scheduler::cur().increment_mepc();
            return None;
        }
        SysCall::IpcSend => {
            sys_ipc_send(_param_0, _param_1);
            scheduler::cur().increment_mepc();
            return None;
        }
        SysCall::IpcReceiver => {
            sys_ipc_receive(_param_0, _param_1);
            scheduler::cur().increment_mepc();
            return None;
        }
    }
}

unsafe fn exit() {
    scheduler::end_prog(scheduler::cur());
    sys_yield();
}

fn task_new(mepc: usize) -> Option<usize> {
    return Some(scheduler::task_new(mepc));
}
unsafe fn sys_get_char() -> Option<usize> {
    return Some(uart::read_char() as usize);
}

pub unsafe fn sys_print_string(str_ptr: usize, size: usize) {
    // cast to u8 to increment Option<usize> to char pointer
    let mut str_ptr = str_ptr.clone();
    for _ in 0..size {
        // Read value from the pointer with MemoryMapping
        let char = MemoryMapping::<char>::new(str_ptr as usize).read();
        uart::print_char(char);
        str_ptr = str_ptr.add(1);
    }
}

unsafe fn sys_yield() {
    scheduler();
}

unsafe fn sys_ipc_send(receiver_id: usize, _length: usize) {
    let receiver_prog: Prog = get_process(receiver_id);
    let sender_prog: Prog = cur();
    set_sending_ipc_block(sender_prog, receiver_id);
    try_exchange(sender_prog, receiver_prog);
}

unsafe fn sys_ipc_receive(sender_id: usize, _length: usize) {
    let sender_prog: Prog = get_process(sender_id);
    let receiver_prog: Prog = cur();
    set_receiver_ipc_block(receiver_prog, sender_id);
    try_exchange(sender_prog, receiver_prog);
}
