
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

use mockall::predicate::*;
use mockall::*;

#[automock]
pub trait Synth{
    fn compute_num(&self, question: i32)->i32;
}

pub fn get_num(synth: &impl Synth, question: i32)-> String{
    format!("The answer is {}", synth.compute_num(question))
}

// impl Synth{
//     fn compute_num(&self, question: i32)->i32{
//         40
//     }
// }

fn compute_ans(question:i32)->i32{
    39
}

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Hash)]
struct Person {
    id: u32,
    name: String,
    phone: u64,
}



//assert!(calculate_hash(&person1) != calculate_hash(&person2));

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    fn ans(){
        assert_eq!(compute_ans(3), 39);
    }

    fn ans1(){
        let mut mock_synth = MockSynth::new();
        mock_synth.expect_compute_num()
            .times(2)
            .return_const(40);

        let ans =get_num(
            &mock_synth,
            2,
        );


    }
    #[test]
    #[should_panic]
    fn test_hash(){
        let person1 = Person {
            id: 5,
            name: "Janet".to_string(),
            phone: 555_666_7777,
            };
        let person2 = Person {
            id: 5,
            name: "Bob".to_string(),
            phone: 555_666_7777,
        };
        assert_eq!(calculate_hash(&person1), calculate_hash(&person2));
    }
}


// cargo test
// cargo test hash