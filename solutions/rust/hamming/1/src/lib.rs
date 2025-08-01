pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        None
    } else {
        Some(s1.chars().zip(s2.chars()).map(|(c1, c2)| (c1 != c2) as usize).sum())
    }    
}
