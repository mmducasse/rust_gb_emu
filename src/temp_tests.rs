use strum::IntoEnumIterator;

use crate::{mem::map::MemSection, sys::Sys};

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
    // let rom_file = ".\\assets\\test_roms\\cpu_instrs\\individual\\01-special.gb";
    // let rom_file = ".\\assets\\test_roms\\cpu_instrs\\individual\\02-interrupts.gb";
    // let rom_file = ".\\assets\\test_roms\\cpu_instrs\\individual\\03-op sp,hl.gb";
    // let rom_file = ".\\assets\\test_roms\\cpu_instrs\\individual\\04-op r,imm.gb";
    // let rom_file = ".\\assets\\test_roms\\cpu_instrs\\individual\\05-op rp.gb";
    // sys.cart.load_from_gb_rom_file(rom_file);

    // sys.cart.print_header_info();
}
