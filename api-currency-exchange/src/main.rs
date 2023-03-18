extern crate reqwest;
extern crate serde_json;

use serde_json::{json, Value};
use reqwest::Error;

const BASE_URL: &str = "https://private-810151-exchangeusdapi.apiary-mock.com";

async fn all_currency_rates() -> Result<(), Error> {
    let url = format!("{}/currency_rate", BASE_URL);
    let response = reqwest::get(&url).await?;
    let result: Vec<Value> = response.json().await?;

    for entry in &result {
        let base_currency = entry["base"].as_str().unwrap();
        let rates = entry["rates"].as_object().unwrap();

        println!("Base Currency: {}", base_currency);
        for (currency, rate) in rates {
            println!("Currency: {} Rate: {}", currency, rate);
        }
        println!();
    }

    Ok(())
}

async fn get_exchange_rate(currency: &str) -> Result<(), Error> {
    let url = format!("{}/exchange_rate/{}", BASE_URL, currency);
    let response = reqwest::get(&url).await?;
    let result: Value = response.json().await?;

    println!("Currency: {}", result["currency"]);
    println!("Exchange Rate: {}", result["exchange_rate"]);
    println!();

    Ok(())
}

async fn converted_amount(from_currency: &str, to_currency: &str, amount: f64) -> Result<(), Error> {
    let url = format!("{}/USD/to/{}", BASE_URL, to_currency);
    let params = [
        ("base", from_currency),
        ("target_currency", to_currency),
        ("amount", &amount.to_string()),
    ];
    let response = reqwest::Client::new().post(&url).form(&params).send().await?;
    let result: Value = response.json().await?;

    let rate = result["exchange_rate"].as_f64().unwrap();
    let converted_amount = rate * amount;

    println!("Base Currency: {}", from_currency);
    println!("Target Currency: {}", to_currency);
    println!("Amount: {}", amount);
    println!("Exchange Rate: {}", rate);
    println!("Converted Amount: {:.2}", converted_amount);
    println!();

    Ok(())
}

async fn add_currency_exchange_rate(currency: &str, rate: f64) -> Result<(), Error> {
    let url = format!("{}/currencies", BASE_URL);
    let payload = json!({
        "currency": currency,
        "exchange_rate": rate,
    });
    let response = reqwest::Client::new().post(&url).json(&payload).send().await?;

    if response.status().is_success() {
        let result: Value = response.json().await?;
        println!("Added Currency Rate");
        println!("Currency: {}", result["currency"]);
        println!("Exchange Rate: {}", result["exchange_rate"]);
        println!();
    } else {
        let result: Value = response.json().await?;
        println!("Error: {}", result["error"]);
    }

    Ok(())
}

async fn update_exchange_rate(currency: &str, rate: f64) -> Result<(), Error> {
    let url = format!("{}/currencies/{}", BASE_URL, currency);
    let payload = json!({
        "currency": currency,
        "exchange_rate": rate,
    });
    let response = reqwest::Client::new().put(&url).json(&payload).send().await?;

    if response.status().is_success() {
        let result: Value = response.json().await?;
        println!("Updated Currency Rate");
        println!("Currency: {}", result["currency"]);
        println!("Exchange Rate: {}", result["exchange_rate"]);
        println!();
    } else {
        let result: Value = response.json().await?;
        println!("Error: {}", result["error"]);
    }

    Ok(())
}

async fn delete_exchange_rate(currency: &str) -> Result<(), Error> {
    let url = format!("{}/currencies/{}", BASE_URL, currency);
    let response = reqwest::Client::new().delete(&url).send().await?;

    if response.status().is_success() {
        println!("Currency exchange rate deleted: {}", currency);
        println!();
    } else {
        let result: Value = response.json().await?;
        println!("Error: {}", result["error"]);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    all_currency_rates().await?;

    let currency = "GBP";
    get_exchange_rate(currency).await?;

    let from_currency = "USD";
    let to_currency = "GBP";
    let amount = 1000.0;
    converted_amount(from_currency, to_currency, amount).await?;

    let new_currency = "AUD";
    let new_rate = 1.410;
    add_currency_exchange_rate(new_currency, new_rate).await?;

    let currency_to_update = "THB";
    let new_rate = 35.500;
    update_exchange_rate(currency_to_update, new_rate).await?;

    let currency_to_delete = "THB";
    delete_exchange_rate(currency_to_delete).await?;

    Ok(())
}

