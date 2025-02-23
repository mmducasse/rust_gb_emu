
// Emulator user settings.
#[derive(Default)]
pub struct Emu {
    pub is_speedup_enabled: bool,

    /// False: the background tilemap is shown.
    /// True: the window tilemap is shown.
    pub show_win_map: bool,
}

impl Emu {
    pub const SPEEDUP_RATE: u32 = 4;

    pub fn speed(&self) -> u32 {
        if self.is_speedup_enabled {
            Self::SPEEDUP_RATE
        } else {
            1
        }
    }
}
