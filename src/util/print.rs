pub fn slice_to_hex_string(slice: &[u8]) -> String {
    let mut s = String::new();
    s.push_str("[");

    for (idx, x) in slice.iter().enumerate() {
        if idx != 0 {
            s.push_str(", ");
        }
        s.push_str(&format!("{:#02X}", x));
    }
    s.push_str("]");

    s
}
