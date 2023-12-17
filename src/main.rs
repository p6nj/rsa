#[cfg(test)]
mod tests;

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
