use std::collections::HashMap;

pub fn starting_new_hash_map() {
    let mut new_hash_map:HashMap<String, u32> = HashMap::new();
    new_hash_map.insert(String::from("a"), 324);
    new_hash_map.insert(String::from("b"), 66);

    println!("{:?}", new_hash_map);
}
// reading and inserting elements
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Gender {
    MALE,
    FEMALE,
}

#[derive(Debug)]
pub struct Friend {
    name: String,
    age: u32,
    gender: Gender,
}

impl Friend{
    pub fn new(name: String, age: u32, gender: Gender) -> Friend{
        Friend {name, age, gender}
    }

    pub fn separate(friends: Vec<Friend>)->HashMap<Gender, Vec<Friend>>{
        let mut report: HashMap<Gender, Vec<Friend>> = HashMap::new();
        report.insert(Gender::FEMALE, Vec::new());
        report.insert(Gender::MALE, Vec::new());
        for friend in friends {
            report
                .get_mut(&friend.gender)
                .map(|group| group.push(friend));
        }//closure is like lambda
        report
    }

    // update value
    fn initialize_gender(gender: Gender, map: &mut HashMap<Gender, Vec<Friend>>) {
        map.entry(gender).or_insert(Vec::new());
    }

    pub fn add_friend(friend: Friend, map: &mut HashMap<Gender, Vec<Friend>>) {
        let friends = map.entry(friend.gender.clone()).or_insert(Vec::new());
        friends.push(friend);
    }
}

pub fn try_to_modify() {
    let mut change: HashMap<Gender, Vec<Friend>> = HashMap::new();
    Friend::initialize_gender(Gender::FEMALE, &mut change);
    change.get_mut(&Gender::FEMALE).map(|g| g.push(Friend::new("Amy".to_string(), 33, Gender::FEMALE)));
    println!("{:?}", change);
    Friend::initialize_gender(Gender::FEMALE, &mut change);
   // println!("{:?}", change);
}



fn main()
{
    println!("Pushing Friends into a map");
    let mut map = HashMap::new();
    Friend::add_friend(Friend::new("Doroti".to_string(), 16, Gender::FEMALE), &mut map);
    println!("{:?}", map);
    try_to_modify();
    println!("--------------");
}
