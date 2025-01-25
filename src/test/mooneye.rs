use macroquad::{
    input::{is_key_pressed, KeyCode},
    window::next_frame,
};

use crate::{debug, sys::Sys};

pub fn run_simple_test(rom_path: &str) {
    let mut sys = Sys::new();
    Sys::initialize(&mut sys);

    sys.mem.cart.load(rom_path);
    sys.run();

    println!("Done");
    debug::print_system_state(&sys);
}
