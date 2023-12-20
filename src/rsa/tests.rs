use super::*;
use crate::{bezout::mod_mul_inv, phi::phi};

#[test]
fn enc_dec() {
    let n = 101 * 103;
    let e = 7;
    let m = 10331;
    // assert_eq!();
}

#[test]
fn mod_exp_normal() {
    assert_eq!(2, mod_exp(3, 7, 5));
    assert_eq!(10, mod_exp(2, 10, 13));
}
