use crate::{bezout::mod_mul_inv, phi::phi};

use super::*;

#[test]
fn enc_dec() {
    let n = 101 * 103;
    let e = 7;
    let m = [1, 0, 3, 3, 1];
    assert_eq!(
        m.to_vec(),
        m.map(|i| rsaenc(n, e, i))
            .map(|i| rsadec(n, mod_mul_inv(e, phi(n)), i))
            .to_vec()
    )
}
