use crate::mem::sections::Addr;

use super::cart_hw::CartHw;

pub struct HwEmpty;

// todo delete this type.
impl CartHw for HwEmpty {
    fn rom(&self) -> &[u8] {
        todo!()
    }

    fn rom_mut(&mut self) -> &mut [u8] {
        todo!()
    }

    fn ram(&self) -> &[u8] {
        todo!()
    }

    fn ram_mut(&mut self) -> &mut [u8] {
        todo!()
    }

    fn read(&self, addr: Addr) -> u8 {
        todo!()
    }

    fn write(&mut self, addr: Addr, data: u8) {
        todo!()
    }
}
