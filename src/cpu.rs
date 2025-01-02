use std::mem::transmute;

use crate::{
    asm::{interpret, Asm, Cond, ImmType, R16Mem, R16Stk, R16, R8},
    data::{join_16, split_16},
    debug::Debug,
    math::{add16_ui, add16_uu, add8_ui, bits16, bits8},
    regs::{self, CpuFlag, CpuReg16, CpuReg8},
    sys::Sys,
};

pub fn execute_next_instr(sys: &mut Sys) {
    let op = sys.rd_mem(sys.get_pc());
    let asm = interpret(op);
    //println!("[{:#02x}] {:?}", pc, asm);

    Debug::record_curr_instr(sys);

    sys.inc_pc();

    match asm {
        // Block 0.
        Asm::Nop => {}
        Asm::Ld_R16_Imm16 { dst } => {
            ld_r16_imm16(sys, dst);
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
            ld_imm16_sp(sys);
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
            ld_r8_imm8(sys, dst);
        }
        Asm::Jr_Imm8 => {
            jr_imm8(sys);
        }
        Asm::Jr_Cond_Imm8 { cond } => {
            jr_cond_imm8(sys, cond);
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

        // Block 3.
        Asm::Add_A_Imm8 => {
            add_a_imm8(sys);
        }
        Asm::Adc_A_Imm8 => {
            adc_a_imm8(sys);
        }
        Asm::Sub_A_Imm8 => {
            sub_a_imm8(sys);
        }
        Asm::Sbc_A_Imm8 => {
            sbc_a_imm8(sys);
        }
        Asm::And_A_Imm8 => {
            and_a_imm8(sys);
        }
        Asm::Xor_A_Imm8 => {
            xor_a_imm8(sys);
        }
        Asm::Or_A_Imm8 => {
            or_a_imm8(sys);
        }
        Asm::Cp_A_Imm8 => {
            cp_a_imm8(sys);
        }

        Asm::Ret_Cond { cond } => {
            ret_cond(sys, cond);
        }
        Asm::Ret => {
            ret(sys);
        }
        Asm::Reti => {
            reti(sys);
        }
        Asm::Jp_Cond_Imm16 { cond } => {
            jp_cond_imm16(sys, cond);
        }
        Asm::Jp_Imm16 => {
            jp_imm16(sys);
        }
        Asm::Jp_Hl => {
            jp_hl(sys);
        }
        Asm::Call_Cond_Imm16 { cond } => {
            call_cond_imm16(sys, cond);
        }
        Asm::Call_Imm16 => {
            call_imm16(sys);
        }
        Asm::Rst_Tgt3 { tgt3 } => {
            rst_tgt3(sys, tgt3);
        }

        Asm::Pop_R16Stk { reg } => {
            pop_r16stk(sys, reg);
        }
        Asm::Push_R16Stk { reg } => {
            push_r16stk(sys, reg);
        }

        // Misc.
        Asm::HardLock => {
            hard_lock(sys);
        }
    }
}

// Helper functions.
fn take_imm8(sys: &mut Sys) -> u8 {
    let imm8 = sys.rd_mem(sys.get_pc());
    //println!("[{:#02x}] {:?}", pc, Asm::Imm8(imm8));
    sys.inc_pc();

    return imm8;
}

fn take_imm16(sys: &mut Sys) -> u16 {
    let lo = sys.rd_mem(sys.get_pc());
    //println!("[{:#02x}] {:?}", pc, Asm::Imm16Lo(lo));
    sys.inc_pc();
    let hi = sys.rd_mem(sys.get_pc());
    //println!("[{:#02x}] {:?}", pc, Asm::Imm16Hi(hi));
    sys.inc_pc();

    let imm16 = join_16(hi, lo);
    return imm16;
}

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

fn push_16(sys: &mut Sys, data: u16) {
    let (hi, lo) = split_16(data);

    sys.dec_sp();
    sys.wr_mem(sys.get_sp(), hi);

    sys.dec_sp();
    sys.wr_mem(sys.get_sp(), lo);
}

fn pop_16(sys: &mut Sys) -> u16 {
    let lo = sys.rd_mem(sys.get_sp());
    sys.inc_sp();

    let hi = sys.rd_mem(sys.get_sp());
    sys.inc_sp();

    return join_16(hi, lo);
}

fn call(sys: &mut Sys, prev_pc: u16, next_pc: u16) {
    push_16(sys, prev_pc);
    sys.set_pc(next_pc);
}

// Block 0 functions.
fn ld_r16_imm16(sys: &mut Sys, dst: R16) {
    let imm16 = take_imm16(sys);
    let dst = match dst {
        R16::BC => CpuReg16::BC,
        R16::DE => CpuReg16::DE,
        R16::HL => CpuReg16::HL,
        R16::SP => CpuReg16::SP,
    };
    sys.regs.set_16(dst, imm16);
}

fn ld_imm16_sp(sys: &mut Sys) {
    let imm16 = take_imm16(sys);
    let addr = imm16;
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

fn ld_r8_imm8(sys: &mut Sys, dst: R8) {
    let imm8 = take_imm8(sys);
    set_r8_data(sys, dst, imm8);
}

fn jr_imm8(sys: &mut Sys) {
    let imm8 = take_imm8(sys);
    let rel: i8 = unsafe { transmute(imm8) };
    let mut pc = sys.get_pc();

    pc = add16_ui(pc, rel as i16);

    sys.set_pc(pc);
}

fn jr_cond_imm8(sys: &mut Sys, cond: Cond) {
    let imm8 = take_imm8(sys);
    if is_condition_met(sys, cond) {
        let rel: i8 = unsafe { transmute(imm8) };
        let mut pc = sys.get_pc();

        pc = add16_ui(pc, rel as i16);

        sys.set_pc(pc);
    }

    // todo jumping from correct starting addr??
}

fn stop(sys: &mut Sys) {
    sys.hard_lock = true; // todo incorrect
}

// Block 1 functions.
fn ld_r8_r8(sys: &mut Sys, dst: R8, src: R8) {
    let data = get_r8_data(sys, src);
    set_r8_data(sys, dst, data);
}

fn halt(sys: &mut Sys) {
    sys.hard_lock = true; // todo incorrect
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

// Block 3 functions.
fn add_a_imm8(sys: &mut Sys) {
    let imm8 = take_imm8(sys);
    let a = sys.regs.get_8(CpuReg8::A);

    let a_ = u8::wrapping_add(a, imm8);
    sys.regs.set_8(CpuReg8::A, a_);

    let h = bits8(&a_, 3, 0) < bits8(&a, 3, 0);
    let c = a_ < a;
    sys.regs.set_flag(CpuFlag::Z, a_ == 0);
    sys.regs.set_flag(CpuFlag::N, false);
    sys.regs.set_flag(CpuFlag::H, h);
    sys.regs.set_flag(CpuFlag::C, c);
}

fn adc_a_imm8(sys: &mut Sys) {
    let imm8 = take_imm8(sys);
    let a = sys.regs.get_8(CpuReg8::A);
    let carry = if sys.regs.get_flag(CpuFlag::C) { 1 } else { 0 };

    let a_ = u8::wrapping_add(a, imm8);
    let a_ = u8::wrapping_add(a_, carry);
    sys.regs.set_8(CpuReg8::A, a_);

    let h = bits8(&a_, 3, 0) < bits8(&a, 3, 0);
    let c = a_ < a;
    sys.regs.set_flag(CpuFlag::Z, a_ == 0);
    sys.regs.set_flag(CpuFlag::N, false);
    sys.regs.set_flag(CpuFlag::H, h);
    sys.regs.set_flag(CpuFlag::C, c);
}

fn sub_a_imm8(sys: &mut Sys) {
    let imm8 = take_imm8(sys);
    let a = sys.regs.get_8(CpuReg8::A);

    let a_ = u8::wrapping_sub(a, imm8);
    sys.regs.set_8(CpuReg8::A, a_);

    let h = bits8(&a_, 3, 0) > bits8(&a, 3, 0);
    let c = a_ > a;
    sys.regs.set_flag(CpuFlag::Z, a_ == 0);
    sys.regs.set_flag(CpuFlag::N, true);
    sys.regs.set_flag(CpuFlag::H, h);
    sys.regs.set_flag(CpuFlag::C, c);
}

fn sbc_a_imm8(sys: &mut Sys) {
    let imm8 = take_imm8(sys);
    let a = sys.regs.get_8(CpuReg8::A);
    let carry = if sys.regs.get_flag(CpuFlag::C) { 1 } else { 0 };

    let a_ = u8::wrapping_sub(a, imm8);
    let a_ = u8::wrapping_sub(a_, carry);
    sys.regs.set_8(CpuReg8::A, a_);

    let h = bits8(&a_, 3, 0) > bits8(&a, 3, 0);
    let c = a_ > a;
    sys.regs.set_flag(CpuFlag::Z, a_ == 0);
    sys.regs.set_flag(CpuFlag::N, true);
    sys.regs.set_flag(CpuFlag::H, h);
    sys.regs.set_flag(CpuFlag::C, c);
}

fn and_a_imm8(sys: &mut Sys) {
    let imm8 = take_imm8(sys);
    let a = sys.regs.get_8(CpuReg8::A);

    let a_ = a & imm8;
    sys.regs.set_8(CpuReg8::A, a_);

    sys.regs.set_flag(CpuFlag::Z, a_ == 0);
    sys.regs.set_flag(CpuFlag::N, false);
    sys.regs.set_flag(CpuFlag::H, true);
    sys.regs.set_flag(CpuFlag::C, false);
}

fn xor_a_imm8(sys: &mut Sys) {
    let imm8 = take_imm8(sys);
    let a = sys.regs.get_8(CpuReg8::A);

    let a_ = a ^ imm8;
    sys.regs.set_8(CpuReg8::A, a_);

    sys.regs.set_flag(CpuFlag::Z, a_ == 0);
    sys.regs.set_flag(CpuFlag::N, false);
    sys.regs.set_flag(CpuFlag::H, false);
    sys.regs.set_flag(CpuFlag::C, false);
}

fn or_a_imm8(sys: &mut Sys) {
    let imm8 = take_imm8(sys);
    let a = sys.regs.get_8(CpuReg8::A);

    let a_ = a | imm8;
    sys.regs.set_8(CpuReg8::A, a_);

    sys.regs.set_flag(CpuFlag::Z, a_ == 0);
    sys.regs.set_flag(CpuFlag::N, false);
    sys.regs.set_flag(CpuFlag::H, false);
    sys.regs.set_flag(CpuFlag::C, false);
}

fn cp_a_imm8(sys: &mut Sys) {
    let imm8 = take_imm8(sys);
    let a = sys.regs.get_8(CpuReg8::A);

    let a_ = u8::wrapping_sub(a, imm8);

    let h = bits8(&a_, 3, 0) > bits8(&a, 3, 0);
    let c = a_ > a;
    sys.regs.set_flag(CpuFlag::Z, a_ == 0);
    sys.regs.set_flag(CpuFlag::N, true);
    sys.regs.set_flag(CpuFlag::H, h);
    sys.regs.set_flag(CpuFlag::C, c);
}

fn ret_cond(sys: &mut Sys, cond: Cond) {
    if is_condition_met(sys, cond) {
        ret(sys);
    }
}

fn ret(sys: &mut Sys) {
    let lo = sys.rd_mem(sys.get_sp());
    sys.inc_sp();
    let hi = sys.rd_mem(sys.get_sp());
    sys.inc_sp();

    let pc = join_16(hi, lo);
    sys.set_pc(pc);
}

fn reti(sys: &mut Sys) {
    todo!("Interrup related instr");
}

fn jp_cond_imm16(sys: &mut Sys, cond: Cond) {
    let imm16 = take_imm16(sys);
    if is_condition_met(sys, cond) {
        sys.set_pc(imm16);
    }
}

fn jp_imm16(sys: &mut Sys) {
    let imm16 = take_imm16(sys);
    sys.set_pc(imm16);
}

fn jp_hl(sys: &mut Sys) {
    let hl = sys.regs.get_16(CpuReg16::HL);
    sys.set_pc(hl);
}

fn call_cond_imm16(sys: &mut Sys, cond: Cond) {
    let imm16 = take_imm16(sys);
    if is_condition_met(sys, cond) {
        let pc = sys.get_pc();
        let imm16 = take_imm16(sys);
        call(sys, pc, imm16);
    }
}

fn call_imm16(sys: &mut Sys) {
    let pc = sys.get_pc();
    let imm16 = take_imm16(sys);
    call(sys, pc, imm16);
}

fn rst_tgt3(sys: &mut Sys, tgt3: u8) {
    let pc = sys.get_pc();
    push_16(sys, pc);

    let tgt = (tgt3 as u16) << 3;
    sys.set_pc(tgt);
}

fn pop_r16stk(sys: &mut Sys, reg: R16Stk) {
    let data = pop_16(sys);
    sys.regs.set_16(reg.get_reg(), data);
}

fn push_r16stk(sys: &mut Sys, reg: R16Stk) {
    let data = sys.regs.get_16(reg.get_reg());
    push_16(sys, data);
}

// Misc functions.
fn hard_lock(sys: &mut Sys) {
    sys.hard_lock = true;
}
