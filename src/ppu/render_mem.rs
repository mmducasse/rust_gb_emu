use xf::num::{
    irect::rect,
    ivec2::{i2, IVec2},
};

use crate::{
    consts::P8,
    mem::{io_regs::IoReg, sections::MemSection, Addr},
    sys::Sys,
    util::math::bit8,
};

use super::{
    consts::*,
    lcdc::LcdcState,
    palette::Palette,
    render_util::{draw_pixel, get_tile_map_addr, tile_data_idx_to_addr},
};

/// Renders one of the tile data blocks to the screen.
#[inline]
pub fn render_tile_data_block(sys: &Sys, block_addr: Addr, org: IVec2) {
    let mut i = 0;
    let range = block_addr..(block_addr + TILE_DATA_BLOCK_SIZE);
    for addr in range.clone().step_by(16) {
        let x = i % TILE_DATA_P8_SIZE.x;
        let y = i / TILE_DATA_P8_SIZE.x;

        let rel_addr = (addr - TILE_DATA_ADDR_8000) as usize;
        let tile_size = TILE_DATA_TILE_SIZE as usize;
        let bytes = &sys.mem.vram.as_slice()[rel_addr..(rel_addr + tile_size)];

        i += 1;

        draw_tile(bytes, org + i2(x * 8, y * 8));
    }
}

/// Renders the entire background tilemap to the screen.
#[inline]
pub fn render_bg_tile_map(sys: &Sys, org: IVec2) {
    let lcdc = LcdcState::from(sys);

    let tile_map_start_addr = get_tile_map_addr(lcdc.bg_tile_map_area_is_9c00);

    for i in 0..TILE_MAP_P8_SIZE.product() {
        let x = i % TILE_MAP_P8_SIZE.x;
        let y = i / TILE_MAP_P8_SIZE.x;
        let addr = (i as u16) + tile_map_start_addr;

        draw_tile_from_map(sys, i2(x, y), addr, org);
    }
}

#[inline]
fn draw_tile_from_map(sys: &Sys, pos: IVec2, map_addr: Addr, org: IVec2) {
    let lcdc = sys.mem.io_regs.get(IoReg::Lcdc);
    let is_mode_8000 = bit8(&lcdc, 4) == 1;
    let data_idx = sys.mem.read(map_addr);

    let data_addr = tile_data_idx_to_addr(data_idx as u16, is_mode_8000);

    let addr = (data_addr - MemSection::Vram.start_addr()) as usize;
    let bytes = &sys.mem.vram.as_slice()[addr..(addr + 16)];

    let org = pos * P8 + org;
    draw_tile(bytes, org);
}

#[inline]
fn draw_tile(bytes: &[u8], org: IVec2) {
    let palette = Palette::default();

    for pos in rect(0, 0, 8, 8).iter() {
        let idx = (pos.y * 2) as usize;
        let bit = 7 - pos.x;
        let lower = bit8(&bytes[idx], bit as u8);
        let upper = bit8(&bytes[idx + 1], bit as u8);

        let color_id = (upper << 1) | lower;

        draw_pixel::<false>(pos + org, &palette, color_id);
    }
}
