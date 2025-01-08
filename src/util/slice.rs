pub fn sum_slice(slice: &[u8]) -> u64 {
    let mut sum: u64 = 0;
    for i in slice {
        sum += *i as u64;
    }
    return sum;
}
