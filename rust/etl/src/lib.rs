use std::collections::BTreeMap;

/// Transform old data keyed by score into new data keyed by word.
///
/// old:
///   ((score, list_of_words), ... )
/// new:
///   ((word, score), ... )  // word is downcased

pub fn transform(input: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
    let mut output = BTreeMap::new();

    for (count, words) in input.iter() {
        for word in words.clone() {
            let word = to_lowercase(&word);
            output.insert(word, *count);
        }
    }

    output
}

fn to_lowercase(s: &str) -> String {
    s.chars()
        .map(|c| c.to_lowercase()
             .next()
             .expect("should be char"))
        .collect::<String>()
}
