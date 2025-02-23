use std::{fs, io::Write, path::Path};

use crate::{sys::Sys, util::slice::copy_from_safe};

const SAVE_FOLDER_PATH: &str = "C:\\Users\\matth\\Desktop";

/// Saves the contents of cartridge RAM to a file named after the currently running game.
pub fn save_state(sys: &Sys) {
    let cart_ram = sys.mem.cart.ram();

    let file_name = sys.mem.cart.header().title();
    let path = format!("{}\\{}.sav", SAVE_FOLDER_PATH, file_name);
    println!("{}", path);
    let path = Path::new(&path);

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(&path)
        .unwrap();

    file.write_all(&cart_ram).unwrap();

    println!("Saved to: {:?}", path);
}

/// Loads the contents of cartridge RAM from a file named after the currently running game.
pub fn load_state(sys: &mut Sys) -> bool {
    let file_name = sys.mem.cart.header().title();
    let path = format!("{}\\{}.sav", SAVE_FOLDER_PATH, file_name);

    let Ok(buffer) = fs::read(&path) else {
        return false;
    };

    let cart_ram = sys.mem.cart.ram_mut();
    copy_from_safe(cart_ram, &buffer);

    println!("Loaded from: {}", path);

    return true;
}
