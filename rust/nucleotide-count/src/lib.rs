use std::collections::HashMap;

static DNA_BASES: &'static str = "ACGT";

/// Count occurences of each DNA base (A,C,G,T) in s.
pub fn nucleotide_counts(s: &str) -> HashMap<char, usize> {
    let mut counts : HashMap<char, usize> = HashMap::new();
    for base in DNA_BASES.chars() {
        counts.insert(base, count(base, s));
    }

    counts
}

/// Count occurences of `c` in `strand`
pub fn count(c: char, strand: &str) -> usize {
    strand
        .chars()
        .filter(|&x| x == c)
        .fold(0, |acc, _| acc + 1)
}
