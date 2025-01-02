use std::ops::Range;

use crate::{
    math::{bit8, bits8},
    regs::{CpuReg16, CpuReg8},
};

#[derive(Clone, Copy, Debug)]
/// Interpretation of a byte of instruction code in ROM.
pub enum Asm {
    // Immediate values.
    Imm8(u8),
    Imm16Hi(u8),
    Imm16Lo(u8),

    // Block 0 instrs.
    Nop,
    Ld_R16_Imm16 { dst: R16 },
    Ld_R16MemP_A { dst: R16Mem },
    Ld_A_R16MemP { src: R16Mem },
    Ld_Imm16P_Sp,

    Inc_R16 { operand: R16 },
    Dec_R16 { operand: R16 },
    Add_Hl_R16 { operand: R16 },

    Inc_R8 { operand: R8 },
    Dec_R8 { operand: R8 },

    Ld_R8_Imm8 { dst: R8 },

    // todo Rlca etc...
    Jr_Imm8,
    Jr_Cond_Imm8 { cond: Cond },

    Stop,

    // Block 1 instrs (8-bit register to register loads).
    Ld_R8_R8 { dst: R8, src: R8 },
    Halt,

    // Block 2 instrs (8-bit arithmetic).
    Add_A_R8 { operand: R8 },
    Adc_A_R8 { operand: R8 },
    Sub_A_R8 { operand: R8 },
    Sbc_A_R8 { operand: R8 },
    And_A_R8 { operand: R8 },
    Xor_A_R8 { operand: R8 },
    Or_A_R8 { operand: R8 },
    Cp_A_R8 { operand: R8 },

    // Block 3 instrs.
    Add_A_Imm8,
    Adc_A_Imm8,
    Sub_A_Imm8,
    Sbc_A_Imm8,
    And_A_Imm8,
    Xor_A_Imm8,
    Or_A_Imm8,
    Cp_A_Imm8,

    Ret_Cond { cond: Cond },
    Ret,
    Reti,
    Jp_Cond_Imm16 { cond: Cond },
    Jp_Imm16,
    Jp_Hl,
    Call_Cond_Imm16 { cond: Cond },
    Call_Imm16,
    Rst_Tgt3 { tgt3: u8 },

    Pop_R16Stk { reg: R16Stk },
    Push_R16Stk { reg: R16Stk },

    // Misc.
    HardLock,
}

impl Asm {
    pub fn imm_type(&self) -> ImmType {
        match self {
            Asm::Ld_R16_Imm16 { .. } => ImmType::Imm16,
            Asm::Ld_Imm16P_Sp => ImmType::Imm16,
            Asm::Ld_R8_Imm8 { .. } => ImmType::Imm8,
            Asm::Jr_Imm8 => ImmType::Imm8,
            Asm::Jr_Cond_Imm8 { .. } => ImmType::Imm8,
            _ => ImmType::None,
        }
    }
}

#[derive(Clone, Copy, FromPrimitive, Debug)]
pub enum R8 {
    B,
    C,
    D,
    E,
    H,
    L,
    HlMem,
    A,
}

impl R8 {
    pub fn from_u8(x: u8) -> Self {
        return num::FromPrimitive::from_u8(x).unwrap();
    }

    pub fn get_reg(self) -> Option<CpuReg8> {
        let reg = match self {
            Self::B => CpuReg8::B,
            Self::C => CpuReg8::C,
            Self::D => CpuReg8::D,
            Self::E => CpuReg8::E,
            Self::H => CpuReg8::H,
            Self::L => CpuReg8::L,
            Self::HlMem => {
                return None;
            }
            Self::A => CpuReg8::A,
        };

        return Some(reg);
    }
}

#[derive(Clone, Copy, FromPrimitive, Debug)]
pub enum R16 {
    BC,
    DE,
    HL,
    SP,
}

impl R16 {
    pub fn from_u8(x: u8) -> Self {
        return num::FromPrimitive::from_u8(x).unwrap();
    }

    pub fn get_reg(self) -> CpuReg16 {
        match self {
            Self::BC => CpuReg16::BC,
            Self::DE => CpuReg16::DE,
            Self::HL => CpuReg16::HL,
            Self::SP => CpuReg16::SP,
        }
    }
}

#[derive(Clone, Copy, FromPrimitive, Debug)]
pub enum R16Stk {
    BC,
    DE,
    HL,
    AF,
}

impl R16Stk {
    pub fn from_u8(x: u8) -> Self {
        return num::FromPrimitive::from_u8(x).unwrap();
    }

    pub fn get_reg(self) -> CpuReg16 {
        match self {
            Self::BC => CpuReg16::BC,
            Self::DE => CpuReg16::DE,
            Self::HL => CpuReg16::HL,
            Self::AF => CpuReg16::AF,
        }
    }
}

