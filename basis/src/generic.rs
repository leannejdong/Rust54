use core::num;
use std::process::Output;

use std::ops::Add;
use std::ops::Mul;

pub fn sum_of_numbers<T: Add<Output=T> /*+ Copy*/>(num1: T, num2: T)->T
{
    num1 + num2
}

pub fn mul_numbers<T: Mul<Output = T>>(num1: T, num2: T)->T
{
    num1*num2
}