use crate::{
    hardware::{binary_struct::BinaryStruct, pcb::*},
    sys::{process::Proc, scheduler, state::Reason},
};
// every word is received Process
// IPC Table
static mut IPC_TABLE: [u64; 64] = [0; 64];

pub unsafe fn try_exchange_all(receiving_prog: Proc) {
    let receiving_id = receiving_prog.id() as usize;
    if IPC_TABLE[receiving_id] == 0 as u64 {
        scheduler::_yield();
        // the receiver expected nothing
        return;
    }
    // we must find who the sender process is
    let word = IPC_TABLE[receiving_id];
    let bit = BinaryStruct::from(word);
    for id in 0..63 {
        if bit.is_set(id) {
            try_exchange(scheduler::get_process(id), receiving_prog);
            return;
        }
    }
    panic!("ERROR in try_exchange_all!");
}

pub unsafe fn try_exchange(sending_prog: Proc, receiving_prog: Proc) {
    let sender_id = sending_prog.id() as usize;
    let receiver_id = receiving_prog.id() as usize;
    // word is one line from the ipc_table
    let word = IPC_TABLE[receiving_prog.id() as usize];
    // bit is a binary struct from word
    let bit = BinaryStruct::from(word);
    // is the sending process and the receiver process valid

    let sending_block = sending_prog
        .is_blocked_of(Reason::SendingIpc, receiver_id)
        .0;
    let receiving_block = receiving_prog
        .is_blocked_of(Reason::ReceiveIpc, sender_id)
        .0;
    let receiving_block_all = receiving_prog.is_blocked_of(Reason::ReceiveIpcAll, 0).0;
    let sender_bit_set = bit.is_set(sending_prog.id() as usize);

    if (sending_block && receiving_block && sender_bit_set)
        || (sending_block && receiving_block_all && sender_bit_set)
    {
        send_ipc(sending_prog, receiving_prog);
        clear_ipc_block(sending_prog, receiving_prog);
    } else {
        scheduler::_yield();
    }
}

unsafe fn send_ipc(sending_prog: Proc, receiving_prog: Proc) {
    // read the message from the sending Process and write it to the receiver Process
    let sending_stack = PCB::new(sending_prog._sp());
    let msg = sending_stack.get(Register::s0);
    let mut receive_stack = PCB::new(receiving_prog._sp());
    receive_stack.set(Register::s0, msg);
    receive_stack.write();
}

pub fn set_sending_ipc_block(sending_prog: Proc, receiving_id: usize) {
    unsafe {
        // set our bit to the receiver process in our table
        let mut word = BinaryStruct::from(IPC_TABLE[receiving_id]);
        word.at(sending_prog.id() as usize, true);
        IPC_TABLE[receiving_id] = word.get();
        // set the sending process to blocked
        sending_prog.set_blocked(Reason::SendingIpc, receiving_id);
    }
}

pub fn set_receiver_ipc_block(receiving_prog: Proc, sender_id: usize) {
    receiving_prog.set_blocked(Reason::ReceiveIpc, sender_id);
}

pub fn set_receiver_ipc_block_all(receiving_prog: Proc) {
    receiving_prog.set_blocked(Reason::ReceiveIpcAll, 0);
}

unsafe fn clear_ipc_block(sending_prog: Proc, receiving_prog: Proc) {
    let word = IPC_TABLE[receiving_prog.id() as usize];
    let mut bit = BinaryStruct::from(word);
    bit.at(sending_prog.id() as usize, false);
    IPC_TABLE[receiving_prog.id() as usize] = bit.get();

    receiving_prog.set_rdy();
    sending_prog.set_rdy();
}
