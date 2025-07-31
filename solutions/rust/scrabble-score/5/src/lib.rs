/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.to_ascii_uppercase()
        .chars()
        .map(|c| match c {
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
            'F' | 'H' | 'V' | 'W' | 'Y' => 4,
            'B' | 'C' | 'M' | 'P' => 3,
            'D' | 'G' => 2,
            'J' | 'X' => 8,
            'Q' | 'Z' => 10,
            'K' => 5,
            _ => 0,
        })
        .sum()
}
