/// Write a program that given a phrase can count the occurrences of each word in that phrase.
///
/// Caveats from tests:
/// 1. numbers count as words
/// 2. downcase all words
/// 3. Vector output should be in same order as string (by first occurance of word)
/// 4. Non-word/number chars as separator.

use std::collections::HashMap;

pub fn word_count(s: &str) -> HashMap<String, u32> {
    let mut results : HashMap<String, u32> = HashMap::new();

    results
}
