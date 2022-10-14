pub mod borrowing;
pub mod struc;
pub mod ptr;
pub mod generic;
pub mod traits;
pub mod err;

pub fn hey_ya(){
    println!("Hey, you!");
    let x = vec![1, 2];
    let x = vec!['1', '2'];
}

#[macro_export]
macro_rules! vec{
    ($($x:expr ),*)=>{
        {
            // create a temporary empty vector
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec

        }
    };
}

// {

//     let mut temp_vec = Vec::new();
//     temp_vec.push(1);
//     temp_vec.push(2);
//     temp_vec
// }


