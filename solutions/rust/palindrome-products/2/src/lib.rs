#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Palindrome(u64);

impl Palindrome {
    pub fn new(value: u64) -> Option<Palindrome> {
        if !Self::palindrome(&Self::into_digits(value.clone())) {
            None
        } else {
            Some(Self(value))
        }
    }

    fn into_digits(mut value: u64) -> Vec<u64> {
        let mut digits = Vec::new();
        if value == 0 {
            digits.push(0);
        }
        while value > 0 {
            digits.push(value % 10);
            value /= 10
        }
        digits
    }

    fn palindrome(digits: &[u64]) -> bool {
        let mut l = 0;
        let mut r = digits.len() - 1;
        while l < r {
            if digits[l] != digits[r] {
                return false;
            }
            l += 1;
            r -= 1;
        }
        return true;
    }

    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let palindrome_products: Vec<Palindrome>
        = (min..=max)
            .flat_map(|a| (min..=max).filter_map(move |b| Palindrome::new(a * b)))
            .collect();
    match palindrome_products.len() {
        0 => None,
        _ => Some((
            *palindrome_products.iter().min().unwrap(),
            *palindrome_products.iter().max().unwrap(),
        ))
    }
}
