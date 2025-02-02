// use macroquad::{
//     input::{is_key_down, is_key_pressed, KeyCode},
//     window::next_frame,
// };
// use strum::IntoEnumIterator;
// use xf::{
//     mq::window::{Window, WindowParams},
//     num::ivec2::IVec2,
// };

// use crate::{
//     consts::PIXEL_SCALE,
//     debug::{self, initialize_debug, DebugConfig},
//     mem::sections::MemSection,
//     ppu::{
//         tile_data_test,
//         tile_map_test::{self, draw_bg_tile_map},
//     },
//     sys::Sys,
// };

// pub fn run(sys: &mut Sys) {
//     // Mem sections
//     for section in MemSection::iter() {
//         let start = section.start_addr();
//         let size = section.size();

//         println!("{:?}: {:#04x} ({:#04x})", section, start, size);
//     }

//     // CPU Registers
//     sys.regs.print();

//     // Cartridge
//     // let rom_file = ".\\assets\\test_roms\\cpu_instrs\\individual\\01-special.gb";
//     // let rom_file = ".\\assets\\test_roms\\cpu_instrs\\individual\\02-interrupts.gb";
//     // let rom_file = ".\\assets\\test_roms\\cpu_instrs\\individual\\03-op sp,hl.gb";
//     // let rom_file = ".\\assets\\test_roms\\cpu_instrs\\individual\\04-op r,imm.gb";
//     // let rom_file = ".\\assets\\test_roms\\cpu_instrs\\individual\\05-op rp.gb";
//     // sys.cart.load_from_gb_rom_file(rom_file);

//     // sys.cart.print_header_info();
// }

// pub async fn draw_vram_tile_data_test(path: &str) {
//     let window = Window::new(WindowParams {
//         //resolution: SCREEN_SIZE,
//         resolution: tile_data_test::SCREEN_SIZE,
//         scale: PIXEL_SCALE,
//     });

//     let mut sys = Sys::new();
//     Sys::initialize(&mut sys);

//     sys.mem.cart.load(path);
//     sys.run();

//     if debug::get_failure().is_none() {
//         debug::print_system_state(&sys);
//     }

//     window.render_pass(|| {
//         tile_data_test::draw_vram_tile_data(&sys, IVec2::ZERO);
//         //draw_bg_tile_map(&sys);
//     });
//     while !is_key_pressed(KeyCode::Escape) {
//         window.render_pass(|| {});
//         next_frame().await;
//     }
// }

// pub async fn draw_vram_tile_map_test(path: &str) {
//     let window = Window::new(WindowParams {
//         resolution: tile_map_test::SCREEN_SIZE,
//         scale: PIXEL_SCALE,
//     });

//     let mut sys = Sys::new();
//     Sys::initialize(&mut sys);

//     sys.mem.cart.load(path, true);
//     //sys.run();
//     while !sys.hard_lock {
//         let do_print = sys.run_one_m_cycle();
//         if do_print && debug::take_pending_breakpoint() {
//             debug::print_last_instr();
//             println!("Press enter: ");
//             while !is_key_pressed(KeyCode::Space)
//                 && !is_key_pressed(KeyCode::Escape)
//                 && !is_key_down(KeyCode::J)
//             {
//                 if is_key_pressed(KeyCode::Escape) {
//                     sys.hard_lock = true;
//                 }
//                 next_frame().await;
//             }
//             next_frame().await;
//         }
//     }

//     if debug::get_failure().is_none() {
//         debug::print_system_state(&sys);
//     }

//     window.render_pass(|| {
//         draw_bg_tile_map(&sys, IVec2::ZERO);
//     });
//     while !is_key_pressed(KeyCode::Escape) {
//         window.render_pass(|| {});
//         next_frame().await;
//     }
// }
