use xf::num::{
    irect::{ir, IRect},
    ivec2::{i2, IVec2},
};

pub const P16: IVec2 = i2(16, 16);

pub const PIXEL_SCALE: f32 = 3.0;

pub const SCREEN_P16_SIZE: IVec2 = i2(8, 8);
pub const SCREEN_SIZE: IVec2 = i2(SCREEN_P16_SIZE.x * P16.x, SCREEN_P16_SIZE.y * P16.y);
pub const SCREEN_BOUNDS: IRect = ir(IVec2::ZERO, SCREEN_SIZE);
