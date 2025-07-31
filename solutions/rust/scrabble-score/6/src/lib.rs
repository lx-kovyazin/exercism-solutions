/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.to_ascii_uppercase()
        .chars()
        .map(|c| match c {
            c if "AEIOULNRST".contains(c) => 1,
            c if "FHVWY".contains(c) => 4,
            c if "BCMP".contains(c) => 3,
            c if "DG".contains(c) => 2,
            c if "JX".contains(c) => 8,
            c if "QZ".contains(c) => 10,
            c if "K".contains(c) => 5,
            _ => 0,
        })
        .sum()
}
