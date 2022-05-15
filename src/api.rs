pub mod api {
    use std::env;
    use std::collections::HashMap;
    use serde::Deserialize;
    use reqwest::blocking::get;
    use crate::database::database;
    use crate::trade::trade;

    #[derive(Deserialize, Debug)]
    struct Price {
        base: String,
        currency: String,
        amount: String
    }

    #[derive(Deserialize, Debug)]
    struct DataPrice {
        data: Price,
    }

    pub fn get_values() {
        // WIP JUST TO TEST THAT THIS WORKS WELL
        // TO REMOVE LAAAAAAAAAAAAAAAAAAAAAAA
        trade::connect(); // HERE LAAA

        let cryptos: Vec<String> = get_cryptos();
        fetch_crypto(cryptos);

    }

    fn fetch_crypto(cryptos: Vec<String>) {
        let datetime = chrono::offset::Local::now();
        let now = datetime.format("%F %T").to_string();

        let mut buy_sell: HashMap<String, String> = HashMap::new();
        buy_sell.insert("buy".to_string(), "BUY_AT".to_string());
        buy_sell.insert("sell".to_string(), "SELL_AT".to_string());

        for (key, value) in buy_sell {
            for crypto in &cryptos {
                let call = format!("https://api.coinbase.com/v2/prices/{}-EUR/{}", crypto, key);

                let res = get(call).unwrap();
                let response = res.json::<DataPrice>().unwrap();

                let price: Price = response.data;
                let _result = database::add_timestamp(price.base.to_string(), value.to_string(), price.amount.to_string(), now.to_string());
            }
        }
    }

    pub fn get_cryptos() -> Vec<String>  {
        let symbols_str = env::var("symbols").expect("Symbols should be seperated by a ,");
        let symbols: Vec<&str> = symbols_str.split(",").collect();

        // Yes this is sad...
        let mut list: Vec<String> = Vec::new();
        for i in symbols {
            list.push(i.to_string());
        };

        list
    }
}
