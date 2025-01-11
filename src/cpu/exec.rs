use std::mem::transmute;

use crate::{
    debug::Debug,
    sys::Sys,
    util::math::{
        add16_ui, add16_uu, add_u16_i8, bit8, bits16, bits8, join_16, set_bit8, split_16,
    },
};

use super::{
    instr::{decode, Cond, Instr, R16Mem, R16Stk, R16, R8},
    regs::{CpuFlag, CpuReg16, CpuReg8, CpuRegs},
};

/// Executes the instruction at PC and updates PC.
/// Returns the number of machine cycles needed to execute
/// the instruction.
pub fn execute_next_instr(sys: &mut Sys) -> u32 {
    let mut pc = sys.get_pc();
    let mut op = sys.mem_get(pc);
    let has_cb_prefix;

    if op == Instr::CB_PREFIX {
        sys.inc_pc();
        pc = sys.get_pc();
        op = sys.mem_get(pc);
        has_cb_prefix = true;
    } else {
        has_cb_prefix = false;
    }
    let instr = match decode(op, has_cb_prefix) {
        Ok(instr) => instr,
        Err(msg) => Debug::fail(sys, msg),
    };

    if sys.debug.enable_debug_print {
        println!("[{:#02x}] {:?}", pc, instr);
    }

    Debug::record_curr_instr(sys);

    sys.inc_pc();

    match instr {
        // Block 0.
        Instr::Nop => {}
        Instr::Ld_R16_Imm16 { dst } => {
            ld_r16_imm16(sys, dst);
        }
        Instr::Ld_R16MemP_A { dst } => {
            ld_r16memp_a(sys, dst);
        }
        Instr::Ld_A_R16MemP { src } => {
            ld_a_r16memp(sys, src);
        }
        Instr::Ld_Imm16P_Sp => {
            ld_imm16_sp(sys);
        }
        Instr::Inc_R16 { operand } => {
            inc_dec_r16(sys, operand, 1);
        }
        Instr::Dec_R16 { operand } => {
            inc_dec_r16(sys, operand, -1);
        }
        Instr::Add_Hl_R16 { operand } => {
            add_hl_r16(sys, operand);
        }
        Instr::Inc_R8 { operand } => {
            inc_r8(sys, operand);
        }
        Instr::Dec_R8 { operand } => {
            dec_r8(sys, operand);
        }
        Instr::Ld_R8_Imm8 { dst } => {
            ld_r8_imm8(sys, dst);
        }

        Instr::Rlca => {
            rlca(sys);
        }
        Instr::RRca => {
            rrca(sys);
        }
        Instr::Rla => {
            rla(sys);
        }
        Instr::Rra => {
            rra(sys);
        }
        Instr::Daa => {
            daa(sys);
        }
        Instr::Cpl => {
            cpl(sys);
        }
        Instr::Scf => {
            scf(sys);
        }
        Instr::Ccf => {
            ccf(sys);
        }

        Instr::Jr_Imm8 => {
            jr_imm8(sys);
        }
        Instr::Jr_Cond_Imm8 { cond } => {
            jr_cond_imm8(sys, cond);
        }
        Instr::Stop => {}

        // Block 1.
        Instr::Ld_R8_R8 { dst, src } => {
            ld_r8_r8(sys, dst, src);
        }
        Instr::Halt => {
            halt(sys);
        }

        // Block 2.
        Instr::Add_A_R8 { operand } => {
            add_a_r8(sys, operand);
        }
        Instr::Adc_A_R8 { operand } => {
            adc_a_r8(sys, operand);
        }
        Instr::Sub_A_R8 { operand } => {
            sub_a_r8(sys, operand);
        }
        Instr::Sbc_A_R8 { operand } => {
            sbc_a_r8(sys, operand);
        }
        Instr::And_A_R8 { operand } => {
            and_a_r8(sys, operand);
        }
        Instr::Xor_A_R8 { operand } => {
            xor_a_r8(sys, operand);
        }
        Instr::Or_A_R8 { operand } => {
            or_a_r8(sys, operand);
        }
        Instr::Cp_A_R8 { operand } => {
            cp_a_r8(sys, operand);
        }

        // Block 3.
        Instr::Add_A_Imm8 => {
            add_a_imm8(sys);
        }
        Instr::Adc_A_Imm8 => {
            adc_a_imm8(sys);
        }
        Instr::Sub_A_Imm8 => {
            sub_a_imm8(sys);
        }
        Instr::Sbc_A_Imm8 => {
            sbc_a_imm8(sys);
        }
        Instr::And_A_Imm8 => {
            and_a_imm8(sys);
        }
        Instr::Xor_A_Imm8 => {
            xor_a_imm8(sys);
        }
        Instr::Or_A_Imm8 => {
            or_a_imm8(sys);
        }
        Instr::Cp_A_Imm8 => {
            cp_a_imm8(sys);
        }

        Instr::Ret_Cond { cond } => {
            ret_cond(sys, cond);
        }
        Instr::Ret => {
            ret(sys);
        }
        Instr::Reti => {
            reti(sys);
        }
        Instr::Jp_Cond_Imm16 { cond } => {
            jp_cond_imm16(sys, cond);
        }
        Instr::Jp_Imm16 => {
            jp_imm16(sys);
        }
        Instr::Jp_Hl => {
            jp_hl(sys);
        }
        Instr::Call_Cond_Imm16 { cond } => {
            call_cond_imm16(sys, cond);
        }
        Instr::Call_Imm16 => {
            call_imm16(sys);
        }
        Instr::Rst_Tgt3 { tgt3 } => {
            rst_tgt3(sys, tgt3);
        }

        Instr::Pop_R16Stk { reg } => {
            pop_r16stk(sys, reg);
        }
        Instr::Push_R16Stk { reg } => {
            push_r16stk(sys, reg);
        }

        Instr::Ldh_CP_A => {
            ldh_cp_a(sys);
        }
        Instr::Ldh_Imm8P_A => {
            ldh_imm8p_a(sys);
        }
        Instr::Ld_Imm16P_A => {
            ld_imm16p_a(sys);
        }
        Instr::Ldh_A_CP => {
            ldh_a_cp(sys);
        }
        Instr::Ldh_A_Imm8P => {
            ldh_a_imm8p(sys);
        }
        Instr::Ld_A_Imm16P => {
            ld_a_imm16p(sys);
        }

        Instr::Add_Sp_Imm8 => {
            add_sp_imm8(sys);
        }
        Instr::Ld_Hl_SpImm8 => {
            ld_hl_spimm8(sys);
        }
        Instr::Ld_Sp_Hl => {
            ld_sp_hl(sys);
        }

        Instr::Di => {
            di(sys);
        }
        Instr::Ei => {
            ei(sys);
        }

        // 0xCB prefix ops.
        Instr::Rlc_R8 { operand } => {
            rlc_r8(sys, operand);
        }
        Instr::Rrc_R8 { operand } => {
            rrc_r8(sys, operand);
        }
        Instr::Rl_R8 { operand } => {
            rl_r8(sys, operand);
        }
        Instr::Rr_R8 { operand } => {
            rr_r8(sys, operand);
        }
        Instr::Sla_R8 { operand } => {
            sla_r8(sys, operand);
        }
        Instr::Sra_R8 { operand } => {
            sra_r8(sys, operand);
        }
        Instr::Swap_R8 { operand } => {
            swap_r8(sys, operand);
        }
        Instr::Srl_R8 { operand } => {
            srl_r8(sys, operand);
        }

        Instr::Bit_B3_R8 { b3, operand } => {
            bit_b3_r8(sys, b3, operand);
        }
        Instr::Res_B3_R8 { b3, operand } => {
            res_b3_r8(sys, b3, operand);
        }
        Instr::Set_B3_R8 { b3, operand } => {
            set_b3_r8(sys, b3, operand);
        }

        // Misc.
        Instr::HardLock => {
            hard_lock(sys);
        }
    }

    print_if_ld_a_a(sys, instr);

    return 2;
}

