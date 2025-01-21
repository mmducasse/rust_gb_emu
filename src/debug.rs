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
    mem::{io_regs::IoReg, map::MemSection},
    sys::Sys,
    util::{math::join_16, ring_buffer::RingBuffer},
};

#[derive(Clone)]
pub struct DebugConfig {
    pub enable_debug_print: bool,
    pub kill_after_cpu_ticks: Option<u64>,
    pub kill_after_nop_count: Option<u64>,
}

static mut DEBUG_STATE: Option<DebugState> = None;

pub struct DebugState {
    failure: Option<String>,
    pub config: DebugConfig,
    pub nop_count: u64,
    pub total_instrs_executed: u64,
    instr_ring_buffer: RingBuffer<InstrRecord>,
    used_instrs: HashMap<Instr, u64>,
    used_instr_variants: HashMap<String, u64>,
    used_io_regs: HashMap<IoReg, IoRegRecord>,
}

pub fn initialize_debug(config: DebugConfig) {
    unsafe {
        DEBUG_STATE = Some(DebugState {
            failure: None,
            config,
            nop_count: 0,
            total_instrs_executed: 0,
            instr_ring_buffer: RingBuffer::new(10),
            used_instrs: HashMap::new(),
            used_instr_variants: HashMap::new(),
            used_io_regs: HashMap::new(),
        });
    }
}

pub fn debug_state() -> &'static DebugState {
    unsafe {
        let Some(debug) = &DEBUG_STATE else {
            unreachable!();
        };
        return debug;
    }
}

pub fn get_failure() -> Option<String> {
    unsafe {
        let Some(debug) = &DEBUG_STATE else {
            unreachable!();
        };
        return debug.failure.clone();
    }
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

pub fn record_curr_instr(sys: &Sys) {
    unsafe {
        let Some(debug) = &mut DEBUG_STATE else {
            unreachable!();
        };
        debug.total_instrs_executed += 1;
    }

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
        Err(msg) => Instr::HardLock,
    };

    unsafe {
        let Some(debug) = &mut DEBUG_STATE else {
            unreachable!();
        };
        if let Instr::Nop = instr {
            // Don't record NOPs.
            debug.nop_count += 1;
            return;
        } else {
            debug.nop_count = 0;
        }
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

    unsafe {
        let Some(debug) = &mut DEBUG_STATE else {
            unreachable!();
        };
        debug.instr_ring_buffer.add(record);

        if debug.used_instrs.get(&instr).is_none() {
            debug.used_instrs.insert(instr, 0);
        }
        let count = debug.used_instrs.get(&instr).unwrap();
        debug.used_instrs.insert(instr, count + 1);

        let variant_str = format!("{:?}", instr).split("{").collect::<Vec<_>>()[0].to_owned();
        if debug.used_instr_variants.get(&variant_str).is_none() {
            debug.used_instr_variants.insert(variant_str.clone(), 0);
        }
        let count = debug.used_instr_variants.get(&variant_str).unwrap();
        debug.used_instr_variants.insert(variant_str, count + 1);
    }
}

/// is_write: false for read, true for write.
pub fn record_io_reg_usage(reg: IoReg, is_write: bool) {
    unsafe {
        let Some(debug) = &mut DEBUG_STATE else {
            unreachable!()
        };

        if !debug.used_io_regs.contains_key(&reg) {
            debug.used_io_regs.insert(
                reg,
                IoRegRecord {
                    reg,
                    reads: 0,
                    writes: 0,
                },
            );
        }
        let record = debug.used_io_regs.get_mut(&reg).unwrap();
        if is_write == false {
            record.reads += 1;
        }
        if is_write == true {
            record.writes += 1;
        }
    }
}

pub fn fail(msg: impl Into<String>) {
    unsafe {
        let Some(debug) = &mut DEBUG_STATE else {
            unreachable!()
        };
        debug.failure = Some(msg.into());
    }
}

pub fn print_system_state(sys: &Sys) {
    unsafe {
        let Some(debug) = &mut DEBUG_STATE else {
            unreachable!()
        };

        // Print Instr record
        println!("last {} instrs executed:", debug.instr_ring_buffer.len());
        for InstrRecord {
            addr,
            instr,
            imm,
            regs,
        } in debug.instr_ring_buffer.iter()
        {
            println!("  [${:0>4X}] {:?}", addr, instr);
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
            regs.print();
        }

        println!("  total instrs executed: {}", debug.total_instrs_executed);

        // Print all used instructions and counts.
        println!("\n  unique instrs executed: {}", debug.used_instrs.len());
        for (instr, count) in &debug.used_instrs {
            println!("    {:?}: {}", instr, count);
        }
        println!(
            "\n  unique instr variants executed: {}",
            debug.used_instr_variants.len()
        );
        for (variant_str, count) in &debug.used_instr_variants {
            println!("    {}: {}", variant_str, count);
        }

        // Print all IO reg usage.
        println!("\nIO Reg usage:");
        for reg in IoReg::iter() {
            if let Some(record) = debug.used_io_regs.get(&reg) {
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
            for data in sys.mem.get_section_slice(section) {
                sum += *data as u64;
            }
            println!("  {:?} data sum: {}", section, sum);
        }

        println!();
    }
}
