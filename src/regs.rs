use std::mem::transmute;

use crate::{
    math::{bit8, join_16, set_bit8, split_16},
    sys::Sys,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CpuReg8 {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CpuReg16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CpuFlag {
    /// Zero flag.
    Z = 7,

    /// Subtraction flag.
    N = 6,

    /// Half Carry flag.
    H = 5,

    /// Carry flag.
    C = 4,
}

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct CpuRegs {
    regs8: [u8; 8],

    sp: u16,
    pc: u16,
}

impl CpuRegs {
    pub fn new() -> Self {
        let mut regs = Self::default();
        regs.pc = 0x0100;
        regs.sp = 0xFFFE;

        regs
    }

    pub fn get_8(&self, reg: CpuReg8) -> u8 {
        let idx = reg as usize;
        self.regs8[idx]
    }

    pub fn set_8(&mut self, reg: CpuReg8, data: u8) {
        let idx = reg as usize;
        self.regs8[idx] = data;
    }

    pub fn mut_8(&mut self, reg: CpuReg8) -> &mut u8 {
        let idx = reg as usize;
        &mut self.regs8[idx]
    }

    pub fn get_16(&self, reg: CpuReg16) -> u16 {
        match reg {
            CpuReg16::SP => self.sp,
            CpuReg16::PC => self.pc,
            _ => {
                let idx = (reg as usize) * 2;
                let hi = self.regs8[idx];
                let lo = self.regs8[idx + 1];
                let data = join_16(hi, lo);
                data
            }
        }
    }

    pub fn set_16(&mut self, reg: CpuReg16, data: u16) {
        match reg {
            CpuReg16::SP => self.sp = data,
            CpuReg16::PC => self.pc = data,
            _ => {
                let (hi, lo) = split_16(data);
                let idx = (reg as usize) * 2;
                self.regs8[idx] = hi;
                self.regs8[idx + 1] = lo;
            }
        }
    }

    pub fn get_flag(&self, flag: CpuFlag) -> bool {
        let idx = flag as u8;
        return bit8(&self.get_8(CpuReg8::F), idx) > 0;
    }

    pub fn set_flag(&mut self, flag: CpuFlag, value: bool) {
        let idx = flag as u8;
        let value = if value { 0b1 } else { 0b0 };
        return set_bit8(self.mut_8(CpuReg8::F), idx, value);
    }

    pub fn print(&self) {
        use CpuReg16::*;
        use CpuReg8::*;

        // println!("Registers:");

        println!(
            "  A={:0>2X} F={:0>2X} AF={:0>4X}",
            self.get_8(A),
            self.get_8(F),
            self.get_16(AF)
        );
        println!(
            "  B={:0>2X} C={:0>2X} BC={:0>4X}",
            self.get_8(B),
            self.get_8(C),
            self.get_16(BC)
        );
        println!(
            "  D={:0>2X} E={:0>2X} DE={:0>4X}",
            self.get_8(D),
            self.get_8(E),
            self.get_16(DE)
        );
        println!(
            "  H={:0>2X} L={:0>2X} HL={:0>4X}",
            self.get_8(H),
            self.get_8(L),
            self.get_16(HL)
        );

        println!(
            "  SP={:0>4X}   PC={:0>4X}",
            self.get_16(SP),
            self.get_16(PC)
        );

        let z: u8 = self.get_flag(CpuFlag::Z).into();
        let n: u8 = self.get_flag(CpuFlag::N).into();
        let h: u8 = self.get_flag(CpuFlag::H).into();
        let c: u8 = self.get_flag(CpuFlag::C).into();
        println!("  Z={} N={} H={} C={}", z, n, h, c);

        println!();
    }

    pub fn print_key_addrs(sys: &Sys) {
        let sp_p = sys.rd_mem(sys.regs.get_16(CpuReg16::SP));
        let pc_p = sys.rd_mem(sys.regs.get_16(CpuReg16::PC));
        let hl_p = sys.rd_mem(sys.regs.get_16(CpuReg16::HL));

        println!(
            "  [SP]={:0>2X}  [PC]={:0>2X}  [HL]={:0>2X}",
            sp_p, pc_p, hl_p
        );
    }
}
