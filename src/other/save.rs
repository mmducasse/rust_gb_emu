use std::{fs, io::Write, path::Path};

use macroquad::input::{is_key_pressed, KeyCode};

use crate::{mem::sections::MemSection, sys::Sys};

const SAVE_FOLDER_PATH: &str = "C:\\Users\\matth\\Desktop";

pub fn check_load_save_inputs(sys: &mut Sys) {
    if is_key_pressed(KeyCode::Backspace) {
        save_state(sys);
    } else if is_key_pressed(KeyCode::Equal) {
        load_state(sys);
    }
}

/// Serializes the system state and saves it to a file with the name of the currently running game.
pub fn save_state(sys: &Sys) {
    let mut buffer = vec![];

    save_section(sys, &mut buffer, MemSection::Vram);
    save_section(sys, &mut buffer, MemSection::Wram);
    save_section(sys, &mut buffer, MemSection::Oam);
    save_section(sys, &mut buffer, MemSection::IoRegs);
    save_section(sys, &mut buffer, MemSection::Hram);
    save_section(sys, &mut buffer, MemSection::IeReg);
    save_cart_ram(sys, &mut buffer);

    let file_name = sys.mem.cart.header().title();
    let path = format!("{}\\{}.sav", SAVE_FOLDER_PATH, file_name);
    println!("{}", path);
    let path = Path::new(&path);

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(&path)
        .unwrap();

    file.write_all(&buffer).unwrap();

    println!("Saved to: {:?}", path);
}

fn save_section(sys: &Sys, buffer: &mut Vec<u8>, section: MemSection) {
    let start = section.start_addr();
    let size = section.size();
    for addr in start..(start + size) {
        let data = sys.mem.read(addr);
        buffer.push(data);
    }
}

fn save_cart_ram(sys: &Sys, buffer: &mut Vec<u8>) {
    let ram = sys.mem.cart.ram();
    for data in ram {
        buffer.push(*data);
    }
}

/// Loads the system state from a save file with the name of the currently running game.
pub fn load_state(sys: &mut Sys) {
    let file_name = sys.mem.cart.header().title();
    let path = format!("{}\\{}.sav", SAVE_FOLDER_PATH, file_name);

    let buffer = fs::read(&path).unwrap();
    let mut idx = 0;

    load_section(sys, &buffer, &mut idx, MemSection::Vram);
    load_section(sys, &buffer, &mut idx, MemSection::Wram);
    load_section(sys, &buffer, &mut idx, MemSection::Oam);
    load_section(sys, &buffer, &mut idx, MemSection::IoRegs);
    load_section(sys, &buffer, &mut idx, MemSection::Hram);
    load_section(sys, &buffer, &mut idx, MemSection::IeReg);
    load_cart_ram(sys, &buffer, &mut idx);

    println!("Loaded from: {}", path);
}

fn load_section(sys: &mut Sys, buffer: &Vec<u8>, idx: &mut usize, section: MemSection) {
    let start = section.start_addr();
    let size = section.size();
    for addr in start..(start + size) {
        let data = buffer[*idx];
        sys.mem.write(addr, data);
        *idx += 1;
    }
}

fn load_cart_ram(sys: &mut Sys, buffer: &Vec<u8>, idx: &mut usize) {
    let ram = sys.mem.cart.ram_mut();

    for data in ram {
        *data = buffer[*idx];
        *idx += 1;
    }
}
