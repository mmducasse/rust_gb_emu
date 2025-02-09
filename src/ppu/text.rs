use xf::{
    mq::{draw::draw_texture, texture::Texture, textures::Textures},
    num::{
        irect::ir,
        ivec2::{i2, IVec2},
    },
};

use crate::{consts::P8, sys::Sys};

const CHAR_SIZE: IVec2 = P8;

static mut TEXTURES: Textures<()> = Textures::new();

pub fn draw_text(s: impl Into<String>, org: IVec2) {
    let s: String = s.into();
    let s = s.trim().to_owned().clone();
    let mut dst_pt = org;
    for c in s.chars() {
        let src_pos = lookup(c);
        let src = ir(src_pos, CHAR_SIZE);

        draw_texture(get_font_texture(), Some(src), dst_pt);

        dst_pt.x += CHAR_SIZE.x;
    }
}

fn get_font_texture() -> Texture {
    unsafe {
        const IMAGE_BYTES: &[u8] = include_bytes!("../../assets/sprites/font_nes.png");
        return TEXTURES.get_or_load((), |id| IMAGE_BYTES);
    }
}

fn lookup(c: char) -> IVec2 {
    let alpha = |x: i32| i2(x % 8, x / 8);

    let char_loc = match c {
        _ if c.is_uppercase() => i2(8, 0) + alpha(c as i32 - 'A' as i32),
        _ if c.is_lowercase() => i2(8, 4) + alpha(c as i32 - 'a' as i32),
        _ if c.is_numeric() => i2(8, 8) + alpha(c as i32 - '0' as i32),

        _ => i2(1, 11),
    };

    char_loc * CHAR_SIZE
}
