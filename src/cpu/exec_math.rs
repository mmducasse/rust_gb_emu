pub struct Result<T> {
    pub ans: T,
    pub h: bool,
    pub c: bool,
}

pub fn add_3_u8(a: u8, b: u8, c: u8) -> Result<u8> {
    let a = a as i16;
    let b = b as i16;
    let c = c as i16;
    let y = a + b + c;

    return Result {
        ans: (y & 0xFF) as u8,
        h: (a & 0xF) + (b & 0xF) + (c & 0xF) > 0xF,
        c: y > 0xFF,
    };
}

pub fn sub_3_u8(a: u8, b: u8, c: u8) -> Result<u8> {
    let a = a as i16;
    let b = b as i16;
    let c = c as i16;
    let y = a - b - c;

    return Result {
        ans: (y & 0xFF) as u8,
        h: (a & 0xF) - (b & 0xF) - (c & 0xF) < 0,
        c: y < 0,
    };
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add_3_u8() {
        let res = add_3_u8(0xFF, 0x01, 0);
        assert_eq!(res.ans, 0x00);
        assert_eq!(res.h, true);
        assert_eq!(res.c, true);

        let res = add_3_u8(0xFF, 0x00, 1);
        assert_eq!(res.ans, 0x00);
        assert_eq!(res.h, true);
        assert_eq!(res.c, true);

        let res = add_3_u8(0xFF, 0x00, 0);
        assert_eq!(res.ans, 0xFF);
        assert_eq!(res.h, false);
        assert_eq!(res.c, false);

        let res = add_3_u8(0xFF, 0xFF, 1);
        assert_eq!(res.ans, 0xFF);
        assert_eq!(res.h, true);
        assert_eq!(res.c, true);

        let res = add_3_u8(0x0F, 0x01, 0);
        assert_eq!(res.ans, 0x10);
        assert_eq!(res.h, true);
        assert_eq!(res.c, false);
    }

    #[test]
    fn test_sub_3_u8() {
        let res = sub_3_u8(0xFF, 0x01, 0);
        assert_eq!(res.ans, 0xFE);
        assert_eq!(res.h, false);
        assert_eq!(res.c, false);

        let res = sub_3_u8(0x01, 0x00, 1);
        assert_eq!(res.ans, 0x00);
        assert_eq!(res.h, false);
        assert_eq!(res.c, false);

        let res = sub_3_u8(0x00, 0x01, 0);
        assert_eq!(res.ans, 0xFF);
        assert_eq!(res.h, true);
        assert_eq!(res.c, true);

        let res = sub_3_u8(0xFF, 0xFF, 1);
        assert_eq!(res.ans, 0xFF);
        assert_eq!(res.h, true);
        assert_eq!(res.c, true);

        let res = sub_3_u8(0x10, 0x01, 0);
        assert_eq!(res.ans, 0x0F);
        assert_eq!(res.h, true);
        assert_eq!(res.c, false);
    }
}
