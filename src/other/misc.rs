use crate::{mem::Addr, sys::Sys};

pub fn shuffle_tile_data(sys: &mut Sys) {
    let len = (0x9800 - 0x8000) / 16;

    for i in 0..len {
        let rand_i = (macroquad::rand::rand() as usize) % len;

        let addr_a = 0x8000 + (i * 16) as Addr;
        let addr_b = 0x8000 + (rand_i * 16) as Addr;
        for j in 0..16 {
            let a = sys.mem.read(addr_a + j);
            let b = sys.mem.read(addr_b + j);
            sys.mem.write(addr_a + j, b);
            sys.mem.write(addr_b + j, a);
        }
    }
}
