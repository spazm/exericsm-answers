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
///
/// Collapsed reverse map:
/// A | W | M | R | D | H | V | N -> A
/// C | S | Y | B -> C
/// G | K -> G
/// T -> T
fn normalize_codon<'a>(s: &'a str) -> Result<String, Error> {
    let s = s.chars().map( |c| match c {
        'A' | 'W' | 'M' | 'R' | 'D' | 'H' | 'V' | 'N' => 'A',
        'C' | 'S' | 'Y' | 'B' => 'C',
        'G' | 'K' => 'G',
        'T' => 'T',
        x => x, //_return Err(Error::NotShorthand),
    }
    ).collect::<String>();
    Ok(s)
}

impl Codons {
    pub fn name_for(&self, s: &str) -> Result<&str, Error>{
        // match guard expression instead of if/else if?
        if s.len() < 3 {
            return Err(Error::TooShort) }
        else if s.len() > 3 {
            return Err(Error::TooLong) }

        assert_eq!(s.len(), 3);
        let s : String = try!(normalize_codon(s));
        match self.codons.get(&*s) {
            Some(codon_name) => return Ok(codon_name),
            None => return Err(Error::NotShorthand)
        }
    }
}
