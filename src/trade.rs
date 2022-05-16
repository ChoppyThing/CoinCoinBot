pub mod trade {
    use std::env;
    use std::time::SystemTime;
    use chrono::{Duration, Utc};
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
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

    #[derive(Serialize, Deserialize, Debug)]
    struct BuyParameters {
        amount: f64,
        currency: String,
    }

    fn connect(method: &str, path: &str, body: &str) -> reqwest::blocking::RequestBuilder {
        let now = SystemTime::now();

        let timestamp:u64 = now.duration_since(SystemTime::UNIX_EPOCH)
            .expect("TimeStamp needed").as_secs();

        let client = reqwest::blocking::Client::new();
        let url = format!("https://api.coinbase.com{}", path);

        if method == "GET" {
            client.get(url)
                .headers(construct_headers(timestamp, method, path, ""))
        } else {
            client.post(url)
                .headers(construct_headers(timestamp, method, path, body))
        }
    }

    /**
     * Inits the program to see if coinbase answers correctly to the basic api call
     */
    pub fn init () {
        let res = connect("GET", "/v2/user", "");

        let response = match res.send() {
            Ok(r) => r.json::<Response>(),
            Err(_) => panic!("Json error")
        };

        let user = response.expect("test").data;

        println!("\n\n");
        println!("=======================================================================");
        println!("Welcome {}", user.name);
        println!("ID : {}", user.id);
        println!("=======================================================================");
    }

    fn buy (currency: &str) {
        let buy_parameter: BuyParameters = BuyParameters {
            amount: 10.00,
            currency: currency.to_string(),
        };
        let buy_parameter_json = serde_json::to_string(&buy_parameter).expect("Expected Json as String");

        println!("{:?}", buy_parameter_json);

        //let id: String = get_account_id();
        let url: String = format!("/v2/accounts/{}/buys", "97cee244-d348-5001-81f1-87b98090fb76");

        let res = connect("POST", &url, &buy_parameter_json);

        let response = match res.body(buy_parameter_json).send() {
            Ok(r) => r.text(),//::<Response>(),
            Err(_) => panic!("Json error")
        };

        println!("Welcome {:?}", response);
    }

    pub fn should_we_buy() {

        let cryptos = get_cryptos();
        for crypto in cryptos {
            println!("\n\n");
            println!("=======================================================================");
            println!("                          Crypto - {}                                  ", crypto);
            println!("=======================================================================");
            println!("\n");

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
                    println!("=======================================================================");
                    println!("Lowest actual price");
                    println!("{:?}", value);
                    println!("Database price : {:?}", value.datetime);
                    println!("Database price : {:?}", value.value);
                    println!("Compare price : {:?}", compare_price);

                    // We should Buy
                    buy(&crypto);
                    break;
                }
            }

            println!("Actual price : {:?}", actual_price);
        }
    }

    /**
     * Generates a token based on coinbase requirements
     * See : https://developers.coinbase.com/docs/wallet/api-key-authentication
     */
    fn get_access_sign(timestamp: u64, method: &str, path: &str, body: &str) -> String {
        let access_sign: String = timestamp.to_string() + &method.to_owned() + &path.to_owned() + &body.to_owned();

        let client_secret = env::var("client_secret").expect("Check your client_secret in .env file");

        let mut mac = HmacSha256::new_from_slice(client_secret.as_bytes())
            .expect("HMAC can take key of any size");
            mac.update(access_sign.as_bytes());

        let result = mac.finalize();
        let hex_result = result.into_bytes();
        let hex_string: String = hex::encode(hex_result);

        return hex_string;
    }

    /**
     * Constructs header needed by coinbase to auth
     */
    fn construct_headers(timestamp: u64, method: &str, path: &str, body: &str) -> HeaderMap {
        let stamp = HeaderValue::try_from(timestamp.to_string())
            .expect("Header error");
        let access_sign = HeaderValue::try_from(get_access_sign(timestamp, method, path, body))
            .expect("Header error");
        let api_key = env::var("api_key").expect("Check your api_key in .env file");

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("CB-ACCESS-KEY", HeaderValue::from_str(&api_key).expect("Access key invalid"));
        headers.insert("CB-ACCESS-SIGN", access_sign);
        headers.insert("CB-ACCESS-TIMESTAMP", stamp);
        headers.insert("CB-VERSION", HeaderValue::from_str("2022-05-10").expect("Wrong API version"));
        headers
    }

    pub fn get_id () -> String {
        let res = connect("GET", "/v2/user", "");

        let response = match res.send() {
            Ok(r) => r.json::<Response>(),
            Err(_) => panic!("Json error")
        };
println!("{:?}", response);
        response.expect("test").data.id
    }

    pub fn get_account_id () -> String {
        #[derive(Deserialize, Debug)]
        struct Account {
            id: String,
            name: String,
            balance: AccountBalance,
        }
        #[derive(Deserialize, Debug)]
        struct AccountBalance {
            amount: String,
            currency: String,
        }
        #[derive(Deserialize, Debug)]
        struct PaginationAccount {
            next_uri: String,
        }
        #[derive(Deserialize, Debug)]
        struct DataAccount {
            data: Vec<Account>,
            pagination: PaginationAccount
        }

        let res = connect("GET", "/v2/accounts", "");

        let mut response = match res.send() {
            Ok(r) => r.json::<DataAccount>(),
            Err(_) => panic!("Json error")
        };

        println!("{:?}", &response);

        for mut i in 0..10 {
            let next:&str = match &response.as_ref() {
                Ok(n) => &n.pagination.next_uri,
                Err(_) => continue,
            };

            // let next: &str = &response.as_ref().expect("Test 1").pagination.next_uri;
            println!("{:?}", next);
            let res = connect("GET", next, "");

            response = match res.send() {
                Ok(r) => r.json::<DataAccount>(),
                Err(_) => panic!("Json error")
            };

            println!("{:?}", response);

            // for data in &response.as_ref().expect("2").data {
            //     println!("{:?}", data);
            // }
            i += 1;
        }

//println!("{:?}", response);

"Test".to_string()
        //response.expect("test").data.id
    }
}
