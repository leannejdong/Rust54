//use basis::borrowing::destroy;
// use basis::borrowing::references;
// use basis::borrowing::deferencing;
// use basis::borrowing::statics;
// use basis::borrowing::usize_example;
// use basis::borrowing::get_first_element;
// use basis::borrowing::fnStr;
use basis::borrowing::*;
// use basis::struc::User;
// use basis::struc::build_user;
// use basis::struc::WavetableOscillator;
use basis::struc::*;
// use basis::struc::WavetableOscillator::init;
// use basis::struc::WavetableOscillator::printer;
use basis::ptr::ptrs;
use basis::generic::sum_of_numbers;
use basis::generic::mul_numbers;
use basis::traits::Animal;
use basis::traits::Cat;
use basis::err::error_handling_example;

use std::collections::HashMap;
#[macro_export]
macro_rules! make_map {
    ($k:expr, $v:expr) => {
      {
        println!("key:{}", $k);
        println!("Value:{}", $v);
        let mut map = HashMap::new();
        map.insert($k, $v);
        map
      }
        
    };
}

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

    ptrs();
    //Unlimited amount of immutable borrows at any given time.

    // One mutable borrow at any given time.

    // Cannot mix mutable borrows and immutable borows at the same time

    let mut s11: String = String::from("Hello");
  //  let s22 = &s11;
    s11.push_str("Scammer!");
    println!("{:?}", s11);
    let s22 = &s11;
    println!("{:?}", s22);

    let mut user1 = User {
      email: String::from("1@example.com"),
      username: String::from("1username123"),
      active: true,
      sign_in_count: 1,
  };
  
  user1.email = String::from("anotheremail@example.com");

  println!("{:?}", user1);
  
  let user2 = User {
    email: String::from("2@example.com"),
    username: String::from("2username123"),
    active: true,
    sign_in_count: 1,
  };
  
  println!("{:?}", user2);

  
  let user3 = User{
    email: String::from("3@example.com"),
    ..user2
  };

  println!("{:?}", user3);

    /*It's more idiomatic to use to_owned() or into() (in case the target type is known) rather than to_string(). The latter works through Display trait which invokes 
    formatting code which introduces some overhead */
  let user4 = build_user("leanne@54.com".to_string(), "leanne".to_owned());


  println!("{:?}", user4);


  // create oscillator
  //let wave_table = vec![0.8,2.2,3.3,0.33];
  let wave_table_size = 64;
  let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);
  for n in 0..wave_table_size{
    wave_table.push((2.0*std::f32::consts::PI*n as f32/wave_table_size as f32).sin());
  }
  let mut oscillator = WavetableOscillator::init(44100, wave_table);
  oscillator.set_freq(440.0);
  oscillator.printer();

  //generics

  print!("{:?}\n", sum_of_numbers(1, 2));

  print!("{:?}\n", mul_numbers(1, 2));

  
  // traits

  let mut miu: Cat = Animal::new("Miu");
  miu.talk();
  miu.status();

  // error handling

  error_handling_example("src");
  error_handling_example("lib");

  // macro
  let int_map:HashMap<i32, i32> = make_map!(1, 3);
  println!("{:#?}", int_map);
  let str_map: HashMap<&str, &str> = make_map!("green", "go");
  println!("{:#?}", str_map);



}
