pub mod dispatcher;
pub mod process;
pub mod scheduler;
pub mod state;

use crate::hardware::binary_struct::BinaryStruct;
use riscv_utils::{read_machine_reg, write_machine_reg, MSTATUS_MIE};

pub unsafe fn set_MIE(set: bool) {
    let mstatus: usize;
    let mut config = MSTATUS_MIE;
    config.1 = set;
    read_machine_reg!("mstatus" => mstatus);
    let mut mstatus = BinaryStruct::from(mstatus);
    mstatus.write_register_entry(MSTATUS_MIE);
    let mstatus = mstatus.get();
    write_machine_reg!(mstatus => "mstatus");
}
