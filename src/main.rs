mod bezout;
mod phi;
mod prime;
mod testing;

use bezout::bezout;
use phi::phi;

fn main() {
    println!("{:?}", bezout(26, 15));
}
