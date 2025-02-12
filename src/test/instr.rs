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
