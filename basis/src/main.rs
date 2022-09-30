use basis::borrowing::destroy;
use basis::borrowing::references;
use basis::borrowing::deferencing;
use basis::borrowing::statics;
use basis::borrowing::usize_example;
use basis::borrowing::get_first_element;



fn main(){
    //ref();
    let a = vec![1, 2, 3];
    let b = get_first_element(&a);
    println!("{:?} {}", a, b); // [1, 2, 3] 1
    references();
    statics();
    deferencing();
    usize_example();

}
