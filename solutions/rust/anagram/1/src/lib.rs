use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercased = |s: &str| -> Vec<String> {
        s.chars().map(|c| c.to_lowercase().to_string()).collect()
    };
    let sorted = |us: &Vec<String>| -> Vec<String> { 
        let mut v = us.clone();
        v.sort_unstable();
        v
    };

    let lowercased_word = lowercased(word);
    let sorted_word = sorted(&lowercased_word);
    possible_anagrams.into_iter().fold(HashSet::new(), |mut res, pa| {
        let lowercased_pa = lowercased(pa);
        if lowercased_word != lowercased_pa && sorted_word == sorted(&lowercased_pa) {
            res.insert(pa);
        }
        res
    })
}
