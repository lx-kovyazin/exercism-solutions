#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

fn validate_set(valid_set: &str, validating: &str) -> Result<String, usize> {
    if let Some(iv) = validating.chars().position(|c| !valid_set.contains(c)) {
        Err(iv)
    } else {
        Ok(validating.to_owned())
    }
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        match validate_set("GCTA", dna) {
            Err(e) => Err(e),
            Ok(s) => Ok(Dna(s)),
        }
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
                    x => x,
                })
                .collect::<String>(),
        )
        .unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        match validate_set("CGAU", rna) {
            Err(e) => Err(e),
            Ok(s) => Ok(Rna(s)),
        }
    }
}
