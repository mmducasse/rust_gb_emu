use crate::mem::sections::Addr;

pub trait CartHw {
    fn rom(&self) -> &[u8];
    fn rom_mut(&mut self) -> &mut [u8];
    fn ram(&self) -> &[u8];

    fn read(&self, addr: Addr) -> u8;
    fn write(&mut self, addr: Addr, data: u8);
}
