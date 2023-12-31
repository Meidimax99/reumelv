#![no_std]
#![no_main]

use user_shared::{message::Message, sys_call::sys_ipc_send};

#[no_mangle]
extern "C" fn main() {
    loop {
        let msg = "Tschüss!\n";
        let msg = Message::from_generic(msg);
        msg.write();
        sys_ipc_send(2);
    }
}
