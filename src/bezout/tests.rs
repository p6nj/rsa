use super::*;

#[test]
fn mod_mul_inv_normal() {
    assert_eq!(854, mod_mul_inv(123, 4567));
}

#[test]
fn mod_mul_inv_prime() {
    assert_eq!(51, mod_mul_inv(101, 103));
}

#[test]
fn mod_mul_inv_special() {
    assert_eq!(8743, mod_mul_inv(7, 10200));
}

const a1: usize = 97;
const b1: usize = 73;
const u1: isize = -3;
const v1: usize = 4;
const a2: usize = 12;
const b2: usize = 42;
const u2: isize = -3;
const v2: usize = 1;

#[test]
fn bezout_r_normal() {
    assert_eq!(v1 as isize, bezout_r(a1, b1));
    assert_eq!(v2 as isize, bezout_r(a2, b2));
}

#[test]
fn vu_normal() {
    assert_eq!(u1, vu(a1, b1, v1 as isize));
    assert_eq!(u2, vu(a2, b2, v2 as isize));
}

#[test]
fn bezout_normal() {
    assert_eq!((-3, 1), bezout(12, 42));
    assert_eq!((-3, 4), bezout(97, 73));
}
