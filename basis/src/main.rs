//use basis::borrowing::destroy;
use basis::borrowing::references;
use basis::borrowing::deferencing;
use basis::borrowing::statics;
use basis::borrowing::usize_example;
use basis::borrowing::get_first_element;
use basis::borrowing::fnStr;


fn main(){
    //ref();
    let a = vec![1, 2, 3];
    let b = get_first_element(&a);
    println!("{:?} {}", a, b); // [1, 2, 3] 1
    references();
    statics();
    deferencing();
    usize_example();

    let s1: String = String::from("Dear programmers, be nice! Don't quit uni!");
    println!("{:?}", s1);
    fnStr(&s1);
    println!("{:?}", s1);// after fn call, memory got deallocated, so get a error if u use it again. 


    //Unlimited amount of immutable borrows at any given time.

    // One mutable borrow at any given time.

    // Cannot mix mutable borrows and immutable borows at the same time

    let mut s11: String = String::from("Hello");
  //  let s22 = &s11;
    s11.push_str("Scammer!");
    println!("{:?}", s11);
    let s22 = &s11;
    println!("{:?}", s22);



}
