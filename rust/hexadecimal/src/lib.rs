/// Convert hex value from string to Option<u32> decimal
pub fn hex_to_int(hex: &str) -> Option<u32> {
    // second attempt: fold forwards, multiply sum by 16
    // Helpful for deciphering `.map()` vs `.and_then()`:
    //    http://hoverbear.org/2014/08/12/option-monads-in-rust/
    hex.chars().fold(Some(0), |sum, c| {
        sum.map(|s| s * 16).and_then(|s| hex_digit(c).map(|h| s + h ))
    })
}

fn hex_digit(c: char) -> Option<u32> {
    match c {
        '0' ... '9' => Some(c as u32 - '0' as u32),
        'a' ... 'f' => Some(c as u32 - 'a' as u32 + 10),
        'A' ... 'F' => Some(c as u32 - 'A' as u32 + 10 ),
        _ => None,
    }
}
