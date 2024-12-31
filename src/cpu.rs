use std::mem::transmute;

use crate::{
    asm::{interpret, Asm, Cond, ImmType, R16Mem, R16, R8},
    data::split_16,
    math::{add16_ui, add16_uu, add8_ui, bits16, bits8},
    regs::{self, CpuFlag, CpuReg16, CpuReg8},
    sys::Sys,
};

pub fn execute_next_instr(sys: &mut Sys) {
    let mut pc = sys.regs.get_16(CpuReg16::PC);

    let op = sys.rd_mem(pc);
    let asm = interpret(op);
    //println!("[{:#02x}] {:?}", pc, asm);
    pc += 1;

    let mut imm_8: u8 = 0;
    let mut imm_16: u16 = 0;
    let imm_type = asm.imm_type();
    match imm_type {
        ImmType::Imm8 => {
            imm_8 = sys.rd_mem(pc);
            //println!("[{:#02x}] {:?}", pc, Asm::Imm8(imm_8));
            pc += 1;
        }
        ImmType::Imm16 => {
            let lo = sys.rd_mem(pc);
            //println!("[{:#02x}] {:?}", pc, Asm::Imm16Lo(lo));
            pc += 1;
            let hi = sys.rd_mem(pc);
            //println!("[{:#02x}] {:?}", pc, Asm::Imm16Hi(hi));
            pc += 1;

            imm_16 = ((hi as u16) << 8) | (lo as u16);
        }
        _ => {}
    }

    sys.regs.set_16(CpuReg16::PC, pc);

    match asm {
        // Immediate values (not used here)/
        Asm::Imm8(_) => todo!(),
        Asm::Imm16Hi(_) => todo!(),
        Asm::Imm16Lo(_) => todo!(),

        // Block 0.
        Asm::Nop => {}
        Asm::Ld_R16_Imm16 { dst } => {
            ld_r16_imm16(sys, dst, imm_16);
        }
        Asm::Ld_R16MemP_A { dst } => {
            let data = sys.regs.get_8(CpuReg8::A);
            set_r16memp(sys, dst, data);
        }
        Asm::Ld_A_R16MemP { src } => {
            let data = get_r16memp(sys, src);
            sys.regs.set_8(CpuReg8::A, data);
        }
        Asm::Ld_Imm16P_Sp => {
            ld_imm16_sp(sys, imm_16);
        }
        Asm::Inc_R16 { operand } => {
            inc_dec_r16(sys, operand, 1);
        }
        Asm::Dec_R16 { operand } => {
            inc_dec_r16(sys, operand, -1);
        }
        Asm::Add_Hl_R16 { operand } => {
            add_hl_r16(sys, operand);
        }
        Asm::Inc_R8 { operand } => {
            inc_r8(sys, operand);
        }
        Asm::Dec_R8 { operand } => {
            dec_r8(sys, operand);
        }
        Asm::Ld_R8_Imm8 { dst } => {
            ld_r8_imm8(sys, dst, imm_8);
        }
        Asm::Jr_Imm8 => {
            jr_imm8(sys, imm_8);
        }
        Asm::Jr_Cond_Imm8 { cond } => {
            jr_cond_imm8(sys, cond, imm_8);
        }
        Asm::Stop => {}

        // Block 1.
        Asm::Ld_R8_R8 { dst, src } => {
            ld_r8_r8(sys, dst, src);
        }
        Asm::Halt => {
            halt(sys);
        }

        // Block 2.
        Asm::Add_A_R8 { operand } => {
            add_a_r8(sys, operand);
        }
        Asm::Adc_A_R8 { operand } => {
            adc_a_r8(sys, operand);
        }
        Asm::Sub_A_R8 { operand } => {
            sub_a_r8(sys, operand);
        }
        Asm::Sbc_A_R8 { operand } => {
            sbc_a_r8(sys, operand);
        }
        Asm::And_A_R8 { operand } => {
            and_a_r8(sys, operand);
        }
        Asm::Xor_A_R8 { operand } => {
            xor_a_r8(sys, operand);
        }
        Asm::Or_A_R8 { operand } => {
            or_a_r8(sys, operand);
        }
        Asm::Cp_A_R8 { operand } => {
            cp_a_r8(sys, operand);
        }
    }
}

// Helper functions.
fn set_r16memp(sys: &mut Sys, dst: R16Mem, data: u8) {
    let (dstp, inc) = dst.get_reg_inc();

    let addr = sys.regs.get_16(dstp);
    sys.wr_mem(addr, data);
    sys.regs.set_16(dstp, add16_ui(addr, inc));
}

fn get_r16memp(sys: &mut Sys, src: R16Mem) -> u8 {
    let (srcp, inc) = src.get_reg_inc();

    let addr = sys.regs.get_16(srcp);
    let data = sys.rd_mem(addr);
    sys.regs.set_16(srcp, add16_ui(addr, inc));

    return data;
}

