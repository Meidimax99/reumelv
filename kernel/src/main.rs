#![no_std]
#![no_main]

mod asm;
mod exception_handler;
mod hardware;
mod ipc;
mod macros;
mod panic_handler;
mod queue;
mod setup;
mod sys;
mod system_calls;

pub(crate) use macros::*;
use sys::scheduler::{self, start_tau};

fn _shutdown() {}

#[no_mangle]
unsafe extern "C" fn kernel_setup() {
    setup::setup();
    start_tau();
}
