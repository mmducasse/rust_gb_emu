use xf::num::{
    irect::{ir, IRect},
    ivec2::{i2, IVec2},
};

use crate::ppu::{self};

pub const FAIL_ON_BAD_RW: bool = false;

pub const P1: IVec2 = i2(1, 1);
pub const P2: IVec2 = i2(2, 2);
pub const P4: IVec2 = i2(4, 4);
pub const P8: IVec2 = i2(8, 8);
pub const P16: IVec2 = i2(16, 16);

pub const PIXEL_SCALE: f32 = 2.0;

pub const SCREEN_SIZE: IVec2 = ppu::consts::DEBUG_SCREEN_SIZE;
pub const SCREEN_BOUNDS: IRect = ir(IVec2::ZERO, SCREEN_SIZE);

// Memory sizes
pub const B_1: usize = 1 << 0;
pub const B_2: usize = 1 << 1;
pub const B_4: usize = 1 << 2;
pub const B_8: usize = 1 << 3;
pub const B_16: usize = 1 << 4;
pub const B_32: usize = 1 << 5;
pub const B_64: usize = 1 << 6;
pub const B_128: usize = 1 << 7;
pub const B_256: usize = 1 << 8;
pub const B_512: usize = 1 << 9;

pub const KB_1: usize = 1 << 10;
pub const KB_2: usize = 1 << 11;
pub const KB_4: usize = 1 << 12;
pub const KB_8: usize = 1 << 13;
pub const KB_16: usize = 1 << 14;
pub const KB_32: usize = 1 << 15;
pub const KB_64: usize = 1 << 16;
pub const KB_128: usize = 1 << 17;
pub const KB_256: usize = 1 << 18;
pub const KB_512: usize = 1 << 19;

pub const MB_1: usize = 1 << 20;
pub const MB_2: usize = 1 << 21;
pub const MB_4: usize = 1 << 22;
pub const MB_8: usize = 1 << 23;
pub const MB_16: usize = 1 << 24;
