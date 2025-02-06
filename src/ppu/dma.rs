use crate::{mem::io_regs::IoReg, sys::Sys};

const DMA_DURATION_M_CYCLES: u16 = 160;

pub struct Dma {
    is_active: bool,
    next_idx: u16,
}

impl Dma {
    pub fn new() -> Self {
        Self {
            is_active: false,
            next_idx: 0,
        }
    }
}

pub fn update_dma(sys: &mut Sys) {
    let dma = sys.ppu.dma_mut();
    if !dma.is_active {
        if sys.mem.io_regs.dma_requested {
            sys.mem.io_regs.dma_requested = false;
            start_dma(sys);
        } else {
            return;
        }
    }

    transfer_one_byte(sys);
}

fn start_dma(sys: &mut Sys) {
    let dma = sys.ppu.dma_mut();
    dma.is_active = true;
    dma.next_idx = 0;

    //
    let dma_val = sys.mem.io_regs.get(IoReg::Dma) as u16;
    let src_addr = dma_val * 0x100;
    println!("Start DMA transfer from: {:0>4X} ", src_addr);
}

fn transfer_one_byte(sys: &mut Sys) {
    let dma = sys.ppu.dma_mut();

    let idx = dma.next_idx;
    let dma_val = sys.mem.io_regs.get(IoReg::Dma) as u16;
    let src_addr = (dma_val * 0x100) + idx;
    let dst_addr = 0xFE00 + idx;

    println!("Transfer from {:0>4X} to {:0>4X}", src_addr, dst_addr);

    let data = sys.mem.read(src_addr);
    sys.mem.write(dst_addr, data);
    let sanity_check = sys.mem.read(dst_addr);

    assert_eq!(data, sanity_check);

    dma.next_idx += 1;
    if dma.next_idx >= DMA_DURATION_M_CYCLES {
        dma.is_active = false;
    }
}
