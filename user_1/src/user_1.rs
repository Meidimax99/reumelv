#![no_std]
#![no_main]

use user_shared::{
    message::*,
    sys_call::{exit, sys_ipc_receive_all, sys_ipc_send, sys_yield},
    traits::Print,
};

const OUT_FMT: &str = "\n[Process 0] ";
const REC: &str = "Receive:\t";
const SND: &str = "Send:\t";

#[no_mangle]
extern "C" fn main() {
    let mut value = 0;

    loop {
        value = value + 1;
        let msg = Message::from_generic(value);
        msg.write();

        OUT_FMT.print();
        SND.print();
        value.print();

        sys_ipc_send(1);
        unsafe {
            value = sys_ipc_receive_all(1).content;
        }

        OUT_FMT.print();
        REC.print();
        value.print();
    }
}
