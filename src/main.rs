mod bezout;
mod phi;
mod prime;
mod testing;

use bezout::{bezout, mod_mul_inv};
use phi::phi;

fn main() {
    println!("{:?}", mod_mul_inv(51, 242));
}
