use crate::mem::Addr;

/// Functionality that any cartridge type (ROM-only, MBC1, etc.) must provide.
pub trait CartHw {
    fn rom_mut(&mut self) -> &mut [u8];
    fn ram(&self) -> &[u8];
    fn ram_mut(&mut self) -> &mut [u8];

    fn read(&self, addr: Addr) -> u8;
    fn write(&mut self, addr: Addr, data: u8);
}
