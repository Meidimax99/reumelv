#![no_std]
#![no_main]

use user_shared::{
    macros::sys_print as print,
    message::*,
    sys_call::{sys_ipc_receive_any, sys_ipc_send},
    traits::Print,
};

#[no_mangle]
extern "C" fn main() {
    loop {
        let (pid, msg) = sys_ipc_receive_any();
        unsafe {
            let content: &str = msg.content;
            print!(content);
        }
    }
}
