use std::{
    ffi::OsStr,
    fs::{self},
    mem::transmute,
    path::Path,
};

use num::FromPrimitive;

use crate::{
    cart::header::CartHeader,
    mem::sections::Addr,
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

    pub fn load(&mut self, file_path: &str, verbose: bool) {
        let path = Path::new(file_path);
        let Some(ext) = path.extension() else {
            panic!("File extension for file {} wasn't specified.", file_path);
        };

        if ext != OsStr::new("gb") {
            panic!("Couldnt load gb rom. Expected a \".gb\" file.");
        }

        let rom = fs::read(file_path).expect(&format!("Unable to read file {}.", file_path));
        if verbose {
            println!(
                "loaded rom: {}",
                path.file_name().unwrap().to_str().unwrap()
            );
        }

        let cart_type_id = rom[0x0147];
        let cart_type = CartType::from_u8(cart_type_id).unwrap();
        if !cart_type.is_supported_by_emu() {
            panic!(
                "Cartridge type not supported: {:?} ({})",
                cart_type, cart_type_id
            );
        }

        let header = CartHeader::parse(&rom);
        if verbose {
            header.print();
        }

        self.hw = Self::create_hw(&header, rom);
    }

    fn create_hw(header: &CartHeader, program: Vec<u8>) -> Box<dyn CartHw> {
        let mut cart_hw: Box<dyn CartHw> = match header.cart_type.mbc_type() {
            Some(MbcType::Mbc1) => {
                Box::new(HwMbc1::new(header.rom_bank_count, header.ram_bank_count))
            }
            Some(MbcType::Mbc2) => todo!(),
            Some(MbcType::Mbc3) => todo!(),
            Some(MbcType::Mbc5) => todo!(),
            Some(MbcType::Mbc6) => todo!(),
            Some(MbcType::Mbc7) => todo!(),
            None => Box::new(HwRomOnly::new(header.rom_bank_count)),
        };

        copy_from_safe(cart_hw.rom_mut(), &program);

        return cart_hw;
    }

    pub fn read(&self, addr: Addr) -> u8 {
        return self.hw.read(addr);
    }

    pub fn write(&mut self, addr: Addr, data: u8) {
        self.hw.write(addr, data);
    }

    pub fn rom(&self) -> &[u8] {
        self.hw.rom()
    }

    pub fn ram(&self) -> &[u8] {
        self.hw.ram()
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
}
