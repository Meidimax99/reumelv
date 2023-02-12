#![no_std]
#![no_main]
use sys_call as sys;
use user_shared::{
    sys_call::{self, *},
    traits::Print,
};

#[no_mangle]
extern "C" fn main() {
    // let msg: char;
    // msg = sys_ipc_receive_all(0, 8) as u8 as char;
    // msg.print();
    // let msg2: char;
    // msg2 = sys_ipc_receive_all(0, 8) as u8 as char;
    // msg2.print();
    loop{
    let msg3: char;
    msg3 = sys_ipc_receive_all(0, 8) as u8 as char;
    msg3.print();
    //for i in 0..100000000{}
    }

    sys::exit();
}
