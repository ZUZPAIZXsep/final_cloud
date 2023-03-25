use actix_web::{get, post ,put ,delete, App, HttpResponse, HttpServer, Responder, web};
use reqwest::Error;
use serde::{Deserialize, Serialize};
use serde_json::{Value, Map, json};



const BASE_URL: &str = "https://private-810151-exchangeusdapi.apiary-mock.com";


#[derive(Debug, Serialize, Deserialize)]
    struct CurrencyRate {
    base: String,
    rates: Value,
}

#[derive(Debug, Serialize, Deserialize)]
    struct ExchangeRate {
    currency: String,
    exchange_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
    struct ConversionRequest {
    from_currency: String,
    to_currency: String,
    amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct CurrencyExchangeRate {
    #[serde(default)]
    currency: String,
    exchange_rate: f64,
}


#[get("/currency_rate")]
async fn currency_rate() -> impl Responder {
    match get_currency_rate().await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn get_currency_rate() -> Result<Vec<CurrencyRate>, Error> {
    let url = format!("{}/currency_rate", BASE_URL);
    let response = reqwest::get(&url).await?;
    let result: Vec<CurrencyRate> = response.json().await?;

    Ok(result)
}

#[get("/exchange_rate/{currency}")]
async fn exchange_rate(info: web::Path<(String,)>) -> impl Responder {
    match get_exchange_rate(&info.0).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn get_exchange_rate(currency: &str) -> Result<ExchangeRate, Error> {
    let url = format!("{}/exchange_rate/{}", BASE_URL, currency);
    let response = reqwest::get(&url).await?;
    let result: Map<String, Value> = response.json().await?;
    let excurrency_rate = ExchangeRate {
        currency: result.get("currency").unwrap().as_str().unwrap().to_owned(),
        exchange_rate: result.get("exchange_rate").unwrap().as_f64().unwrap(),
    };
    Ok(excurrency_rate)
}

#[post("/convert_currency")]
async fn convert_currency(info: web::Json<ConversionRequest>) -> impl Responder {
    let from_currency = &info.from_currency;
    let to_currency = &info.to_currency;
    let amount = info.amount;

    match converted_amount(from_currency, to_currency, amount).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn converted_amount(from_currency: &str, to_currency: &str, amount: f64) -> Result<(), Error> {
    let url = format!("{}/USD/to/{}", BASE_URL, to_currency);
    let params = [
        ("from_currency", from_currency),
        ("to_currency", to_currency),
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


#[post("/currencies")]
async fn add_currency_exchange_rate(info: web::Json<CurrencyExchangeRate>) -> impl Responder {
    let currency = &info.currency;
    let rate = info.exchange_rate;

    match add_currency(currency, rate).await {
        Ok(_) => HttpResponse::Created().json(info.into_inner()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn add_currency(currency: &str, rate: f64) -> Result<(), Error> {
    let url = format!("{}/currencies", BASE_URL);
    let payload = json!({
        "currency": currency,
        "exchange_rate": rate,
    });
    let response = reqwest::Client::new().post(&url).json(&payload).send().await?;

    if response.status().is_success() {
        println!("Added Currency Rate");
        println!("Currency: {}", currency);
        println!("Exchange Rate: {}", rate);
        println!();
    } else {
        let result: Value = response.json().await?;
        println!("Error: {}", result["error"]);
    }

    Ok(())
}

#[put("/currencies/{currency}")]
async fn update_currency_exchange_rate(info: web::Path<(String,)>, new_rate: web::Json<Map<String, Value>>) -> impl Responder {
    let currency = &info.0;
    let rate = new_rate["exchange_rate"].as_f64().unwrap();
    match update_currency(currency, rate).await {
        Ok(_) => HttpResponse::Created().json(json!({
            "currency": currency,
            "exchange_rate": rate
        })),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn update_currency(currency: &str, rate: f64) -> Result<(), Error> {
    let url = format!("{}/currencies/{}", BASE_URL, currency);
    let payload = json!({
        "exchange_rate": rate,
    });
    let response = reqwest::Client::new().put(&url).json(&payload).send().await?;

    if response.status().is_success() {
        println!("Updated Currency Rate");
        println!("Currency: {}", currency);
        println!("New Exchange Rate: {}", rate);
        println!();
    } else {
        let result: Value = response.json().await?;
        println!("Error: {}", result["error"]);
    }

    Ok(())
}

#[delete("/currencies/{currency}")]
async fn delete_currency(info: web::Path<(String,)>) -> impl Responder {
    let currency = &info.0;
    match delete_currency_data(currency).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn delete_currency_data(currency: &str) -> Result<(), Error> {
    let url = format!("{}/currencies/{}", BASE_URL, currency);
    let response = reqwest::Client::new().delete(&url).send().await?;

    if response.status().is_success() {
        println!("Deleted Currency");
        println!("Currency: {}", currency);
        println!();
    } else {
        let result: Value = response.json().await?;
        println!("Error: {}", result["error"]);
    }

    Ok(())
}

    #[actix_web::main]
    async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
    App::new()
    .service(currency_rate)
    .service(exchange_rate)
    .service(convert_currency)
    .service(add_currency_exchange_rate)
    .service(update_currency_exchange_rate)
    .service(delete_currency)

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
