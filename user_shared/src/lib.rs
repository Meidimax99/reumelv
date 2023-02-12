#![no_std]
#![allow(dead_code)]

use riscv_utils;

pub mod asm;
pub mod panic_handler;
pub mod sys_call;
pub mod traits;
#[macro_use]
pub mod macros;

pub fn write_ipc(information: usize) {
    // we want to write in s0
    unsafe {
        riscv_utils::write_function_reg!(
            information => "s1"
        );
    }
}

pub fn get_length() {
    // TODO
}
