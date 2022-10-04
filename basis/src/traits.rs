pub trait Animal {
    fn new(name: &'static str)->Self;

    fn name(&self) -> &'static str;
    fn noise(&self)-> &'static str;

    fn talk(&self){
        println!("{} says {}", self.name(), self.noise());
    }

}

pub struct Cat{ fed: bool, name: &'static str}

impl Cat {

    fn is_fed(&self)->bool{
        self.fed
    }
    pub fn status(&mut self){
        if self.is_fed(){
            println!("{} is already fed..", self.name());
        } else {
            println!("{} go sleep!", self.name);
            self.fed = true;
        }
    }
    
}

// Implement Animal trait for cat

impl Animal for Cat {
    fn new(name: &'static str)-> Cat{
        Cat { name:name, fed: false }
    }

    fn name(&self)-> &'static str{
        self.name
    }

    fn noise(&self) -> &'static str{
        if self.is_fed(){
            "hahhh?"
        } else {
            "meo!"
        }
    }

    fn talk(&self){
        println!("{} pauses briefly...{}", self.name, self.noise());
    }
}

    
