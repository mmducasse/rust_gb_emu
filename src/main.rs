//#![forbid(unsafe_code)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(static_mut_refs)]

use consts::{PIXEL_SCALE, SCREEN_SIZE};
use debug::Debug;
use macroquad::{
    input::{is_key_pressed, KeyCode},
    window::next_frame,
};
use ppu::{
    tile_data_test::{self, draw_vram_tile_data},
    tile_map_test::{self, draw_bg_tile_map},
};
use sys::Sys;
use xf::{
    mq::window::{Window, WindowParams},
    num::ivec2::{i2, IVec2},
};

extern crate num;
#[macro_use]
extern crate num_derive;

mod consts;
mod cpu;
mod debug;
mod mem;
mod ppu;
mod sys;
mod temp_tests;
mod test;
mod time;
mod util;

#[macroquad::main("rust_gb_emu")]
async fn main() {
    println!("*** RUST GAMEBOY EMU (Matthew Ducasse 2025) ***");

    //let path = ".\\assets\\files\\custom_roms\\ld_r8_r8\\rom.gb";
    //let path = ".\\assets\\real_gb_roms\\tetris.gb";
    //let path = ".\\assets\\real_gb_roms\\Pokemon.gb";
    //let path = ".\\assets\\real_gb_roms\\Zelda.gb";
    let path = ".\\assets\\imported_test_roms\\other\\hello_world\\rom.gb";

    temp_tests::draw_vram_tile_data_test(path).await;
    //temp_tests::draw_vram_tile_map_test(path).await;
}
