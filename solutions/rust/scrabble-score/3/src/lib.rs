const POINT_TABLE: [(u64, &[char]); 7] = [
    (1,  &['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T']),
    (2,  &['D', 'G']),
    (3,  &['B', 'C', 'M', 'P']),
    (4,  &['F', 'H', 'V', 'W', 'Y']),
    (5,  &['K']),
    (8,  &['J', 'X']),
    (10, &['Q', 'Z']),
];

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.to_ascii_uppercase()
        .chars()
        .flat_map(|c|
            POINT_TABLE.map(|(point, letters)| 
                if letters.contains(&c) { point } else { 0 }
            )
        )
        .sum()
}
