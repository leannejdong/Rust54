extern crate serde;
//use serde::{Serialize, Deserialize};
extern crate reqwest;
use reqwest::blocking::Client;

use crate::time::*;

fn print_ctime(cTime:Time){
    println!("SPOT:{base}--{iso}: {epoch}",
    base = cTime.base,
    iso = cTime.iso,
    epoch = cRate.epoch);
}

pub fn rust_struct_c() {
    let spot_url = format!("https://api.coinbase.com/v2/time",
        iso = Utc::now(),
        epoch = SystemTime::now());

    let client = Client::new();
    let resp_spot_time = client.get(&spot_url)
        .send();
    
    match resp_spot_rate {
        Ok(parsed_spot_rate) => {
            let cointime = parsed_spot_rate.json::<CoinbaseTime>().unwrap();
            
            let spot_rate = CoinbaseTime {
                base: cointime.data.base,
                iso: cointime.data.iso,
                epoch: cointime.data.epoch
            };
            println!("SPOT: {base}-{iso}: {epoch}",
                base=spot_rate.base,
                iso=spot_rate.iso,
                epoch=spot_rate.epoch);
        }
        Err(e) => println!("Err: {:?}", e),
    }    
}