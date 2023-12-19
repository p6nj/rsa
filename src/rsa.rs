#[cfg(test)]
mod tests;

fn rsaenc(n: usize, e: usize, message: usize) -> usize {
    message.pow(e.try_into().unwrap()) % n
}

fn rsadec(n: usize, d: usize, message: usize) -> usize {
    message.pow(d.try_into().unwrap()) % n
}
