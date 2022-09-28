//! generate more than 32 random bytes
use rand::Rng;

fn main() {
    let random_bytes: Vec<u8> = (0..1024).map(|_| { rand::random::<u8>() }).collect();
    println!("{:?}", random_bytes);
}