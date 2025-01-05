use crate::{
    cpu::regs::{CpuReg16, CpuReg8},
    util::math::{bit8, bits8},
};

#[derive(Clone, Copy, Debug)]
/// Interpretation of a 1-byte opcode instruction in ROM.
pub enum Instr {
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

    Rlca,
    RRca,
    Rla,
    Rra,
    Daa,
    Cpl,
    Scf,
    Ccf,

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

    Ldh_CP_A,
    Ldh_Imm8P_A,
    Ld_Imm16P_A,
    Ldh_A_CP,
    Ldh_A_Imm8P,
    Ld_A_Imm16P,

    Add_Sp_Imm8,
    Ld_Hl_SpImm8,
    Ld_Sp_Hl,

    Di,
    Ei,

    // 0xCB prefix instrs.
    Rlc_R8 { operand: R8 },
    Rrc_R8 { operand: R8 },
    Rl_R8 { operand: R8 },
    Rr_R8 { operand: R8 },
    Sla_R8 { operand: R8 },
    Sra_R8 { operand: R8 },
    Swap_R8 { operand: R8 },
    Srl_R8 { operand: R8 },

    Bit_B3_R8 { b3: u8, operand: R8 },
    Res_B3_R8 { b3: u8, operand: R8 },
    Set_B3_R8 { b3: u8, operand: R8 },

    // Misc.
    HardLock,
}

impl Instr {
    pub const CB_PREFIX: u8 = 0xCB;

