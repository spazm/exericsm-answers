//! RNA Transcription
//!
//! given a DNA strand, returns its RNA complement (per RNA transcription).
//!
//! Both DNA and RNA strands are a sequence of nucleotides.
//!
//! The four nucleotides found in DNA are adenine (**A**), cytosine (**C**),
//! guanine (**G**) and thymine (**T**).
//!
//! The four nucleotides found in RNA are adenine (**A**), cytosine (**C**),
//! guanine (**G**) and uracil (**U**).
//!
//! Given a DNA strand, its transcribed RNA strand is formed by replacing
//! each nucleotide with its complement:
//!
//! * `G` -> `C`
//! * `C` -> `G`
//! * `T` -> `A`
//! * `A` -> `U`

// RNA
// support comparison for equality
// supports new("STRING")
// supports as_ref() to return "STRING"
#[derive(Clone, Debug, PartialEq)]
pub struct RibonucleicAcid {pub strand: String}

#[derive(Clone, Debug, PartialEq)]
pub struct DeoxyribonucleicAcid {pub strand: String}

impl RibonucleicAcid {
    pub fn new<>(s: &str) -> RibonucleicAcid {
        RibonucleicAcid {strand: String::from(s)}
    }

    pub fn as_ref(&self) -> &str{
        &self.strand
    }
}

// DNA
// support comparison for equality
// supports to_rna() to transcribe to RNA
// supports new("STRING")
// supports as_ref() to return "STRING"

impl DeoxyribonucleicAcid {
    pub fn new(s: &str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid {strand: String::from(s)}
    }

    pub fn to_rna(self) -> RibonucleicAcid{
        let rna: String = self.strand.chars()
            .map(|x| match x {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _   => unreachable!()}).collect::<String>();
        RibonucleicAcid::new(rna.as_ref())
    }

    pub fn as_ref(&self) -> &str{
        &self.strand
    }
}
