use crate::ipc::*;
use crate::macros::log;
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
            log!(
                "\n{string:<15}GetChar from Process {id}!",
                string = "[Sys_Calls]",
                id = scheduler::cur().id
            );
            let char = sys_get_char();
            scheduler::cur().increment_mepc();
            char
        }
        SysCall::Print => {
            log!(
                "\n{string:<15}Print from Process {id}!",
                string = "[Sys_Calls]",
                id = scheduler::cur().id
            );
            sys_print_string(_param_0, _param_1);
            scheduler::cur().increment_mepc();
            None
        }
        SysCall::Exit => {
            log!(
                "\n{string:<15}Exit from Process {id}!",
                string = "[Sys_Calls]",
                id = scheduler::cur().id
            );
            exit();
            None
        }
        SysCall::Yield => {
            log!(
                "\n{string:<15}Yield from Process {id}!",
                string = "[Sys_Calls]",
                id = scheduler::cur().id
            );
            scheduler::cur().increment_mepc();
            scheduler::schedule();
            None
        }
        SysCall::TaskNew => {
            log!(
                "\n{string:<15}TaskNew from Process {id}!",
                string = "[Sys_Calls]",
                id = scheduler::cur().id
            );
            scheduler::cur().increment_mepc();
            task_new(_param_0)
        }
        SysCall::IpcSend => {
            log!(
                "\n{string:<15}IpcSend from Process {id}!",
                string = "[Sys_Calls]",
                id = scheduler::cur().id
            );
            scheduler::cur().increment_mepc();
            sys_ipc_send(_param_0);
            None
        }
        SysCall::IpcReceiver => {
            log!(
                "\n{string:<15}IpcReceive from Process {id}!",
                string = "[Sys_Calls]",
                id = scheduler::cur().id
            );
            scheduler::cur().increment_mepc();
            sys_ipc_receive(_param_0);
            None
        }
        SysCall::IpcReceiverAll => {
            log!(
                "\n{string:<15}IpcReceiveAll from Process {id}!",
                string = "[Sys_Calls]",
                id = scheduler::cur().id
            );
            scheduler::cur().increment_mepc();
            sys_ipc_receive_any()
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

unsafe fn sys_ipc_receive(sender_id: usize) {
    let sender_prog: Proc = get_process(sender_id);
    let receiver_prog: Proc = cur();
    set_receiver_ipc_block(receiver_prog, sender_id);
    try_exchange(sender_prog, receiver_prog);
}

unsafe fn sys_ipc_receive_any() -> Option<usize> {
    let receiver_prog: Proc = cur();
    set_receiver_ipc_block_all(receiver_prog);
    try_exchange_any(receiver_prog)
}
