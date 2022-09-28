//! byte arrays of 1 to 32 elements

use rand::Rng;

fn main() {
    let random_bytes = rand::thread_rng().gen::<[u8; 32]>();
    println!("{:?}", random_bytes);
}