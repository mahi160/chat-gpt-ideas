use dotenv::dotenv;
use fetch;

pub fn run() {
    dotenv().ok();
    let api_key: String = std::env::var("EXCHANGE_RATE_CURRENCY_API").unwrap();
    let host: String = String::from("https://v6.exchangerate-api.com/v6");

    let mut amount = 100;

    let mut from = "USD";
    let mut to = "BDT";

    let url = format!("{}/{}/pair/{}/{}/{}", host, api_key, from, to, amount);

    let response = fetch::get(&url).unwrap();
    let result = response["conversion_result"].as_f64().unwrap();
    println!("{:?}", result);
}
