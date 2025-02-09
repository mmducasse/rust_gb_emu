use macroquad::color::{Color, BLACK, DARKGRAY, LIGHTGRAY, WHITE};
use xf::{
    mq::draw::draw_rect,
    num::{
        irect::ir,
        ivec2::{i2, IVec2},
    },
};

use crate::{mem::io_regs::IoReg, sys::Sys};

pub struct Palette {
    pub id0: u8,
    pub id1: u8,
    pub id2: u8,
    pub id3: u8,
}

impl Palette {
    pub fn from_reg(sys: &Sys, reg: IoReg) -> Self {
        let data = sys.mem.io_regs.get(reg);
        return Self::new(data);
    }

    pub fn new(data: u8) -> Self {
        Self {
            id0: (data >> 0) & 0b11,
            id1: (data >> 2) & 0b11,
            id2: (data >> 4) & 0b11,
            id3: (data >> 6) & 0b11,
        }
    }

    pub fn map(&self, color_id: u8) -> u8 {
        match color_id {
            0b00 => self.id0,
            0b01 => self.id1,
            0b10 => self.id2,
            0b11 => self.id3,
            _ => unreachable!(),
        }
    }
}

pub fn draw_pixel<const TRANSPARENT: bool>(pos: IVec2, palette: &Palette, color_id: u8) {
    if TRANSPARENT && (color_id == 0) {
        return;
    }
    let color = get_color(palette.map(color_id));
    draw_rect(ir(pos, i2(1, 1)), color);
}

#[inline]
fn get_color(color_value: u8) -> Color {
    return match color_value {
        0b00 => WHITE,
        0b01 => LIGHTGRAY,
        0b10 => DARKGRAY,
        0b11 => BLACK,
        _ => unreachable!(),
    };
}
