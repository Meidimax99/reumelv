use crate::{
    hardware::{binary_struct::BinaryStruct, stack_image},
    macros::log,
    queue::*,
    sys::{process::Proc, scheduler, state::Reason},
};
// every word is received Process
// IPC Table
static mut IPC_TABLE: [u64; 64] = [0; 64];
static mut MSG_QUEUES: [ByteQueue<64usize>; 64] = [ByteQueue::new(); 64];

pub unsafe fn try_exchange_any(receiving_prog: Proc) -> Option<usize> {
    let receiving_id = receiving_prog.id() as usize;
    if IPC_TABLE[receiving_id] == 0 {
        scheduler::schedule();
        // the receiver expected nothing
        return None;
    }
    if let Ok(id) = MSG_QUEUES[receiving_id].pop() {
        try_exchange(scheduler::get_process(id as usize), receiving_prog);
        return Some(id as usize);
    }
    panic!("ERROR in try_exchange_any!");
}

pub unsafe fn try_exchange(sending_prog: Proc, receiving_prog: Proc) {
    let sender_id = sending_prog.id() as usize;
    let receiver_id = receiving_prog.id() as usize;
    // word is one line from the ipc_table
    let word = IPC_TABLE[receiving_prog.id() as usize];
    // bit is a binary struct from word
    let bit = BinaryStruct::from(word);
    // is the sending process and the receiver process valid

    let sending_block = sending_prog.is_blocked_of(Reason::SendingIpc, receiver_id);
    let receiving_block = receiving_prog.is_blocked_of(Reason::ReceiveIpc, sender_id);
    let receiving_block_all = receiving_prog.is_blocked(Reason::ReceiveIpcAll);
    let sender_bit_set = bit.is_set(sending_prog.id() as usize); //sender bit necessary if all the information is already in State? Maybe only necessary for receive

    //TODO sending_block && (receiving_block ||receiving_block_all ) &&sender_bit_set
    if sending_block && sender_bit_set && (receiving_block || receiving_block_all) {
        log!(
            "Exchange Message from {sender} to {receiver}!",
            sender = sender_id,
            receiver = receiver_id
        );
        send_ipc(sending_prog, receiving_prog);
        clear_ipc_block(sending_prog, receiving_prog);
    } else {
        scheduler::schedule();
    }
}

unsafe fn send_ipc(sending_prog: Proc, receiving_prog: Proc) {
    // read the message from the sending Process and write it to the receiver Process
    let snd_sp = sending_prog._sp();
    let rcv_sp = receiving_prog._sp();
    stack_image::copy_ipc_regs_img(snd_sp, rcv_sp);
}

pub fn set_sending_ipc_block(sending_prog: Proc, receiving_id: usize) {
    unsafe {
        // set our bit to the receiver process in our table
        let mut word = BinaryStruct::from(IPC_TABLE[receiving_id]);
        word.at(sending_prog.id() as usize, true);
        IPC_TABLE[receiving_id] = word.get();
        // set the sending process to blocked
        sending_prog.set_blocked(Reason::SendingIpc, receiving_id);
        MSG_QUEUES[receiving_id].push(sending_prog.id as u8);
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
