use super::{binary_struct::BinaryStruct, memory_mapping::MemoryMapping};
use riscv_utils::*;

/// Interrupt request.
pub enum IRQ {
    Uart = 10,
}

pub unsafe fn init() {
    let uart_priority_addr = get_priority_addr(IRQ::Uart);
    MemoryMapping::new(uart_priority_addr).write(5);
    // Enable in context.
    let mut enable_c0 = [BinaryStruct::from(0u32); 32];
    let (uart_idx, uart_bit) = bin32_idx_pos(IRQ::Uart);
    enable_c0[uart_idx].at(uart_bit, true);
    let enable_addr = get_enable_addr(uart_idx);
    MemoryMapping::new(enable_addr).write(enable_c0[uart_idx].get());
    // Set thresholds for context.
    MemoryMapping::new(THRESHOLD_ADDR_C0).write(0u32);
}

pub unsafe fn read_claim() -> IRQ {
    let claim: u32 = MemoryMapping::new(CLAIM_COMP_ADDR_C0).read();
    let claim = claim as usize;
    crate::enum_matching!(claim: IRQ::Uart);
    panic!("Unknown plic interrupt request: {}", claim);
}

pub unsafe fn write_complete(irq: IRQ) {
    MemoryMapping::new(CLAIM_COMP_ADDR_C0).write(irq as u32);
}

fn get_priority_addr(irq: IRQ) -> usize {
    PRIORITY_BASE_ADDR + 4 * irq as usize
}

/// Returns the (index, pos) of an irq if every bit is used as an id for an irq.
fn bin32_idx_pos(irq: IRQ) -> (usize, usize) {
    let irq = irq as usize;
    (irq / 32, irq % 32)
}

fn get_enable_addr(idx: usize) -> usize {
    ENABLE_ADDR + 4 * idx
}
