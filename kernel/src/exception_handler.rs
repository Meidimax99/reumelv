use riscv_utils::read_machine_reg;

use crate::hardware::binary_struct::BinaryStruct;
use crate::hardware::clint;
use crate::hardware::plic;
use crate::hardware::stack_image::*;
use crate::hardware::uart;
use crate::macros::log;
use crate::sys::dispatcher;
use crate::sys::scheduler;
use crate::system_calls;
use riscv_utils::*;

#[no_mangle]
unsafe extern "C" fn exception_handler(mepc: usize, mcause: usize, sp: usize) -> usize {
    dispatcher::save_cur_prog(mepc, sp);
    let mut mcause = BinaryStruct::from(mcause);
    let interrupt = mcause.is_set(63);
    if interrupt {
        mcause.at(63, false);
        handle_interrupt(mcause.get());
    } else {
        handle_exception(mcause.get(), mepc, sp);
    }
    dispatcher::restore_cur_prog()
}

//https://people.eecs.berkeley.edu/~krste/papers/riscv-privileged-v1.9.pdf -- Page 34
unsafe fn handle_interrupt(mcause: usize) {
    match mcause {
        7 => {
            log!("\n{string:<15}Timer Interrupt!", string = "[Exc_Handler]");
            // Timer interrupt
            scheduler::schedule();
            clint::set_time_cmp();
        }
        11 => {
            // Extern interrupt
            let irq = plic::read_claim();
            match irq {
                plic::IRQ::Uart => {
                    log!("{}", uart::read_char());
                }
            }
            plic::write_complete(irq);
        }
        _ => {
            panic!("Unsupported interrupt with code: {}", mcause);
        }
    }
}

unsafe fn handle_exception(mcause: usize, mepc: usize, sp: usize) {
    match mcause {
        1 => {
            // Instruction access fault
            let mtval: usize;
            read_machine_reg!("mtval" => mtval);
            panic!(
                "Instruction access fault in user prog: {:?}, mepc: 0x{:x}, mtval: 0x{:x}",
                scheduler::cur().id(),
                mepc,
                mtval
            );
        }
        2 => {
            // Illegal instruction fault
            let mtval: usize;
            read_machine_reg!("mtval" => mtval);
            panic!(
                "Illegal instruction fault in user prog: {:?}, mepc: 0x{:x}, mtval: 0x{:x}",
                scheduler::cur().id(),
                mepc,
                mtval
            );
        }
        5 => {
            // Load access fault
            let mtval: usize;
            read_machine_reg!("mtval" => mtval);
            panic!(
                "Load access fault in user prog: {:?}, mepc: 0x{:x}, sp: 0x{:x}, mtval: 0x{:x}",
                scheduler::cur().id(),
                mepc,
                sp,
                mtval
            );
        }
        8 => {
            // Ecall from user-mode
            let mut image = Stack_Image::new(sp);
            let number = image.get(Register::a7);
            let param_0 = image.get(Register::a0);
            let param_1 = image.get(Register::a1);
            if let Some(ret) = system_calls::syscall(number, param_0, param_1) {
                write_function_reg!(ret => "a0");
            }
        }
        _ => {
            // Unsupported exception
            let mtval: usize;
            read_machine_reg!("mtval" => mtval);
            panic!(
                "Unsupported exception with code: {}, mepc: 0x{:x}, sp: 0x{:x}, mtval: 0x{:x}",
                mcause, mepc, sp, mtval
            );
        }
    }
}
