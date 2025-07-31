pub fn is_armstrong_number(num: u32) -> bool {
    match (num as f32).log10() {
        f32::NEG_INFINITY => true,
        x => {
            let dig_n = 1 + x as u32;
            (0..dig_n)
                .fold((0u32, num), |(sum, quotient), _| {
                    (sum + (quotient % 10).pow(dig_n), quotient / 10u32)
                })
                .0
                == num
        }
    }
}
