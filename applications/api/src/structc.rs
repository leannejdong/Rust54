extern crate serde;
//use serde::{Serialize, Deserialize};
extern crate reqwest;
use reqwest::blocking::Client;

use crate::rate::*;

fn print_cprice(cRate:FXrate){
    println!("SPOT:{base}--{currency}: {amount}",
    base = cRate.base,
    currency = cRate.currency,
    amount = cRate.amount);
}

pub fn rust_struct_c() {
    let spot_url = format!("https://api.coinbase.com/v2/exchange-rates?currency=BTC",
        currency = "BTC",
        rates = "USD");

    let client = Client::new();
    let resp_spot_rate = client.get(&spot_url)
        .send();
    
    match resp_spot_rate {
        Ok(parsed_spot_rate) => {
            let coinrate = parsed_spot_rate.json::<CoinbaseRate>().unwrap();
            
            let spot_rate = CoinbaseRate {
                base: coinrate.data.base,
                currency: coinrate.data.currency,
                amount: coinrate.data.amount
            };
            println!("SPOT: {base}-{rate}: {amount}",
                base=spot_rate.base,
                currency=spot_rate.currency,
                amount=spot_rate.amount);
        }
        Err(e) => println!("Err: {:?}", e),
    }    
}