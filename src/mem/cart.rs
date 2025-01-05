use std::{
    ffi::OsStr,
    fs::{self},
    mem::transmute,
    path::Path,
};

use crate::util::string::slice_to_hex_string;

pub struct Cart {
    pub rom: Vec<u8>,
}

impl Cart {
    pub fn new() -> Self {
        Self { rom: vec![] }
    }

    pub fn load(&mut self, file_path: &str) {
        let path = Path::new(file_path);
        let Some(ext) = path.extension() else {
            panic!("File extension for file {} wasn't specified.", file_path);
        };

        if ext == OsStr::new("gb") {
            let s = path.file_name().unwrap().to_str().unwrap();
            println!("loaded rom: {}", s);
            self.load_from_gb_rom_file(file_path);
        } else {
            todo!()
        }
    }

    pub fn load_from_script_file(&mut self, file_path: &str) {
        self.rom.append(&mut vec![0; 0x100]);

        let script =
            fs::read_to_string(file_path).expect(&format!("Unable to read file {}.", file_path));

        let lines = script.split('\n').collect::<Vec<_>>();

        let mut ops = vec![];
        for line in lines {
            println!("line {}", line);
            if line.starts_with("0x") {
                let hex = u8::from_str_radix(&line[2..=3], 16).unwrap();
                ops.push(hex);
            } else {
                let first = line
                    .split_ascii_whitespace()
                    .nth(0)
                    .unwrap()
                    .replace(";", "");
                let decimal = first.parse::<i8>().unwrap();
                let u: u8 = unsafe { transmute(decimal) };
                ops.push(u);
            }
        }

        self.rom.append(&mut ops);

        self.rom.append(&mut vec![0; 0xFFFF]);
    }

    pub fn load_from_gb_rom_file(&mut self, file_path: &str) {
        self.rom = fs::read(file_path).expect(&format!("Unable to read file {}.", file_path));
    }

    pub fn print_header_info(&self) {
        println!("Cartridge:");

        println!("  Logo Matches: {}", self.check_nintendo_logo());

        let title = &self.rom[0x134..0x144];
        println!("  Title bytes: {}", slice_to_hex_string(title));

        if let Ok(title) = std::str::from_utf8(title) {
            println!("  Title: {}", title);
        }

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
