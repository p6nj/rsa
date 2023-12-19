use crate::{bezout::mod_mul_inv, phi::phi};

use super::*;

#[test]
fn enc_dec() {
    let n = 101 * 103;
    let e = 7;
    let m = 10331;
    assert_eq!(m, rsadec(n, mod_mul_inv(e, phi(n)), rsaenc(n, e, m)));
}
