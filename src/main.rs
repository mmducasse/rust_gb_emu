//#![forbid(unsafe_code)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(static_mut_refs)]

use consts::{PIXEL_SCALE, SCREEN_SIZE};
use debug::{initialize_debug, DebugConfig};
use macroquad::{
    color::BLACK,
    input::{is_key_pressed, KeyCode},
    window::next_frame,
};
use ppu::{
    consts::{TILE_MAP_P8_SIZE, VIEWPORT_ORG, WINDOW_BOUNDS},
    debug_draw::render_screen,
    render::render_scanline,
    tile_data_test::{self, draw_vram_tile_data},
    tile_map_test::{self, draw_bg_tile_map},
    ui::render_ui,
};
use sys::{Options, Sys};
use test::instr::test_all_opcodes;
use xf::{
    mq::{
        draw::draw_rect,
        window::{Window, WindowParams},
    },
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
mod other;
mod ppu;
mod sys;
mod test;
mod time;
mod util;

#[macroquad::main("rust_gb_emu")]
async fn main() {
    println!("*** RUST GAMEBOY EMU (Matthew Ducasse 2025) ***");

    test().await;
    //run_blaargs_suite().await;
}

async fn test() {
    initialize_debug(DebugConfig {
        enable_debug_print: false,
        kill_after_cpu_ticks: None, //Some(2_000_000),
        kill_after_nop_count: None, // Some(16),
        last_instr_count: 15,
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
    //let path = ".\\assets\\blaargs\\cpu_instrs\\individual\\05-op rp.gb";
    //let path = ".\\assets\\blaargs\\cpu_instrs\\individual\\06-ld r,r.gb";
    //let path = ".\\assets\\blaargs\\cpu_instrs\\individual\\07-jr,jp,call,ret,rst.gb";
    //let path = ".\\assets\\blaargs\\cpu_instrs\\individual\\08-misc instrs.gb";
    //let path = ".\\assets\\blaargs\\cpu_instrs\\individual\\09-op r,r.gb";
    //let path = ".\\assets\\blaargs\\cpu_instrs\\individual\\10-bit ops.gb";
    //let path = ".\\assets\\blaargs\\cpu_instrs\\individual\\11-op a,(hl).gb";

    //let path = ".\\assets\\blaargs\\instr_timing\\instr_timing.gb";

    //let path = ".\\assets\\blaargs\\interrupt_time\\interrupt_time.gb";

    //let path = ".\\assets\\blaargs\\mem_timing\\mem_timing\\mem_timing.gb";

    //let path = ".\\assets\\blaargs\\mem_timing-2\\mem_timing-2\\mem_timing.gb";
    //let path = ".\\assets\\blaargs\\mem_timing-2\\mem_timing-2\\rom_singles\\01-read_timing.gb";
    //let path = ".\\assets\\blaargs\\mem_timing-2\\mem_timing-2\\rom_singles\\02-write_timing.gb";
    //let path = ".\\assets\\blaargs\\mem_timing-2\\mem_timing-2\\rom_singles\\03-modify_timing.gb";

    //let path = ".\\assets\\mooneye\\acceptance\\add_sp_e_timing.gb";
    //let path = ".\\assets\\mooneye\\acceptance\\bits\\reg_f.gb";
    //let path = ".\\assets\\mooneye\\acceptance\\interrupts\\ie_push.gb";
    //let path = ".\\assets\\mooneye\\acceptance\\ppu\\hblank_ly_scx_timing-GS.gb";
    //let path = ".\\assets\\mooneye\\acceptance\\timer\\div_write.gb";
    //let path = ".\\assets\\mooneye\\emulator-only\\mbc1\\bits_bank1.gb";
    //let path = ".\\assets\\mooneye\\emulator-only\\mbc1\\rom_1Mb.gb";

    //let path = ".\\assets\\real_gb_roms\\tetris.gb";
    //let path = ".\\assets\\real_gb_roms\\Dr_Mario.gb";
    //let path = ".\\assets\\real_gb_roms\\Pokemon.gb";
    let path = ".\\assets\\real_gb_roms\\Zelda.gb";
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

    let mut sys = Sys::new(Options {
        kill_on_infinite_loop: true,
    });
    Sys::initialize(&mut sys);
    sys.mem.cart.load(path, true);

    while !sys.hard_lock {
        if is_key_pressed(KeyCode::Escape) {
            sys.hard_lock = true;
        }
        //sys.run_one_m_cycle();

        window.render_pass(|| {
            draw_rect(WINDOW_BOUNDS, BLACK);
            while !sys.is_render_pending {
                sys.run_one_m_cycle();
            }

            render_ui(&mut sys);
            sys.is_render_pending = false;
        });

        next_frame().await;

        // if let Some(ly) = sys.is_scanline_render_pending.take() {
        //     window.render_pass(|| {
        //         render_scanline(&mut sys, ly, VIEWPORT_ORG);
        //     });
        // }
        // if sys.is_debug_render_pending {
        //     window.render_pass(|| {
        //         render_screen(&mut sys);
        //     });
        //     next_frame().await;
        //     sys.is_debug_render_pending = false;
        // }
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

async fn run_blaargs_suite() {
    initialize_debug(DebugConfig {
        enable_debug_print: false,
        kill_after_cpu_ticks: None, //Some(1__000),
        kill_after_nop_count: None, // Some(16),
        last_instr_count: 5,
    });

    let window = Window::new(WindowParams {
        resolution: SCREEN_SIZE,
        scale: PIXEL_SCALE,
    });

    window.render_pass(|| {});
    next_frame().await;

    let rom_paths = [
        ".\\assets\\blaargs\\cpu_instrs\\individual\\01-special.gb",
        ".\\assets\\blaargs\\cpu_instrs\\individual\\02-interrupts.gb",
        ".\\assets\\blaargs\\cpu_instrs\\individual\\03-op sp,hl.gb",
        ".\\assets\\blaargs\\cpu_instrs\\individual\\04-op r,imm.gb",
        ".\\assets\\blaargs\\cpu_instrs\\individual\\05-op rp.gb",
        ".\\assets\\blaargs\\cpu_instrs\\individual\\06-ld r,r.gb",
        ".\\assets\\blaargs\\cpu_instrs\\individual\\07-jr,jp,call,ret,rst.gb",
        ".\\assets\\blaargs\\cpu_instrs\\individual\\08-misc instrs.gb",
        ".\\assets\\blaargs\\cpu_instrs\\individual\\09-op r,r.gb",
        ".\\assets\\blaargs\\cpu_instrs\\individual\\10-bit ops.gb",
        ".\\assets\\blaargs\\cpu_instrs\\individual\\11-op a,(hl).gb",
    ];

    for path in rom_paths {
        let mut sys = Sys::new(Options {
            kill_on_infinite_loop: true,
        });
        Sys::initialize(&mut sys);
        sys.mem.cart.load(path, false);

        let rom_name = std::path::Path::new(path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        println!("{}: ", rom_name);

        while !sys.hard_lock {
            if is_key_pressed(KeyCode::Escape) {
                sys.hard_lock = true;
            }
            sys.run_one_m_cycle();

            if sys.is_render_pending {
                window.render_pass(|| {
                    render_screen(&mut sys);
                });
                next_frame().await;
                sys.is_render_pending = false;
            }
        }

        debug::flush_serial_char();
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_() {
        let x = 0xFF;
        let y = u8::wrapping_shr(x, 1);
        assert_eq!(y, 0x7F);

        let x = 0b1011_1111;
        let y = u8::rotate_right(x, 2);
        assert_eq!(y, 0b1110_1111);

        let x = 0b1110_1000;
        let y = 0b0111_0011;
        assert_eq!(x ^ y, 0b1001_1011);
    }
}
