use core::mem;

use super::memory_mapping::MemoryMapping;

//https://en.wikichip.org/wiki/risc-v/registers
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum Register {
    ra = 0,
    sp,
    gp,
    tp,
    t0,
    t1,
    t2,
    s0,
    s1,
    a0,
    a1,
    a2,
    a3,
    a4,
    a5,
    a6,
    a7,
    s2,
    s3,
    s4,
    s5,
    s6,
    s7,
    s8,
    s9,
    s10,
    s11,
    t3,
    t4,
    t5,
    t6,
    zero,
}
#[repr(C)]
//Process Control Block
pub struct Stack_Image {
    mapping: MemoryMapping<[usize; 32]>,
    state: [usize; 32],
}

impl Stack_Image {
    pub unsafe fn new(sp: usize) -> Self {
        let mem_stack = MemoryMapping::new(sp);
        let stack = mem_stack.read();
        Stack_Image {
            mapping: mem_stack,
            state: stack,
        }
    }
    pub fn get(&self, register: Register) -> usize {
        self.state[register as usize]
    }

    pub fn set(&mut self, register: Register, value: usize) {
        self.state[register as usize] = value;
    }

    pub unsafe fn write(&mut self) {
        self.mapping.write(self.state);
    }
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! copy_registers {
    ( $from:ident => $to:ident ; $($enum:expr), +) => {
        $(
            copy_reg_stack_value($from, $to, $enum);
        )+
    };
}

//This function is used in the central exchange of ipc data
//Would be cleaner to perform this using two pcbs, one for the sending and one for the receiving process
//and then cloning the contents over
//That would be quite expensive since there would be a copy of the stack contents to the pcb struct
//Then there would be a copy of all registers between the pcb structs
//and finally there would be a copy back from the pcb struct to the stack of the receiver

/// Copy the stack image of the registers to another processes register image
pub unsafe fn copy_ipc_regs_img(from: usize, to: usize) {
    copy_registers!(from => to ; Register::s0, Register::s1, Register::s2, Register::s3, Register::s4, Register::s5, Register::s6, Register::s7, Register::s8, Register::s9, Register::s10, Register::s11);
}

#[allow(dead_code)]
unsafe fn copy_reg_stack_value(sp_from: usize, sp_to: usize, register: Register) {
    let reg_num = register as usize;
    get_reg_relative_addr(sp_to, reg_num)
        .write_volatile(get_reg_relative_addr(sp_from, reg_num).read_volatile());
}

/// Returns the address that would correspond to the given register if the given address points to the beginning of a registerimage on the stack
fn get_reg_relative_addr(addr: usize, register: usize) -> *mut usize {
    (addr + mem::size_of::<usize>() * register) as *mut usize
}
