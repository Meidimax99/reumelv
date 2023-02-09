use super::{process::*, state::*};
use crate::hardware::{clint, pmp};
use riscv_utils;

static mut CUR_PROG_IDX: usize = 0;
static mut ID_SEED: u16 = 0xACE1;
const NONE: Option<ProcessData> = None;
pub static mut PROCS: [Option<ProcessData>; 64] = [NONE; 64];

unsafe fn boot_proc(proc: Proc) {
    let proc_data = proc.get();
    proc_data.state = State::Rdy;
    switch(proc);
    riscv_utils::write_machine_reg!(proc_data.init_proc_state.init_mepc => "mepc");
    crate::println!("\n\n## Starting {:?} ##", proc_data.id);
    clint::set_time_cmp();
    core::arch::asm!("mret");
}
pub unsafe fn end_prog(proc: Proc) {
    if cur().id == 0 {
        panic!("can't exit process tau");
    }
    PROCS[proc.idx] = Some(ProcessData::new(proc.id, InitProcState::new(proc.idx)));
}
// starts the first process which starts all others, never exits except when system is shutdown
pub unsafe fn start_tau() {
    init_procs();
    if let Some(proc) = &mut PROCS[0] {
        proc.init_proc_state.init_mepc = 0x80100000;
        proc.state = State::Starting;
    }
    boot_proc(cur());
}

//Fills the PROCS array with inactive programs
unsafe fn init_procs() {
    for idx in 0..PROCS.len() {
        PROCS[idx] = Some(ProcessData::new(idx, InitProcState::new(idx)));
    }
}
// creates a new process with the physical memory address of the first instruction of said process
// For example:
// Consider some inactive or not present process at address 0x80100000.
// Then we will generate a new processes with the given mepc and sets its state to starting.
// Finally, we boot the given process via [boot_proc]
// See also [get_inact_proc].
pub fn task_new(mepc: usize) -> usize {
    let info = get_inact_proc();
    if info == (0,0) {
        return 0;
    }
    unsafe {
        if let Some(proc) = &mut PROCS[info.1] {
            proc.id = info.0;
            proc.init_proc_state.init_mepc = mepc;
            proc.init_proc_state.pmp_idx = get_pmpidx(mepc);
            proc.state = State::Starting;
        }
        CUR_PROG_IDX = info.1;
        boot_proc(cur());
    }
    return info.0;
}
// returns a tuple (id, idx) of the next inactive process, returns 0 if no inactive process is found. (0 because tau never quits)
fn get_inact_proc() -> (usize, usize) {
    unsafe {
        for idx in 0..PROCS.len() {
            if let Some(proc) = &PROCS[idx] {
                if proc.state == State::Inactive {
                    gen_id();
                    return (ID_SEED as usize, idx);
                }
            }
        }
    };
    return (0, 0);
}

unsafe fn gen_id() {
    ID_SEED ^= ID_SEED << 7;
    ID_SEED ^= ID_SEED >> 9;
    ID_SEED ^= ID_SEED << 8;
}

fn get_pmpidx(mepc: usize) -> usize {
    return ((mepc - 0x80000000) / 0x00100000) - 1;
}

pub fn scheduler() {
    switch(next().expect("No next programm"));
}

/// Returns the current user prog.
pub fn cur() -> Proc {
    unsafe {
        if let Some(cur) = &mut PROCS[CUR_PROG_IDX] {
            return Proc {
                idx: CUR_PROG_IDX,
                id: cur.id,
            };
        }
        panic!("Tried to access current user prog. But none was running");
    }
}
/// Returns the next rdy or starting user prog after round robin.
fn next() -> Option<Proc> {
    unsafe {
        let start = CUR_PROG_IDX + 1;
        for i in 0..PROCS.len() {
            let idx = (start + i) % PROCS.len();
            if let Some(next) = &mut PROCS[idx] {
                if next.state == State::Rdy || next.state == State::Starting {
                    return Some(Proc { idx, id: next.id });
                }
            }
        }
    }
    return None;
}
/// Switches the program.
fn switch(prog: Proc) {
    unsafe {
        let prog_data = prog.get();
        match prog_data.state {
            State::Rdy => {
                CUR_PROG_IDX = prog.idx;
                pmp::switch_prog_pmp(prog_data.init_proc_state.pmp_idx);
            }
            State::Starting => {
                boot_proc(prog);
            }
            State::Inactive => {
                panic!(
                    "Tried to switch to user proc: {:?}, with state: {:?}",
                    prog_data.id, prog_data.state
                )
            }
            State::_Blocked(_) => {
                panic!(
                    "Tried to switch to user proc: {:?}, with state: {:?}",
                    prog_data.id, prog_data.state
                )
            }
        }
    }
}