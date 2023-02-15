pub struct Queue<T: Default + Copy + Sized, const N: usize> {
    index_in: usize,
    index_out: usize,
    count: usize,
    arr: [T; N],
}

impl<T: Default + Copy + Sized, const N: usize> Queue<T, N> {
    pub fn new() -> Queue<T, N> {
        Queue {
            index_in: 0,
            index_out: 0,
            count: 0,
            arr: [T::default(); N],
        }
    }
    pub fn push(&mut self, elem: T) {
        if !self.isEmpty() && self.index_in == self.index_out {
            panic!("Queue is full, cant push!");
        } else {
            self.arr[self.index_in] = elem;
            self.count = (self.count + 1) % N;
        }
    }
    pub fn pop(&mut self) -> Result<T, &'static str> {
        if self.isEmpty() {
            Err("Can't pop, queue is empty")
        } else {
            let elem = self.arr[self.index_out];
            self.index_out = (self.index_out + 1) % N;
            self.count -= 1;
            Ok(elem)
        }
    }
    pub fn isEmpty(&self) -> bool {
        self.count == 0
    }
    pub fn spaceLeft(&self) -> usize {
        N - self.count
    }
}
