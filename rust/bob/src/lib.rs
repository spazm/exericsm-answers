//!Bob is a lackadaisical teenager. In conversation, his responses are very limited.
//!
//! * Bob answers 'Sure.' if you ask him a question.
//! * He answers 'Whoa, chill out!' if you yell at him.
//! * He says 'Fine. Be that way!' if you address him without actually saying anything.
//! * He answers 'Whatever.' to anything else.

extern crate regex;

/// Produce the correct bob response for the msg with the following rules:
///
/// * question: ends with a ?
/// * shout: contains upper case and no lower case letters
/// * empty: empty
/// * all others
pub fn reply(msg: &str) -> &'static str {
    match msg {
        msg if regex::is_match(r"\?$", msg).unwrap() => "Sure.",
        msg if regex::is_match(r"\p{Lu}", msg).unwrap() 
            && ! regex::is_match(r"\p{Ll}", msg).unwrap() => "Whoa, chill out!",
        msg if msg.is_empty() => "Fine. Be that way!",
        _ => "Whatever."
    }
}
