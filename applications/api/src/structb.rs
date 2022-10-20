extern crate serde;
//use serde::{Serialize, Deserialize};
extern crate reqwest;
use reqwest::blocking::Client;

use crate::price::*;

impl CoinPrice{
    fn print_cprice(self){
        println!("SPOT: {base}-{currency}: {amount}",
        base = self.base,
        currency = self.currency,
        amount = self.amount)
    }
}

// fn print_cprice(cprice:CoinPrice){
//     println!("SPOT: {base}-{currency}: {amount}",
//     base = cprice.base,
//     currency = cprice.currency,
//     amount = cprice.amount)
// }

pub trait Price{
    fn format_cprice(&self)-> String;
    fn return_cprice(&self)-> String;
}

impl Price for CoinPrice{
    fn format_cprice(&self)-> String{
        return format!("SPOT: {base}-{currency}: {amount}",
            base = self.base,
            currency = self.currency,
            amount = self.amount
        );
    }

    fn return_cprice(&self)-> String {
        return format!("SPOT: {amount}",
                amount = self.amount);
    }
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
            //spot_price.print_cprice();
            println!("Format price: {}", spot_price.format_cprice());
            println!("Return price {}", spot_price.return_cprice());

        }
        Err(e) => println!("Err: {:?}", e),
    }    
}