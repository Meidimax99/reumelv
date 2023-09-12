#![no_std]
#![allow(dead_code)]
mod reg;
mod sys_call;
pub use reg::*;
pub use sys_call::SysCall;

#[macro_export]
macro_rules! read_machine_reg {
    ($($register:literal => $data:ident), +) => {
        core::arch::asm!(
            $(concat!("csrr ", "{", stringify!($data), "}, ", $register)), +,
            $($data = out(reg) $data), +
        )
    }
}
#[macro_export]
macro_rules! write_machine_reg {
    ($($data:ident => $register:literal), +) => {
        $(let $data: usize = $data;) +
        core::arch::asm!(
            $(concat!("csrw ", $register, ", {}")), +,
            $(in(reg) $data), +
        )
    };
    ($data:expr => $register:literal) => {
        let data: usize = $data;
        core::arch::asm!(concat!("csrw ", $register, ", {}"), in(reg) data)
    };
}
///!!!ALWAYS read in descending register order!!!
#[macro_export]
macro_rules! read_function_reg {
    ($($register:literal => $data:ident), +) => {
        core::arch::asm!(
            $(concat!("mv ", "{}, ", $register)), +,
            $(out(reg) $data), +
        )
    }
}
#[macro_export]
///!!!ALWAYS write in function parameter order!!!
macro_rules! write_function_reg {
    ($($data:ident => $register:literal), +) => {
        core::arch::asm!(
            $(concat!("mv ", $register, ", {}")), +,
            $(in(reg) $data), +
        )
    };
    ($data:expr => $register:literal) => {
        let data: u64 = $data;
        core::arch::asm!(concat!("mv ", $register, ", {}"), in(reg) data)
    };
}
