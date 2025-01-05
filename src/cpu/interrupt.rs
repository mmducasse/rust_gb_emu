#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InterruptType {
    VBlank,
    Lcd,
    Timer,
    Serial,
    Joypad,
}

impl InterruptType {
    pub fn flag_idx(self) -> u8 {
        return self as u8;
    }
}
