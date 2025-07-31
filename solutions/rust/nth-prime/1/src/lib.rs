pub fn nth(n: u32) -> u32 {
    let n = n as usize;
    let mut primes = vec![];
    for i in 2.. {
        if primes.iter().any(|p| i % p == 0) {
            continue;
        } else {
            primes.push(i);
            if primes.len() - 1 == n {
                break;
            }
        }
    }
    return primes[n];
}
