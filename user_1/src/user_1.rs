#![no_std]
#![no_main]

use sys_call as sys;
use user_shared::{macros::sys_print as print, traits::Print, *};

#[no_mangle]
extern "C" fn main() {
    let string: &str = "\nUser 1\n";
    let number: usize = 1234567890;
    "\n".print();
    number.print();
    "\n".print();
    string.print();
    "User 1 ist fertig\n".print();

    print!("Macro ");
    print!("786");
    println!();
    "\n".print();
    println!("Hello World", "Is this printed");
    println!(1024);
    println!('c');
    println!(string.as_ptr());
    sys::exit();
}
