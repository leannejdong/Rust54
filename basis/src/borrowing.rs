// references
pub fn destroy(val: String){
    println!("{:?}", val);
}

pub fn references(){
    let mut original_value: String = String::from("Cat");

    let x = &original_value;

    original_value = String::from("Me");

    let x = &original_value;

    println!("{:?}", x);

    destroy(original_value);
    //println!("{:?}", x);
}

// Dereferencing
pub fn deferencing() {

    let a = 1;
    let b = &a;

    assert_eq!(1, a);
    assert_eq!(1, *b); // *b = a
}

// Static
pub fn statics() {
    let x: &'static str = "Dave";
    let y = &x;
    destroy(x.to_string()); // x is not destroyed here
    println!("{}", y);
}

// usize
pub fn usize_example() {
    let x: usize = 2; // Depends on OS architecture (x32, x64)
    println!("{:?}", x);
}

pub fn get_first_element(a: &Vec<i32>) -> i32 {
    a[0]
}