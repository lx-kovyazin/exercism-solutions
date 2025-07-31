use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    note.iter()
        .fold(HashMap::new(), |mut map, keyword| {
            map.entry(dbg!(keyword)).and_modify(|count| *count += 1).or_insert(1);
            map
        })
        .into_iter()
        .all(|(&keyword, count)| {
            count <= magazine.iter().filter(|&&word| word == keyword).count()
        })
}
