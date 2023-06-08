use reqwest::{Client, Error};
use serde_json::Value;

#[tokio::main]
pub async fn get(url: &String) -> Result<Value, Error> {
    let client = Client::new();
    let res = client.get(url).send().await?;
    let json = res.json::<serde_json::Value>().await?;
    Ok(json)
}