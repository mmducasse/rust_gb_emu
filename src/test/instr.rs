use crate::cpu::instr::{decode, ImmType};

pub fn test_all_opcodes() {
    // Block 0 - 3
    for op in 0x00..=0xFF {
        let value = match decode(op, false) {
            Ok(instr) => {
                let len = match instr.imm_type() {
                    ImmType::None => 1,
                    ImmType::Imm8 => 2,
                    ImmType::Imm16 => 3,
                };
                format!("{:?}  (len = {})", instr, len)
            }
            Err(err) => err,
        };
        println!("${:0>2X} = {}", op, value);
    }

    // 0xCB prefix
    for op in 0x00..=0xFF {
        let value = match decode(op, true) {
            Ok(instr) => {
                let len = match instr.imm_type() {
                    ImmType::None => 2,
                    ImmType::Imm8 => 3,
                    ImmType::Imm16 => 4,
                };
                format!("{:?}  (len = {})", instr, len)
            }
            Err(err) => err,
        };
        println!("$CB{:0>2X} = {}", op, value);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cpu::regs::{CpuReg16, CpuReg8},
        sys::Sys,
    };

    #[test]
    fn test_ld() {
        let mut sys = Sys::new();
        sys.mem
            .cart
            .load(".\\assets\\files\\custom_roms\\ld_r8_r8\\rom.gb");
        sys.run();

        assert_eq!(sys.regs.get_8(CpuReg8::B), 1);
        assert_eq!(sys.regs.get_8(CpuReg8::C), 2);
        assert_eq!(sys.regs.get_8(CpuReg8::D), 3);
        assert_eq!(sys.regs.get_8(CpuReg8::E), 4);

        assert_eq!(sys.regs.get_8(CpuReg8::H), 5);
        assert_eq!(sys.regs.get_8(CpuReg8::L), 6);
        assert_eq!(sys.mem.read(sys.regs.get_16(CpuReg16::HL)), 7);
        assert_eq!(sys.regs.get_8(CpuReg8::A), 8);
    }
}
