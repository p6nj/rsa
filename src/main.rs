use std::num::NonZeroUsize;

fn main() {
    println!("{:?}", primes(NonZeroUsize::new(300).unwrap()));
}

fn primes(max: NonZeroUsize) -> Vec<usize> {
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
