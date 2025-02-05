use xf::num::ivec2::{i2, IVec2};

use crate::{consts::P8, mem::sections::Addr};

pub const TILE_SIZE: IVec2 = P8;
pub const VIEWPORT_P8_SIZE: IVec2 = i2(20, 18);

pub const TILE_MAP_P8_SIZE: IVec2 = i2(32, 32);
pub const TILE_MAP_SIZE: IVec2 = IVec2::mul(TILE_MAP_P8_SIZE, P8);
pub const TILE_DATA_P8_SIZE: IVec2 = i2(16, 24);
pub const TILE_DATA_SIZE: IVec2 = IVec2::mul(TILE_DATA_P8_SIZE, P8);

pub const RELEASE_SCREEN_SIZE: IVec2 = VIEWPORT_P8_SIZE;
pub const DEBUG_SCREEN_P8_SIZE: IVec2 =
    i2(TILE_MAP_P8_SIZE.x + TILE_DATA_P8_SIZE.x, TILE_MAP_P8_SIZE.y);
pub const TILE_MAP_ADDR_9800: Addr = 0x9800;
pub const TILE_MAP_ADDR_9C00: Addr = 0x9C00;
