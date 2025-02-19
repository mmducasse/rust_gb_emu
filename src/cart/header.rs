use num::FromPrimitive;

use crate::cart::consts::{RAM_BANK_SIZE, ROM_BANK_SIZE};

use super::type_::CartType;

const NINTENDO_LOGO: &[u8] = include_bytes!("..\\..\\assets\\files\\nintendo_logo.txt");

/// The interpretation of the data in the cartridge ROM header (addresses 0x0100-0x014F).
pub struct CartHeader {
    title: Option<String>,
    pub cgb_flag: u8,
    pub cart_type: CartType,
    pub rom_bank_count: usize,
    pub ram_bank_count: usize,
    pub is_nintendo_logo_matching: bool,
    pub checksum: u8,
    pub is_checksum_matching: bool,
}

impl CartHeader {
    /// Parses the entire cartridge ROM and returns the interpretation of its header.
    pub fn parse(rom: &[u8]) -> Result<Self, String> {
        let title = {
            let title = &rom[0x134..=0x142];

            // Workaround for bug where null character would
            // be included in title string.
            let title = title
                .iter()
                .take_while(|c| **c != 0)
                .map(|c| *c)
                .collect::<Vec<_>>();

            std::str::from_utf8(&title)
                .ok()
                .map(|s| s.trim().to_owned())
        };

        let cgb_flag = rom[0x0143];

        let cart_type_id = rom[0x0147];
        let Some(cart_type) = CartType::from_u8(cart_type_id) else {
            return Err(format!(
                "Invalid cart type id at header address [0x0147]: {}",
                cart_type_id
            ));
        };

        let rom_banks_code = rom[0x0148];
        let Some(rom_bank_count) = get_rom_bank_count(rom_banks_code) else {
            return Err(format!(
                "Unknown cartridge rom size code in header address [0x0148]: {}",
                rom_banks_code
            ));
        };

        let ram_banks_code = rom[0x0149];
        let Some(ram_bank_count) = get_ram_bank_count(ram_banks_code) else {
            return Err(format!(
                "Unknown cartridge ram size code in header address [0x0149]: {}",
                ram_banks_code
            ));
        };

        let is_nintendo_logo_matching = check_nintendo_logo(rom);

        let (checksum, is_matching) = check_header_checksum(rom);

        return Ok(Self {
            title,
            cgb_flag,
            cart_type,
            rom_bank_count,
            ram_bank_count,
            is_nintendo_logo_matching,
            checksum,
            is_checksum_matching: is_matching,
        });
    }

    pub fn title(&self) -> &str {
        if let Some(title) = &self.title {
            title
        } else {
            ""
        }
    }

    pub fn print(&self) {
        println!("Cartridge Header:");

        if let Some(title) = &self.title {
            println!("  Title: {}", title);
        }

        let compatibility = match self.cgb_flag {
            0x80 => "CGB (backward compatibile)",
            0xC0 => "CGB only",
            _ => "DMG only",
        };
        println!("  Compatibility = 0x{:0>2X}: {}", self.cgb_flag, compatibility);

        println!("  Type: {:?} ({})", self.cart_type, self.cart_type as u8);

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

        println!("  Logo Matches: {}", self.is_nintendo_logo_matching);

        println!(
            "  Checksum ({:#02x}) Matches: {}",
            self.checksum, self.is_checksum_matching
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

fn check_nintendo_logo(rom: &[u8]) -> bool {
    let logo_text = String::from_utf8(NINTENDO_LOGO.to_vec())
        .expect(&format!("Unable to read nintendo logo file.",));

    let logo_bytes = logo_text
        .split_ascii_whitespace()
        .map(|s| u8::from_str_radix(&s, 16).unwrap())
        .collect::<Vec<_>>();

    let cart_rom_span = &rom[0x104..0x134];

    for (logo, cart) in logo_bytes.iter().zip(cart_rom_span.iter()) {
        if *logo != *cart {
            return false;
        }
    }

    return true;
}

fn get_rom_bank_count(code: u8) -> Option<usize> {
    let count = match code {
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
            return None;
        }
    };

    return Some(count);
}

fn get_ram_bank_count(code: u8) -> Option<usize> {
    let count = match code {
        0x00 => 0,
        // 0x01 => unused
        0x02 => 1,
        0x03 => 4,
        0x04 => 16,
        0x05 => 8,
        _ => {
            return None;
        }
    };

    return Some(count);
}
