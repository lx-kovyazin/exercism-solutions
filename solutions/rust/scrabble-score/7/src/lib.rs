macro_rules! when {
    ($m_ex:expr; $($arm:pat $(in $str:literal)? => $ex:expr),*$(,)?) => {
        match $m_ex {
            $(_m $(if $str.contains(_m))? => $ex),*
        }
    };
}

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.to_ascii_uppercase()
        .chars()
        .map(|c| when! { c;
            _ in "AEIOULNRST" => 1,
            _ in "FHVWY" => 4,
            _ in "BCMP" => 3,
            _ in "DG" => 2,
            _ in "JX" => 8,
            _ in "QZ" => 10,
            _ in "K" => 5,
            _ => 0,
        })
        .sum()
}
