pub fn series(digits: &str, len: usize) -> Vec<String> {
    if digits.len() == 0 {
        Vec::new()
    } else if len == 0 {
        vec![String::new(); digits.len() + 1]
    } else {
        digits
            .chars()
            .collect::<Vec<_>>()
            .windows(len)
            .map(|cs| cs.iter().collect())
            .collect()
    }
}
