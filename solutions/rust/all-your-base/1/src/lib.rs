#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

fn convert_to(digit: u32, radix: u32) -> Vec<u32> {
    let div = digit / radix;
    let rem = digit % radix;
    let mut vec = vec![rem];
    if div != 0 {
        let mut res = convert_to(div, radix);
        res.append(&mut vec);
        vec = res
    }
    vec
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }
    if let Some(&e) = number.iter().find(|n| from_base.le(n)) {
        return Err(Error::InvalidDigit(e));
    }
    let dec = number.iter().rev().enumerate().map(|(i, n)| n * from_base.pow(i as u32)).sum();
    Ok(convert_to(dec, to_base))
}