// Helper functions.
fn print_if_ld_a_a(sys: &mut Sys, instr: Instr) {
    if sys.debug.enable_debug_print
        && matches!(
            instr,
            Instr::Ld_R8_R8 {
                dst: R8::A,
                src: R8::A
            }
        )
    {
        sys.regs.print();
        CpuRegs::print_key_addrs(sys);
    }
}

fn take_imm8(sys: &mut Sys) -> u8 {
    let imm8 = sys.mem_get(sys.get_pc());
    sys.inc_pc();

    if sys.debug.enable_debug_print {
        println!("  imm8: {:0>2X} ({})", imm8, imm8);
    }

    return imm8;
}

fn take_imm16(sys: &mut Sys) -> u16 {
    let lo = sys.mem_get(sys.get_pc());
    sys.inc_pc();
    let hi = sys.mem_get(sys.get_pc());
    sys.inc_pc();

    let imm16 = join_16(hi, lo);

    if sys.debug.enable_debug_print {
        println!("  imm16: {:0>4X} ({})", imm16, imm16);
    }

    return imm16;
}

fn ld_r16memp_a(sys: &mut Sys, dst: R16Mem) {
    let data = sys.regs.get_8(CpuReg8::A);
    let (dstp, inc) = dst.get_reg_inc();

    let addr = sys.regs.get_16(dstp);
    sys.mem_set(addr, data);
    sys.regs.set_16(dstp, add16_ui(addr, inc));
}

