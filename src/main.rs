//#![forbid(unsafe_code)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(static_mut_refs)]

use cpu::execute_next_instr;
use debug::Debug;
use sys::Sys;

extern crate num;
#[macro_use]
extern crate num_derive;

mod cart;
mod cpu;
mod data;
mod debug;
mod instr;
mod math;
mod mem;
mod print;
mod ram;
mod regs;
mod sys;
mod temp_tests;
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
        .load_from_gb_rom_file(".\\assets\\files\\custom_roms\\ld_r8_r8\\rom.gb");

    sys.debug.enable = true;

    while !sys.hard_lock {
        execute_next_instr(&mut sys);
        if sys.debug.nop_count > Debug::EXIT_AFTER_NOP_COUNT {
            break;
        }
        //sys.regs.print();
    }
}
