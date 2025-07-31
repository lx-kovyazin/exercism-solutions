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

fn find_palindrome_products(mut min: u64, max: u64) -> Vec<Palindrome> {
    let mut palindrome_products = Vec::new();
    while min < max {
        palindrome_products.append(
            &mut (min..=max).filter_map(|i| Palindrome::new(min * i)).collect()
        );        
        min += 1;
    }
    palindrome_products
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let palindrome_products = find_palindrome_products(min, max);
    Some((*palindrome_products.iter().min()?, *palindrome_products.iter().max()?))
}
