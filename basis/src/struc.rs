#[derive(Debug)]


pub struct User {
    pub active: bool,
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
}

pub fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}


pub struct WavetableOscillator{
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
}

impl WavetableOscillator{
    pub fn init(sample_rate: u32, wave_table: Vec<f32>)->WavetableOscillator{
        return WavetableOscillator{
            sample_rate: sample_rate,
            wave_table: wave_table,
            index: 0.0,
            index_increment: 0.0,
        };
    }
}
impl WavetableOscillator{
    pub fn printer(&self){
        println!("{:?}", self.sample_rate);
        println!("{:?}", self.wave_table);
        println!("{:?}", self.index);
        println!("{:?}", self.index_increment);
    }
}

impl WavetableOscillator{
    pub fn set_freq(&mut self, frequency: f32){
        self.index_increment = frequency*self.wave_table.len() as f32/self.sample_rate as f32
    }
}




