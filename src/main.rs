mod api;
mod database;
mod trade;

use std::{thread, time};
use std::env;
use chrono;
use dotenvy::dotenv;
use rusqlite::Result;
use crate::database::database::{insert};
use crate::api::api::{get_values};

fn main() -> Result<(), std::fmt::Error> {
    dotenv().ok();

    let refresh_rate: u64 = match env::var("refresh_rate") {
        Ok(s) => match s.parse::<u64>() {
            Ok(is_number) => is_number,
            Err(_) => panic!("Error : refresh_rate should be an integer.")
        },
        Err(_) => panic!("Environment variable refresh_rate not found.")
    };
    let delay = time::Duration::from_secs(refresh_rate);

    get_values();

    loop{
        let _database: Result<()> = insert();

        let time = chrono::offset::Local::now();
        println!("{}", time.format("%F %T"));
        println!("sleeping for {}  sec", refresh_rate);

        thread::sleep(delay);
    }


    // Ok(())
}