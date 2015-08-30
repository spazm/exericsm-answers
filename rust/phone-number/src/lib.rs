/// return US phone number as string of 10 digits,
/// as parsed from input string in 10 digit or 11 digit
/// with leading 1.
pub fn number(n: &str) -> Option<String> {
    println!("n: {}", n);
    let n = n.chars().filter(|&c| c.is_digit(10)).collect::<String>();
    println!("n: {}", n);
    match n.len() {
        10 => Some(n),
        11 if n.starts_with("1") => Some(n[1..].to_string()),
        _  => None,
    }
}

/// The area code of a 10 digit US phone number is the first three digits
pub fn area_code(n: &str) -> Option<String> {
    match number(n) {
        None => None,
        Some(n) => Some(n[0..3].to_string()),
    }
}

/// pretty print a US phone number in "(XXX) XXX-XXXX" format
pub fn pretty_print(n: &str) -> String {
    match number(n) {
        Some(n) => format!("({}) {}-{}", &n[0..3], &n[3..6], &n[6..10]),
        None => "invalid".to_string(),
    }
}
