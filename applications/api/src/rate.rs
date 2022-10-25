use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FXrate {
    pub base: String,
    pub currency: String,
    pub rate: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinbaseRate {
    pub data: FXrate
}