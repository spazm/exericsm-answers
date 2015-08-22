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

// RNA:
// * support comparison for equality
// * supports new("STRING")
// * supports as_ref() to return "STRING"

/// The four nucleotides found in RNA are
/// adenine (**A**),
/// cytosine (**C**),
/// guanine (**G**), and
/// uracil (**U**).
#[derive(Debug, PartialEq)]
pub struct RibonucleicAcid {pub strand: String}

// DNA
// * support comparison for equality
// * supports to_rna() to transcribe to RNA
// * supports new("STRING")
// * supports as_ref() to return "STRING"

/// The four nucleotides found in DNA are
/// adenine (**A**),
/// cytosine (**C**),
/// guanine (**G**), and
/// thymine (**T**).
#[derive(Debug, PartialEq)]
pub struct DeoxyribonucleicAcid {pub strand: String}

impl RibonucleicAcid {
    /// Create a RibonucleicAcid from s, a valid string of nucleotides.
    pub fn new<>(s: &str) -> RibonucleicAcid {
        let strand: String = s.chars()
            .filter(|&x| match x {
                'A' | 'C' | 'G' | 'U' => true,
                _   => panic!(format!("invalid nucleotide: {}", x))})
            .collect::<String>();
        RibonucleicAcid {strand: strand}
    }

    pub fn as_ref(&self) -> &str{
        &self.strand
    }
}


impl DeoxyribonucleicAcid {
    /// Create a DeoxyribonucleicAcid from s, a valid string of nucleotides.
    pub fn new(s: &str) -> DeoxyribonucleicAcid {
        // verify valid nucleotides
        let strand: String = s.chars()
            .filter(|&x| match x {
                'G' | 'C' | 'T' | 'A' => true,
                _   => panic!(format!("invalid nucleotide: {}", x))})
            .collect::<String>();
        DeoxyribonucleicAcid {strand: strand}
    }

    /// Transcribes DNA to RNA.
    ///
    /// Given a DNA strand, its transcribed RNA strand is formed by replacing
    /// each nucleotide with its complement:
    ///
    /// * `G` -> `C`
    /// * `C` -> `G`
    /// * `T` -> `A`
    /// * `A` -> `U`
    pub fn to_rna(self) -> RibonucleicAcid{
        let rna: String = self.strand
            .chars()
            .map(|x| match x {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _   => panic!(format!("invalid nucleotide: {} in dna {:?}", x, self))})
            .collect::<String>();
        RibonucleicAcid::new(rna.as_ref())
    }

    pub fn as_ref(&self) -> &str{
        &self.strand
    }
}
