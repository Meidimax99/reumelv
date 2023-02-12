pub struct MemoryMapping<T> {
    val: *mut T,
}
impl<T> MemoryMapping<T> {
    pub const fn new(address: usize) -> Self {
        MemoryMapping {
            val: address as *mut T,
        }
    }
    pub unsafe fn read(&self) -> T {
        return self.val.read_volatile();
        }
    pub unsafe fn write(&mut self, val: T) {
        self.val.write_volatile(val);
    }
}
