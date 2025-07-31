/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code: String = code.split_whitespace().collect();
    match code.len() {
        0 | 1 => false,
        _ => match code.parse::<u64>() {
            Err(_) => false,
            Ok(_) => {
                code.chars()
                    .enumerate()
                    .map(|(i, c)| {
                        let d = c.to_string().parse::<u8>().unwrap();
                        let odd = (code.len() % 2 != 0) as usize;
                        if (i + odd) % 2 != 0 {
                            d
                        } else {
                            match 2u8 * d {
                                prod if prod > 9 => prod - 9,
                                prod => prod,
                            }
                        }
                    })
                    .sum::<u8>() % 10u8 == 0
            }
        }
    }
}
