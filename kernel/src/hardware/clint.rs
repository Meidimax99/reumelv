use super::memory_mapping::MemoryMapping;
use riscv_utils::*;

const TIMER_DURATION: u64 = 10000000; //QEMU 10Mhz -> 1s

pub unsafe fn set_time_cmp() {
    let mut mtimecmp = MemoryMapping::new(MTIMECMP_ADDR);
    let mtime: u64 = MemoryMapping::new(MTIME_ADDR).read();
    mtimecmp.write(mtime + TIMER_DURATION);
}

pub unsafe fn init() {
    let mut mtimecmp = MemoryMapping::new(MTIMECMP_ADDR);
    mtimecmp.write(u64::MAX);
}
