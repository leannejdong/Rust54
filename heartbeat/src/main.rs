#[derive(Debug, Clone, Copy)]
// dangling ref eg
struct SomeGarbage{
    num:  i32,
}
#[derive(Debug, Clone, Copy)]
struct SomeCat<'a>{
    num:  &'a i32,
}

fn print_garbage(the_garbage: &SomeGarbage){
    println!("{:?}", the_garbage);
}

fn print_cat(the_cat: &SomeCat){
    println!("{:?}", the_cat);
}


fn mutate_garbage(the_garbage: &mut SomeGarbage){
    the_garbage.num = 5;

}

fn biggest<'a>(a: &'a SomeGarbage, b: &'a SomeGarbage)->&'a SomeGarbage{
    if a.num > b.num{
        a
    } else {
        b
    }
}

fn main() {
    let mut some_garbage = SomeGarbage{num: 3};
    print_garbage(&some_garbage/* .clone()*/);
    mutate_garbage(&mut some_garbage);
    print_garbage(&some_garbage);

    let bigger: &SomeGarbage;
    let other_garbage: SomeGarbage = SomeGarbage { num: 5 };
    bigger = biggest(&some_garbage, &other_garbage);
    print_garbage(&bigger);

    let some_cat: SomeCat;


    let num = &3;
    some_cat = SomeCat{num};
    print_cat(&some_cat);



}
