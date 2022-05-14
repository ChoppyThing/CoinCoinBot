mod api;
mod database;
mod trade;

use std::{thread, time};
use std::env;
use chrono;
use dotenvy::dotenv;
use rusqlite::Result;
// use crate::database::database::{insert};
use crate::api::api::{get_values};
use crate::trade::trade::should_we_buy;

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


    loop{
        //let _database: Result<()> = insert();

        let time = chrono::offset::Local::now();
        println!("{}", time.format("%F %T"));
        println!("sleeping for {}  sec", refresh_rate);

        get_values();
        should_we_buy();

        thread::sleep(delay);
    }


    // Ok(())
}