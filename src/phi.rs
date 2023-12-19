use crate::prime::primes;
#[cfg(test)]
mod tests;

fn div_exhaust_r(n: usize, p: usize, pow: usize) -> (usize, usize) {
    match n % p {
        0 => div_exhaust_r(n.div_euclid(p), p, pow + 1),
        _ => (n, pow),
    }
}

fn div_exhaust(n: usize, p: usize) -> (usize, usize) {
    div_exhaust_r(n, p, 0)
}

fn decomp_r<'a>(
    n: &'a usize,
    ps: &'a [usize],
    acc: Box<[(usize, usize)]>,
) -> Box<[(usize, usize)]> {
    match *n == 1 || ps.is_empty() {
        true => acc,
        false => {
            let (p, ps) = ps.split_first().unwrap();
            let (n, pow) = div_exhaust(*n, *p);
            decomp_r(
                &n,
                ps,
                match pow {
                    0 => acc,
                    _ => [acc, Box::new([(*p, pow)])].concat().into(),
                },
            )
        }
    }
}

fn decomp(n: usize) -> Box<[(usize, usize)]> {
    if n < 2 {
        return Box::new([]);
    }
    decomp_r(
        &n,
        &[primes(n.div_euclid(2)), Box::new([n])].concat(),
        Box::new([]),
    )
}

pub(super) fn phi(n: usize) -> usize {
    if n < 2 {
        return n;
    }
    decomp(n)
        .iter()
        .map(|(p, k)| p.pow((k - 1).try_into().unwrap()) * (p - 1))
        .reduce(|acc, e| acc * e)
        .unwrap()
}
