pub fn copy_from_safe<T: Clone>(dst: &mut [T], src: &[T]) {
    let len = usize::min(dst.len(), src.len());

    for i in 0..len {
        dst[i] = src[i].clone();
    }
}
