//#![forbid(unsafe_code)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(static_mut_refs)]

use consts::{PIXEL_SCALE, SCREEN_SIZE};
use debug::{initialize_debug, DebugConfig};
use macroquad::{
    input::{is_key_pressed, KeyCode},
    window::next_frame,
};
use ppu::{
    consts::TILE_MAP_P8_SIZE,
    draw::render_screen,
    tile_data_test::{self, draw_vram_tile_data},
    tile_map_test::{self, draw_bg_tile_map},
};
use sys::Sys;
use test::{
    blargg::run_blargg_test, gb_microtest::run_gb_microtest, instr::test_all_opcodes,
    mooneye::run_simple_test, temp_tests,
};
use xf::{
    mq::window::{Window, WindowParams},
    num::ivec2::{i2, IVec2},
};

extern crate num;
#[macro_use]
extern crate num_derive;

mod cart;
mod consts;
mod cpu;
mod debug;
mod mem;
mod ppu;
mod sys;
mod test;
mod time;
mod util;

#[macroquad::main("rust_gb_emu")]
async fn main() {
    println!("*** RUST GAMEBOY EMU (Matthew Ducasse 2025) ***");

    initialize_debug(DebugConfig {
        enable_debug_print: false,
        kill_after_cpu_ticks: None, // Some(90),
        kill_after_nop_count: None, //Some(64),
        last_instr_count: 3,
    });

    //std::env::set_var("RUST_BACKTRACE", "1");

    //test_all_opcodes();

    //let path = ".\\assets\\files\\custom_roms\\ld_r8_r8\\rom.gb";
    //let path = ".\\assets\\gb_microtest\\000-write_to_x8000.gb";

    //let path = ".\\assets\\blaargs\\cpu_instrs\\cpu_instrs.gb";
    //let path = ".\\assets\\blaargs\\cpu_instrs\\individual\\01-special.gb";
    //let path = ".\\assets\\blaargs\\cpu_instrs\\individual\\02-interrupts.gb";
    //let path = ".\\assets\\blaargs\\cpu_instrs\\individual\\03-op sp,hl.gb";
    //let path = ".\\assets\\blaargs\\cpu_instrs\\individual\\04-op r,imm.gb";
    // let path = ".\\assets\\blaargs\\cpu_instrs\\individual\\05-op rp.gb";
    let path = ".\\assets\\blaargs\\cpu_instrs\\individual\\06-ld r,r.gb";
    // let path = ".\\assets\\blaargs\\cpu_instrs\\individual\\07-jr,jp,call,ret,rst.gb";
    // let path = ".\\assets\\blaargs\\cpu_instrs\\individual\\08-misc instrs.gb";
    // let path = ".\\assets\\blaargs\\cpu_instrs\\individual\\09-op r,r.gb";
    // let path = ".\\assets\\blaargs\\cpu_instrs\\individual\\10-bit ops.gb";
    // let path = ".\\assets\\blaargs\\cpu_instrs\\individual\\11-op a,(hl).gb";

    //let path = ".\\assets\\mooneye\\acceptance\\add_sp_e_timing.gb";
    //let path = ".\\assets\\mooneye\\acceptance\\bits\\reg_f.gb";

    //let path = ".\\assets\\real_gb_roms\\tetris.gb";
    //let path = ".\\assets\\real_gb_roms\\Dr_Mario.gb";
    //let path = ".\\assets\\real_gb_roms\\Pokemon.gb";
    //let path = ".\\assets\\real_gb_roms\\Zelda.gb";
    //let path = ".\\assets\\real_gb_roms\\Kirby.gb";

    //let path = ".\\assets\\homebrew_roms\\porklike.gb";
    //let path = ".\\assets\\homebrew_roms\\20y.gb";
    //let path = ".\\assets\\homebrew_roms\\64boy-opcode-scroll.gb";
    //let path = ".\\assets\\homebrew_roms\\life.gb";

    //let path = ".\\assets\\other\\hello_world\\rom.gb";

    //emp_tests::draw_vram_tile_data_test(path).await;
    //temp_tests::draw_vram_tile_map_test(path).await;
    //run_blargg_test(path).await;
    //run_gb_microtest(&path).await;
    //run_simple_test(&path);
    run_normal(&path).await;
}

async fn run_normal(path: &str) {
    let window = Window::new(WindowParams {
        resolution: SCREEN_SIZE,
        scale: PIXEL_SCALE,
    });

    window.render_pass(|| {});
    next_frame().await;

    let mut sys = Sys::new();
    Sys::initialize(&mut sys);
    sys.mem.cart.load(path);

    while !sys.hard_lock {
        if is_key_pressed(KeyCode::Escape) {
            sys.hard_lock = true;
        }
        sys.run_one_m_cycle();

        // blarggs test - serial output
        if sys.mem.io_regs.get(mem::io_regs::IoReg::Sc) == 0x81 {
            let data = sys.mem.io_regs.get(mem::io_regs::IoReg::Sb);
            let c = data as char;
            println!("> {}", c);

            sys.mem.io_regs.set(mem::io_regs::IoReg::Sc, 0x00);
        }

        if sys.is_render_pending {
            window.render_pass(|| {
                render_screen(&mut sys);
            });
            next_frame().await;
            sys.is_render_pending = false;
        }
    }

    while !is_key_pressed(KeyCode::Escape) {
        window.render_pass(|| {});
        next_frame().await;
    }
}
