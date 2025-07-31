const POINT_TABLE: [(u64, &str); 7] = [
    (1,  "AEIOULNRST"),
    (2,  "DG"),
    (3,  "BCMP"),
    (4,  "FHVWY"),
    (5,  "K"),
    (8,  "JX"),
    (10, "QZ"),
];

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.to_ascii_uppercase()
        .chars()
        .flat_map(|c|
            POINT_TABLE.map(|(point, letters)| 
                if letters.contains(c) { point } else { 0 }
            )
        )
        .sum()
}