#[derive(Clone, Copy, FromPrimitive, Debug)]
pub enum R16Mem {
    BC,
    DE,
    HlInc,
    HlDec,
}

impl R16Mem {
    pub fn from_u8(x: u8) -> Self {
        return num::FromPrimitive::from_u8(x).unwrap();
    }

    /// Returns the corresponding CPU Reg16 and increment behavior.
    pub fn get_reg_inc(self) -> (CpuReg16, i16) {
        let reg = match self {
            Self::BC => CpuReg16::BC,
            Self::DE => CpuReg16::DE,
            Self::HlInc => CpuReg16::HL,
            Self::HlDec => CpuReg16::HL,
        };
        let inc = match self {
            Self::HlInc => 1,
            Self::HlDec => -1,
            _ => 0,
        };

        return (reg, inc);
    }
}

#[derive(Clone, Copy, FromPrimitive, Debug)]
pub enum Cond {
    NZ,
    Z,
    NC,
    C,
}

impl Cond {
    pub fn from_u8(x: u8) -> Cond {
        return num::FromPrimitive::from_u8(x).unwrap();
    }
}

#[derive(Clone, Copy)]
pub enum ImmType {
    None,
    Imm8,
    Imm16,
}

// pub fn interpret(bytes: &[u8], range: Range<usize>) -> Vec<Asm> {
//     let mut idx = range.start;
//     let mut asm_list = vec![];

//     while idx < range.end {
//         let op = bytes[idx];
//         let asm = interpret_opcode(op);
//         idx += 1;

//         // todo: Get following imm values.
//         let imm_type = asm.imm_type();
//         match imm_type {
//             ImmType::Imm8 => {
//                 let imm = bytes[idx];
//                 asm_list.push(Asm::Imm8(imm));
//                 idx += 1;
//             },
//             ImmType::Imm16 => {
//                 let lo = bytes[idx];
//                 asm_list.push(Asm::Imm16Lo(lo));
//                 idx += 1;
//                 let hi = bytes[idx];
//                 asm_list.push(Asm::Imm16Hi(hi));
//                 idx += 1;
//             },
//             _ => {},
//         }

//         asm_list.push(asm);
//     }

//     return asm_list;
// }

pub fn interpret(op: u8) -> Asm {
    if op == 0xCB {
        panic!("0xCB opcode");
    }

    let block = (op >> 6) & 0b11;

    match block {
        0b00 => interpret_block_0_opcode(op),
        0b01 => interpret_block_1_opcode(op),
        0b10 => interpret_block_2_opcode(op),
        0b11 => interpret_block_3_opcode(op),
        _ => Asm::Nop,
    }
}

fn interpret_block_0_opcode(op: u8) -> Asm {
    // NOP
    if op == 0x00 {
        return Asm::Nop;
    }

    // STOP
    if op == 0x10 {
        return Asm::Stop;
    }

    // JR
    if bits8(&op, 2, 0) == 0b000 {
        if bit8(&op, 5) == 0b1 {
            let cond = Cond::from_u8(bits8(&op, 5, 4));
            return Asm::Jr_Cond_Imm8 { cond };
        } else {
            return Asm::Jr_Imm8;
        }
    }

    // RCLA, etc...
    if bits8(&op, 2, 0) == 0b111 {
        todo!("Implement RCLA, etc...");
    }

    // LD R8 IMM8
    if bits8(&op, 2, 0) == 0b110 {
        let dst = R8::from_u8(bits8(&op, 5, 3));
        return Asm::Ld_R8_Imm8 { dst };
    }

    // INC R8, DEC R8
    let operand = R8::from_u8(bits8(&op, 5, 3));
    if bits8(&op, 2, 0) == 0b100 {
        return Asm::Inc_R8 { operand };
    } else if bits8(&op, 2, 0) == 0b101 {
        return Asm::Dec_R8 { operand };
    }

    // INC R16, DEC R16, and ADD HL R16
    let operand = R16::from_u8(bits8(&op, 5, 4));
    if bits8(&op, 3, 0) == 0b0011 {
        return Asm::Inc_R16 { operand };
    } else if bits8(&op, 3, 0) == 0b1011 {
        return Asm::Dec_R16 { operand };
    } else if bits8(&op, 3, 0) == 0b1001 {
        return Asm::Add_Hl_R16 { operand };
    }

    // LD R16 IMM16, LD R16MEMP A, LD A R16MEMP, LD IMM16P SP
    if bits8(&op, 3, 0) == 0b0001 {
        let dst = R16::from_u8(bits8(&op, 5, 4));
        return Asm::Ld_R16_Imm16 { dst };
    } else if bits8(&op, 3, 0) == 0b0010 {
        let dst = R16Mem::from_u8(bits8(&op, 5, 4));
        return Asm::Ld_R16MemP_A { dst };
    } else if bits8(&op, 3, 0) == 0b1010 {
        let src = R16Mem::from_u8(bits8(&op, 5, 4));
        return Asm::Ld_A_R16MemP { src };
    } else if bits8(&op, 3, 0) == 0b1000 {
        return Asm::Ld_Imm16P_Sp;
    }

    panic!("Unexpected block 0 opcode: {:#02x} ({:#02b})", op, op);
}

