use crate::ipc::*;
use crate::scheduler::{self};
use crate::{
    hardware::{
        memory_mapping::MemoryMapping,
        uart::{self},
    },
    sys::process::Proc,
};
use crate::{ipc::set_sending_ipc_block, scheduler::*};
pub use core::arch::asm;
use core::ops::Add;
use riscv_utils::*;

pub fn syscall_from(number: usize) -> SysCall {
    crate::enum_matching!(
        number: SysCall::GetChar,
        SysCall::Print,
        SysCall::Yield,
        SysCall::Exit,
        SysCall::TaskNew,
        SysCall::IpcSend,
        SysCall::IpcReceiver,
        SysCall::IpcReceiverAll
    );
    panic!("Illegal syscall: {}", number);
}

//TODO give whole Stack_Image to the syscall function??
pub unsafe fn syscall(number: usize, _param_0: usize, _param_1: usize) -> Option<usize> {
    match syscall_from(number) {
        SysCall::GetChar => {
            let char = sys_get_char();
            scheduler::cur().increment_mepc();
            char
        }
        SysCall::Print => {
            sys_print_string(_param_0, _param_1);
            scheduler::cur().increment_mepc();
            None
        }
        SysCall::Exit => {
            exit();
            None
        }
        SysCall::Yield => {
            scheduler::cur().increment_mepc();
            scheduler::schedule();
            None
        }
        SysCall::TaskNew => {
            scheduler::cur().increment_mepc();
            task_new(_param_0)
        }
        SysCall::IpcSend => {
            scheduler::cur().increment_mepc();
            sys_ipc_send(_param_0);
            None
        }
        SysCall::IpcReceiver => {
            scheduler::cur().increment_mepc();
            sys_ipc_receive(_param_0, _param_1);
            None
        }
        SysCall::IpcReceiverAll => {
            scheduler::cur().increment_mepc();
            sys_ipc_receive_all(_param_0);
            None
        }
    }
}

unsafe fn exit() {
    scheduler::end_prog(scheduler::cur());
    scheduler::schedule();
}

fn task_new(mepc: usize) -> Option<usize> {
    Some(scheduler::task_new(mepc))
}
unsafe fn sys_get_char() -> Option<usize> {
    Some(uart::read_char() as usize)
}

pub unsafe fn sys_print_string(str_ptr: usize, size: usize) {
    // cast to u8 to increment Option<usize> to char pointer
    let mut str_ptr = str_ptr;
    for _ in 0..size {
        // Read value from the pointer with MemoryMapping
        let char = MemoryMapping::<char>::new(str_ptr as usize).read();
        uart::print_char(char);
        str_ptr = str_ptr.add(1);
    }
}

unsafe fn sys_ipc_send(receiver_id: usize) {
    let receiver_prog: Proc = get_process(receiver_id);
    let sender_prog: Proc = cur();
    set_sending_ipc_block(sender_prog, receiver_id);
    try_exchange(sender_prog, receiver_prog);
}

unsafe fn sys_ipc_receive(sender_id: usize, _length: usize) {
    let sender_prog: Proc = get_process(sender_id);
    let receiver_prog: Proc = cur();
    set_receiver_ipc_block(receiver_prog, sender_id);
    try_exchange(sender_prog, receiver_prog);
}

unsafe fn sys_ipc_receive_all(_length: usize) {
    let receiver_prog: Proc = cur();
    set_receiver_ipc_block_all(receiver_prog);
    try_exchange_all(receiver_prog);
}
