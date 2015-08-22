use std::collections::HashMap;

pub fn count(c: char, strand: &str) -> usize {
    // fold
    let count = strand
        .chars()
        .filter(|&x| x == c)
        .fold(0, |acc, _| acc + 1);

    count
}

pub fn nucleotide_counts(s: &str) -> HashMap<char, usize> {
    let mut counts : HashMap<char, usize> = HashMap::new();
    let bases = vec!('A', 'T', 'C', 'G');
    for base in bases {
        counts.insert(base, count(base, s));
    }

    counts
}
