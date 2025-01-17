use std::{
    ffi::OsStr,
    fs::{self},
    mem::transmute,
    path::Path,
};

use num::FromPrimitive;

use crate::{
    mem::map::Addr,
    util::{slice::copy_from_safe, string::slice_to_hex_string},
};

use super::{
    cart_hw::CartHw,
    hw_empty::HwEmpty,
    hw_mbc1::HwMbc1,
    hw_rom_only::HwRomOnly,
    type_::{CartType, MbcType},
};

pub struct Cart {
    hw: Box<dyn CartHw>,
}

impl Cart {
    pub fn new() -> Self {
        Self {
            hw: Box::new(HwEmpty),
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

        println!(
            "loaded rom: {}",
            path.file_name().unwrap().to_str().unwrap()
        );
        let program = fs::read(file_path).expect(&format!("Unable to read file {}.", file_path));

        let cart_type_id = program[0x0147];
        let cart_type = CartType::from_u8(cart_type_id).unwrap();
        if !cart_type.is_supported_by_emu() {
            panic!(
                "Cartridge type not supported: {:?} ({})",
                cart_type, cart_type_id
            );
        }

        self.hw = Self::create_hw(cart_type, program);

        self.print_header_info();
    }

    fn create_hw(cart_type: CartType, program: Vec<u8>) -> Box<dyn CartHw> {
        let mut cart_hw: Box<dyn CartHw> = match cart_type.mbc_type() {
            Some(MbcType::Mbc1) => Box::new(HwMbc1::new()),
            Some(MbcType::Mbc2) => todo!(),
            Some(MbcType::Mbc3) => todo!(),
            Some(MbcType::Mbc5) => todo!(),
            Some(MbcType::Mbc6) => todo!(),
            Some(MbcType::Mbc7) => todo!(),
            None => Box::new(HwRomOnly::new()),
        };

        copy_from_safe(cart_hw.rom_mut(), &program);

        return cart_hw;
    }

    pub fn rd(&self, addr: Addr) -> u8 {
        return self.hw.rd(addr);
    }

    pub fn wr(&mut self, addr: Addr, data: u8) {
        self.hw.wr(addr, data);
    }

    pub fn rom(&self) -> &[u8] {
        self.hw.rom()
    }

    pub fn ram(&self) -> &[u8] {
        self.hw.ram()
    }

    pub fn print_header_info(&self) {
        println!("Cartridge:");

        println!("  Logo Matches: {}", self.check_nintendo_logo());

        let title = &self.hw.rom()[0x134..0x144];
        println!("  Title bytes: {}", slice_to_hex_string(title));

        if let Ok(title) = std::str::from_utf8(title) {
            println!("  Title: {}", title);
        }

        let cart_type_id = self.hw.rom()[0x0147];
        let cart_type = CartType::from_u8(cart_type_id).unwrap();
        println!("  Type: {:?} ({})", cart_type, cart_type as u8);

        let (checksum, is_matching) = self.check_header_checksum();
        println!("  Checksum ({:#02x}) Matches: {}", checksum, is_matching);

        println!("  Total rom size: {}", self.hw.rom().len());

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

        let cart_rom_span = &self.hw.rom()[0x104..0x134];

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

        for byte in self.hw.rom()[0x134..=0x14C].iter() {
            checksum = u8::wrapping_sub(checksum, *byte);
            checksum = u8::wrapping_sub(checksum, 1);
        }

        let is_matching = checksum == self.hw.rom()[0x14D];

        return (checksum, is_matching);
    }
}
