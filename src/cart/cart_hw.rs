use crate::mem::map::Addr;

pub trait CartHw {
    fn rom(&self) -> &[u8];
    fn rom_mut(&mut self) -> &mut [u8];

    fn rd(&self, addr: Addr) -> u8;
    fn wr(&mut self, addr: Addr, data: u8);
}
