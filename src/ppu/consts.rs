use xf::num::{
    irect::IRect,
    ivec2::{i2, IVec2},
};

use crate::{consts::P8, mem::Addr};

pub const TILE_SIZE: IVec2 = P8;
pub const VIEWPORT_P8_SIZE: IVec2 = i2(20, 18);
pub const VIEWPORT_SIZE: IVec2 = IVec2::mul(VIEWPORT_P8_SIZE, P8);

pub const TILE_MAP_P8_SIZE: IVec2 = i2(32, 32);
pub const TILE_MAP_SIZE: IVec2 = IVec2::mul(TILE_MAP_P8_SIZE, P8);
pub const TILE_DATA_BLOCK_DRAW_P8_SIZE: IVec2 = i2(16, 8);
pub const TILE_DATA_BLOCK_DRAW_SIZE: IVec2 = IVec2::mul(TILE_DATA_BLOCK_DRAW_P8_SIZE, P8);
pub const TILE_DATA_P8_SIZE: IVec2 = IVec2::mul(TILE_DATA_BLOCK_DRAW_P8_SIZE, i2(1, 3));
pub const TILE_DATA_SIZE: IVec2 = IVec2::mul(TILE_DATA_P8_SIZE, P8);

pub const TILE_MAP_ADDR_9800: Addr = 0x9800;
pub const TILE_MAP_ADDR_9C00: Addr = 0x9C00;

pub const TILE_DATA_TILE_SIZE: u16 = 16;
pub const TILE_DATA_BLOCK_SIZE: u16 = 0x0800;
pub const TILE_DATA_ADDR_8000: Addr = 0x8000;
pub const TILE_DATA_ADDR_8800: Addr = 0x8800;
pub const TILE_DATA_ADDR_9000: Addr = 0x9000;

pub const OAM_OBJ_SIZE: u16 = 4;
pub const OAM_ADDR_FE00: Addr = 0xFE00;

pub const VIEWPORT_ORG: IVec2 = P8;
pub const TILE_MAP_ORG: IVec2 = i2(VIEWPORT_ORG.x + (VIEWPORT_P8_SIZE.x + 1) * P8.x, P8.y);
pub const TILE_DATA_ORG: IVec2 = i2(TILE_MAP_ORG.x + (TILE_MAP_P8_SIZE.x + 1) * P8.x, P8.y);
pub const JOYPAD_ORG: IVec2 = i2(
    VIEWPORT_ORG.x,
    VIEWPORT_ORG.y + (VIEWPORT_P8_SIZE.y + 1) * P8.y,
);

pub const WINDOW_P8_SIZE_NORMAL: IVec2 = i2(VIEWPORT_P8_SIZE.x + 2, VIEWPORT_P8_SIZE.y + 10);
pub const WINDOW_SIZE_NORMAL: IVec2 = IVec2::mul(WINDOW_P8_SIZE_NORMAL, P8);
pub const WINDOW_BOUNDS_NORMAL: IRect = IRect::of_size(WINDOW_SIZE_NORMAL);

pub const WINDOW_P8_SIZE_DEBUG: IVec2 = i2(
    VIEWPORT_P8_SIZE.x + TILE_MAP_P8_SIZE.x + TILE_DATA_P8_SIZE.x + 4,
    TILE_MAP_P8_SIZE.y + 2,
);
pub const WINDOW_SIZE_DEBUG: IVec2 = IVec2::mul(WINDOW_P8_SIZE_DEBUG, P8);
pub const WINDOW_BOUNDS_DEBUG: IRect = IRect::of_size(WINDOW_SIZE_DEBUG);

pub fn window_size(show_debug_views: bool) -> IVec2 {
    if show_debug_views {
        WINDOW_SIZE_DEBUG
    } else {
        WINDOW_SIZE_NORMAL
    }
}
