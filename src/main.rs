//#![forbid(unsafe_code)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(static_mut_refs)]

use sys::Sys;

extern crate num;
#[macro_use]
extern crate num_derive;

mod cpu;
mod debug;
mod mem;
mod sys;
mod temp_tests;
mod test;
mod time;
mod util;

fn main() {
    println!("*** RUST GAMEBOY EMU (Matthew Ducasse 2025) ***");

    let mut sys = Sys::new();
    // temp_tests::run(&mut sys);

    //sys.cart.load(".\\assets\\files\\custom_roms\\ld_r8_r8\\rom.gb");
    sys.cart.load(".\\assets\\real_gb_roms\\tetris.gb");
    sys.cart.load(".\\assets\\real_gb_roms\\Pokemon.gb");

    sys.debug.enable_debug_print = false; //true;
    sys.debug.kill_after_seconds = Some(5.0);
    sys.run();
}
