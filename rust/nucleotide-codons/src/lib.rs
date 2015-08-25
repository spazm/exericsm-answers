//!# NAME:
//! Nucleotide Codons
//!
//!# SYNOPSIS:
//!
//!```
//! extern crate nucleotide_codons;
//! let name_pairs : Vec<(&str, &str)> = vec!(("ATG", "methionine"));
//! let info = nucleotide_codons::parse(name_pairs);
//! assert_eq!(info.name_for("RTG"), Ok("methionine"));
//!```
//!
//!# DESCRIPTION:
//!
//!Within DNA sequences of 3 nucleotides, called codons, encode for amino acids.
//!Often several codons encode for the same amino acid. The International Union of Pure
//!and Applied Chemistry developed a shorthand system for designating groups of
//!codons that encode for the same amino acid.
//!
//!Simply put they've expanded the four letters A, C, G and T with a bunch of
//!letters that stand for different possibilities. For example R means A and G.
//!So TAR stands for TAA and TAG (think of "TAR" as "TA[AG]" in regex notation).
//!
//!Write some code that given a codon, which may use shorthand, returns the
//!name of the amino acid that that codon encodes for. You will be given
//!a list of non-shorthand-codon/name pairs to base your computation on.
//!
//!See: [wikipedia](https://en.wikipedia.org/wiki/DNA_codon_table).

use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Debug, PartialEq)]
pub struct Codons {
    codons: HashMap<&'static str, &'static str>
}

#[derive(Debug, PartialEq)]
pub enum Error { TooShort, TooLong, NotShorthand }

pub fn parse(pairs: Vec<(&'static str, &'static str)>) -> Codons {
    Codons {
        codons: HashMap::from_iter(pairs)
    }
}

/// return a normalized codon matching compressed IUPAC notation
///
/// compressed IUPAC notation can refer to many possible codons, we will return a single one to use
/// for name look-up.
///
/// Maps from https://en.wikipedia.org/wiki/Nucleic_acid_notation
///
///```text
/// A -> A
/// C -> C
/// G -> G
/// T -> T
/// U -> U
/// W -> A | T
/// S -> C | G
/// M -> A | C
/// K -> G | T
/// R -> A | G
/// Y -> C | T
/// B -> C | G | T
/// D -> A | G | T
/// H -> A | C | T
/// V -> A | C | G
/// N -> A | C | G | T (gap, any)
///```
///
/// Collapsed reverse map:
///
///```text
/// A | W | M | R | D | H | V | N -> A
/// C | S | Y | B -> C
/// G | K -> G
/// T -> T
///```
///
/// Valid codons are ok:
///
///```
/// assert!(nucleotide_codons::normalize_codon("A").is_ok());
/// assert_eq!(nucleotide_codons::normalize_codon("R").ok().expect(""), "A");
///```
///
/// Invalid codons are not ok:
///
///```
/// assert!(!nucleotide_codons::normalize_codon("Z").is_ok());
///```
pub fn normalize_codon<'a>(s: &'a str) -> Result<String, Error> {
    let mut valid_shorthand = true;
    let s = s.chars().map( |c| match c {
        'A' | 'W' | 'M' | 'R' | 'D' | 'H' | 'V' | 'N' => 'A',
        'C' | 'S' | 'Y' | 'B' => 'C',
        'G' | 'K' => 'G',
        'T' => 'T',
        x => {valid_shorthand = false; x},
    }
    ).collect::<String>();
    if !valid_shorthand { return Err(Error::NotShorthand)}
    Ok(s)
}

impl Codons {
    pub fn name_for(&self, s: &str) -> Result<&str, Error>{
        match s.len() {
            i if i < 3 => Err(Error::TooShort),
            i if i > 3 => Err(Error::TooLong),
            _ => {
                let s = try!(normalize_codon(s));
                match self.codons.get(&*s) {
                    Some(codon_name) => Ok(codon_name),
                    None => Err(Error::NotShorthand)
                }
            }
        }
    }
}