fn is_condition_met(sys: &mut Sys, cond: Cond) -> bool {
    let z = sys.regs.get_flag(CpuFlag::Z);
    let c = sys.regs.get_flag(CpuFlag::C);

    match cond {
        Cond::NZ => !z,
        Cond::Z => z,
        Cond::NC => !c,
        Cond::C => c,
    }
}

fn get_r8_data(sys: &mut Sys, operand: R8) -> u8 {
    if let Some(reg) = operand.get_reg() {
        return sys.regs.get_8(reg);
    } else {
        let addr = sys.regs.get_16(CpuReg16::HL);
        return sys.rd_mem(addr);
    }
}

fn set_r8_data(sys: &mut Sys, operand: R8, data: u8) {
    if let Some(reg) = operand.get_reg() {
        sys.regs.set_8(reg, data);
    } else {
        let addr = sys.regs.get_16(CpuReg16::HL);
        sys.wr_mem(addr, data);
    }
}

// Block 0 functions.
fn ld_r16_imm16(sys: &mut Sys, dst: R16, imm_16: u16) {
    let dst = match dst {
        R16::BC => CpuReg16::BC,
        R16::DE => CpuReg16::DE,
        R16::HL => CpuReg16::HL,
        R16::SP => CpuReg16::SP,
    };
    sys.regs.set_16(dst, imm_16);
}

fn ld_imm16_sp(sys: &mut Sys, imm_16: u16) {
    let addr = imm_16;
    let data = sys.regs.get_16(CpuReg16::SP);
    let (hi, lo) = split_16(data);
    sys.wr_mem(addr, lo);
    sys.wr_mem(addr + 1, hi);
}

fn inc_dec_r16(sys: &mut Sys, operand: R16, inc: i16) {
    let mut data = sys.regs.get_16(operand.get_reg());
    data = add16_ui(data, inc);
    sys.regs.set_16(operand.get_reg(), data);
}

// fn add_r16(sys: &mut Sys, dst: R16, operand: R16) {
//     let mut a = sys.regs.get_16(dst.get_reg());
//     let b = sys.regs.get_16(operand.get_reg());

//     a = add16_uu(a, b);
//     sys.regs.set_16(dst.get_reg(), a);
// }

fn add_hl_r16(sys: &mut Sys, operand: R16) {
    let hl = sys.regs.get_16(CpuReg16::HL);
    let operand = sys.regs.get_16(operand.get_reg());

    let hl_ = add16_uu(hl, operand);
    sys.regs.set_16(CpuReg16::HL, hl_);
    let h = bits16(&hl_, 11, 0) < bits16(&hl, 11, 0); // todo correct??
    let c = hl_ < hl;
    sys.regs.set_flag(CpuFlag::N, false);
    sys.regs.set_flag(CpuFlag::H, h);
    sys.regs.set_flag(CpuFlag::C, c);
}

fn inc_r8(sys: &mut Sys, operand: R8) {
    let mut data = get_r8_data(sys, operand);
    let h = bits8(&data, 3, 0) == 0b1111;

    data = u8::wrapping_add(data, 1);

    set_r8_data(sys, operand, data);
    sys.regs.set_flag(CpuFlag::Z, data == 0);
    sys.regs.set_flag(CpuFlag::N, false);
    sys.regs.set_flag(CpuFlag::H, h);
}

fn dec_r8(sys: &mut Sys, operand: R8) {
    let mut data = get_r8_data(sys, operand);
    let h = bits8(&data, 3, 0) == 0b0000;

    data = u8::wrapping_sub(data, 1);

    set_r8_data(sys, operand, data);
    sys.regs.set_flag(CpuFlag::Z, data == 0);
    sys.regs.set_flag(CpuFlag::N, true);
    sys.regs.set_flag(CpuFlag::H, h);
}

fn ld_r8_imm8(sys: &mut Sys, dst: R8, imm_8: u8) {
    set_r8_data(sys, dst, imm_8);
}

fn jr_imm8(sys: &mut Sys, imm_8: u8) {
    let rel: i8 = unsafe { transmute(imm_8) };
    let mut pc = sys.get_pc();

    pc = add16_ui(pc, rel as i16);

    sys.set_pc(pc);
}

fn jr_cond_imm8(sys: &mut Sys, cond: Cond, imm_8: u8) {
    if is_condition_met(sys, cond) {
        let rel: i8 = unsafe { transmute(imm_8) };
        let mut pc = sys.get_pc();

        pc = add16_ui(pc, rel as i16);

        sys.set_pc(pc);
    }
}

fn stop(sys: &mut Sys) {
    sys.crash = true; // todo incorrect
}

// Block 1 functions.
fn ld_r8_r8(sys: &mut Sys, dst: R8, src: R8) {
    let data = get_r8_data(sys, src);
    set_r8_data(sys, dst, data);
}

fn halt(sys: &mut Sys) {
    sys.crash = true; // todo incorrect
}

