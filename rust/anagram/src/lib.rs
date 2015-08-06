pub fn anagrams_for(s: &str, anagrams: &[&str]) -> Vec<String> {
    let sorted_string = lc_sorted_str(s); // : String
    let lowercase_s = to_lowercase(s); // : String
    let mut answers = Vec::new();
    for anagram in anagrams {
        let sorted_anagram = lc_sorted_str(&anagram);
        if sorted_string == sorted_anagram {
            if lowercase_s != to_lowercase(&anagram) {
                answers.push(anagram.to_string()) }}
    }
    answers
}

fn lc_sorted_str(s: &str) -> String {
    // lower case the string and then sort.

    // to_lowercase is unstable on Strings/collections, so manually
    // unpack the string to chars and lowercase each char
    //   str -> chars -> lowercase (collapse iterator of Some)
    //       -> into_iter -> collect to Vec<char>.
    let mut chars: Vec<char> = s.chars().map(|c| c.to_lowercase()
                                             .next()
                                             .expect("should be char")).into_iter().collect();
    chars.sort();  // sort works in-place on mutable vec
    let lc_sorted_string: String = chars.into_iter().collect();
    lc_sorted_string
}

fn to_lowercase(instr: &str) -> String {
    // copied from jhod0 on exercism.io
    // http://exercism.io/submissions/dcf0f37018d7470db5a5aab54ed04563
    // to_lowercase returns an iterator of valid lowercase characters,
    // we choose the first one with next()
    instr.chars().map(|c| c.to_lowercase()
                      .next()
                      .expect("Should be char"))
        .collect::<String>()
}
