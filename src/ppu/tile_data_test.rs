use macroquad::color::{Color, BLACK, BLUE, DARKGRAY, LIGHTGRAY, RED, WHITE, YELLOW};
use xf::{
    mq::draw::draw_rect,
    num::{
        irect::{ir, rect},
        ivec2::{i2, IVec2},
    },
};

use crate::{consts::P8, sys::Sys, util::math::bit8};

pub const SCREEN_P8_SIZE: IVec2 = i2(16, 24);
pub const SCREEN_SIZE: IVec2 = IVec2::mul(SCREEN_P8_SIZE, P8);

pub fn draw_vram_tile_data(sys: &Sys, org: IVec2) {
    // let max = SCREEN_SIZE.x;
    // for i in 0..max {
    //     draw_pixel(i2(i, i), 0b01);
    // }

    let mut i = 0;
    let range = 0x8000..0x9800;
    for addr in range.clone().step_by(16) {
        let x = i % SCREEN_P8_SIZE.x;
        let y = i / SCREEN_P8_SIZE.x;

        let rel_addr = addr - range.start;
        let bytes = &sys.mem.vram.as_slice()[rel_addr..(rel_addr + 16)];

        // if sum_slice(bytes) == 0 {
        //     continue;
        // }
        i += 1;

        let block_color = if y < 8 {
            RED
        } else if y < 16 {
            BLUE
        } else {
            YELLOW
        };
        draw_rect(rect(org.x + x * 8, org.y + y * 8, 8, 8), block_color);

        draw_tile(bytes, org + i2(x * 8, y * 8));
    }

    // let offset = 0x9000; // MemSection::Vram.start_addr();
    // for idx in 0..MemSection::Vram.size() {
    //     let data = sys.rd_mem(offset + idx);
    //     println!("[{:0>4X}]: {:0>2X}", idx, data);
    // }
}

fn draw_tile(bytes: &[u8], org: IVec2) {
    // print!("[{}]: ", idx);
    // for byte in bytes {
    //     print!("  {:0>2x}", byte);
    // }
    // println!();

    for pos in rect(0, 0, 8, 8).iter() {
        let idx = (pos.y * 2) as usize;
        let lower = bit8(&bytes[idx], pos.x as u8);
        let upper = bit8(&bytes[idx + 1], pos.x as u8);

        let color_id = (upper << 1) | lower;
        // println!("color: {}", color_id);

        if color_id != 0 {
            draw_pixel(pos + org, color_id);
        }
    }
}

fn draw_pixel(pos: IVec2, color_id: u8) {
    draw_rect(ir(pos, i2(1, 1)), get_color(color_id));
}

#[inline]
fn get_color(color_id: u8) -> Color {
    return match color_id {
        0b11 => WHITE,
        0b10 => LIGHTGRAY,
        0b01 => DARKGRAY,
        0b00 => BLACK,
        _ => unreachable!(),
    };
}
