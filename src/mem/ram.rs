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

    pub fn rd(&self, addr: impl Into<Addr>) -> u8 {
        let addr: Addr = addr.into();
        return self.memory[addr as usize];
    }

    pub fn wr(&mut self, addr: impl Into<Addr>, data: u8) {
        let addr: Addr = addr.into();
        self.memory[addr as usize] = data;
    }

    pub fn as_slice(&self) -> &[u8] {
        return self.memory.as_slice();
    }
}
