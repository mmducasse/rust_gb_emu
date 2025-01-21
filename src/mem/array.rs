use crate::mem::map::Addr;

use super::map::MemSection;

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

    pub fn from_mem_section(section: MemSection) -> Self {
        return Self::new(section.start_addr(), section.size());
    }

    pub fn start_addr(&self) -> Addr {
        self.start_addr
    }

    pub fn contains_addr(&self, addr: Addr) -> bool {
        return (self.start_addr <= addr)
            && (((addr - self.start_addr) as usize) < self.memory.len());
    }

    pub fn rd(&self, abs_addr: impl Into<Addr>) -> u8 {
        let abs_addr: Addr = abs_addr.into();
        let rel_addr = abs_addr - self.start_addr;
        return self.memory[rel_addr as usize];
    }

    pub fn wr(&mut self, abs_addr: impl Into<Addr>, data: u8) {
        let abs_addr: Addr = abs_addr.into();
        let rel_addr = abs_addr - self.start_addr;
        self.memory[rel_addr as usize] = data;
    }

    pub fn mut_(&mut self, abs_addr: impl Into<Addr>) -> &mut u8 {
        let abs_addr: Addr = abs_addr.into();
        let rel_addr = abs_addr - self.start_addr;
        return &mut self.memory[rel_addr as usize];
    }

    pub fn as_slice(&self) -> &[u8] {
        return self.memory.as_slice();
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        return self.memory.as_mut_slice();
    }
}
