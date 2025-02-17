use macroquad::input::{is_key_pressed, KeyCode};

// Emulator user settings.
#[derive(Default)]
pub struct Emu {
    /// False: the background tilemap is shown.
    /// True: the window tilemap is shown.
    pub show_win_map: bool,
}

impl Emu {
    pub fn update(&mut self) {
        if is_key_pressed(KeyCode::T) {
            self.show_win_map = !self.show_win_map;
        }
    }
}
