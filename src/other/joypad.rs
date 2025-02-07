use macroquad::{
    color::{BLACK, RED, WHITE},
    input::{is_key_down, KeyCode},
};
use xf::{
    mq::draw::draw_rect,
    num::{
        irect::ir,
        ivec2::{i2, IVec2},
    },
};

use crate::{
    consts::P8,
    mem::io_regs::IoReg,
    ppu::consts::{TILE_DATA_P8_SIZE, TILE_DATA_SIZE, TILE_MAP_SIZE},
    sys::Sys,
    util::{
        draw::draw_empty_rect,
        math::{bit8, set_bit8, set_bits8},
    },
};

#[derive(Clone, Copy)]
enum Button {
    Up,
    Right,
    Down,
    Left,

    A,
    B,

    Start,
    Select,
}

impl Button {
    pub fn key_code(self) -> KeyCode {
        match self {
            Button::Up => KeyCode::Up,
            Button::Right => KeyCode::Right,
            Button::Down => KeyCode::Down,
            Button::Left => KeyCode::Left,

            Button::A => KeyCode::Z,
            Button::B => KeyCode::X,

            Button::Start => KeyCode::Enter,
            Button::Select => KeyCode::RightShift,
        }
    }
}

pub fn draw_joypad_state(org: IVec2) {
    draw_button(Button::Up, i2(2, 1), org);
    draw_button(Button::Right, i2(3, 2), org);
    draw_button(Button::Down, i2(2, 3), org);
    draw_button(Button::Left, i2(1, 2), org);

    draw_button(Button::A, i2(9, 3), org);
    draw_button(Button::B, i2(10, 2), org);

    draw_button(Button::Start, i2(5, 4), org);
    draw_button(Button::Select, i2(7, 4), org);
}

fn draw_button(button: Button, pos: IVec2, org: IVec2) {
    let bounds = ir(org + (pos * P8), P8);
    if is_key_down(button.key_code()) {
        draw_rect(bounds, RED);
    } else {
        draw_rect(bounds, BLACK);
    }
    draw_empty_rect(bounds, WHITE);
}

static mut PREV_LO_4: u8 = 0xF;

pub fn handle_joypad_inputs(sys: &mut Sys) {
    let p1 = sys.mem.io_regs.get(IoReg::P1);
    let select_btns = bit8(&p1, 5) == 0;
    let select_dpad = bit8(&p1, 4) == 0;

    let mut lo_4 = 0xF;
    if select_btns {
        read_button(&mut lo_4, 0, Button::A);
        read_button(&mut lo_4, 1, Button::B);
        read_button(&mut lo_4, 2, Button::Select);
        read_button(&mut lo_4, 3, Button::Start);
    }

    if select_dpad {
        read_button(&mut lo_4, 0, Button::Right);
        read_button(&mut lo_4, 1, Button::Left);
        read_button(&mut lo_4, 2, Button::Up);
        read_button(&mut lo_4, 3, Button::Down);
    }

    sys.mem.io_regs.mut_(IoReg::P1, |p1| {
        set_bits8(p1, 3, 0, lo_4);
        unsafe {
            if PREV_LO_4 != lo_4 {
                println!("btns: {:0>8b}", p1);
                PREV_LO_4 = lo_4;
            }
        }
    });
}

fn read_button(p1: &mut u8, idx: u8, button: Button) {
    let value = if is_key_down(button.key_code()) { 0 } else { 1 };
    let mut mask = 0xFF;
    set_bit8(&mut mask, 0, value);
    mask = u8::rotate_left(mask, idx as u32);
    *p1 &= mask;
}
