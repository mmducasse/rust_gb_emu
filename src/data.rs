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

pub fn get_bit_u8(data: u8, idx: usize) -> u8 {
    return (data >> idx) & 0x1;
}

pub fn set_bit_u8(data: &mut u8, idx: usize, value: u8) {
    let mask = 0x1 << idx;

    if (value & 0x1) > 0 {
        *data |= mask;
    } else {
        *data &= !mask;
    }
}
