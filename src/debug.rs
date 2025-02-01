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
        sections::{Addr, MemSection},
    },
    sys::Sys,
    util::{math::join_16, ring_buffer::RingBuffer},
};

#[derive(Clone)]
pub struct DebugConfig {
    pub enable_debug_print: bool,
    pub kill_after_cpu_ticks: Option<u64>,
    pub kill_after_nop_count: Option<u64>,
    pub last_instr_count: usize,
}

static mut DEBUG_STATE: Option<DebugState> = None;

pub struct DebugState {
    failure: Option<String>,
    pending_breakpoint: bool,
    pub config: DebugConfig,
    pub nop_count: u64,
    pub total_instrs_executed: u64,
    instr_ring_buffer: RingBuffer<InstrRecord>,
    used_instrs: HashMap<Instr, u64>,
    used_instr_variants: HashMap<String, u64>,
    used_io_regs: HashMap<IoReg, IoRegRecord>,
    pub print_instrs: u16,
}

pub fn initialize_debug(config: DebugConfig) {
    unsafe {
        let last_instr_count = config.last_instr_count;

        DEBUG_STATE = Some(DebugState {
            failure: None,
            pending_breakpoint: false,
            config,
            nop_count: 0,
            total_instrs_executed: 0,
            instr_ring_buffer: RingBuffer::new(last_instr_count),
            used_instrs: HashMap::new(),
            used_instr_variants: HashMap::new(),
            used_io_regs: HashMap::new(),
            print_instrs: 0,
        });
    }
}

