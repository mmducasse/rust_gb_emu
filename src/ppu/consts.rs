use xf::num::ivec2::{i2, IVec2};

use crate::{consts::P8, mem::sections::Addr};

pub const TILE_SIZE: IVec2 = P8;
pub const VIEWPORT_P8_SIZE: IVec2 = i2(20, 18);
pub const VIEWPORT_SIZE: IVec2 = IVec2::mul(VIEWPORT_P8_SIZE, P8);

pub const TILE_MAP_P8_SIZE: IVec2 = i2(32, 32);
pub const TILE_MAP_SIZE: IVec2 = IVec2::mul(TILE_MAP_P8_SIZE, P8);
pub const TILE_DATA_P8_SIZE: IVec2 = i2(16, 24);
pub const TILE_DATA_SIZE: IVec2 = IVec2::mul(TILE_DATA_P8_SIZE, P8);

pub const DEBUG_SCREEN_SIZE: IVec2 = i2(
    VIEWPORT_SIZE.x + TILE_MAP_SIZE.x + TILE_DATA_SIZE.x,
    TILE_MAP_SIZE.y,
);

pub const TILE_MAP_ADDR_9800: Addr = 0x9800;
pub const TILE_MAP_ADDR_9C00: Addr = 0x9C00;

pub const VIEWPORT_ORG: IVec2 = IVec2::ZERO;
pub const TILE_MAP_ORG: IVec2 = i2(VIEWPORT_SIZE.x, 0);
pub const TILE_DATA_ORG: IVec2 = i2(TILE_MAP_ORG.x + TILE_MAP_SIZE.x, 0);
pub const JOYPAD_ORG: IVec2 = i2(TILE_DATA_ORG.x, TILE_DATA_SIZE.y);
