#![allow(unused)]
#![allow(unused_imports)]
fn main() {
    use std::io::prelude::*;
    let a = String::from("hello");
    let chars: Vec<_> = a.chars().collect();
    for ch in a.chars().collect::<Vec<char>>() {
        println!("{}", ch);
    }
    for ch in a.chars() {
        println!("{}", ch);
    }
    for (i,ch) in a.chars().enumerate() {
        println!("{} {}", i, ch);
    }
}