fn interpret_block_1_opcode(op: u8) -> Asm {
    if op == 0b0111_0110 {
        return Asm::Halt;
    } else {
        let dst = R8::from_u8(bits8(&op, 5, 3));
        let src = R8::from_u8(bits8(&op, 2, 0));

        return Asm::Ld_R8_R8 { dst, src };
    }
}

fn interpret_block_2_opcode(op: u8) -> Asm {
    let operand = R8::from_u8(bits8(&op, 2, 0));

    match bits8(&op, 5, 3) {
        0b000 => Asm::Add_A_R8 { operand },
        0b001 => Asm::Adc_A_R8 { operand },
        0b010 => Asm::Sub_A_R8 { operand },
        0b011 => Asm::Sbc_A_R8 { operand },

        0b100 => Asm::And_A_R8 { operand },
        0b101 => Asm::Xor_A_R8 { operand },
        0b110 => Asm::Or_A_R8 { operand },
        0b111 => Asm::Cp_A_R8 { operand },

        _ => {
            panic!()
        }
    }
}

fn interpret_block_3_opcode(op: u8) -> Asm {
    const INVALID_OPS: &[u8] = &[
        0xD3, 0xDB, 0xDD, 0xE3, 0xE4, 0xEB, 0xEC, 0xED, 0xF4, 0xFC, 0xFD,
    ];
    if INVALID_OPS.contains(&op) {
        return Asm::HardLock;
    }

    if op == 0xCB {
        panic!();
    }

    // ARITH A IMM8
    if bits8(&op, 2, 0) == 0b110 {
        return match bits8(&op, 5, 3) {
            0b000 => Asm::Add_A_Imm8,
            0b001 => Asm::Adc_A_Imm8,
            0b010 => Asm::Sub_A_Imm8,
            0b011 => Asm::Sbc_A_Imm8,

            0b100 => Asm::And_A_Imm8,
            0b101 => Asm::Xor_A_Imm8,
            0b110 => Asm::Or_A_Imm8,
            0b111 => Asm::Cp_A_Imm8,

            _ => {
                panic!()
            }
        };
    }

    // RET COND, RET, RETI
    let cond = Cond::from_u8(bits8(&op, 4, 3));
    if bit8(&op, 5) == 0b0 && bits8(&op, 2, 0) == 0b000 {
        return Asm::Ret_Cond { cond };
    }
    if bits8(&op, 5, 0) == 0b00_1001 {
        return Asm::Ret;
    }
    if bits8(&op, 5, 0) == 0b01_1001 {
        return Asm::Reti;
    }

    // JP COND IMM16, JP IMM16, JP HL
    if bit8(&op, 5) == 0b0 && bits8(&op, 2, 0) == 0b010 {
        return Asm::Jp_Cond_Imm16 { cond };
    }
    if bits8(&op, 5, 0) == 0b00_0011 {
        return Asm::Jp_Imm16;
    }
    if bits8(&op, 5, 0) == 0b10_1001 {
        return Asm::Jp_Hl;
    }

    // CALL COND IMM16, CALL IMM16, RST TGT3
    if bit8(&op, 5) == 0b0 && bits8(&op, 2, 0) == 0b110 {
        return Asm::Call_Cond_Imm16 { cond };
    }
    if bits8(&op, 5, 0) == 0b00_1101 {
        return Asm::Call_Imm16;
    }
    if bits8(&op, 2, 0) == 0b111 {
        let tgt3 = bits8(&op, 5, 3);
        return Asm::Rst_Tgt3 { tgt3 };
    }

    // POP R16STK, PUSH R16STK
    let reg = R16Stk::from_u8(bits8(&op, 5, 4));
    if bits8(&op, 3, 0) == 0b0001 {
        return Asm::Pop_R16Stk { reg };
    }
    if bits8(&op, 3, 0) == 0b0101 {
        return Asm::Push_R16Stk { reg };
    }

    panic!("Unexpected block 3 opcode: {:#02x} ({:#02b})", op, op);
}
