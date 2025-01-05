use std::mem::transmute;

use crate::{
    cpu::{
        instr::{decode, ImmType, Instr},
        regs::CpuRegs,
    },
    sys::Sys,
    util::{math::join_16, ring_buffer::RingBuffer},
};

pub struct Debug {
    pub enable: bool,
    pub nop_count: u32,
    pub total_instrs_executed: u64,
    instr_ring_buffer: RingBuffer<InstrRecord>,
}

struct InstrRecord {
    addr: u16,
    instr: Instr,
    imm: ImmValue,
    regs: CpuRegs,
}

enum ImmValue {
    None,
    Imm8(u8),
    Imm16(u16),
}

impl Debug {
    pub const EXIT_AFTER_NOP_COUNT: u32 = 8;

    pub fn new() -> Self {
        Self {
            enable: false,
            nop_count: 0,
            total_instrs_executed: 0,
            instr_ring_buffer: RingBuffer::new(10),
        }
    }

    pub fn record_curr_instr(sys: &mut Sys) {
        sys.debug.total_instrs_executed += 1;

        let mut pc = sys.get_pc();
        let mut op = sys.rd_mem(pc);
        let mut has_cb_prefix = false;
        if op == Instr::CB_PREFIX {
            sys.inc_pc();
            pc = sys.get_pc();
            op = sys.rd_mem(pc);
            has_cb_prefix = true;
        }
        let instr = match decode(op, has_cb_prefix) {
            Ok(instr) => instr,
            Err(msg) => Debug::fail(sys, msg),
        };

        if let Instr::Nop = instr {
            // Don't record NOPs.
            sys.debug.nop_count += 1;
            return;
        } else {
            sys.debug.nop_count = 0;
        }

        let imm_value = match instr.imm_type() {
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

        let record = InstrRecord {
            addr: pc,
            instr,
            imm: imm_value,
            regs: sys.regs.clone(),
        };

        sys.debug.instr_ring_buffer.add(record);
    }

    pub fn fail(sys: &Sys, msg: impl Into<String>) -> ! {
        // Print Instr record
        println!(
            "last {} instrs executed:",
            sys.debug.instr_ring_buffer.len()
        );
        for InstrRecord {
            addr,
            instr,
            imm,
            regs,
        } in sys.debug.instr_ring_buffer.iter()
        {
            println!("  [${:0>4X}] {:?}", addr, instr);
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

        println!("FAILURE: {}\n", msg.into());
        println!(
            "  total instrs executed: {}",
            sys.debug.total_instrs_executed
        );

        // // System state.
        // println!("\nFinal state:");
        // sys.regs.print();

        println!();
        panic!("");
    }
}
