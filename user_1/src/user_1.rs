#![no_std]
#![no_main]

use sys_call as sys;
use user_shared::{macros::sys_print, sys_call::sys_ipc_send, traits::Print, *};

#[no_mangle]
extern "C" fn main() {
    let char = 'y';
    write_ipc(char as usize);
    sys_ipc_send(1, 8);
    write_ipc('e' as usize);
    sys_ipc_send(1, 8);
    write_ipc('s' as usize);
    sys_ipc_send(1, 8);
    sys::exit();
}
