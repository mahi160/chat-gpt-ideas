use dotenv::dotenv;
use fetch::get;
use utils::take_input;

pub fn run() {
    dotenv().ok();
    let api_key: String = std::env::var("EXCHANGE_RATE_CURRENCY_API").unwrap();
    let host: String = String::from("https://v6.exchangerate-api.com/v6");

    let amount: f64 = take_input(Some("What is the amount?"))
        .trim()
        .parse()
        .unwrap();
    let from: String = take_input(Some("From?")).trim().parse().unwrap();
    let to: String = take_input(Some("To?")).trim().parse().unwrap();

    let url = format!("{}/{}/pair/{}/{}/{}", host, api_key, from, to, amount);

    let response = get(&url).unwrap();
    let result = response["conversion_result"].as_f64().unwrap();
    println!(
        "As of today's Exchange Rate, {:.2} {} is {:.2} {}!",
        amount, from, result, to
    );
}
