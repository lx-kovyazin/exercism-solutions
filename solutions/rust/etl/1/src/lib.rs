use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter().fold(BTreeMap::new(), |mut transformed, (&point, letters)| {
        letters.iter().flat_map(|letter| letter.to_lowercase()).for_each(|letter| {
            transformed.insert(letter, point);
        });
        transformed
    })
}
