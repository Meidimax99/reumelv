use riscv_utils::*;

use super::{scheduler::*, state::*};

//saves the sp and pc of the current program so we continue executing where we left of when switching back
pub fn save_cur_prog(mepc: usize, sp: usize) {
    unsafe {
        if mepc < 0x80100000usize {
            let mcause: usize;
            read_machine_reg!("mcause" => mcause);

            panic!("Interrupt in exception, mepc: {}, mcause: {}", mepc, mcause);
        }
        let prog = cur().get();
        prog.mepc = mepc;
        prog.sp = sp;
    }
}
/// Returns the stack pointer to restore it.
pub fn restore_cur_prog() -> usize {
    unsafe {
        let prog = cur().get();
        if prog.state == State::Rdy {
            write_machine_reg!(prog.mepc => "mepc");
            return prog.sp;
        }
        panic!(
            "Tried to restore user prog: {:?}, with state: {:?}",
            prog.id, prog.state
        );
    }
}
