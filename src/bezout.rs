#[cfg(test)]
mod tests;

fn bezout_r(a: usize, b: usize) -> isize {
    match a % b {
        0 => 1,
        r => uv(a, b, bezout_r(b, r)),
    }
}

// right way : u to v. wrong way : v to u.
fn uv(a: usize, b: usize, u: isize) -> isize {
    (1 - u * a as isize).div_euclid(b as isize)
}

#[cfg(test)]
fn vu(a: usize, b: usize, v: isize) -> isize {
    (1 - v * b as isize) / a as isize
}

#[cfg(test)]
pub(super) fn bezout(a: usize, b: usize) -> (isize, isize) {
    let v = bezout_r(a, b);
    (vu(a, b, v), v)
}

pub(super) fn mod_mul_inv(n: usize, m: usize) -> usize {
    uv(m, n, bezout_r(n, m)) as usize
}
