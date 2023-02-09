#![no_std]
#![no_main]

use sys_call as sys;
use user_shared::{macros::sys_print as print, traits::Print, *};

#[no_mangle]
extern "C" fn main() {
    loop {
        for i in 0..1000000{
            if i == 999999 {
                let string: &str = "\nUser 1\n";
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
                print!('c');
                sys::task_new(0x80200000);
                print!("Returned\n");
            }
        }
    }
}
