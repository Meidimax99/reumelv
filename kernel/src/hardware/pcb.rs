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
pub struct PCB {
    mapping: MemoryMapping<[usize; 32]>,
    state: [usize; 32],
}

impl PCB {
    pub unsafe fn new(sp: usize) -> Self {
        let mem_stack = MemoryMapping::new(sp);
        let stack = mem_stack.read();
        PCB {
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
