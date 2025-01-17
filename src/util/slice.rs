pub fn sum_slice(slice: &[u8]) -> u64 {
    let mut sum: u64 = 0;
    for i in slice {
        sum += *i as u64;
    }
    return sum;
}

pub fn copy_from_safe<T: Clone>(dst: &mut [T], src: &[T]) {
    let len = usize::min(dst.len(), src.len());

    for i in 0..len {
        dst[i] = src[i].clone();
    }
}
