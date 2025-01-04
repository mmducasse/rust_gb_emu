use crate::mem::map::Addr;

pub struct Ram {
    memory: Vec<u8>,
}

impl Ram {
    pub fn new(size: u16) -> Self {
        Self {
            memory: vec![0; size as usize],
        }
    }

    pub fn rd(&self, addr: Addr) -> u8 {
        return self.memory[addr as usize];
    }

    pub fn wr(&mut self, addr: Addr, data: u8) {
        self.memory[addr as usize] = data;
    }
}
