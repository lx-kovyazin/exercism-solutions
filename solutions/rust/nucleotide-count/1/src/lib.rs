use std::{collections::HashMap, ops::AddAssign};

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match nucleotide_counts(dna)?.get(&nucleotide) {
        None => Err(nucleotide),
        Some(&count) => Ok(count)
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    dna.chars().try_fold(
        HashMap::from_iter("ACGT".chars().map(|c| (c, 0))),
        |mut nucleotide_map, nucleotide| {
            if !nucleotide_map.contains_key(&nucleotide) {
                Err(nucleotide)
            } else {
                nucleotide_map
                    .entry(nucleotide)
                    .and_modify(|n| n.add_assign(1));
                Ok(nucleotide_map)
            }
        },
    )
}
