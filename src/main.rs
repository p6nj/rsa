fn main() {
    println!("{:?}", primes(50));
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

#[cfg(test)]
mod tests {
    use super::*;

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
        primes(1000000);
    }
}
