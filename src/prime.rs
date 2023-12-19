#[cfg(test)]
mod tests;

pub(super) fn primes(max: usize) -> Box<[usize]> {
    if max < 2 {
        return Box::new([]);
    }
    let mut candidates: Box<[Option<usize>]> = (2..=max).map(Some).collect();
    (2..=(max as f32).sqrt() as usize).for_each(|i| {
        let pow = i.pow(2);
        (0..=(max - pow).div_euclid(i)).for_each(|j| {
            candidates[pow + i * j - 2] = None;
        })
    });
    return candidates.iter().cloned().flatten().collect();
}
