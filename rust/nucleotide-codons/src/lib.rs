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

impl Codons {
    pub fn name_for(&self, s: &str) -> Result<&str, Error>{
        println!("self: {:?}", self);
        return Ok("ABC");
//        match s.len()
//            Err(Error::TooShort)
//            Err(Error::TooLong)
//            Err(Error::NotShorthand)
    }
}
