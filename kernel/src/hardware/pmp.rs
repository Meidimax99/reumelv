use riscv_utils::write_machine_reg;

use super::binary_struct::Byte;

pub unsafe fn init() {
    let pmp_addr_0 = 0x80000000 >> 2; // devices  0x0-0x80000000 >> 2
    let pmp_addr_1 = 0x80100000 >> 2; // kernel
    let pmp_addr_2 = 0x80200000 >> 2; // u1
    let pmp_addr_3 = 0x80300000 >> 2; // u2
    let pmp_addr_4 = 0x80400000 >> 2; // u3
    let pmp_addr_5 = 0x80500000 >> 2; // u4
    let pmp_addr_6 = 0x80600000 >> 2; // u5
    let pmp_addr_7 = 0x80700000 >> 2; // u6
    let pmp_addr_8 = 0x80800000 >> 2; // u7
    let pmpcfg0 = 0;

    write_machine_reg!(
        pmp_addr_0 => "pmpaddr0",
        pmp_addr_1 => "pmpaddr1",
        pmp_addr_2 => "pmpaddr2",
        pmp_addr_3 => "pmpaddr3",
        pmp_addr_4 => "pmpaddr4",
        pmp_addr_5 => "pmpaddr5",
        pmp_addr_6 => "pmpaddr6",
        pmp_addr_7 => "pmpaddr7",
        pmp_addr_8 => "pmpaddr8",
        pmpcfg0 => "pmpcfg0"
    );
}

pub unsafe fn switch_prog_pmp(idx: usize) {
    let prog_index = idx + 2; // device and kernel offset
    let mut pmpcfg0 = Pmpcfg::new();
    pmpcfg0.set_rwx(prog_index);
    write_machine_reg!(pmpcfg0.to_usize() => "pmpcfg0");
}

#[repr(C)]
struct Pmpcfg([Byte; 8]);
impl Pmpcfg {
    #[allow(clippy::needless_range_loop)]
    fn to_usize(&self) -> usize {
        let mut arr = [0; usize::BITS as usize / 8];
        for i in 0..self.0.len() {
            arr[i] = self.0[i].get();
        }
        usize::from_ne_bytes(arr)
    }
    fn set_rwx(&mut self, at: usize) {
        let reg = &mut self.0[at];
        reg.at(0, true); // R
        reg.at(1, true); // W
        reg.at(2, true); // X
        reg.at(3, true); // A - top of range
    }
    fn new() -> Self {
        let bytes = [Byte::from(0); 8];
        Pmpcfg(bytes)
    }
}
