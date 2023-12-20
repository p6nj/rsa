mod bezout;
mod phi;
mod prime;
mod rsa;
mod testing;

use bezout::mod_mul_inv;
use phi::phi;
use rsa::{rsadec, rsaenc};

use crate::prime::primes;

fn main() {
    let n = 101 * 103;
    let e = 7;
    let m = 10331;
    let d = mod_mul_inv(e, dbg!(phi(n)));
    let encrypted = rsaenc(n, e, m);
    println!("{:?}", encrypted);
    println!("{:?}", rsadec(n, 3643, encrypted));
    assert_eq!(1, (8743 * e) % phi(n));
    println!("{:?}", primes(100));
}
