#![no_std]
#![no_main]

use user_shared::{
    message::*,
    sys_call::{sys_ipc_receive_all, sys_ipc_send},
    sys_print,
    traits::Print,
};

const OUT_FMT: &str = "\n[Process 0]    ";
const REC: &str = "Receive:\t";
const SND: &str = "Send:\t";

#[no_mangle]
extern "C" fn main() {
    let mut value = 0;

    loop {
        value = value + 1;
        let msg = Message::from_generic(value);
        msg.write();

        "\n[Process 0]    Start sending!".print();

        sys_ipc_send(1);
        "\n[Process 0]    Finish sending!".print();
        unsafe {
            "\n[Process 0]    Start receiving!".print();
            value = sys_ipc_receive_all(1).content;
            sys_print!("\n[Process 0]    Receive value: ", value);

            "\n[Process 0]    Finish receiving!".print();
        }

        //Timer Interrupt
    }

    /* loop {
        "1\n".print();
    } */
}
