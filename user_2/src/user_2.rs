#![no_std]
#![no_main]
use user_shared::{message::*, sys_call::*, sys_print, traits::Print};

#[no_mangle]
extern "C" fn main() {
    loop {
        let mut value: usize;
        unsafe {
            "\n[Process 1]    Start receiving!".print();
            value = sys_ipc_receive_all(0).content;
            sys_print!("\n[Process 1]    Receive value: ", value);
            "\n[Process 1]    End receiving!".print();
        }
        value += 1;
        let msg = Message::from_generic(value);
        msg.write();

        "\n[Process 1]    Start sending!".print();
        sys_ipc_send(0);
        //here
        "\n[Process 1]    Finish sending!".print();
    }
    /* loop {
        "2\n".print();
    } */
}
