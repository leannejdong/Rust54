macro_rules! add {
    ($a:expr,$b:expr) => {
        $a+$b
    };

    ($a:expr)=>{
        {
            $a
        }
    }
}


macro_rules! add_as {
    ($a:expr, $b:expr, $typ:ty) => {
        $a as $typ + $b as $typ
    };
}
fn main() {
    let x = 0;
    let y = add!(1,2);
    add!(x);
    println!("{}", y);
    println!("{}", add_as!(0,2,u8));
}
