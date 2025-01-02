pub struct RingBuffer<T> {
    max_len: usize,
    index: usize,
    data: Vec<T>,
}

impl<T> RingBuffer<T> {
    pub fn new(size: usize) -> Self {
        Self {
            max_len: size,
            index: 0,
            data: vec![],
        }
    }

    pub fn len(&self) -> usize {
        return self.data.len();
    }

    pub fn max_len(&self) -> usize {
        return self.max_len;
    }

    pub fn add(&mut self, value: T) {
        if self.data.len() == self.max_len {
            self.data[self.index] = value;
            self.index = (self.index + 1) % self.max_len;
        } else {
            self.data.push(value);
        }
    }

    pub fn iter<'a>(&'a self) -> RingBufferIterator<'a, T> {
        return RingBufferIterator {
            ring_buffer: self,
            index: self.index,
            is_done: false,
        };
    }
}

pub struct RingBufferIterator<'a, T> {
    ring_buffer: &'a RingBuffer<T>,
    index: usize,
    is_done: bool,
}

impl<'a, T> Iterator for RingBufferIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }

        let value = &self.ring_buffer.data[self.index];
        self.index = (self.index + 1) % self.ring_buffer.len();
        if self.index == self.ring_buffer.index {
            self.is_done = true;
        }
        return Some(value);
    }
}
