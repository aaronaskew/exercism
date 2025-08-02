#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        dna.char_indices().try_for_each(|(i, c)| match c {
            'G' | 'C' | 'T' | 'A' => Ok(()),
            _ => Err(i),
        })?;
        Ok(Dna(dna.to_string()))
    }

    pub fn into_rna(self) -> Rna {
        Rna::new(
            &self
                .0
                .chars()
                .map(|c| match c {
                    'G' => 'C',
                    'C' => 'G',
                    'T' => 'A',
                    'A' => 'U',
                    _ => panic!("Invalid DNA"),
                })
                .collect::<String>(),
        )
        .unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        rna.char_indices().try_for_each(|(i, c)| match c {
            'G' | 'C' | 'U' | 'A' => Ok(()),
            _ => Err(i),
        })?;

        Ok(Rna(rna.to_string()))
    }
}
