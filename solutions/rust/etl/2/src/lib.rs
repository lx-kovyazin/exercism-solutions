use std::collections::BTreeMap;

pub fn transform(prev: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    prev.iter()
        .flat_map(|(&point, letters)| {
            letters.iter().map(move |letter| (letter.to_ascii_lowercase(), point))
        })
        .collect()
}
