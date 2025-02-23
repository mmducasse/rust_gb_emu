use macroquad::{
    input::{is_key_pressed, KeyCode},
    window::next_frame,
};
use xf::mq::window::{Window, WindowParams};

use crate::{
    cart::cart::Cart,
    consts::{PIXEL_SCALE, SCREEN_SIZE},
    debug::{self, initialize_debug, DebugConfig},
    ppu::ui::render_ui,
    sys::{Options, Sys},
};

/// Runs every blargg's cpu_instrs test in sequence and prints the results
/// to the console.
async fn run_blarggs_test_suite() {
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
        let options = Options {
            kill_on_infinite_loop: true,
            show_vram_views: true,
        };
        let cart = Cart::load_from(&path, false).unwrap();
        let mut sys = Sys::new(options, cart);

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
                    render_ui(&mut sys);
                });
                next_frame().await;
                sys.is_render_pending = false;
            }
        }

        debug::flush_serial_char();
    }
}
