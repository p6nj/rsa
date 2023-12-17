fn main() {
    println!("{:?}", phi(20));
}

fn primes(max: usize) -> Vec<usize> {
    if max < 2 {
        return vec![];
    }
    let mut candidates: Vec<Option<usize>> = (2..=max).map(|i| Some(i)).collect();
    (2..=(max as f32).sqrt() as usize).for_each(|i| {
        let pow = i.pow(2);
        (0..=(max - pow).div_euclid(i)).for_each(|j| {
            candidates[pow + i * j - 2] = None;
        })
    });
    return candidates.iter().cloned().flatten().collect();
}

fn decomp(n: usize) -> Vec<(usize, usize)> {
    if n < 2 {
        return vec![];
    }
    let ps = [primes(n.div_euclid(2)), vec![n]].concat();
    let mut n = n;
    ps.iter()
        .map(|p| {
            if n == 1 {
                return (*p, 0);
            }
            let mut pow = 0;
            while n % p == 0 {
                n = n.div_euclid(*p);
                pow += 1;
            }
            (*p, pow)
        })
        .filter(|(_, n)| *n != 0)
        .collect()
}

fn phi(n: usize) -> usize {
    if n < 2 {
        return n;
    }
    decomp(n)
        .iter()
        .map(|(p, k)| p.pow((k - 1).try_into().unwrap()) * (p - 1))
        .reduce(|acc, e| acc * e)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const HUGE_NUMBER: usize = 10000000;

    #[test]
    fn primes_normal() {
        assert_eq!(
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47],
            primes(50)
        );
    }

    #[test]
    fn primes_illegal() {
        assert_eq!(Vec::<usize>::new(), primes(0));
        assert_eq!(Vec::<usize>::new(), primes(1));
        assert_eq!(vec![2], primes(2));
    }

    #[test]
    fn primes_bully() {
        primes(HUGE_NUMBER);
    }

    #[test]
    fn decomp_normal() {
        assert_eq!(vec![(2, 1), (5, 2)], decomp(50));
    }

    #[test]
    fn decomp_prime() {
        let prime = *primes(500).last().unwrap();
        assert_eq!(vec![(prime, 1)], decomp(prime));
    }

    #[test]
    fn decomp_illegal() {
        assert_eq!(Vec::<(usize, usize)>::new(), decomp(0));
        assert_eq!(Vec::<(usize, usize)>::new(), decomp(1));
        assert_eq!(vec![(2, 1)], decomp(2));
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
}
