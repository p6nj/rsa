fn main() {
    println!("{:?}", decomp(100));
}

fn primes(max: usize) -> Vec<usize> {
    (2..((usize::from(max) as f32).sqrt() as usize) + 1).fold(
        (2..=max.into()).collect(),
        |mut acc, n: usize| {
            (0..usize::from(max) / n - 1)
                .map(|i| n.pow(2) + i * n)
                .for_each(|i| {
                    if let Ok(index) = acc.binary_search(&i) {
                        acc.remove(index);
                    }
                });
            acc
        },
    )
}

fn phi(n: usize) -> usize {
    let prs = primes(n.div_euclid(2));
    prs.clone()
        .iter()
        .cloned()
        .zip(prs.iter().map(|prime| divn(n, *prime)))
        .fold(1, |acc, (p, k)| {
            acc * p.pow((k - 1).try_into().unwrap()) * (p - 1)
        })
}

fn divn(n: usize, p: usize) -> usize {
    match n > p {
        true => 1 + divn(n.div_euclid(p.into()), p),
        false => 1,
    }
}

fn decomp(n: usize) -> (Vec<usize>, Vec<usize>) {
    let prs = primes(n.div_euclid(2));
    (
        prs.clone(),
        prs.iter().map(|prime| divn(n, *prime)).collect(),
    )
}
