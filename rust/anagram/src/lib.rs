pub fn anagrams_for(s: &str, anagrams: &[&str]) -> Vec<String> {
    let sorted_string = to_lowercase(s, true); // : String
    let lowercase_s = to_lowercase(s, false);  // : String
    let mut answers = Vec::new();
    for anagram in anagrams {
        let sorted_anagram = to_lowercase(&anagram, true);
        if sorted_string == sorted_anagram {
            if lowercase_s != to_lowercase(&anagram, false) {
                answers.push(anagram.to_string()) }}
    }
    answers
}

fn to_lowercase(s: &str, sort: bool) -> String {
    //! lower case and optionally sort the string
    // to_lowercase is unstable on Strings/collections, so manually
    // unpack the string to chars and lowercase each char
    //   str -> chars -> lowercase (collapse iterator of Some)
    //       -> into_iter -> collect to Vec<char>.

    // to_lowercase returns an iterator of valid lowercase characters,
    // we choose the first one with next()
    // initially copied from jhod0 on exercism.io
    // http://exercism.io/submissions/dcf0f37018d7470db5a5aab54ed04563

    // map string to a collection of lowercase chars
    let lc_char_map = s.chars().map(|c| c.to_lowercase()
                                    .next()
                                    .expect("should be char"));
    // -> core::iter::Map<core::str::Chars<_>

    // sort if necessary
    let lc_string: String = if sort {
        let mut chars: Vec<char> = lc_char_map.into_iter().collect();
        chars.sort();  // sort works in-place on mutable vec
        chars.into_iter().collect()
    } else {
        lc_char_map.collect::<String>()
    };
    lc_string
}
