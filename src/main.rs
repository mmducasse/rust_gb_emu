//#![forbid(unsafe_code)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(static_mut_refs)]

use cpu::execute_next_instr;
use sys::Sys;

extern crate num;
#[macro_use]
extern crate num_derive;

mod alu;
mod asm;
mod cart;
mod cpu;
mod data;
mod math;
mod mem_map;
mod print;
mod ram;
mod regs;
mod sys;
mod temp_tests;

fn main() {
    println!("*** rust_gb_2 EMU ***");

    let mut sys = Sys::new();
    temp_tests::run(&mut sys);

    while !sys.crash {
        execute_next_instr(&mut sys);
    }
}
