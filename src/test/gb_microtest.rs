use macroquad::{
    input::{is_key_pressed, KeyCode},
    window::next_frame,
};
use xf::{
    mq::window::{Window, WindowParams},
    num::ivec2::i2,
};

use crate::{consts::PIXEL_SCALE, debug::Debug, sys::Sys};

pub async fn run_gb_microtest(rom_path: &str) {
    let window = Window::new(WindowParams {
        resolution: i2(256, 256),
        scale: PIXEL_SCALE,
    });

    let mut sys = Sys::new();
    Sys::initialize(&mut sys);

    sys.cart.load(rom_path);
    sys.debug.kill_after_cpu_ticks = Some(1_000_000);
    sys.debug.kill_after_nop_count = Some(64);
    sys.debug.enable_debug_print = false; //true;
    sys.run();

    println!("Done");
    Debug::print_system_state(&sys);

    //0xFF80 - Test result
    let result = sys.mem_get(0xFF80u16);
    println!("0xFF80 - Result: {:0>2X}", result);

    // 0xFF81 - Expected result
    let expected_result = sys.mem_get(0xFF81u16);
    println!("0xFF81 - Expected result: {:0>2X}", result);

    // 0xFF82 - 0x01 if the test passed, 0xFF if the test failed.
    let pass_fail = sys.mem_get(0xFF82u16);
    println!("0xFF82 - Pass ($01) or Fail ($FF): {:0>2X}", pass_fail);

    while !is_key_pressed(KeyCode::Escape) {
        window.render_pass(|| {});
        next_frame().await;
    }
}

fn print_output_char(sys: &Sys) {
    if sys.mem_get(0xFF02u16) == 0x81 {
        let data = sys.mem_get(0xFF01u16);
        let c = char::from_u32(data as u32).unwrap_or('?');
        // if c.is_whitespace() {
        //     println!();
        // } else {
        //     print!("{}", c);
        // }
        println!("{:02x}", data);
    }
}
