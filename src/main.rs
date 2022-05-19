mod api;
mod database;
mod trade;

use std::{env, thread, time};
use chrono;
use dotenvy::dotenv;
use rusqlite::Result;
// use crate::database::database::{insert};
use crate::api::api::{get_values};
use crate::trade::trade::{init, should_we_buy, should_we_sell};


fn main() -> Result<(), std::fmt::Error> {
    dotenv().ok();
    //env::set_var("RUST_BACKTRACE", "full"); // HEY Comment this after debug

    let refresh_rate: u64 = match env::var("refresh_rate") {
        Ok(s) => match s.parse::<u64>() {
            Ok(is_number) => is_number,
            Err(_) => panic!("Error : refresh_rate should be an integer.")
        },
        Err(_) => panic!("Environment variable refresh_rate not found.")
    };
    let delay = time::Duration::from_secs(refresh_rate);

    init();

    loop{
        let time = chrono::offset::Local::now();
        println!("{}", time.format("%F %T"));
        println!("sleeping for {}  sec", refresh_rate);

        get_values();
        should_we_buy();
        should_we_sell();

        thread::sleep(delay);
    }


    // Ok(())
}