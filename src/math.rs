pub fn bit8(op: &u8, idx: usize) -> u8 {
    (op >> idx) & 0b1
}

pub fn bits8(op: &u8, hi: usize, lo: usize) -> u8 {
    let mask = 0xFF;
    //println!("mask = {:#08b}", mask); //
    let mask = mask << (8 - (hi + 1));
    //println!("mask = {:#08b}", mask); //
    let mask = mask >> (8 - (hi + 1 - lo));
    //println!("mask = {:#08b}", mask); //

    return (op >> lo) & mask;
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
        assert_eq!(bit8(&a0, 3), 0b1);
        assert_eq!(bit8(&a0, 4), 0b0);
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
}
