fn bezout_r(a: usize, b: usize) -> isize {
    match a % b {
        0 => 1,
        r => uv(a, b, bezout_r(b, r)),
    }
}

// right way : u to v. wrong way : v to u.
fn uv(a: usize, b: usize, x: isize) -> isize {
    (1 - x * a as isize).div_euclid(b as isize)
}

pub(super) fn bezout(a: usize, b: usize) -> (isize, usize) {
    let v = bezout_r(a, b);
    (uv(b, a, v), v as usize)
}
