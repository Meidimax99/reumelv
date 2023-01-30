use super::memory_mapping::MemoryMapping;

#[repr(C)]
pub struct Stack(MemoryMapping<[usize; 32]>, [usize; 32]);

impl Stack {
    pub unsafe fn new(sp: usize) -> Self {
        let mem_stack = MemoryMapping::new(sp);
        let stack = mem_stack.read();
        Stack(mem_stack, stack)
    }
    pub fn a0(&self) -> usize {
        self.1[9]
    }
    pub fn a1(&self) -> usize {
        self.1[10]
    }
    pub fn a7(&self) -> usize {
        self.1[16]
    }

    pub fn s0(&self) -> usize {
        self.1[7]
    }

    pub fn write_s0(&mut self, value:usize) {
        self.1[7] = value;
    }
    /// Sets the return value.
    pub fn set_ret(&mut self, ret: usize) {
        self.1[9] = ret;
    }
    /// Writes the stack to memory.
    pub unsafe fn write(&mut self) {
        self.0.write(self.1);
    }
}
