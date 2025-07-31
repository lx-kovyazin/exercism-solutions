fn modpow(base: u64, exponent: u64, modulus: u64) -> u64 {
    use num::ToPrimitive;
    use num::bigint::ToBigUint;

    base.to_biguint().unwrap().modpow(
        &exponent.to_biguint().unwrap(),
        &modulus.to_biguint().unwrap()
    ).to_u64().unwrap()
}

pub fn private_key(p: u64) -> u64 {
    use rand::Rng;
    rand::thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modpow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modpow(b_pub, a, p)
}
