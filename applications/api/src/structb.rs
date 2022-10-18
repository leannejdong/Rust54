extern crate serde;
//use serde::{Serialize, Deserialize};
extern crate reqwest;
use reqwest::blocking::Client;

use crate::price::*;

fn print_cprice(cprice:CoinPrice){
    println!("SPOT: {base}-{currency}: {amount}",
    base = cprice.base,
    currency = cprice.currency,
    amount = cprice.amount)
}

pub fn rust_struct_b() {
    let spot_url = format!("https://api.coinbase.com/v2/prices/{currency}-{rates}/spot",
        currency = "USDT",
        rates = "USD");

    let client = Client::new();
    let resp_spot_price = client.get(&spot_url)
        .send();
    
    match resp_spot_price {
        Ok(parsed_spot_price) => {
            let coinprice = parsed_spot_price.json::<CoinbasePrice>().unwrap();
            
            let spot_price = CoinPrice {
                base: coinprice.data.base,
                currency: coinprice.data.currency,
                amount: coinprice.data.amount
            };
            print_cprice(spot_price);
        }
        Err(e) => println!("Err: {:?}", e),
    }    
}