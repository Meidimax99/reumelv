#![no_std]
#![no_main]
use sys_call as sys;
use user_shared::{sys_call::sys_ipc_receive, traits::Print, *};

#[no_mangle]
extern "C" fn main() {
    let msg: char;
    msg = sys_ipc_receive() as u8 as char;
    msg.print();
    sys::exit();
}