fn ld_a_r16memp(sys: &mut Sys, src: R16Mem) {
    let (srcp, inc) = src.get_reg_inc();

    let addr = sys.regs.get_16(srcp);
    let data = sys.mem_get(addr);
    sys.regs.set_16(srcp, add16_ui(addr, inc));

    sys.regs.set_8(CpuReg8::A, data);
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
        return sys.mem_get(addr);
    }
}

fn set_r8_data(sys: &mut Sys, operand: R8, data: u8) {
    if let Some(reg) = operand.get_reg() {
        sys.regs.set_8(reg, data);
    } else {
        let addr = sys.regs.get_16(CpuReg16::HL);
        sys.mem_set(addr, data);
    }
}

fn push_16(sys: &mut Sys, data: u16) {
    let (hi, lo) = split_16(data);

    sys.dec_sp();
    sys.mem_set(sys.get_sp(), hi);

    sys.dec_sp();
    sys.mem_set(sys.get_sp(), lo);
}

fn pop_16(sys: &mut Sys) -> u16 {
    let lo = sys.mem_get(sys.get_sp());
    sys.inc_sp();

    let hi = sys.mem_get(sys.get_sp());
    sys.inc_sp();

    return join_16(hi, lo);
}

pub fn call(sys: &mut Sys, prev_pc: u16, next_pc: u16) {
    push_16(sys, prev_pc);
    sys.set_pc(next_pc);
}

// Block 0 functions.
fn ld_r16_imm16(sys: &mut Sys, dst: R16) {
    let imm16 = take_imm16(sys);
    let reg = dst.get_reg();
    sys.regs.set_16(reg, imm16);
}

fn ld_imm16_sp(sys: &mut Sys) {
    let imm16 = take_imm16(sys);
    let addr = imm16;
    let data = sys.regs.get_16(CpuReg16::SP);
    let (hi, lo) = split_16(data);
    sys.mem_set(addr, lo);
    sys.mem_set(addr + 1, hi);
}

fn inc_dec_r16(sys: &mut Sys, operand: R16, inc: i16) {
    let mut data = sys.regs.get_16(operand.get_reg());
    data = add16_ui(data, inc);
    sys.regs.set_16(operand.get_reg(), data);
}

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
    let h = bits8(&data, 4, 0) == 0b1_0000;

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

fn rlca(sys: &mut Sys) {
    let mut data = sys.regs.get_8(CpuReg8::A);
    let c = bit8(&data, 7) == 0b1;
    data = u8::rotate_left(data, 1);
    sys.regs.set_8(CpuReg8::A, data);

    sys.regs.set_flag(CpuFlag::Z, false);
    sys.regs.set_flag(CpuFlag::N, false);
    sys.regs.set_flag(CpuFlag::H, false);
    sys.regs.set_flag(CpuFlag::C, c);
}

