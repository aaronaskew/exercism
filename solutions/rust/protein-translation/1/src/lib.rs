use std::collections::HashMap;

use itertools::Itertools;

pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let mut codons = HashMap::new();

    codons.insert("AUG", "Methionine");
    codons.insert("UUU", "Phenylalanine");
    codons.insert("UUC", "Phenylalanine");
    codons.insert("UUA", "Leucine");
    codons.insert("UUG", "Leucine");
    codons.insert("UCU", "Serine");
    codons.insert("UCC", "Serine");
    codons.insert("UCA", "Serine");
    codons.insert("UCG", "Serine");
    codons.insert("UAU", "Tyrosine");
    codons.insert("UAC", "Tyrosine");
    codons.insert("UGU", "Cysteine");
    codons.insert("UGC", "Cysteine");
    codons.insert("UGG", "Tryptophan");
    codons.insert("UAA", "STOP");
    codons.insert("UAG", "STOP");
    codons.insert("UGA", "STOP");

    let mut aminos = Vec::new();

    for chunk in rna.chars().chunks(3).into_iter() {
        let codon: String = chunk.collect();

        if codon.len() != 3 {
            return None;
        }

        if let Some(amino) = codons.get(codon.as_str()) {
            if *amino == "STOP" {
                break;
            }

            aminos.push(*amino);
        } else {
            return None;
        }
    }

    Some(aminos)
}
