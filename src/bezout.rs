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

pub(super) fn bezout(a: usize, b: usize) -> (isize, usize) {
    let v = bezout_r(a, b);
    (uv(b, a, v), v as usize)
}

pub(super) fn mod_mul_inv(n: usize, m: usize) -> usize {
    uv(m, n, bezout_r(n, m)) as usize % m
}
