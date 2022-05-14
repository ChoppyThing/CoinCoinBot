pub mod trade {
    use std::env;
    use std::time::SystemTime;
    use serde::Deserialize;
    use reqwest::header::{HeaderMap, HeaderValue};
    use sha2::Sha256;
    use hmac::{Hmac, Mac};
    use hex;

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

        let timestamp:u64;
        match now.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => {
                timestamp = n.as_secs();
                println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs())
            }
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }

        let access_sign: String = get_access_sign(timestamp, "GET", "/v2/user");

        println!("Hash : {}", access_sign);

        fn construct_headers(timestamp: u64) -> HeaderMap {
            let stamp = match HeaderValue::try_from(timestamp.to_string()) {
                Ok(test) => test,
                Err(_) => panic!("SystemTime before UNIX EPOCH!")
            };

            let access_sign = match HeaderValue::try_from(get_access_sign(timestamp, "GET", "/v2/user")) {
                Ok(test) => test,
                Err(_) => panic!("SystemTime before UNIX EPOCH!")
            };

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
}
