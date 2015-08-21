/// calculates the Hamming difference between two DNA strands
///
/// Hamming distance between two strings is the count of single point mutations
/// between the two strings, where one character has been replaced with another.
///
/// The Hamming distance is only defined for sequences of equal length. Hence you
/// may assume that only sequences of equal length will be passed to your hamming
/// distance function.
///
/// Example:
///    GAGCCTACTAACGGGAT
///    CATCGTAATGACGGCCT
///    ^ ^ ^  ^ ^    ^^
///
///    The Hamming distance between these two DNA strands is 7.
///
///    assert_eq!(hamming_distance("GAGCCTACTAACGGGAT", "CATCGTAATGACGGCCT"), 7);
pub fn hamming_distance(s1 : &str, s2: &str) -> u32
{
    let mut difference = 0;
    println!("s1: {},\ns2: {}", s1, s2);
    let s1_chars = s1.chars();
    let mut s2_chars = s2.chars();

    for s1_char in s1_chars {
        let s2_char = s2_chars.next().expect("second string should have as many chars as first");
        if s1_char != s2_char {
            difference += 1
        }
        println!("s1_char: {:?}, s2_char: {:?}, diff:{} ", s1_char, s2_char, difference)
    }
    difference
}

#[test]
fn test_readme_example() {
    assert_eq!(hamming_distance("GAGCCTACTAACGGGAT", "CATCGTAATGACGGCCT"), 7);
}
