use chrono::prelude::*;
use chrono::offset::LocalResult;

use std::time::{Duration, SystemTime};
use std::str::FromStr;
use iso_8601::*;

// fn ran(){
//     let dt = Utc.ymd(2014, 7, 8).and_hms(9, 10, 11); // `2014-07-08T09:10:11Z`
// }
// pub struct Time {
//     pub base: DateTime<Utc>,
//     //pub iso: YMD(Da),
//     pub epoch: SystemTime::UNIX_EPOCH,
// }

// //#[derive(Serialize, Deserialize, Debug)]
// pub struct CoinbaseTime {
//     pub data: Time
// }