use strum::IntoEnumIterator;

use crate::{mem_map::MemSection, sys::Sys};

pub fn run(sys: &mut Sys) {
    // Mem sections
    for section in MemSection::iter() {
        let start = section.start_addr();
        let size = section.size();

        println!("{:?}: {:#04x} ({:#04x})", section, start, size);
    }

    // CPU Registers
    sys.regs.print();

    // Cartridge
    sys.cart
        .load_from_gb_rom_file(".\\assets\\test_roms\\cpu_instrs\\individual\\03-op sp,hl.gb");

    sys.cart.print_header_info();
}
