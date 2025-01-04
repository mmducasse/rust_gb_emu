//#![forbid(unsafe_code)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(static_mut_refs)]

use cpu::exec::execute_next_instr;
use debug::Debug;
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
mod util;

fn main() {
    println!("*** rust_gb_2 EMU ***");

    let mut sys = Sys::new();
    //temp_tests::run(&mut sys);

    // sys.cart
    //     .load_from_script_file(".\\assets\\files\\script_01.txt");
    // sys.cart.load_from_gb_rom_file(
    //     ".\\assets\\imported_test_roms\\cpu_instrs\\individual\\03-op sp,hl.gb",
    // );
    sys.cart
        .load(".\\assets\\files\\custom_roms\\ld_r8_r8\\rom.gb");

    sys.debug.enable = true;
    sys.run();
}