fn rrca(sys: &mut Sys) {
    let mut data = sys.regs.get_8(CpuReg8::A);
    let c = bit8(&data, 0) == 0b1;
    data = u8::rotate_right(data, 1);
    sys.regs.set_8(CpuReg8::A, data);

    sys.regs.set_flag(CpuFlag::Z, false);
    sys.regs.set_flag(CpuFlag::N, false);
    sys.regs.set_flag(CpuFlag::H, false);
    sys.regs.set_flag(CpuFlag::C, c);
}

fn rla(sys: &mut Sys) {
    let c = if sys.regs.get_flag(CpuFlag::C) {
        0b1
    } else {
        0b0
    };
    let mut data = sys.regs.get_8(CpuReg8::A);
    let c_ = bit8(&data, 7) == 0b1;
    data = u8::rotate_left(data, 1);
    set_bit8(&mut data, 0, c);
    sys.regs.set_8(CpuReg8::A, data);

    sys.regs.set_flag(CpuFlag::Z, false);
    sys.regs.set_flag(CpuFlag::N, false);
    sys.regs.set_flag(CpuFlag::H, false);
    sys.regs.set_flag(CpuFlag::C, c_);
}

fn rra(sys: &mut Sys) {
    let c = if sys.regs.get_flag(CpuFlag::C) {
        0b1
    } else {
        0b0
    };
    let mut data = sys.regs.get_8(CpuReg8::A);
    let c_ = bit8(&data, 0) == 0b1;
    data = u8::rotate_right(data, 1);
    set_bit8(&mut data, 7, c);
    sys.regs.set_8(CpuReg8::A, data);

    sys.regs.set_flag(CpuFlag::Z, false);
    sys.regs.set_flag(CpuFlag::N, false);
    sys.regs.set_flag(CpuFlag::H, false);
    sys.regs.set_flag(CpuFlag::C, c_);
}

fn daa(sys: &mut Sys) {
    todo!("todo DAA");
}

fn cpl(sys: &mut Sys) {
    let mut data = sys.regs.get_8(CpuReg8::A);
    data = !data;
    sys.regs.set_8(CpuReg8::A, data);

    sys.regs.set_flag(CpuFlag::N, true);
    sys.regs.set_flag(CpuFlag::H, true);
}

fn scf(sys: &mut Sys) {
    sys.regs.set_flag(CpuFlag::N, false);
    sys.regs.set_flag(CpuFlag::H, false);
    sys.regs.set_flag(CpuFlag::C, true);
}

fn ccf(sys: &mut Sys) {
    let c = sys.regs.get_flag(CpuFlag::C);

    sys.regs.set_flag(CpuFlag::N, false);
    sys.regs.set_flag(CpuFlag::H, false);
    sys.regs.set_flag(CpuFlag::C, !c);
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
    sys.cpu_enable = false;
    sys.lcd_enable = false;
}

// Block 1 functions.
fn ld_r8_r8(sys: &mut Sys, dst: R8, src: R8) {
    let data = get_r8_data(sys, src);
    set_r8_data(sys, dst, data);
}

fn halt(sys: &mut Sys) {
    if sys.interrupt_master_enable {
        sys.cpu_enable = false;
    }
    // todo is there more to do here?
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
    let addr = pop_16(sys);
    sys.set_pc(addr);
}

