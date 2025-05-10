pub fn copy_from_safe<T: Clone>(dst: &mut [T], src: &[T]) {
    let len = usize::min(dst.len(), src.len());
    dst[..len].clone_from_slice(&src[..len])
}
