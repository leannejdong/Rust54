use std::process::Output;

use std::ops::Add;
use std::ops::Mul;

//enum Option<T>{
//    Some(T),
//    None,
//}

fn opt_eg(x: i32)->Option<String>{
    match x > 2{
        true => Some(String::from("result")),
        false => None,
    }
}

enum Result<T, E>{
    Ok(T),
    Err(E),
}
pub fn sum_of_numbers<T: Add<Output=T> /*+ Copy*/>(num1: T, num2: T)->T
{
    num1 + num2
}

pub fn mul_numbers<T: Mul<Output = T>>(num1: T, num2: T)->T
{
    num1*num2
}

fn main()
{
   // let x: Option<i32> = 3;

    opt_eg(3);
}
