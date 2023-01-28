#![no_std]
#![no_main]

use sys_call as sys;
use user_shared::{macros::sys_print, sys_call::sys_ipc_send, traits::Print, *};

#[no_mangle]
extern "C" fn main() {
    /*let string: &str = "\nUser 1\n";
    let number: usize = 1234567890;
    "\n".print();
    number.print();
    "\n".print();
    string.print();
    "\nUser 1 ist fertig\n".print();
    sys_print!("Macro");
    sys_print!("786");
    "\n".print();
    println!("Hello World");
    println!(1024);
    print!("Finishing\n");
    print!('c');*/
    let char = 'y';
    write_ipc(char as usize);
    sys_ipc_send(1, 8);
    sys::exit();
}
