#![no_std]
#![no_main]

use user_shared::{macros::sys_print, traits::Print, *};

#[no_mangle]
extern "C" fn main() {
    for i in 0..1000000 {
        if i % 999999 == 0 {
            println!("This is a new process");
            println!("this process is vewy fancy UwU łeđð");
        }
    }
    sys_call::exit();
}
