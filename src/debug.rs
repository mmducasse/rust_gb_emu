use std::{
    collections::{HashMap, HashSet},
    fmt::format,
    mem::transmute,
};

use strum::IntoEnumIterator;

use crate::{
    cpu::{
        instr::{decode, ImmType, Instr},
        regs::CpuRegs,
    },
    mem::{
        io_regs::IoReg,
        map::{get_section_slice, print_section, MemSection},
    },
    sys::Sys,
    util::{math::join_16, ring_buffer::RingBuffer},
};

pub struct Debug {
    pub enable_debug_print: bool,
    pub nop_count: u64,
    pub total_instrs_executed: u64,
    instr_ring_buffer: RingBuffer<InstrRecord>,
    used_instrs: HashMap<Instr, u64>,
    used_instr_variants: HashMap<String, u64>,
    used_io_regs: HashMap<IoReg, IoRegRecord>,
    pub kill_after_cpu_ticks: Option<u64>,
    pub kill_after_nop_count: Option<u64>,
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

struct IoRegRecord {
    reg: IoReg,
    reads: u64,
    writes: u64,
}

impl Debug {
    pub fn new() -> Self {
        Self {
            enable_debug_print: false,
            nop_count: 0,
            total_instrs_executed: 0,
            instr_ring_buffer: RingBuffer::new(10),
            used_instrs: HashMap::new(),
            used_instr_variants: HashMap::new(),
            used_io_regs: HashMap::new(),
            kill_after_cpu_ticks: None,
            kill_after_nop_count: None,
        }
    }

    pub fn record_curr_instr(sys: &mut Sys) {
        sys.debug.total_instrs_executed += 1;

        let mut pc = sys.get_pc();
        let mut op = sys.mem_get(pc);
        let mut has_cb_prefix = false;
        if op == Instr::CB_PREFIX {
            pc += 1;
            op = sys.mem_get(pc);
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
                let imm8 = sys.mem_get(pc + 1);
                ImmValue::Imm8(imm8)
            }
            ImmType::Imm16 => {
                let lo = sys.mem_get(pc + 1);
                let hi = sys.mem_get(pc + 2);
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

        if sys.debug.used_instrs.get(&instr).is_none() {
            sys.debug.used_instrs.insert(instr, 0);
        }
        let count = sys.debug.used_instrs.get(&instr).unwrap();
        sys.debug.used_instrs.insert(instr, count + 1);

        let variant_str = format!("{:?}", instr).split("{").collect::<Vec<_>>()[0].to_owned();
        if sys.debug.used_instr_variants.get(&variant_str).is_none() {
            sys.debug.used_instr_variants.insert(variant_str.clone(), 0);
        }
        let count = sys.debug.used_instr_variants.get(&variant_str).unwrap();
        sys.debug.used_instr_variants.insert(variant_str, count + 1);
    }

    /// is_write: false for read, true for write.
    pub fn record_io_reg_usage(&mut self, reg: IoReg, is_write: bool) {
        if !self.used_io_regs.contains_key(&reg) {
            self.used_io_regs.insert(
                reg,
                IoRegRecord {
                    reg,
                    reads: 0,
                    writes: 0,
                },
            );
        }
        let record = self.used_io_regs.get_mut(&reg).unwrap();
        if is_write == false {
            record.reads += 1;
        }
        if is_write == true {
            record.writes += 1;
        }
    }

    pub fn fail(sys: &Sys, msg: impl Into<String>) -> ! {
        Self::print_system_state(sys);
        println!("\nFAILURE: {}\n", msg.into());
        panic!("");
    }

    pub fn print_system_state(sys: &Sys) {
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

        println!(
            "  total instrs executed: {}",
            sys.debug.total_instrs_executed
        );

        // Print all used instructions and counts.
        println!(
            "\n  unique instrs executed: {}",
            sys.debug.used_instrs.len()
        );
        for (instr, count) in &sys.debug.used_instrs {
            println!("    {:?}: {}", instr, count);
        }
        println!(
            "\n  unique instr variants executed: {}",
            sys.debug.used_instr_variants.len()
        );
        for (variant_str, count) in &sys.debug.used_instr_variants {
            println!("    {}: {}", variant_str, count);
        }

        // Print all IO reg usage.
        println!("IO Reg usage:");
        for reg in IoReg::iter() {
            if let Some(record) = sys.debug.used_io_regs.get(&reg) {
                println!(
                    "  {:?}: {} reads, {} writes",
                    reg, record.reads, record.writes
                );
            }
        }

        // System state.
        println!("\nFinal state:");
        sys.print();

        // Sample of each memory section.
        println!("\nMemory section sums:");
        for section in MemSection::iter() {
            let mut sum = 0;
            for data in get_section_slice(sys, section) {
                sum += *data as u64;
            }
            println!("  {:?} data sum: {}", section, sum);
        }

        println!();
    }
}
