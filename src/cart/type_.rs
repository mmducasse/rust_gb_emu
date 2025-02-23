#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MbcType {
    Mbc1,
    Mbc2,
    Mbc3,
    Mbc5,
    Mbc6,
    Mbc7,
}

// impl MbcType {
//     pub fn max_rom_size(self) -> usize {
//         match self {
//             MbcType::Mbc1 => MB_2,
//             MbcType::Mbc2 => KB_256,
//             MbcType::Mbc3 => MB_2,
//             MbcType::Mbc5 => MB_8,
//             MbcType::Mbc6 => todo!(),
//             MbcType::Mbc7 => todo!(),
//         }
//     }

//     pub fn max_ram_size(self) -> usize {
//         match self {
//             MbcType::Mbc1 => KB_32,
//             MbcType::Mbc2 => B_512,
//             MbcType::Mbc3 => KB_64,
//             MbcType::Mbc5 => KB_128,
//             MbcType::Mbc6 => todo!(),
//             MbcType::Mbc7 => todo!(),
//         }
//     }
// }

#[derive(Clone, Copy, PartialEq, Eq, Debug, FromPrimitive)]
pub enum CartType {
    RomOnly = 0x00,
    Mbc1 = 0x01,
    Mbc1_Ram = 0x02,
    Mbc1_Ram_Battery = 0x03,
    Mbc2 = 0x05,
    Mbc2_Battery = 0x06,
    Rom_Ram = 0x08,
    Rom_Ram_Battery = 0x09,
    Mmm01 = 0x0B,
    Mmm01_Ram = 0x0C,
    Mmm01_Ram_Battery = 0x0D,
    Mbc3_Timer_Battery = 0x0F,
    Mbc3_Timer_Ram_Battery = 0x10,
    Mbc3 = 0x11,
    Mbc3_Ram = 0x12,
    Mbc3_Ram_Battery = 0x13,
    Mbc5 = 0x19,
    Mbc5_Ram = 0x1A,
    Mbc5_Ram_Battery = 0x1B,
    Mbc5_Rumble = 0x1C,
    Mbc5_Rumble_Ram = 0x1D,
    Mbc5_Rumble_Ram_Battery = 0x1E,
    Mbc6 = 0x20,
    Mbc7_Sensor_Rumble_Ram_Battery = 0x22,
    Pocket_Camera = 0xFC,
    Bandai_Tama5 = 0xFD,
    Hu3 = 0xFE,
    HuC1_Ram_Battery = 0xFF,
}

impl CartType {
    pub fn is_supported_by_emu(self) -> bool {
        use CartType::*;

        return false
            == matches!(
                self,
                Mmm01
                    | Mmm01_Ram
                    | Mmm01_Ram_Battery
                    | Pocket_Camera
                    | Bandai_Tama5
                    | Hu3
                    | HuC1_Ram_Battery
            );
    }

    pub fn mbc_type(self) -> Option<MbcType> {
        use CartType::*;

        let type_ = match self {
            Mbc1 | Mbc1_Ram | Mbc1_Ram_Battery => MbcType::Mbc1,
            Mbc2 | Mbc2_Battery => MbcType::Mbc2,
            Mbc3_Timer_Battery | Mbc3_Timer_Ram_Battery | Mbc3 | Mbc3_Ram | Mbc3_Ram_Battery => {
                MbcType::Mbc3
            }
            Mbc5
            | Mbc5_Ram
            | Mbc5_Ram_Battery
            | Mbc5_Rumble
            | Mbc5_Rumble_Ram
            | Mbc5_Rumble_Ram_Battery => MbcType::Mbc5,
            Mbc6 => MbcType::Mbc6,
            Mbc7_Sensor_Rumble_Ram_Battery => MbcType::Mbc7,

            _ => {
                return None;
            }
        };

        return Some(type_);
    }

    // pub fn max_rom_size(self) -> usize {
    //     if self.is_rom_only() {
    //         return KB_32;
    //     }
    //     if let Some(mbc_type) = self.mbc_type() {
    //         return mbc_type.max_rom_size();
    //     }

    //     todo!()
    // }

    // pub fn max_ram_size(self) -> usize {
    //     if self.is_rom_only() {
    //         return 0;
    //     }
    //     if let Some(mbc_type) = self.mbc_type() {
    //         return mbc_type.max_ram_size();
    //     }

    //     todo!()
    // }
}
