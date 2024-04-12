use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct QuoteData {
    symbol: String,
    name: String,
    price: f64,

}

fn main() -> Result<(), Box<dyn Error>> {
    // --- Configuration ---
    let api_key = "L5b6pXQVARZSXHvJsfb4cockmNY9zlWP";

    // --- Fetch Crypto Prices ---
    let crypto_symbols = "BTCUSD,ETHUSD";
    let crypto_url = format!(
        "https://financialmodelingprep.com/api/v3/quote/{}?apikey={}",
        crypto_symbols, api_key
    );

    println!("Fetching crypto data from: {}", crypto_url);
    let crypto_response = reqwest::blocking::get(&crypto_url)?; // Synchronous GET

    // --- Status Check (Crypto) ---
    let crypto_status = crypto_response.status();

    if !crypto_status.is_success() {
        let error_body = crypto_response
            .text()
            .unwrap_or_else(|_| "Could not read error body".to_string());
        eprintln!("Error fetching crypto data: HTTP Status {}", crypto_status);
        eprintln!("Response body: {}", error_body);
        return Err(format!("Crypto API request failed with status: {}", crypto_status).into());
    }

    // --- Parse Successful Crypto Response ---
    let crypto_data: Vec<QuoteData> = crypto_response.json()?;

    println!("\n--- Crypto Prices (USD) ---");
    if crypto_data.is_empty() {
        println!("No crypto data received (Check API key or symbols).");
    } else {
        for coin in &crypto_data {
            println!("{} ({}): ${:.2}", coin.name, coin.symbol, coin.price);
        }
    }


    let index_symbol = "SPY";
    let index_url = format!(
        "https://financialmodelingprep.com/api/v3/quote/{}?apikey={}",
        index_symbol, api_key
    );

    println!("\nFetching index data (using SPY) from: {}", index_url);
    let index_response = reqwest::blocking::get(&index_url)?; // Synchronous GET

    // --- Status Check (Index) ---
    let index_status = index_response.status();

    if !index_status.is_success() {
        let error_body = index_response
            .text()
            .unwrap_or_else(|_| "Could not read error body".to_string());
        eprintln!("Error fetching index data (SPY): HTTP Status {}", index_status);
        eprintln!("Response body: {}", error_body);
        return Err(format!("Index API request (SPY) failed with status: {}", index_status).into());
    }

    // --- Parse Successful Index Response ---
    let index_data_vec: Vec<QuoteData> = index_response.json()?;

    // Extract the first element (should be SPY data)
    let spy_data = index_data_vec // Renamed variable for clarity
        .into_iter()
        .next()
        .ok_or("SPY data array was unexpectedly empty")?;

    println!("\n--- Market Index ---");

    println!("S&P 500 (via {}): ${:.2}", spy_data.symbol, spy_data.price);



    Ok(())
}