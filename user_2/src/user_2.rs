#![no_std]
#![no_main]
use sys_call as sys;
use user_shared::{
    message::*,
    sys_call::{self, *},
    traits::Print,
};

const OUT_FMT: &str = "\n[Process 1] ";
const REC: &str = "Receive:\t";
const SND: &str = "Send:\t";

#[no_mangle]
extern "C" fn main() {
    /* loop {
        let mut value: usize;
        unsafe {
            value = sys_ipc_receive_all(0).content;
            OUT_FMT.print();
            REC.print();
            value.print();
        }
        value = value + 1;
        let msg = Message::from_generic(value);
        msg.write();

        OUT_FMT.print();
        SND.print();
        value.print();

        sys_ipc_send(0);
    } */
    
    let str: Message<&str> = sys_ipc_receive(0);
    unsafe {
        str.content.print();
    }
}
