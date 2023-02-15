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

//Generic and const doesnt mix yet as const traits dont exist, so here is a ByteQueue for IPC

#[derive(Clone, Copy)]
pub struct ByteQueue<const N: usize> {
    index_in: usize,
    index_out: usize,
    count: usize,
    arr: [u8; N],
}

impl<const N: usize> ByteQueue<N> {
    pub const fn new() -> ByteQueue<N> {
        ByteQueue {
            index_in: 0,
            index_out: 0,
            count: 0,
            arr: [0u8; N],
        }
    }
    pub fn push(&mut self, elem: u8) {
        if !self.isEmpty() && self.index_in == self.index_out {
            panic!("Queue is full, cant push!");
        } else {
            self.arr[self.index_in] = elem;
            self.index_in = (self.index_in + 1) % N;
            self.count += 1;
        }
    }
    pub fn pop(&mut self) -> Result<u8, &'static str> {
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
