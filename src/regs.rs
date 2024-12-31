use crate::data::{get_bit_u8, join_16, set_bit_u8, split_16};

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

#[derive(Default)]
pub struct CpuRegs {
    regs8: [u8; 8],

    sp: u16,
    pc: u16,
}

impl CpuRegs {
    pub fn new() -> Self {
        let mut regs = Self::default();
        regs.pc = 0x100;

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
        let idx = flag as usize;
        return get_bit_u8(self.get_8(CpuReg8::F), idx) > 0;
    }

    pub fn set_flag(&mut self, flag: CpuFlag, value: bool) {
        let idx = flag as usize;
        let value = if value { 0b1 } else { 0b0 };
        return set_bit_u8(self.mut_8(CpuReg8::F), idx, value);
    }

    pub fn print(&self) {
        use CpuReg16::*;
        use CpuReg8::*;

        println!("Registers:");

        println!(
            "  A = {:#02x}, F = {:#02x}, AF = {:#04x}",
            self.get_8(A),
            self.get_8(F),
            self.get_16(AF)
        );
        println!(
            "  B = {:#02x}, C = {:#02x}, BC = {:#04x}",
            self.get_8(B),
            self.get_8(C),
            self.get_16(BC)
        );
        println!(
            "  D = {:#02x}, E = {:#02x}, DE = {:#04x}",
            self.get_8(D),
            self.get_8(E),
            self.get_16(DE)
        );
        println!(
            "  H = {:#02x}, L = {:#02x}, HL = {:#04x}",
            self.get_8(H),
            self.get_8(L),
            self.get_16(HL)
        );

        println!("  SP = {:#04x}", self.get_16(SP));
        println!("  PC = {:#04x}", self.get_16(PC));

        println!();
    }
}