fn reti(sys: &mut Sys) {
    let addr = pop_16(sys);
    sys.set_pc(addr);

    sys.interrupt_master_enable = true;
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

fn ldh_cp_a(sys: &mut Sys) {
    let a_data = sys.regs.get_8(CpuReg8::A);
    let c_data = sys.regs.get_8(CpuReg8::C);
    let addr = join_16(0xFF, c_data);

    sys.mem_set(addr, a_data);
}

fn ldh_imm8p_a(sys: &mut Sys) {
    let imm8 = take_imm8(sys);
    let data = sys.regs.get_8(CpuReg8::A);
    let addr = join_16(0xFF, imm8);

    sys.mem_set(addr, data);
}

fn ld_imm16p_a(sys: &mut Sys) {
    let imm16 = take_imm16(sys);
    let data = sys.regs.get_8(CpuReg8::A);
    let addr = imm16;

    sys.mem_set(addr, data);
}

fn ldh_a_cp(sys: &mut Sys) {
    let c_data = sys.regs.get_8(CpuReg8::C);
    let addr = join_16(0xFF, c_data);
    let data = sys.mem_get(addr);

    sys.regs.set_8(CpuReg8::A, data);
}

fn ldh_a_imm8p(sys: &mut Sys) {
    let imm8 = take_imm8(sys);
    let addr = join_16(0xFF, imm8);
    let data = sys.mem_get(addr);

    sys.regs.set_8(CpuReg8::A, data);
}

fn ld_a_imm16p(sys: &mut Sys) {
    let imm16 = take_imm16(sys);
    let addr = imm16;
    let data = sys.mem_get(addr);

    sys.regs.set_8(CpuReg8::A, data);
}

fn add_sp_imm8(sys: &mut Sys) {
    let sp = sys.get_sp();
    let imm8 = take_imm8(sys);
    let s_imm8 = unsafe { transmute(imm8) };
    let sp_ = add_u16_i8(sp, s_imm8);
    sys.set_sp(sp_);

    let h;
    let c;
    if s_imm8 >= 0 {
        h = bits16(&sp_, 11, 0) < bits16(&sp, 11, 0); // todo correct??
        c = sp_ < sp;
    } else {
        h = bits16(&sp_, 11, 0) > bits16(&sp, 11, 0);
        c = sp_ > sp;
    }
    sys.regs.set_flag(CpuFlag::H, h);
    sys.regs.set_flag(CpuFlag::C, c);
}

fn ld_hl_spimm8(sys: &mut Sys) {
    let sp = sys.get_sp();
    let imm8 = take_imm8(sys);
    let s_imm8 = unsafe { transmute(imm8) };
    let sp_ = add_u16_i8(sp, s_imm8);
    sys.regs.set_16(CpuReg16::HL, sp_);

    let h;
    let c;
    if s_imm8 >= 0 {
        h = bits16(&sp_, 11, 0) < bits16(&sp, 11, 0); // todo correct??
        c = sp_ < sp;
    } else {
        h = bits16(&sp_, 11, 0) > bits16(&sp, 11, 0);
        c = sp_ > sp;
    }
    sys.regs.set_flag(CpuFlag::H, h);
    sys.regs.set_flag(CpuFlag::C, c);
}

fn ld_sp_hl(sys: &mut Sys) {
    let data = sys.regs.get_16(CpuReg16::HL);
    sys.set_sp(data);
}

fn di(sys: &mut Sys) {
    sys.interrupt_master_enable = false;
}

fn ei(sys: &mut Sys) {
    sys.interrupt_master_enable = true;
}

// 0xCB prefix functions.
fn rlc_r8(sys: &mut Sys, operand: R8) {
    let mut data = get_r8_data(sys, operand);
    let c_ = bit8(&data, 7);

    data = u8::rotate_left(data, 1);
    set_r8_data(sys, operand, data);

    sys.regs.set_flag(CpuFlag::Z, data == 0);
    sys.regs.set_flag(CpuFlag::N, false);
    sys.regs.set_flag(CpuFlag::H, false);
    sys.regs.set_flag(CpuFlag::C, c_ == 1);
}

fn rrc_r8(sys: &mut Sys, operand: R8) {
    let mut data = get_r8_data(sys, operand);
    let c_ = bit8(&data, 0);

    data = u8::rotate_right(data, 1);
    set_r8_data(sys, operand, data);

    sys.regs.set_flag(CpuFlag::Z, data == 0);
    sys.regs.set_flag(CpuFlag::N, false);
    sys.regs.set_flag(CpuFlag::H, false);
    sys.regs.set_flag(CpuFlag::C, c_ == 1);
}

fn rl_r8(sys: &mut Sys, operand: R8) {
    let mut data = get_r8_data(sys, operand);
    let c = sys.regs.get_flag(CpuFlag::C).into();
    let c_ = bit8(&data, 7);

    data = u8::rotate_left(data, 1);
    set_bit8(&mut data, 0, c);
    set_r8_data(sys, operand, data);

    sys.regs.set_flag(CpuFlag::Z, data == 0);
    sys.regs.set_flag(CpuFlag::N, false);
    sys.regs.set_flag(CpuFlag::H, false);
    sys.regs.set_flag(CpuFlag::C, c_ == 1);
}

fn rr_r8(sys: &mut Sys, operand: R8) {
    let mut data = get_r8_data(sys, operand);
    let c = sys.regs.get_flag(CpuFlag::C).into();
    let c_ = bit8(&data, 0);

    data = u8::rotate_right(data, 1);
    set_bit8(&mut data, 7, c);
    set_r8_data(sys, operand, data);

    sys.regs.set_flag(CpuFlag::Z, data == 0);
    sys.regs.set_flag(CpuFlag::N, false);
    sys.regs.set_flag(CpuFlag::H, false);
    sys.regs.set_flag(CpuFlag::C, c_ == 1);
}

fn sla_r8(sys: &mut Sys, operand: R8) {
    let mut data = get_r8_data(sys, operand);
    let c_ = bit8(&data, 7);

    data = u8::wrapping_shl(data, 1); // todo correct??
    set_r8_data(sys, operand, data);

    sys.regs.set_flag(CpuFlag::Z, data == 0);
    sys.regs.set_flag(CpuFlag::N, false);
    sys.regs.set_flag(CpuFlag::H, false);
    sys.regs.set_flag(CpuFlag::C, c_ == 1);
}

fn sra_r8(sys: &mut Sys, operand: R8) {
    let mut data = get_r8_data(sys, operand);
    let data7 = bit8(&data, 7);
    let c_ = bit8(&data, 0);

    data = u8::wrapping_shr(data, 1); // todo correct??
    set_bit8(&mut data, 7, data7);
    set_r8_data(sys, operand, data);

    sys.regs.set_flag(CpuFlag::Z, data == 0);
    sys.regs.set_flag(CpuFlag::N, false);
    sys.regs.set_flag(CpuFlag::H, false);
    sys.regs.set_flag(CpuFlag::C, c_ == 1);
}

fn swap_r8(sys: &mut Sys, operand: R8) {
    let mut data = get_r8_data(sys, operand);

    data = u8::rotate_left(data, 4);

    set_r8_data(sys, operand, data);

    sys.regs.set_flag(CpuFlag::Z, data == 0);
    sys.regs.set_flag(CpuFlag::N, false);
    sys.regs.set_flag(CpuFlag::H, false);
    sys.regs.set_flag(CpuFlag::C, false);
}

fn srl_r8(sys: &mut Sys, operand: R8) {
    let mut data = get_r8_data(sys, operand);
    let c_ = bit8(&data, 0);

    data = u8::wrapping_shr(data, 1); // todo correct??
    set_r8_data(sys, operand, data);

    sys.regs.set_flag(CpuFlag::Z, data == 0);
    sys.regs.set_flag(CpuFlag::N, false);
    sys.regs.set_flag(CpuFlag::H, false);
    sys.regs.set_flag(CpuFlag::C, c_ == 1);
}

fn bit_b3_r8(sys: &mut Sys, b3: u8, operand: R8) {
    let data = get_r8_data(sys, operand);
    let bit = bit8(&data, b3);

    sys.regs.set_flag(CpuFlag::Z, bit == 0);
    sys.regs.set_flag(CpuFlag::N, false);
    sys.regs.set_flag(CpuFlag::H, true);
}

fn res_b3_r8(sys: &mut Sys, b3: u8, operand: R8) {
    let mut data = get_r8_data(sys, operand);
    set_bit8(&mut data, b3, 0);
    set_r8_data(sys, operand, data);
}

fn set_b3_r8(sys: &mut Sys, b3: u8, operand: R8) {
    let mut data = get_r8_data(sys, operand);
    set_bit8(&mut data, b3, 1);
    set_r8_data(sys, operand, data);
}

// Misc functions.
fn hard_lock(sys: &mut Sys) {
    sys.hard_lock = true;
    Debug::fail(sys, "Invalid instr occurred.");
}
