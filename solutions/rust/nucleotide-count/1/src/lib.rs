use std::collections::HashMap;

const VALID_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !is_valid_nucleotide(nucleotide) {
        return Err(nucleotide);
    }

    let mut invalid_chars = Vec::<char>::new();
    let count = dna.chars().fold(0, |acc, c| {
        if !is_valid_nucleotide(c) {
            invalid_chars.push(c);
        }

        if c != nucleotide {
            return acc;
        }

        acc + 1
    });

    if !invalid_chars.is_empty() {
        return Err(invalid_chars[0]);
    }

    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::<char, usize>::new();

    for nucleotide in VALID_NUCLEOTIDES {
        counts.insert(nucleotide, count(nucleotide, dna)?);
    }

    Ok(counts)
}

pub fn is_valid_nucleotide(nucleotide: char) -> bool {
    VALID_NUCLEOTIDES.contains(&nucleotide)
}
