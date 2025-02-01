pub fn bit8(op: &u8, idx: u8) -> u8 {
    (op >> idx) & 0b1
}

pub fn bits8(op: &u8, hi: usize, lo: usize) -> u8 {
    let mask = 0xFF;
    let mask = mask << (8 - (hi + 1));
    let mask = mask >> (8 - (hi + 1 - lo));

    return (op >> lo) & mask;
}

pub fn set_bit8(data: &mut u8, idx: u8, value: u8) {
    let mask = 0x1 << idx;

    if (value & 0x1) > 0 {
        *data |= mask;
    } else {
        *data &= !mask;
    }
}

pub fn set_bits8(data: &mut u8, hi: u8, lo: u8, value: u8) {
    let shift_r = 7 - (hi - lo);
    let shift_l = lo;
    let mask = (0xFF >> shift_r) << shift_l;
    let value = value << lo;

    set_bits8_masked(data, mask, value);
}

#[inline]
pub fn set_bits8_masked(data: &mut u8, mask: u8, value: u8) {
    *data = (*data & !mask) | (value & mask);
}

pub fn bit16(op: &u16, idx: usize) -> u16 {
    (op >> idx) & 0b1
}

pub fn bits16(op: &u16, hi: usize, lo: usize) -> u16 {
    let mask = 0xFF;
    let mask = mask << (16 - (hi + 1));
    let mask = mask >> (16 - (hi + 1 - lo));

    return (op >> lo) & mask;
}

pub fn split_16(data: u16) -> (u8, u8) {
    let hi = ((data & 0xFF00) >> 8) as u8;
    let lo = (data & 0x00FF) as u8;

    return (hi, lo);
}

pub fn join_16(hi: u8, lo: u8) -> u16 {
    let hi = (hi as u16) << 8;
    let lo = lo as u16;

    return hi | lo;
}

pub fn add8_ui(a: u8, b: i8) -> u8 {
    if b >= 0 {
        let b = b as u8;
        u8::wrapping_add(a, b)
    } else {
        let b = (-b) as u8;
        u8::wrapping_sub(a, b)
    }
}

pub fn add8_uu(a: u8, b: u8) -> u8 {
    u8::wrapping_add(a, b)
}

pub fn add16_ui(a: u16, b: i16) -> u16 {
    if b >= 0 {
        let b = b as u16;
        u16::wrapping_add(a, b)
    } else {
        let b = (-b) as u16;
        u16::wrapping_sub(a, b)
    }
}

pub fn add16_uu(a: u16, b: u16) -> u16 {
    u16::wrapping_add(a, b)
}

pub fn add_u16_i8(u: u16, i: i8) -> u16 {
    if i >= 0 {
        return u16::wrapping_add(u, i as u16);
    } else {
        return u16::wrapping_sub(u, (-i) as u16);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_bit8() {
        let a0 = 0b0000_1110;
        assert_eq!(bit8(&a0, 0), 0b0);
        assert_eq!(bit8(&a0, 1), 0b1);
        assert_eq!(bit8(&a0, 3), 0b1);
        assert_eq!(bit8(&a0, 4), 0b0);
        assert_eq!(bit8(&a0, 7), 0b0);
    }

    #[test]
    fn test_bits8() {
        let x = 0b0000_1110;
        assert_eq!(bits8(&x, 3, 1), 0b111);
        assert_eq!(bits8(&x, 4, 2), 0b011);

        let x = 0b0100_0001;
        assert_eq!(bits8(&x, 7, 0), 0b0100_0001);
        assert_eq!(bits8(&x, 3, 0), 0b0001);
        assert_eq!(bits8(&x, 7, 4), 0b0100);
    }

    #[test]
    fn test_set_bit8() {
        let mut x = 0b0000_0000;
        set_bit8(&mut x, 5, 1);
        assert_eq!(x, 0b0010_0000);

        let mut x = 0b1111_1111;
        set_bit8(&mut x, 5, 0);
        assert_eq!(x, 0b1101_1111);
    }

    #[test]
    fn test_set_bits8() {
        let mut x = 0b0000_0000;
        set_bits8(&mut x, 5, 2, 0b1111);
        assert_eq!(x, 0b0011_1100);

        let mut x = 0b1011_0110;
        set_bits8(&mut x, 6, 3, 0b1001);
        assert_eq!(x, 0b1100_1110);
    }

    #[test]
    fn test_set_bits8_masked() {
        let mut x = 0b0000_0000;
        set_bits8_masked(&mut x, 0b1010_1010, 0b1111_1111);
        assert_eq!(x, 0b1010_1010);

        let mut x = 0b1010_1010;
        set_bits8_masked(&mut x, 0b0000_1111, 0b0000_0000);
        assert_eq!(x, 0b1010_0000);
    }

    #[test]
    fn test_join16_split16() {
        let x = join_16(0xFF, 0x77);
        assert_eq!(x, 0xFF77);

        let (hi, lo) = split_16(x);
        assert_eq!(hi, 0xFF);
        assert_eq!(lo, 0x77);

        let x = join_16(0x12, 0xAD);
        assert_eq!(x, 0x12AD);

        let (hi, lo) = split_16(x);
        assert_eq!(hi, 0x12);
        assert_eq!(lo, 0xAD);
    }

    #[test]
    fn test_add16_ui() {
        let y = add16_ui(0xFFFF, 0);
        assert_eq!(y, 0xFFFF);

        let y = add16_ui(0xFFFF, 1);
        assert_eq!(y, 0);

        let y = add16_ui(0, -1);
        assert_eq!(y, 0xFFFF);
    }
}