    pub fn imm_type(&self) -> ImmType {
        match self {
            Instr::Ld_R16_Imm16 { .. } => ImmType::Imm16,
            Instr::Ld_Imm16P_Sp => ImmType::Imm16,
            Instr::Ld_R8_Imm8 { .. } => ImmType::Imm8,
            Instr::Jr_Imm8 => ImmType::Imm8,
            Instr::Jr_Cond_Imm8 { .. } => ImmType::Imm8,

            Instr::Add_A_Imm8 => ImmType::Imm8,
            Instr::Adc_A_Imm8 => ImmType::Imm8,
            Instr::Sub_A_Imm8 => ImmType::Imm8,
            Instr::Sbc_A_Imm8 => ImmType::Imm8,
            Instr::And_A_Imm8 => ImmType::Imm8,
            Instr::Xor_A_Imm8 => ImmType::Imm8,
            Instr::Or_A_Imm8 => ImmType::Imm8,
            Instr::Cp_A_Imm8 => ImmType::Imm8,

            Instr::Jp_Cond_Imm16 { .. } => ImmType::Imm16,
            Instr::Jp_Imm16 => ImmType::Imm16,
            Instr::Call_Cond_Imm16 { .. } => ImmType::Imm16,
            Instr::Call_Imm16 => ImmType::Imm16,

            Instr::Ldh_Imm8P_A => ImmType::Imm8,
            Instr::Ld_Imm16P_A => ImmType::Imm16,
            Instr::Ldh_A_Imm8P => ImmType::Imm8,
            Instr::Ld_A_Imm16P => ImmType::Imm16,

            Instr::Add_Sp_Imm8 => ImmType::Imm8,
            Instr::Ld_Hl_SpImm8 => ImmType::Imm8,

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

pub fn decode(op: u8, has_cb_prefix: bool) -> Instr {
    if has_cb_prefix {
        return decode_cp_prefix_opcode(op);
    }

    let block = bits8(&op, 7, 6);
    return match block {
        0b00 => decode_block_0_opcode(op),
        0b01 => decode_block_1_opcode(op),
        0b10 => decode_block_2_opcode(op),
        0b11 => decode_block_3_opcode(op),
        _ => unreachable!(),
    };
}

fn decode_block_0_opcode(op: u8) -> Instr {
    // NOP
    if op == 0x00 {
        return Instr::Nop;
    }

    // STOP
    if op == 0x10 {
        return Instr::Stop;
    }

    // JR
    if bits8(&op, 2, 0) == 0b000 {
        if bit8(&op, 5) == 0b1 {
            let cond = Cond::from_u8(bits8(&op, 4, 3));
            return Instr::Jr_Cond_Imm8 { cond };
        } else {
            return Instr::Jr_Imm8;
        }
    }

    // RCLA, etc...
    if bits8(&op, 2, 0) == 0b111 {
        return match bits8(&op, 7, 3) {
            0b0000_0 => Instr::Rlca,
            0b0000_1 => Instr::RRca,
            0b0001_0 => Instr::Rla,
            0b0001_1 => Instr::Rra,

            0b0010_0 => Instr::Daa,
            0b0010_1 => Instr::Cpl,
            0b0011_0 => Instr::Scf,
            0b0011_1 => Instr::Ccf,

            _ => unreachable!(),
        };
    }

    // LD R8 IMM8
    if bits8(&op, 2, 0) == 0b110 {
        let dst = R8::from_u8(bits8(&op, 5, 3));
        return Instr::Ld_R8_Imm8 { dst };
    }

    // INC R8, DEC R8
    let operand = R8::from_u8(bits8(&op, 5, 3));
    if bits8(&op, 2, 0) == 0b100 {
        return Instr::Inc_R8 { operand };
    } else if bits8(&op, 2, 0) == 0b101 {
        return Instr::Dec_R8 { operand };
    }

    // INC R16, DEC R16, and ADD HL R16
    let operand = R16::from_u8(bits8(&op, 5, 4));
    if bits8(&op, 3, 0) == 0b0011 {
        return Instr::Inc_R16 { operand };
    } else if bits8(&op, 3, 0) == 0b1011 {
        return Instr::Dec_R16 { operand };
    } else if bits8(&op, 3, 0) == 0b1001 {
        return Instr::Add_Hl_R16 { operand };
    }

    // LD R16 IMM16, LD R16MEMP A, LD A R16MEMP, LD IMM16P SP
    if bits8(&op, 3, 0) == 0b0001 {
        let dst = R16::from_u8(bits8(&op, 5, 4));
        return Instr::Ld_R16_Imm16 { dst };
    } else if bits8(&op, 3, 0) == 0b0010 {
        let dst = R16Mem::from_u8(bits8(&op, 5, 4));
        return Instr::Ld_R16MemP_A { dst };
    } else if bits8(&op, 3, 0) == 0b1010 {
        let src = R16Mem::from_u8(bits8(&op, 5, 4));
        return Instr::Ld_A_R16MemP { src };
    } else if bits8(&op, 3, 0) == 0b1000 {
        return Instr::Ld_Imm16P_Sp;
    }

    panic!("Unexpected block 0 opcode: {:#02x} ({:#02b})", op, op);
}

fn decode_block_1_opcode(op: u8) -> Instr {
    if op == 0b0111_0110 {
        return Instr::Halt;
    } else {
        let dst = R8::from_u8(bits8(&op, 5, 3));
        let src = R8::from_u8(bits8(&op, 2, 0));

        return Instr::Ld_R8_R8 { dst, src };
    }
}

fn decode_block_2_opcode(op: u8) -> Instr {
    let operand = R8::from_u8(bits8(&op, 2, 0));

    match bits8(&op, 5, 3) {
        0b000 => Instr::Add_A_R8 { operand },
        0b001 => Instr::Adc_A_R8 { operand },
        0b010 => Instr::Sub_A_R8 { operand },
        0b011 => Instr::Sbc_A_R8 { operand },

        0b100 => Instr::And_A_R8 { operand },
        0b101 => Instr::Xor_A_R8 { operand },
        0b110 => Instr::Or_A_R8 { operand },
        0b111 => Instr::Cp_A_R8 { operand },

        _ => unreachable!(),
    }
}

fn decode_block_3_opcode(op: u8) -> Instr {
    const INVALID_OPS: &[u8] = &[
        0xD3, 0xDB, 0xDD, 0xE3, 0xE4, 0xEB, 0xEC, 0xED, 0xF4, 0xFC, 0xFD,
    ];
    if INVALID_OPS.contains(&op) {
        return Instr::HardLock;
    }

    if op == 0xCB {
        unreachable!();
    }

    // ARITH A IMM8
    if bits8(&op, 2, 0) == 0b110 {
        return match bits8(&op, 5, 3) {
            0b000 => Instr::Add_A_Imm8,
            0b001 => Instr::Adc_A_Imm8,
            0b010 => Instr::Sub_A_Imm8,
            0b011 => Instr::Sbc_A_Imm8,

            0b100 => Instr::And_A_Imm8,
            0b101 => Instr::Xor_A_Imm8,
            0b110 => Instr::Or_A_Imm8,
            0b111 => Instr::Cp_A_Imm8,

            _ => unreachable!(),
        };
    }

    // RET COND, RET, RETI
    let cond = Cond::from_u8(bits8(&op, 4, 3));
    if bit8(&op, 5) == 0b0 && bits8(&op, 2, 0) == 0b000 {
        return Instr::Ret_Cond { cond };
    }
    if bits8(&op, 5, 0) == 0b00_1001 {
        return Instr::Ret;
    }
    if bits8(&op, 5, 0) == 0b01_1001 {
        return Instr::Reti;
    }

    // JP COND IMM16, JP IMM16, JP HL
    if bit8(&op, 5) == 0b0 && bits8(&op, 2, 0) == 0b010 {
        return Instr::Jp_Cond_Imm16 { cond };
    }
    if bits8(&op, 5, 0) == 0b00_0011 {
        return Instr::Jp_Imm16;
    }
    if bits8(&op, 5, 0) == 0b10_1001 {
        return Instr::Jp_Hl;
    }

    // CALL COND IMM16, CALL IMM16, RST TGT3
    if bit8(&op, 5) == 0b0 && bits8(&op, 2, 0) == 0b110 {
        return Instr::Call_Cond_Imm16 { cond };
    }
    if bits8(&op, 5, 0) == 0b00_1101 {
        return Instr::Call_Imm16;
    }
    if bits8(&op, 2, 0) == 0b111 {
        let tgt3 = bits8(&op, 5, 3);
        return Instr::Rst_Tgt3 { tgt3 };
    }

    // POP R16STK, PUSH R16STK
    let reg = R16Stk::from_u8(bits8(&op, 5, 4));
    if bits8(&op, 3, 0) == 0b0001 {
        return Instr::Pop_R16Stk { reg };
    }
    if bits8(&op, 3, 0) == 0b0101 {
        return Instr::Push_R16Stk { reg };
    }

    // LD Ptr, LDH Ptr, ADD SP, LD SP, EI, DI
    match op {
        0b1110_0010 => {
            return Instr::Ldh_CP_A;
        }
        0b1110_0000 => {
            return Instr::Ldh_Imm8P_A;
        }
        0b1110_1010 => {
            return Instr::Ld_Imm16P_A;
        }
        0b1111_0010 => {
            return Instr::Ldh_A_CP;
        }
        0b1111_0000 => {
            return Instr::Ldh_A_Imm8P;
        }
        0b11111010 => {
            return Instr::Ld_A_Imm16P;
        }

        0b1110_1000 => {
            return Instr::Add_Sp_Imm8;
        }
        0b1111_1000 => {
            return Instr::Ld_Hl_SpImm8;
        }
        0b1111_1001 => {
            return Instr::Ld_Sp_Hl;
        }

        0b1111_0011 => {
            return Instr::Ei;
        }
        0b1111_1011 => {
            return Instr::Di;
        }

        _ => {}
    };

    panic!("Unexpected block 3 opcode: {:#02x} ({:#02b})", op, op);
}

fn decode_cp_prefix_opcode(op: u8) -> Instr {
    let operand = R8::from_u8(bits8(&op, 2, 0));

    if bits8(&op, 7, 6) == 0b00 {
        return match bits8(&op, 5, 3) {
            0b000 => Instr::Rlc_R8 { operand },
            0b001 => Instr::Rrc_R8 { operand },
            0b010 => Instr::Rl_R8 { operand },
            0b011 => Instr::Rr_R8 { operand },

            0b100 => Instr::Sla_R8 { operand },
            0b101 => Instr::Sra_R8 { operand },
            0b110 => Instr::Swap_R8 { operand },
            0b111 => Instr::Srl_R8 { operand },

            _ => unreachable!(),
        };
    }

    let b3 = bits8(&op, 5, 3);
    return match bits8(&op, 7, 6) {
        0b01 => Instr::Bit_B3_R8 { b3, operand },
        0b10 => Instr::Res_B3_R8 { b3, operand },
        0b11 => Instr::Set_B3_R8 { b3, operand },

        _ => unreachable!(),
    };
}
