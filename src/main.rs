fn main() {
    println!("{:?}", primes(100000));
}

fn primes(max: usize) -> Vec<usize> {
    let mut candidates: Vec<Option<usize>> = (2..=max).map(|i| Some(i)).collect();
    (2..=(max as f32).sqrt() as usize).for_each(|i| {
        let pow = i.pow(2);
        (0..=(max - pow).div_euclid(i)).for_each(|j| {
            candidates[pow + i * j - 2] = None;
        })
    });
    return candidates.iter().cloned().flatten().collect();
}
