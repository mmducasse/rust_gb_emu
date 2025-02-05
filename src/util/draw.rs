use macroquad::color::Color;
use xf::{
    mq::draw::draw_rect,
    num::{
        irect::{ir, IRect},
        ivec2::i2,
    },
};

pub fn draw_empty_rect(rect: IRect, color: Color) {
    draw_rect(ir(rect.pos, i2(rect.w(), 1)), color);
    draw_rect(ir(rect.pos, i2(1, rect.h())), color);
    draw_rect(ir(rect.pos + i2(0, rect.h()), i2(rect.w(), 1)), color);
    draw_rect(ir(rect.pos + i2(rect.w(), 0), i2(1, rect.h())), color);
}