pub fn debug_state() -> &'static mut DebugState {
    unsafe {
        let Some(debug) = &mut DEBUG_STATE else {
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

pub fn request_breakpoint() {
    unsafe {
        let Some(debug) = &mut DEBUG_STATE else {
            unreachable!();
        };
        debug.pending_breakpoint = true;
    }
}

pub fn take_pending_breakpoint() -> bool {
    unsafe {
        let Some(debug) = &mut DEBUG_STATE else {
            unreachable!();
        };
        let pending_breakpoint = debug.pending_breakpoint;
        debug.pending_breakpoint = false;

        return pending_breakpoint;
    }
}

struct InstrRecord {
    addr: u16,
    instr: Instr,
    imm: ImmValue,
    regs: CpuRegs,

    stack_record: StackRecord,
}

struct StackRecord {
    pub offset: Addr,
    pub sp: Addr,
    pub items: Vec<u8>,
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
    last_write_data: u8,
}

const DO_RECORD_NOP: bool = false;

pub fn record_curr_instr(sys: &Sys) {
    unsafe {
        let Some(debug) = &mut DEBUG_STATE else {
            return;
        };
        debug.total_instrs_executed += 1;
    }

    let mut pc = sys.get_pc();
    let addr = pc;
    let mut op = sys.mem.read(pc);
    let mut has_cb_prefix = false;
    if op == Instr::CB_PREFIX {
        pc += 1;
        op = sys.mem.read(pc);
        has_cb_prefix = true;
    }
    let instr = match decode(op, has_cb_prefix) {
        Ok(instr) => instr,
        Err(msg) => {
            return;
        }
    };

    unsafe {
        let Some(debug) = &mut DEBUG_STATE else {
            unreachable!();
        };
        if let Instr::Nop = instr {
            // Don't record NOPs.
            debug.nop_count += 1;
            if !DO_RECORD_NOP {
                return;
            }
        } else {
            debug.nop_count = 0;
        }
    }

    let imm_value = match instr.imm_type() {
        ImmType::None => ImmValue::None,
        ImmType::Imm8 => {
            let imm8 = sys.mem.read(pc + 1);
            ImmValue::Imm8(imm8)
        }
        ImmType::Imm16 => {
            let lo = sys.mem.read(pc + 1);
            let hi = sys.mem.read(pc + 2);
            let imm16 = join_16(hi, lo);
            ImmValue::Imm16(imm16)
        }
    };

    let stack_record = {
        let sp = sys.get_sp();
        let range_min = u16::saturating_sub(sp, 2);
        let range_max = u16::saturating_add(sp, 2);
        let offset = range_min;
        let mut items = vec![];
        for addr in range_min..=range_max {
            let data = sys.mem.read(addr);
            items.push(data);
        }

        StackRecord { offset, sp, items }
    };

    let record = InstrRecord {
        addr,
        instr,
        imm: imm_value,
        regs: sys.regs.clone(),

        stack_record,
    };

    if debug_state().print_instrs > 0 {
        print_instr_record(&record);
        debug_state().print_instrs -= 1;
    }

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

pub fn print_last_instr() {
    unsafe {
        let Some(debug) = &mut DEBUG_STATE else {
            unreachable!()
        };

        if let Some(last) = debug.instr_ring_buffer.iter().last() {
            print_instr_record(last);
        };
    }
}

/// is_write: false for read, true for write.
pub fn record_io_reg_usage(reg: IoReg, is_write: bool, data: u8) {
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
                    last_write_data: 0,
                },
            );
        }
        let record = debug.used_io_regs.get_mut(&reg).unwrap();
        if is_write == false {
            record.reads += 1;
        }
        if is_write == true {
            record.writes += 1;
            record.last_write_data = data;
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

const PRINT_LAST_INSTRS: bool = true;
const PRINT_TOTAL_INSTRS: bool = true;
const PRINT_IO_REG_USAGE: bool = true;
const PRINT_SYS_STATE: bool = true;
const PRINT_MEM_SUMS: bool = true;
const PRINT_STACK_RECORDS: bool = true;

pub fn print_system_state(sys: &Sys) {
    unsafe {
        let Some(debug) = &mut DEBUG_STATE else {
            unreachable!()
        };

        if PRINT_LAST_INSTRS {
            // Print Instr record
            println!("last {} instrs executed:", debug.instr_ring_buffer.len());
            for record in debug.instr_ring_buffer.iter() {
                print_instr_record(record);
            }
        }

        if PRINT_TOTAL_INSTRS {
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
        }

        if PRINT_IO_REG_USAGE {
            // Print all IO reg usage.
            println!("\nIO Reg usage:");
            for reg in IoReg::iter() {
                if let Some(record) = debug.used_io_regs.get(&reg) {
                    println!(
                        "  {:?}: {} reads, {} writes, [last write = 0b{:0>8b}]",
                        reg, record.reads, record.writes, record.last_write_data
                    );
                }
            }
        }

        if PRINT_SYS_STATE {
            // System state.
            println!("\nFinal state:");
            sys.print();
        }

        if PRINT_MEM_SUMS {
            // Sample of each memory section.
            println!("\nMemory section sums:");
            for section in MemSection::iter() {
                let mut sum = 0;
                for data in sys.mem.get_section_slice(section) {
                    sum += *data as u64;
                }
                println!("  {:?} data sum: {}", section, sum);
            }
        }

        println!();
    }
}

fn print_instr_record(record: &InstrRecord) {
    let InstrRecord {
        addr,
        instr,
        imm,
        regs,
        stack_record,
    } = record;

    println!("  [${:0>4X}] {:?}", addr, instr);
    match imm {
        ImmValue::None => {}
        ImmValue::Imm8(imm8) => println!("     imm8 = {:#02x} (u{}) (s{})", imm8, imm8, unsafe {
            transmute::<u8, i8>(*imm8)
        }),
        ImmValue::Imm16(imm16) => {
            println!("     imm16 = {:#04x} (u{}) (s{})", imm16, imm16, unsafe {
                transmute::<u16, i16>(*imm16)
            })
        }
    };
    regs.print();

    if PRINT_STACK_RECORDS {
        let sp = stack_record.sp;
        let mut addr = stack_record.offset;

        for item in &stack_record.items {
            let prefix = if sp == addr { "sp>" } else { "   " };
            println!("{} | 0x{:0>4X} | {:0>2x} |", prefix, addr, *item);

            addr = u16::saturating_add(addr, 1);
        }
        println!();
    }
}
