use super::*;
use crate::testing::HUGE_NUMBER;

#[test]
fn primes_normal() {
    assert_eq!(
        vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47],
        primes(50).to_vec()
    );
}

#[test]
fn primes_illegal() {
    assert_eq!(Vec::<usize>::new(), primes(0).to_vec());
    assert_eq!(Vec::<usize>::new(), primes(1).to_vec());
    assert_eq!(vec![2], primes(2).to_vec());
}

#[test]
fn primes_bully() {
    primes(HUGE_NUMBER);
}
