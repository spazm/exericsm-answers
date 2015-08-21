/// Write a program that given a phrase can count the occurrences of each word in that phrase.
///
/// Caveats from tests:
/// 1. numbers count as words
/// 2. downcase all words
/// 3. Vector output should be in same order as string (by first occurance of word)
/// 4. Non-word/number chars as separator.

use std::collections::HashMap;

pub fn word_count(s: &str) -> HashMap<String, u32> {
    let mut word_count : HashMap<String, u32> = HashMap::new();

//    break string of input into words on non-alphanumeric
    for word in s.split(|c: char| !c.is_alphanumeric()) {
        // if word is empty, continue
        if word.len() == 0 { continue }
        // will need to downcase word
        println!("word: {:?}", word);

        //entry() returns a mutable pointer to the hash value for a given key
        let counter = word_count.entry(String::from(word)).or_insert(0);
        *counter += 1;
    }
    word_count
}
