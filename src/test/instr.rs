#[cfg(test)]
mod tests {
    use crate::{cpu::regs::CpuReg8, sys::Sys};

    #[test]
    fn test_ld() {
        let mut sys = Sys::new();
        sys.cart
            .load(".\\assets\\files\\custom_roms\\ld_r8_r8\\rom.gb");
        sys.run();

        assert_eq!(sys.regs.get_8(CpuReg8::B), 1);
        assert_eq!(sys.regs.get_8(CpuReg8::C), 2);
        assert_eq!(sys.regs.get_8(CpuReg8::D), 3);
        assert_eq!(sys.regs.get_8(CpuReg8::E), 4);

        assert_eq!(sys.regs.get_8(CpuReg8::H), 5);
        assert_eq!(sys.regs.get_8(CpuReg8::L), 6);
        assert_eq!(sys.rd_hl_p(), 7);
        assert_eq!(sys.regs.get_8(CpuReg8::A), 8);
    }
}
