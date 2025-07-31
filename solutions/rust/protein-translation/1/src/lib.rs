use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    codon_map: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.codon_map.get(codon).copied()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let rna_vec = rna.chars().collect::<Vec<_>>();
        let mut result = Vec::new();
        for chunk in rna_vec.chunks(3) {
            let codon: String = chunk.iter().collect();
            match self.codon_map.get(codon.as_str()).copied() {
                Some("stop codon") => break,
                Some(valid) => result.push(valid),
                None => return None,
            };
        }
        Some(result)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo { codon_map: pairs.iter().cloned().collect() }
}
