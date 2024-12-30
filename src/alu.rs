use std::mem::transmute;

pub struct Data8(u8);

pub fn add_u8(a: Data8, b: Data8) -> Data8 {
    let c = u8::wrapping_add(a.0, b.0);
    return Data8(c);
}

pub fn add_i8(a: Data8, b: Data8) -> Data8 {
    unsafe {
        let a = transmute(a.0);
        let b = transmute(b.0);
        let c = i8::wrapping_add(a, b);
        let c = transmute(c);
        return Data8(c);
    }
}
