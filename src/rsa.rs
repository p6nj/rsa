#[cfg(test)]
mod tests;

pub(crate) fn rsaenc(n: usize, e: usize, message: usize) -> usize {
    mod_exp(message, e, n) as usize
}

pub(crate) fn rsadec(n: usize, d: usize, message: usize) -> usize {
    mod_exp(message, d, n) as usize
}

fn mod_exp(n: usize, p: usize, m: usize) -> isize {
    match p {
        0 => 1,
        _ => match p % 2 {
            0 => mod_exp(n, p.div_euclid(2), m).pow(2) % m as isize,
            1 => (n as isize * mod_exp(n, p-1, m)) % m as isize,
            _ => unreachable!(),
        },
    }
}
