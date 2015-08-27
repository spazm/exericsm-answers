/// Raindrops
///
/// Write a program that converts a number to a string, the contents of which depends on the
/// number's prime factors.
///
/// - If the number contains 3 as a prime factor, output 'Pling'.
/// - If the number contains 5 as a prime factor, output 'Plang'.
/// - If the number contains 7 as a prime factor, output 'Plong'.
/// - If the number does not contain 3, 5, or 7 as a prime factor,
///   just pass the number's digits straight through.
///
/// # Examples
///
/// - 28's prime-factorization is 2, 2, 7.
///   - In raindrop-speak, this would be a simple "Plong".
///
/// ```
/// assert_eq!(raindrops::raindrops(28), "Plong");
/// ```
///
///   - 1755 prime-factorization is 3, 3, 3, 5, 13.
///     - In raindrop-speak, this would be a "PlingPlang".
///
/// ```
/// assert_eq!(raindrops::raindrops(1755), "PlingPlang");
/// ```
///
///   - The prime factors of 34 are 2 and 17.
///     - Raindrop-speak doesn't know what to make of that,
///           so it just goes with the straightforward "34".
///
/// ```
/// assert_eq!(raindrops::raindrops(34), "34");
/// ```
///
pub fn raindrops(n: u32) -> String {
    assert!(n > 0, "n is positive");

    let mut rain = "".to_string();

    if n % 3 == 0 { rain.push_str("Pling") }
    if n % 5 == 0 { rain.push_str("Plang") }
    if n % 7 == 0 { rain.push_str("Plong") }

    if rain.is_empty() {
        rain = format!("{}", n)
    }
    rain
}
