#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Palindrome(u64);

impl Palindrome {
    pub fn new(value: u64) -> Option<Palindrome> {
        if value == Self::reverse(value) {
            Some(Self(value))
        } else {
            None
        }
    }

    fn reverse(mut value: u64) -> u64 {
        let mut reverse = 0u64;
        while value != 0 {
            reverse = reverse * 10 + value % 10;
            value = value / 10;
        }
        reverse
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
    palindrome_products.sort_unstable();
    palindrome_products
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let palindrome_products = find_palindrome_products(min, max);
    Some((*palindrome_products.first()?, *palindrome_products.last()?))
}
