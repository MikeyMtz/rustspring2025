use std::fs::File;          
use std::io::Write;         
use std::thread;            
use std::time::Duration;    
use serde::Deserialize;

// Define an empty struct for Bitcoin.
#[derive(Deserialize)]
struct Bitcoin {}

// Define an empty struct for Ethereum.
#[derive(Deserialize)]
struct Ethereum {}

// Define an empty struct for S&P 500.
#[derive(Deserialize)]
struct SP500 {}

// Define a trait Pricing which uses a fetch_price() method.
trait Pricing {
    fn fetch_price(&self) -> Result<f64, String>;
}

// Implement the `Pricing` trait for Bitcoin.
impl Pricing for Bitcoin {
    fn fetch_price(&self) -> Result<f64, String> {
        // Send a GET request to the Bitcoin price page on Coindesk
        let body = ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd")
            .call()                                 // Executes the request
            .map_err(|e| e.to_string())?            // Handle request errors
            .into_string()                          // Convert response body to String
            .map_err(|e| e.to_string())?;           // Handle conversion errors

        // Parse the JSON string into a generic JSON structure
        let json: serde_json::Value = serde_json::from_str(&body).map_err(|e| e.to_string())?;

        // Attempt to extract the float price value from the JSON
        json["bitcoin"]["usd"]
            .as_f64()
            .ok_or_else(|| "Failed to parse Bitcoin price".to_string())
    }
}

// Implement the `Pricing` trait for Ethereum.
impl Pricing for Ethereum {
    fn fetch_price(&self) -> Result<f64, String> {
        // Send GET request to Ethereum price page
        let body = ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd")
            .call()
            .map_err(|e| e.to_string())?
            .into_string()
            .map_err(|e| e.to_string())?;

        // Parse response body into JSON
        let json: serde_json::Value = serde_json::from_str(&body).map_err(|e| e.to_string())?;

        // Extract the price (placeholder path, may not be valid unless actual structure matches)
        json["ethereum"]["usd"]
            .as_f64()
            .ok_or_else(|| "Failed to parse Ethereum price".to_string())
    }
}

// Implement the `Pricing` trait for SP500
impl Pricing for SP500 {
    fn fetch_price(&self) -> Result<f64, String> {
        // Send GET request to SP500 price page for  data
        let body = ureq::get("https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m")
            .call()
            .map_err(|e| e.to_string())?
            .into_string()
            .map_err(|e| e.to_string())?;

        // Parse the response string into a JSON value
        let json: serde_json::Value = serde_json::from_str(&body).map_err(|e| e.to_string())?;

        // Extract the regular market price from nested structure
        json["chart"]["result"][0]["meta"]["regularMarketPrice"]
            .as_f64()
            .ok_or_else(|| "Failed to parse S&P 500 price".to_string())
    }
}

// Saves the fetched price into a file
fn save_to_file(filename: &str, data: f64) {
    let mut file = File::create(filename).expect("Unable to create file"); // Create or overwrite file
    writeln!(file, "{}", data).expect("Unable to write data");             // Write the price to the file
}

fn main() {
    // Create instances of each struct
    let bitcoin = Bitcoin {};
    let ethereum = Ethereum {};
    let sp500 = SP500 {};

    // Loop to fetch and store data every 10 seconds
    loop {
        // Fetch Bitcoin price and save it to a file
        match bitcoin.fetch_price() {
            Ok(price) => {
                save_to_file("bitcoin_price.txt", price);
                println!("Bitcoin price: ${:.2}", price);
            }
            Err(err) => println!("Failed to fetch Bitcoin price: {}", err),
        }

        // Fetch Ethereum price and save it to a file
        match ethereum.fetch_price() {
            Ok(price) => {
                save_to_file("ethereum_price.txt", price);
                println!("Ethereum price: ${:.2}", price);
            }
            Err(err) => println!("Failed to fetch Ethereum price: {}", err),
        }

        // Fetch SP500 price and save it to a file
        match sp500.fetch_price() {
            Ok(price) => {
                save_to_file("sp500_price.txt", price);
                println!("S&P 500 price: ${:.2}", price);
            }
            Err(err) => println!("Failed to fetch S&P 500 price: {}", err),
        }

        // Sleep for 10 seconds before fetching data again
        thread::sleep(Duration::from_secs(10));
    }
}
  