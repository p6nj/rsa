use super::*;
use crate::testing::HUGE_NUMBER;

#[test]
fn decomp_normal() {
    assert_eq!(vec![(2, 1), (5, 2)], decomp(50).to_vec());
}

#[test]
fn decomp_prime() {
    let prime = *primes(500).last().unwrap();
    assert_eq!(vec![(prime, 1)], decomp(prime).to_vec());
}

#[test]
fn decomp_illegal() {
    assert_eq!(Vec::<(usize, usize)>::new(), decomp(0).to_vec());
    assert_eq!(Vec::<(usize, usize)>::new(), decomp(1).to_vec());
    assert_eq!(vec![(2, 1)], decomp(2).to_vec());
}

#[test]
fn decomp_bully() {
    decomp(HUGE_NUMBER);
}

#[test]
fn phi_normal() {
    assert_eq!(144, phi(456));
}

#[test]
fn phi_prime() {
    let prime = *primes(500).last().unwrap();
    assert_eq!(prime - 1, phi(prime));
}

#[test]
fn phi_illegal() {
    assert_eq!(0, phi(0));
    assert_eq!(1, phi(1));
    assert_eq!(1, phi(2));
}

#[test]
fn phi_bully() {
    phi(HUGE_NUMBER);
}
