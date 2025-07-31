// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut mag_map: HashMap<&str, usize> = HashMap::new();
    let mut not_map: HashMap<&str, usize> = HashMap::new();

    magazine
        .iter()
        .for_each(|w| *mag_map.entry(w).or_default() += 1);
    note.iter()
        .for_each(|w| *not_map.entry(w).or_default() += 1);

    not_map.iter().all(|ni| match mag_map.get(ni.0) {
        Some(count) => count.ge(ni.1),
        None => false,
    })
}
