use advtools::prelude::*;
use advtools::input::iter_input;

fn pow_mod(mut base: u64, mut exp: u64) -> u64 {
    let mut result = 1;
    while exp != 0 {
        if exp & 1 != 0 { result = (result * base) % 20201227; }
        base = (base * base) % 20201227;
        exp >>= 1;
    }
    result
}

fn main() {
    let (k1, k2) = iter_input::<u64>().collect_tuple().unwrap();
    for l in 0.. {
        let tf = pow_mod(7, l);
        if tf == k1 || tf == k2 {
            let key = if tf == k1 { pow_mod(k2, l) } else { pow_mod(k1, l) };
            return advtools::verify("Encryption key", key, 3286137);
        }
    }
}
