use crate::{
    hardware::{binary_struct::BinaryStruct, memory_mapping::MemoryMapping, stack::Stack},
    scheduler::*,
};
static mut ipc: [u64; 64] = [0; 64];

// every word is received Process

// IPC Table

static mut ipc_sending_table: [u64; 64] = [0; 64];

// lock setzen
pub fn set_sending_ipc_lock(sending_Prog: Prog, receiving_Prog: Prog) {
    unsafe {
        // setzen in unserer table unser bit auf den receiver Process
        let mut word = BinaryStruct::from(ipc_sending_table[receiving_Prog.id() as usize]);
        word.at(sending_Prog.id() as usize, true);
        ipc_sending_table[receiving_Prog.id() as usize] = word.get();
        sending_Prog.set_blocked(Reason::SendingIpc, receiving_Prog.id() as usize);
    }
}

pub fn set_receiver_ipc_lock(receiving_Prog: Prog, sending_Prog: Prog) {
    receiving_Prog.set_blocked(Reason::ReceiveIpc, sending_Prog.id() as usize);
}

// lock auflösen
pub fn clear_ipc_lock(sending_Prog: Prog, receiving_Prog: Prog, reason: Reason) {
    unsafe {
        let word = ipc_sending_table[receiving_Prog.id() as usize];
        let mut bit = BinaryStruct::from(word);
        if bit.is_set(sending_Prog.id() as usize) {
            if receiving_Prog
                .is_blocked_of(Reason::ReceiveIpc, sending_Prog.id() as usize)
                .0
            {
                stack_copy();
                bit.at(sending_Prog.id() as usize, false);
                ipc_sending_table[receiving_Prog.id() as usize] = bit.get();
                sending_Prog.set_rdy();
                receiving_Prog.set_rdy();
            }
        }
    }
}

pub fn stack_copy() {
    // TODO
}

// send
pub fn send(sending_Prog: Prog, receiving_Prog: Prog, lenght: usize) {
    unsafe {
        let cur_stack: Stack = Stack::new(sending_Prog._sp());
        let msg = cur_stack.s0();

        /*

        if ipc_receiving_table[receiving_Prog.id() as usize] == 0{
            // nothing is set so the sending_Prog must wait

        }else {
            // if the word is not 0, we must look if the
        }
        // look if receiving Prog want something

        */

        // write it back
        /*
        let mut next_stack: Stack = Stack::new(receiving_Prog._sp());

        next_stack.write_s0(msg);
        next_stack.write();
        */

        unsafe {
            let word = ipc_sending_table[receiving_Prog.id() as usize];
            let mut bit = BinaryStruct::from(word);
            if receiving_Prog
                .is_blocked_of(Reason::ReceiveIpc, sending_Prog.id() as usize)
                .0
            {
                // sending after receive
                sending_Prog.is_blocked(Reason::SendingIpc, receiving_Prog.id() as usize);
                clear_ipc_lock(sending_Prog, receiving_Prog, Reason::ReceiveIpc);
            } else {
                // sending for receive
                sending_Prog.is_blocked(Reason::SendingIpc, receiving_Prog.id() as usize);
            }
        }
    }
}

// receive
pub fn receive(receiving_Prog: Prog, sending_Prog: Prog) {
    unsafe {
        let word = ipc_sending_table[receiving_Prog.id() as usize];
        let mut bit = BinaryStruct::from(word);
        if bit.is_set(sending_Prog.id() as usize) {
            // receive after sending
            receiving_Prog.is_blocked(Reason::ReceiveIpc, sending_Prog.id() as usize);
            clear_ipc_lock(sending_Prog, receiving_Prog, Reason::ReceiveIpc);
        } else {
            // receive for sending
            receiving_Prog.is_blocked(Reason::ReceiveIpc, sending_Prog.id() as usize);
        }
    }
}
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
