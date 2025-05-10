// /////////////////////////////////////////////////////////// //
//                                                             //
// Project: Rust Game Boy Emulator                             //
// Author:  Matthew M. Ducasse                                 //
// Date:    Jan 2025                                           //
//                                                             //
// Description: An emulator for the Nintendo Game Boy (1989).  //
//                                                             //
// /////////////////////////////////////////////////////////// //

use std::{env, fs};

use cart::cart::Cart;
use consts::PIXEL_SCALE;
use debug::{initialize_debug, DebugConfig};
use macroquad::{
    color::BLACK,
    input::{is_key_pressed, KeyCode},
    window::next_frame,
};
use other::save::{load_state, save_state};
use ppu::{consts::window_size, ui::render_ui};
use sys::{Options, Sys};
use xf::mq::{
    draw::draw_rect,
    window::{Window, WindowParams},
};

extern crate num;
#[macro_use]
extern crate num_derive;

mod cart;
mod consts;
mod cpu;
mod debug;
mod mem;
mod other;
mod ppu;
mod sys;
mod test;
mod time;
mod util;

#[macroquad::main("rust_gb_emu")]
async fn main() {
    println!("*** RUST GAMEBOY EMU (Matthew Ducasse 2025) ***");

    if let Some(rom_path) = validate_args(env::args().collect()) {
        run_emu(&rom_path).await;
    }
}

fn validate_args(mut args: Vec<String>) -> Option<String> {
    const USAGE_STR: &str = "usage: rust_gb_2.exe <gb-rom-file-path>";

    if args.len() != 2 {
        println!("Expected a file path to a .gb rom file.");
        println!("{}", USAGE_STR);
        return None;
    }

    let rom_path = args.remove(1);
    return match fs::exists(&rom_path) {
        Ok(true) => Some(rom_path),
        Ok(false) => {
            println!("File does not exist: {}", rom_path);
            None
        }
        Err(msg) => {
            println!("{}", msg);
            None
        }
    };
}

async fn run_emu(rom_path: &str) {
    initialize_debug(DebugConfig {
        enable_debug_print: false,
        kill_after_cpu_ticks: None,
        kill_after_nop_count: None,
        last_instr_count: 15,
    });

    // Instantiate the game cartridge.
    let cart = match Cart::load_from(rom_path, true) {
        Ok(cart) => cart,
        Err(msg) => {
            panic!("{}", msg);
        }
    };

    // Set emulator options.
    let show_vram_views = true;
    let options = Options {
        kill_on_infinite_loop: true,
        show_vram_views,
    };

    // Instantiate the emulator state.
    let mut sys = Sys::new(options, cart);

    // Instantiate the UI window.
    let window = Window::new(WindowParams {
        resolution: window_size(show_vram_views),
        scale: PIXEL_SCALE,
    });

    // Load the saved game state.
    load_state(&mut sys);

    // Main loop.
    while !sys.hard_lock {
        check_misc_inputs(&mut sys);

        window.render_pass(|| {
            draw_rect(window.bounds(), BLACK);
            let speed = sys.emu.speed();
            for _ in 0..speed {
                while !sys.is_render_pending && !sys.hard_lock {
                    sys.run_one_m_cycle();
                }
                sys.is_render_pending = false;
            }

            render_ui(&mut sys);
            sys.is_render_pending = false;
        });

        next_frame().await;
    }

    debug::flush_serial_char();
    debug::print_system_state(&sys);

    loop {
        window.render_pass(|| {});
        next_frame().await;
        if is_key_pressed(KeyCode::Escape) {
            return;
        }
    }
}

fn check_misc_inputs(sys: &mut Sys) {
    if is_key_pressed(KeyCode::Escape) {
        sys.hard_lock = true;
    }

    if is_key_pressed(KeyCode::Backspace) {
        save_state(sys);
    }
    if is_key_pressed(KeyCode::Equal) {
        load_state(sys);
    }

    if is_key_pressed(KeyCode::Space) {
        sys.emu.is_speedup_enabled = !sys.emu.is_speedup_enabled;
    }
    if is_key_pressed(KeyCode::T) {
        sys.emu.show_win_map = !sys.emu.show_win_map;
    }
}
