pub mod trade {
    use std::env;
    use std::time::SystemTime;
    use chrono::{Duration, Utc};
    use serde::Deserialize;
    use reqwest::header::{HeaderMap, HeaderValue};
    use sha2::Sha256;
    use hmac::{Hmac, Mac};
    use hex;

    use crate::database::database;
    use crate::api::api::get_cryptos;

    type HmacSha256 = Hmac<Sha256>;

    #[derive(Deserialize, Debug)]
    pub struct Response {
        pub data: Data
    }

    #[derive(Deserialize, Debug)]
    pub struct Data {
        pub id: String,
        pub name: String,
        //pub username: String,
        pub email: String,
        pub referral_money: ReferralMoney,
    }

    #[derive(Deserialize, Debug)]
    pub struct ReferralMoney {
        pub amount: String,
        pub currency: String,
        pub referral_threshold: String,
    }

    pub fn connect() {
        let now = SystemTime::now();

        let timestamp:u64 = now.duration_since(SystemTime::UNIX_EPOCH)
            .expect("TimeStamp needed").as_secs();

        let access_sign: String = get_access_sign(timestamp, "GET", "/v2/user");

        println!("Hash : {}", access_sign);

        fn construct_headers(timestamp: u64) -> HeaderMap {
            let stamp = HeaderValue::try_from(timestamp.to_string()).expect("Header error");
            let access_sign = HeaderValue::try_from(get_access_sign(timestamp, "GET", "/v2/user")).expect("Header error");
            let api_key = env::var("api_key").expect("Check your api_key in .env file");

            let mut headers = HeaderMap::new();
            headers.insert("CB-ACCESS-KEY", HeaderValue::from_str(&api_key).expect("Hey"));
            headers.insert("CB-ACCESS-SIGN", access_sign);
            headers.insert("CB-ACCESS-TIMESTAMP", stamp);
            headers
        }

        let file = "";
        let client = reqwest::blocking::Client::new();
        let res = client.get("https://api.coinbase.com/v2/user")
            .headers(construct_headers(timestamp))
            .body(file);

        let response = match res.send() {
            Ok(r) => println!("{:?}", r.json::<Response>()),
            Err(_) => panic!("Json error")
        };

        println!("{:?}", response);
    }

    fn get_access_sign(timestamp: u64, method: &str, path: &str) -> String {
        let access_sign: String = timestamp.to_string() + &method.to_owned() + &path.to_owned() + &"".to_owned();
        println!("Prehash : {}", access_sign);

        let client_secret = env::var("client_secret").expect("Check your client_secret in .env file");

        let mut mac = HmacSha256::new_from_slice(client_secret.as_bytes())
            .expect("HMAC can take key of any size");
            mac.update(access_sign.as_bytes());

        let result = mac.finalize();
        let hex_result = result.into_bytes();
        let hex_string: String = hex::encode(hex_result);

        return hex_string;
    }

    pub fn should_we_buy() {

        let cryptos = get_cryptos();
        for crypto in cryptos {

            let percent = env::var("percentage").expect("Check your api_key in .env file");
            let percentage: f64 = percent.parse::<f64>().unwrap();
            let check_period = env::var("check_period").expect("Check your api_key in .env file");
            let dt = Utc::now() +- Duration::days(check_period.parse::<i64>().expect("check_period must be an integer"));

            let timestamp = database::last_sell_prices(&dt.format("%F").to_string(), &crypto);
            let actual_price = database::get_last_sell_price(&crypto);

            for value in timestamp {
                let compare_price = ((percentage / 100.0)) * value.value;

                // If actual price is x% less than the prices we have stored these last n number of days
                if actual_price.value < (compare_price as f64) {
                    println!("{:?}", value);
                    println!("===================");
                    println!("Database price : {:?}", value.datetime);
                    println!("Database price : {:?}", value.value);
                    println!("Compare price : {:?}", compare_price);

                    // We should Buy
                }
            }

            println!("Actual price : {:?}", actual_price);
        }
    }
}
