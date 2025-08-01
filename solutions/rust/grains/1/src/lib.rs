pub fn square(s: u32) -> u64 {
    assert!(1 <= s && s <= 64, "Square must be between 1 and 64");
    1 << (s - 1)
}

pub fn total() -> u64 {
    (1..=64).map(|s| square(s)).sum()
}
