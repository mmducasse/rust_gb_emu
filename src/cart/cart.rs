use std::{
    ffi::OsStr,
    fs::{self},
    path::Path,
};

use num::FromPrimitive;

use crate::{cart::header::CartHeader, mem::Addr, util::slice::copy_from_safe};

use super::{
    cart_hw::CartHw,
    hw_mbc1::HwMbc1,
    hw_mbc3::HwMbc3,
    hw_rom_only::HwRomOnly,
    type_::{CartType, MbcType},
};

/// Represents a GameBoy cartridge.
pub struct Cart {
    header: CartHeader,
    hw: Box<dyn CartHw>,
}

impl Cart {
    /// Attempts to load a gb file at the given `file_path` and create a new `Cart` instance.
    pub fn load_from(file_path: &str, verbose: bool) -> Result<Self, String> {
        let path = Path::new(file_path);
        let Some(ext) = path.extension() else {
            return Err(format!(
                "File extension for file {} wasn't specified.",
                file_path
            ));
        };

        if ext != OsStr::new("gb") && ext != OsStr::new("gbc") {
            return Err(format!(
                "Couldnt load gb rom. Expected a \".gb\" or \".gbc\" file."
            ));
        }

        let Ok(rom) = fs::read(file_path) else {
            return Err(format!("Unable to read file {}.", file_path));
        };

        let cart_type_id = rom[0x0147];
        let Some(cart_type) = CartType::from_u8(cart_type_id) else {
            return Err(format!("Invalid cart type ID in header: {}.", cart_type_id));
        };

        if !cart_type.is_supported_by_emu() {
            return Err(format!(
                "Cartridge type not supported: {:?} ({})",
                cart_type, cart_type_id
            ));
        }

        let header = CartHeader::parse(&rom)?;
        if verbose {
            header.print();
        }

        let hw = Self::create_hw(&header, &rom);

        return Ok(Self { header, hw });
    }

    /// Creates the specific cartridge hardware implementation for the cartridge type
    /// specified in the header.
    fn create_hw(header: &CartHeader, rom: &[u8]) -> Box<dyn CartHw> {
        let mut cart_hw: Box<dyn CartHw> = match header.cart_type.mbc_type() {
            Some(MbcType::Mbc1) => {
                Box::new(HwMbc1::new(header.rom_bank_count, header.ram_bank_count))
            }
            Some(MbcType::Mbc2) => todo!(),
            Some(MbcType::Mbc3) => {
                Box::new(HwMbc3::new(header.rom_bank_count, header.ram_bank_count))
            }
            Some(MbcType::Mbc5) => todo!(),
            Some(MbcType::Mbc6) => todo!(),
            Some(MbcType::Mbc7) => todo!(),
            None => Box::new(HwRomOnly::new(header.rom_bank_count)),
        };

        copy_from_safe(cart_hw.rom_mut(), rom);

        return cart_hw;
    }

    pub fn read(&self, addr: Addr) -> u8 {
        return self.hw.read(addr);
    }

    pub fn write(&mut self, addr: Addr, data: u8) {
        self.hw.write(addr, data);
    }

    pub fn ram(&self) -> &[u8] {
        self.hw.ram()
    }

    pub fn ram_mut(&mut self) -> &mut [u8] {
        self.hw.ram_mut()
    }

    pub fn header(&self) -> &CartHeader {
        &self.header
    }
}
