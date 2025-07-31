pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        1 => return Some(0),
        even if n % 2 == 0 => even.checked_div(2),
        odd => odd.checked_mul(3)?.checked_add(1),
    }
    .and_then(|val| collatz(val)?.checked_add(1))
}
