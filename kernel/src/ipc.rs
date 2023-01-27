use crate::hardware::binary_struct::*;
use crate::hardware::{
    binary_struct::{self, BinaryOperations},
    memory_mapping::MemoryMapping,
};
use crate::sys::{process::Proc as Prog, state::Reason};
use crate::{
    hardware::uart::print_char,
    macros::print,
    scheduler::{self, *},
};
static mut ipc: [u64; 64] = [0; 64];

// every word is received Process

// lock setzen
pub fn set_ipc_lock(prog: Prog, receiver: usize) {
    prog.set_blocked(Reason::IPC, receiver); // receiver is a Prog ID
}
// lock auflösen
pub fn clear_ipc_lock(sending_Prog: Prog, receiving_Prog: Prog) {
    if sending_Prog.is_blocked_of(Reason::IPC, receiving_Prog.id() as usize)
        == (true, receiving_Prog.id() as usize)
    {
        // if the received Prog read the message, the sending Prog change to ready
        sending_Prog.set_rdy();
        // bit oben lösen TODO
    }
}
// send
pub fn send(sending_Prog: Prog, receiving_Prog: Prog, lenght: usize) {
    unsafe {
        set_ipc_lock(sending_Prog, receiving_Prog.id() as usize);
        let mut word: BinaryStruct<u64> = BinaryStruct::from(ipc[receiving_Prog.id() as usize]); // take old receiver word
        BinaryStruct::at(&mut word, receiving_Prog.id() as usize, true); // set new sending bit
        ipc[receiving_Prog.id() as usize] = word.get() as u64;
    }
}
// receive
pub fn receive() {}

pub fn copy_stack(process: Prog, length: usize) -> [usize; 20] {
    let context_end = process._sp();
    let mut stack_information: [usize; 20] = [0; 20];
    for i in 0..20 {
        unsafe {
            stack_information[i] = MemoryMapping::new(context_end - 5 + i).read();
        }
    }
    return stack_information;
}

pub fn print_msg(sender_prog: Prog, length: usize) {
    // s0 = sp+6*8
    let sender_sp = sender_prog._sp();
    let sender_s0 = sender_sp + 6 * 8;
    let mut tmp: char;

    for i in 0..length {
        unsafe {
            tmp = MemoryMapping::new(sender_s0 + i).read();
            print_char(tmp);
        }
    }
}
