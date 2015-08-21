/// Write a program that given a phrase can count the occurrences of each word in that phrase.
///
/// Caveats from tests:
/// 1. numbers count as words
/// 2. downcase all words
/// 3. Vector output should be in same order as string (by first occurance of word)
///    -- this seems to be handled by HashMap automatically.
/// 4. Non-word/number chars as separator.

use std::collections::HashMap;

pub fn word_count(s: &str) -> HashMap<String, u32> {
    let mut word_count : HashMap<String, u32> = HashMap::new();

    // break string of input into words on non-alphanumeric
    for word in s.split(|c: char| !c.is_alphanumeric()) {
        // skip empty words returned by split
        if word.len() == 0 { continue }

        let word = to_lowercase(word);  // now a String
        // println!("word: {:?}", word);

        // entry() returns a mutable pointer to the hash value for a given key
        let counter = word_count.entry(word).or_insert(0);
        *counter += 1;
    }
    word_count
}

fn to_lowercase(s: &str) -> String {
    // to_lowercase is unstable on Strings/collections, so manually
    // unpack the string to chars and lowercase each char

    // to_lowercase returns an iterator of valid lowercase characters,
    // we choose the first one with next().  We are concerned only with
    // a consistent mapping, not necessarily a fully correct one.
    // initially copied from jhod0 on exercism.io
    // http://exercism.io/submissions/dcf0f37018d7470db5a5aab54ed04563

    // map string to a collection of lowercase chars
    s.chars()
        .map(|c| c.to_lowercase()
             .next()
             .expect("should be char"))
        .collect::<String>()
}
