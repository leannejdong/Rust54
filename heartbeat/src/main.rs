#[derive(Debug, Clone, Copy)]

struct SomeGarbage{
    num: i32
}

fn print_garbage(the_garbage: &SomeGarbage){
    println!("{:?}", the_garbage);
}

fn mutate_garbage(the_garbage: &mut SomeGarbage){
    the_garbage.num = 5;

}

fn main() {
    let mut some_garbage = SomeGarbage{num: 3};
    print_garbage(&some_garbage/* .clone()*/);
    mutate_garbage(&mut some_garbage);
    print_garbage(&some_garbage);


}
