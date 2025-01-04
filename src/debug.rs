use std::mem::transmute;

use crate::{
    instr::{decode, ImmType, Instr},
    math::join_16,
    regs::CpuRegs,
    sys::Sys,
    util::ring_buffer::RingBuffer,
};

pub struct Debug {
    asm_ring_buffer: RingBuffer<AsmRecord>,
}

struct AsmRecord {
    addr: u16,
    asm: Instr,
    imm: ImmValue,
    regs: CpuRegs,
}

enum ImmValue {
    None,
    Imm8(u8),
    Imm16(u16),
}

impl Debug {
    pub fn new() -> Self {
        Self {
            asm_ring_buffer: RingBuffer::new(10),
        }
    }

    pub fn record_curr_instr(sys: &mut Sys) {
        let pc = sys.get_pc();
        let op = sys.rd_mem(pc);
        let asm = decode(op);

        let imm_value = match asm.imm_type() {
            ImmType::None => ImmValue::None,
            ImmType::Imm8 => {
                let imm8 = sys.rd_mem(pc + 1);
                ImmValue::Imm8(imm8)
            }
            ImmType::Imm16 => {
                let lo = sys.rd_mem(pc + 1);
                let hi = sys.rd_mem(pc + 2);
                let imm16 = join_16(hi, lo);
                ImmValue::Imm16(imm16)
            }
        };

        let record = AsmRecord {
            addr: pc,
            asm,
            imm: imm_value,
            regs: sys.regs.clone(),
        };

        sys.debug.asm_ring_buffer.add(record);
    }

    pub fn fail(sys: &Sys, msg: impl Into<String>) -> ! {
        println!("FAILURE: {}\n\n", msg.into());

        // Print ASM record
        println!("last {} instrs executed:", sys.debug.asm_ring_buffer.len());
        for AsmRecord {
            addr,
            asm,
            imm,
            regs,
        } in sys.debug.asm_ring_buffer.iter()
        {
            println!("  [${:0>4X}] {:?}", addr, asm);
            unsafe {
                match imm {
                    ImmValue::None => {}
                    ImmValue::Imm8(imm8) => println!(
                        "     imm8 = {:#02x} (u{}) (s{})",
                        imm8,
                        imm8,
                        transmute::<u8, i8>(*imm8)
                    ),
                    ImmValue::Imm16(imm16) => println!(
                        "     imm16 = {:#04x} (u{}) (s{})",
                        imm16,
                        imm16,
                        transmute::<u16, i16>(*imm16)
                    ),
                };
            }
            regs.print();
        }

        // // System state.
        // println!("\nFinal state:");
        // sys.regs.print();

        panic!("");
    }
}
