use std::collections::HashMap;

const NUCLEOTIDES: &str = "ACGT";

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    NUCLEOTIDES.find(nucleotide).ok_or(nucleotide)?;
    dna.chars().try_fold(0, |count, n| {
        NUCLEOTIDES.find(n).ok_or(n)?;
        Ok(count + if n == nucleotide { 1 } else { 0 })
    })
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let init_counts: HashMap<_, _> = NUCLEOTIDES.chars().map(|c| (c, 0)).collect();
    dna.chars().try_fold(init_counts, |mut acc, c| {
        *acc.get_mut(&c).ok_or(c)? += 1;
        Ok(acc)
    })
}
