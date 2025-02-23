use crate::mem::Addr;

/// Array of bytes that represents a segment of memory.
pub struct Array {
    start_addr: Addr,
    memory: Vec<u8>,
}

impl Array {
    pub fn new(start_addr: Addr, size: u16) -> Self {
        Self {
            start_addr,
            memory: vec![0; size as usize],
        }
    }

    pub fn read(&self, abs_addr: impl Into<Addr>) -> u8 {
        let idx = self.to_idx(abs_addr);
        return self.memory[idx];
    }

    pub fn write(&mut self, abs_addr: impl Into<Addr>, data: u8) {
        let idx = self.to_idx(abs_addr);
        self.memory[idx] = data;
    }

    pub fn mut_(&mut self, abs_addr: impl Into<Addr>) -> &mut u8 {
        let idx = self.to_idx(abs_addr);
        return &mut self.memory[idx];
    }

    #[inline]
    fn to_idx(&self, abs_addr: impl Into<Addr>) -> usize {
        let abs_addr: Addr = abs_addr.into();
        let rel_addr = abs_addr - self.start_addr;
        return rel_addr as usize;
    }

    pub fn as_slice(&self) -> &[u8] {
        return self.memory.as_slice();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rw() {
        let start_addr = 0x1234;
        let mut array = Array::new(start_addr, 0x0200);

        for addr in 0..0x0200u16 {
            let addr = start_addr + addr;

            let write_value = ((addr * 3) & 0xFF) as u8;
            array.write(addr, write_value);
            let read_value = array.read(addr);

            assert_eq!(write_value, read_value);

            let mut_value = ((addr * 7) & 0xFF) as u8;
            *array.mut_(addr) = mut_value;
            let read_value = array.read(addr);

            assert_eq!(mut_value, read_value);
        }
    }
}
