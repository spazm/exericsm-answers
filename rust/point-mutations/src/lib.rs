//! calculates the Hamming difference between two DNA strands
//!
//! Hamming distance between two strings is the count of single point mutations
//! between the two strings, where one character has been replaced with another.


/// calculates the hamming distance between two strings.  This is the number
/// of single point-mutations required to turn string `s1` into string `s2`.
///
/// The Hamming distance is only defined for sequences of equal length. Hence you
/// may assume that only sequences of equal length will be passed to your hamming
/// distance function.
///
/// # Panics:
/// Both strings must be the same length.
///
/// ```should_panic(expected = "assertion failed")
/// assert_eq!(point_mutations::hamming_distance("", "A"), 1);
/// ```
///
/// # Examples:
/// ```text
///    GAGCCTACTAACGGGAT
///    CATCGTAATGACGGCCT
///    ^ ^ ^  ^ ^    ^^
///
///    The Hamming distance between these two DNA strands is 7.
/// ```
///
/// ```
/// assert_eq!(point_mutations::hamming_distance("GAGCCTACTAACGGGAT", "CATCGTAATGACGGCCT"), 7);
/// ```
pub fn hamming_distance(s1 : &str, s2: &str) -> u32
{
    // println!("s1: {},\ns2: {}", s1, s2);
    assert!(s1.len() == s2.len());

    let mut distance = 0;
    let s1_chars = s1.chars();
    let mut s2_chars = s2.chars();  // must be mut to exercise iterator via next()

    for s1_char in s1_chars {
        let s2_char = s2_chars.next()
            .expect("s2 too short");
        if s1_char != s2_char {
            distance += 1
        }
        // println!("s1_char: {:?}, s2_char: {:?}, diff:{} ", s1_char, s2_char, distance)
    }
    distance
}
