#![no_std]
#![no_main]

use sys_call as sys;
use user_shared::{sys_call::sys_ipc_send, *};

#[no_mangle]
extern "C" fn main() {
    let char = 'y';
    write_ipc(char as usize);
    sys_ipc_send(1, 8);
    sys::exit();
}
