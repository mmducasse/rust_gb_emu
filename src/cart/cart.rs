use std::{
    ffi::OsStr,
    fs::{self},
    mem::transmute,
    path::Path,
};

use num::FromPrimitive;

use crate::{mem::map::Addr, util::string::slice_to_hex_string};

use super::type_::CartType;

pub struct Cart {
    type_: CartType,

    rom_bank_sel: u8,
    rom: Vec<u8>,

    ram_bank_sel: u8,
    ram: Vec<u8>,
}

impl Cart {
    pub fn new() -> Self {
        Self {
            type_: CartType::RomOnly,

            rom_bank_sel: 0,
            rom: vec![],

            ram_bank_sel: 0,
            ram: vec![],
        }
    }

    pub fn load(&mut self, file_path: &str) {
        let path = Path::new(file_path);
        let Some(ext) = path.extension() else {
            panic!("File extension for file {} wasn't specified.", file_path);
        };

        if ext != OsStr::new("gb") {
            panic!("Couldnt load gb rom. Expected a \".gb\" file.");
        }

        let s = path.file_name().unwrap().to_str().unwrap();
        println!("loaded rom: {}", s);
        let program = fs::read(file_path).expect(&format!("Unable to read file {}.", file_path));

        let type_ = program[0x0147];
        self.type_ = CartType::from_u8(type_).unwrap();
        if !self.type_.is_supported_by_emu() {
            panic!(
                "EMU doesn't currenty support cartridge type: {:?} ({})",
                self.type_, self.type_ as u8
            );
        }

        let rom_size = self.type_.max_rom_size();
        self.rom = vec![0; rom_size];
        {
            let (rom, _) = self.rom.split_at_mut(program.len());
            rom.copy_from_slice(&program);
        }

        let ram_size = self.type_.max_ram_size();
        self.ram = vec![0; ram_size];

        self.print_header_info();
    }

    pub fn rd(&self, addr: Addr) -> u8 {
        return self.rom[addr as usize];
    }

    pub fn wr(&mut self, addr: Addr, data: u8) {
        self.rom[addr as usize] = data;
    }

    pub fn print_header_info(&self) {
        println!("Cartridge:");

        println!("  Logo Matches: {}", self.check_nintendo_logo());

        let title = &self.rom[0x134..0x144];
        println!("  Title bytes: {}", slice_to_hex_string(title));

        if let Ok(title) = std::str::from_utf8(title) {
            println!("  Title: {}", title);
        }

        println!("  Type: {:?} ({})", self.type_, self.type_ as u8);

        let (checksum, is_matching) = self.check_header_checksum();
        println!("  Checksum ({:#02x}) Matches: {}", checksum, is_matching);

        println!("  Total rom size: {}", self.rom.len());

        println!();
    }

    pub fn check_nintendo_logo(&self) -> bool {
        let logo_path = ".\\assets\\files\\nintendo_logo.txt";
        let logo_text = fs::read_to_string(logo_path)
            .expect(&format!("Unable to read nintendo logo file {}.", logo_path));
        let logo_bytes = logo_text
            .split_ascii_whitespace()
            .map(|s| u8::from_str_radix(&s, 16).unwrap())
            .collect::<Vec<_>>();

        let cart_rom_span = &self.rom[0x104..0x134];

        for (logo, cart) in logo_bytes.iter().zip(cart_rom_span.iter()) {
            // println!("logo = {:#02x}, cart = {:#02x}", logo, cart);
            if *logo != *cart {
                return false;
            }
        }

        return true;
    }

    pub fn check_header_checksum(&self) -> (u8, bool) {
        /*
        uint8_t checksum = 0;
        for (uint16_t address = 0x0134; address <= 0x014C; address++) {
            checksum = checksum - rom[address] - 1;
        }
         */
        let mut checksum = 0;

        for byte in self.rom[0x134..=0x14C].iter() {
            checksum = u8::wrapping_sub(checksum, *byte);
            checksum = u8::wrapping_sub(checksum, 1);
        }

        let is_matching = checksum == self.rom[0x14D];

        return (checksum, is_matching);
    }
}
