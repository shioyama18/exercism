extern crate rand;
use rand::Rng;
use rand::os::OsRng;

pub fn private_key(p: u64) -> u64 {
    OsRng::new().unwrap().gen_range(2, p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    if a < std::u32::MAX as u64 {
        // check for overflow
        g.pow(a as u32) % p
    } else {
        powmod(g, a, p)
    }
    // code below is too slow when a is large
    // (0..a).fold(1, |acc, _| (g * acc) % p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    if a < std::u32::MAX as u64 {
        // check for overflow
        b_pub.pow(a as u32) % p
    } else {
        powmod(b_pub, a, p)
    }
    // code below is too slow when a is large
    // (0..a).fold(1, |acc, _| (b_pub * acc) % p)

}

fn powmod(mut base: u64, mut exp: u64, p: u64) -> u64 {
    let mut total = 1;

    while exp > 0 {
        if exp % 2 != 0 {
            total *= base;
            total %= p;
        } 
        base *= base;
        base %= p;
        exp >>= 1;
    }

    total
}
