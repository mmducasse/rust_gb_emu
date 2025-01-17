use crate::mem::map::Addr;

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

    fn rd(&self, addr: Addr) -> u8 {
        todo!()
    }

    fn wr(&mut self, addr: Addr, data: u8) {
        todo!()
    }
}
