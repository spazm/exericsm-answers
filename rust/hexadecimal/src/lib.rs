/// Convert hex value from string to Option<u32> decimal
pub fn hex_to_int(hex: &str) -> Option<u32> {
    // first attempt: manual loop backwards through chars
    let mut value = 0;
    let mut multiple = 1;
    for c in hex.chars().rev() {
        match hex_digit(c) {
            None => return None,
            Some(d) => value += d * multiple,
        }
        multiple *= 16;
    }
    Some(value)
}

fn hex_digit(c: char) -> Option<u32> {
    match c {
        _ if c >= '0' && c <= '9'
            => Some(c as u32 - '0' as u32),
        'a' | 'A' => Some(10),
        'b' | 'B' => Some(11),
        'c' | 'C' => Some(12),
        'd' | 'D' => Some(13),
        'e' | 'E' => Some(14),
        'f' | 'F' => Some(15),
        _ => None,
    }
}