// Block 2 functions.
fn add_a_r8(sys: &mut Sys, operand: R8) {
    let a = sys.regs.get_8(CpuReg8::A);
    let operand = get_r8_data(sys, operand);

    let a_ = u8::wrapping_add(a, operand);
    sys.regs.set_8(CpuReg8::A, a_);

    let h = bits8(&a_, 3, 0) < bits8(&a, 3, 0);
    let c = a_ < a;
    sys.regs.set_flag(CpuFlag::Z, a_ == 0);
    sys.regs.set_flag(CpuFlag::N, false);
    sys.regs.set_flag(CpuFlag::H, h);
    sys.regs.set_flag(CpuFlag::C, c);
}

fn adc_a_r8(sys: &mut Sys, operand: R8) {
    let a = sys.regs.get_8(CpuReg8::A);
    let operand = get_r8_data(sys, operand);
    let carry = if sys.regs.get_flag(CpuFlag::C) { 1 } else { 0 };

    let a_ = u8::wrapping_add(a, operand);
    let a_ = u8::wrapping_add(a_, carry);
    sys.regs.set_8(CpuReg8::A, a_);

    let h = bits8(&a_, 3, 0) < bits8(&a, 3, 0);
    let c = a_ < a;
    sys.regs.set_flag(CpuFlag::Z, a_ == 0);
    sys.regs.set_flag(CpuFlag::N, false);
    sys.regs.set_flag(CpuFlag::H, h);
    sys.regs.set_flag(CpuFlag::C, c);
}

fn sub_a_r8(sys: &mut Sys, operand: R8) {
    let a = sys.regs.get_8(CpuReg8::A);
    let operand = get_r8_data(sys, operand);

    let a_ = u8::wrapping_sub(a, operand);
    sys.regs.set_8(CpuReg8::A, a_);

    let h = bits8(&a_, 3, 0) > bits8(&a, 3, 0);
    let c = a_ > a;
    sys.regs.set_flag(CpuFlag::Z, a_ == 0);
    sys.regs.set_flag(CpuFlag::N, true);
    sys.regs.set_flag(CpuFlag::H, h);
    sys.regs.set_flag(CpuFlag::C, c);
}

fn sbc_a_r8(sys: &mut Sys, operand: R8) {
    let a = sys.regs.get_8(CpuReg8::A);
    let operand = get_r8_data(sys, operand);
    let carry = if sys.regs.get_flag(CpuFlag::C) { 1 } else { 0 };

    let a_ = u8::wrapping_sub(a, operand);
    let a_ = u8::wrapping_sub(a_, carry);
    sys.regs.set_8(CpuReg8::A, a_);

    let h = bits8(&a_, 3, 0) > bits8(&a, 3, 0);
    let c = a_ > a;
    sys.regs.set_flag(CpuFlag::Z, a_ == 0);
    sys.regs.set_flag(CpuFlag::N, true);
    sys.regs.set_flag(CpuFlag::H, h);
    sys.regs.set_flag(CpuFlag::C, c);
}

fn and_a_r8(sys: &mut Sys, operand: R8) {
    let a = sys.regs.get_8(CpuReg8::A);
    let operand = get_r8_data(sys, operand);

    let a_ = a & operand;
    sys.regs.set_8(CpuReg8::A, a_);

    sys.regs.set_flag(CpuFlag::Z, a_ == 0);
    sys.regs.set_flag(CpuFlag::N, false);
    sys.regs.set_flag(CpuFlag::H, true);
    sys.regs.set_flag(CpuFlag::C, false);
}

fn xor_a_r8(sys: &mut Sys, operand: R8) {
    let a = sys.regs.get_8(CpuReg8::A);
    let operand = get_r8_data(sys, operand);

    let a_ = a ^ operand;
    sys.regs.set_8(CpuReg8::A, a_);

    sys.regs.set_flag(CpuFlag::Z, a_ == 0);
    sys.regs.set_flag(CpuFlag::N, false);
    sys.regs.set_flag(CpuFlag::H, false);
    sys.regs.set_flag(CpuFlag::C, false);
}

fn or_a_r8(sys: &mut Sys, operand: R8) {
    let a = sys.regs.get_8(CpuReg8::A);
    let operand = get_r8_data(sys, operand);

    let a_ = a | operand;
    sys.regs.set_8(CpuReg8::A, a_);

    sys.regs.set_flag(CpuFlag::Z, a_ == 0);
    sys.regs.set_flag(CpuFlag::N, false);
    sys.regs.set_flag(CpuFlag::H, false);
    sys.regs.set_flag(CpuFlag::C, false);
}

fn cp_a_r8(sys: &mut Sys, operand: R8) {
    let a = sys.regs.get_8(CpuReg8::A);
    let operand = get_r8_data(sys, operand);

    let a_ = u8::wrapping_sub(a, operand);

    let h = bits8(&a_, 3, 0) > bits8(&a, 3, 0);
    let c = a_ > a;
    sys.regs.set_flag(CpuFlag::Z, a_ == 0);
    sys.regs.set_flag(CpuFlag::N, true);
    sys.regs.set_flag(CpuFlag::H, h);
    sys.regs.set_flag(CpuFlag::C, c);
}
