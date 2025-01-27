use num::FromPrimitive;

use crate::{
    cart::consts::{RAM_BANK_SIZE, ROM_BANK_SIZE},
    util::string::slice_to_hex_string,
};

use super::{cart::Cart, type_::CartType};

pub struct CartHeader {
    pub title: Option<String>,
    pub cart_type: CartType,
    pub rom_bank_count: usize,
    pub ram_bank_count: usize,
    pub checksum: u8,
    pub is_checksum_matching: bool,
}

impl CartHeader {
    pub fn parse(rom: &[u8]) -> Self {
        let title = {
            let title = &rom[0x134..0x144];
            std::str::from_utf8(title).ok().map(|s| s.to_owned())
        };

        let cart_type_id = rom[0x0147];
        let cart_type = CartType::from_u8(cart_type_id).unwrap();

        let (checksum, is_matching) = check_header_checksum(rom);

        let rom_bank_count = get_rom_bank_count(rom);
        let ram_bank_count = get_ram_bank_count(rom);

        return Self {
            title,
            cart_type,
            rom_bank_count: rom_bank_count,
            ram_bank_count: ram_bank_count,
            checksum,
            is_checksum_matching: is_matching,
        };
    }

    pub fn print(&self) {
        println!("Cartridge Header:");

        //println!("  Logo Matches: {}", self.check_nintendo_logo());

        if let Some(title) = &self.title {
            println!("  Title: {}", title);
        }

        println!("  Type: {:?} ({})", self.cart_type, self.cart_type as u8);
        println!(
            "  Checksum ({:#02x}) Matches: {}",
            self.checksum, self.is_checksum_matching
        );

        println!(
            "  Rom: {} banks, 0x{:0>8X} bytes",
            self.rom_bank_count,
            self.rom_bank_count * ROM_BANK_SIZE
        );
        println!(
            "  Ram: {} banks, 0x{:0>8X} bytes",
            self.ram_bank_count,
            self.ram_bank_count * RAM_BANK_SIZE
        );

        println!();
    }
}

fn check_header_checksum(rom: &[u8]) -> (u8, bool) {
    /*
    uint8_t checksum = 0;
    for (uint16_t address = 0x0134; address <= 0x014C; address++) {
        checksum = checksum - rom[address] - 1;
    }
     */
    let mut checksum = 0;

    for byte in rom[0x134..=0x14C].iter() {
        checksum = u8::wrapping_sub(checksum, *byte);
        checksum = u8::wrapping_sub(checksum, 1);
    }

    let is_matching = checksum == rom[0x14D];

    return (checksum, is_matching);
}

fn get_rom_bank_count(rom: &[u8]) -> usize {
    let code = rom[0x0148];

    return match code {
        0x00 => 2,
        0x01 => 4,
        0x02 => 8,
        0x03 => 16,
        0x04 => 32,
        0x05 => 64,
        0x06 => 128,
        0x07 => 256,
        0x08 => 512,
        _ => {
            panic!("Unknown cartridge rom size code: {}", code);
        }
    };
}

fn get_ram_bank_count(rom: &[u8]) -> usize {
    let code = rom[0x0149];

    return match code {
        0x00 => 0,
        // 0x01 => unused
        0x02 => 1,
        0x03 => 4,
        0x04 => 16,
        0x05 => 8,
        _ => {
            panic!("Unknown cartridge ram size code: {}", code);
        }
    };
}
