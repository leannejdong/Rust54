use std::cell::RefCell;


pub fn ptrs() {
    let a = 2;
    let b = RefCell::new(a);

   // let a_mut = b.borrow_mut(); //error as we already borrow in line8
    let a_immut = b.borrow();

    // [Warn!] this will make our program crash at runtime
   //  let another_mutable=b.borrow_mut();

    // OK
    let another_immut = b.borrow();

    println!("Happy Labor Day!\n");
}