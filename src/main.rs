mod bezout;
mod phi;
mod prime;
mod rsa;
mod testing;

use bezout::mod_mul_inv;
use phi::phi;
use rsa::{rsadec, rsaenc};

fn main() {
    let n = 101 * 103;
    let e = 7;
    let m = 10331;
    let encrypted = rsaenc(n, e, m);
    println!("{:?}", encrypted);
    println!("{:?}", rsadec(n, mod_mul_inv(e, phi(n)), encrypted));
}
