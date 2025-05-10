use std::mem::transmute;

pub struct Result<T> {
    pub ans: T,
    pub h: bool,
    pub c: bool,
}

pub fn add_2_u8(a: u8, b: u8) -> Result<u8> {
    add_3_u8(a, b, 0)
}

pub fn add_3_u8(a: u8, b: u8, c: u8) -> Result<u8> {
    let a = a as i16;
    let b = b as i16;
    let c = c as i16;
    let y = a + b + c;

    Result {
        ans: (y & 0xFF) as u8,
        h: (a & 0xF) + (b & 0xF) + (c & 0xF) > 0xF,
        c: y > 0xFF,
    }
}

pub fn sub_2_u8(a: u8, b: u8) -> Result<u8> {
    sub_3_u8(a, b, 0)
}

pub fn sub_3_u8(a: u8, b: u8, c: u8) -> Result<u8> {
    let a = a as i16;
    let b = b as i16;
    let c = c as i16;
    let y = a - b - c;

    Result {
        ans: (y & 0xFF) as u8,
        h: (a & 0xF) - (b & 0xF) - (c & 0xF) < 0,
        c: y < 0,
    }
}

pub fn add_u16_i8(a: u16, b: i8) -> Result<u16> {
    let a = a as i32;
    let b = b as i32;
    let y = a + b;
    let mut h = false;
    let mut c = false;
    if b > 0 {
        h = (a & 0xFFF) + b > 0xFFF;
        c = y > 0xFFFF;
    } else if b < 0 {
        h = (a & 0xFFF) + b < 0;
        c = y < 0;
    }

    Result {
        ans: (y & 0xFFFF) as u16,
        h,
        c,
    }
}

pub fn add_sp_i8(sp: u16, b: i8) -> Result<u16> {
    let ans = add_u16_i8(sp, b).ans;

    let lo = (sp & 0xFF) as u8;
    let b = unsafe { transmute(b) };
    let Result { ans: _, h, c } = add_2_u8(lo, b);

    Result { ans, h, c }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add_3_u8() {
        let res = add_3_u8(0xFF, 0x01, 0);
        assert_eq!(res.ans, 0x00);
        assert!(res.h);
        assert!(res.c);

        let res = add_3_u8(0xFF, 0x00, 1);
        assert_eq!(res.ans, 0x00);
        assert!(res.h);
        assert!(res.c);

        let res = add_3_u8(0xFF, 0x00, 0);
        assert_eq!(res.ans, 0xFF);
        assert!(!res.h);
        assert!(!res.c);

        let res = add_3_u8(0xFF, 0xFF, 1);
        assert_eq!(res.ans, 0xFF);
        assert!(res.h);
        assert!(res.c);

        let res = add_3_u8(0x0F, 0x01, 0);
        assert_eq!(res.ans, 0x10);
        assert!(res.h);
        assert!(!res.c);
    }

    #[test]
    fn test_sub_3_u8() {
        let res = sub_3_u8(0xFF, 0x01, 0);
        assert_eq!(res.ans, 0xFE);
        assert!(!res.h);
        assert!(!res.c);

        let res = sub_3_u8(0x01, 0x00, 1);
        assert_eq!(res.ans, 0x00);
        assert!(!res.h);
        assert!(!res.c);

        let res = sub_3_u8(0x00, 0x01, 0);
        assert_eq!(res.ans, 0xFF);
        assert!(res.h);
        assert!(res.c);

        let res = sub_3_u8(0xFF, 0xFF, 1);
        assert_eq!(res.ans, 0xFF);
        assert!(res.h);
        assert!(res.c);

        let res = sub_3_u8(0x10, 0x01, 0);
        assert_eq!(res.ans, 0x0F);
        assert!(res.h);
        assert!(!res.c);
    }

    #[test]
    fn test_add_u16_i8() {
        let res = add_u16_i8(0xFFFF, 1);
        assert_eq!(res.ans, 0x0000);
        assert!(res.h);
        assert!(res.c);

        let res = add_u16_i8(0xFFFF, -1);
        assert_eq!(res.ans, 0xFFFE);
        assert!(!res.h);
        assert!(!res.c);

        let res = add_u16_i8(0, 1);
        assert_eq!(res.ans, 0x0001);
        assert!(!res.h);
        assert!(!res.c);

        let res = add_u16_i8(0, -1);
        assert_eq!(res.ans, 0xFFFF);
        assert!(res.h);
        assert!(res.c);

        let res = add_u16_i8(0x0FFF, 1);
        assert_eq!(res.ans, 0x1000);
        assert!(res.h);
        assert!(!res.c);

        let res = add_u16_i8(0x1000, 1);
        assert_eq!(res.ans, 0x1001);
        assert!(!res.h);
        assert!(!res.c);

        let res = add_u16_i8(0x1000, -1);
        assert_eq!(res.ans, 0x0FFF);
        assert!(res.h);
        assert!(!res.c);

        let res = add_u16_i8(0x0FFF, -1);
        assert_eq!(res.ans, 0x0FFE);
        assert!(!res.h);
        assert!(!res.c);
    }
}
