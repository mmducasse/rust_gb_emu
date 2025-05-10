use crate::util::math::{bit8, join_16, set_bit8, split_16};

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

impl CpuReg16 {
    pub fn get_parts(self) -> (CpuReg8, CpuReg8) {
        match self {
            CpuReg16::AF => (CpuReg8::A, CpuReg8::F),
            CpuReg16::BC => (CpuReg8::B, CpuReg8::C),
            CpuReg16::DE => (CpuReg8::D, CpuReg8::E),
            CpuReg16::HL => (CpuReg8::H, CpuReg8::L),
            _ => {
                panic!("Cannot split {:?} into 8-bit registers.", self);
            }
        }
    }
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

    pub fn set_8(&mut self, reg: CpuReg8, mut data: u8) {
        if reg == CpuReg8::F {
            data &= 0xF0;
        }
        let idx = reg as usize;
        self.regs8[idx] = data;
    }

    pub fn get_16(&self, reg: CpuReg16) -> u16 {
        match reg {
            CpuReg16::SP => self.sp,
            CpuReg16::PC => self.pc,
            _ => {
                let (reg_hi, reg_lo) = reg.get_parts();
                let hi = self.get_8(reg_hi);
                let lo = self.get_8(reg_lo);

                join_16(hi, lo)
            }
        }
    }

    pub fn set_16(&mut self, reg: CpuReg16, data: u16) {
        match reg {
            CpuReg16::SP => self.sp = data,
            CpuReg16::PC => self.pc = data,
            _ => {
                let (hi, lo) = split_16(data);
                let (reg_hi, reg_lo) = reg.get_parts();
                self.set_8(reg_hi, hi);
                self.set_8(reg_lo, lo);
            }
        }
    }

    pub fn pc(&self) -> u16 {
        self.get_16(CpuReg16::PC)
    }

    pub fn sp(&self) -> u16 {
        self.get_16(CpuReg16::SP)
    }

    pub fn get_flag(&self, flag: CpuFlag) -> bool {
        let idx = flag as u8;
        bit8(&self.get_8(CpuReg8::F), idx) == 1
    }

    pub fn set_flag(&mut self, flag: CpuFlag, value: bool) {
        let idx = flag as u8;
        let mut f_data = self.get_8(CpuReg8::F);
        set_bit8(&mut f_data, idx, value.into());
        self.set_8(CpuReg8::F, f_data);
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
}
