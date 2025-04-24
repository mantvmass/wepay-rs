use dotenvy::dotenv;
use std::env;
use wepay::{Wepay, error::WepayError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // โหลด environment variables จาก .env
    dotenv().ok();

    // ดึง environment variables
    let endpoint = env::var("WEPAY_ENDPOINT").map_err(|_| "WEPAY_ENDPOINT must be set")?;
    let username = env::var("WEPAY_USERNAME").map_err(|_| "WEPAY_USERNAME must be set")?;
    let password = env::var("WEPAY_PASSWORD").map_err(|_| "WEPAY_PASSWORD must be set")?;

    // สร้าง Wepay instance
    let wepay = Wepay::builder()
        .base_url(&endpoint)
        .username(&username)
        .password(&password)
        .build()
        .map_err(|e| format!("Failed to build Wepay client: {}", e))?;

    // ทดสอบ balance API
    match wepay.balance().await {
        Ok(balance) => {
            println!("Balance API response:");
            println!("Code: {}", balance.code);
            println!("Ledger Balance: {:.2}", balance.ledger_balance);
            println!("Available Balance: {:.2}", balance.available_balance);
        }
        Err(WepayError::ApiError { code, desc }) => {
            eprintln!("Balance API failed: Code={}, Desc={:?}", code, desc);
            return Err(format!("Balance API error: {}", code).into());
        }
        Err(e) => {
            eprintln!("Balance API failed: {:?}", e);
            return Err(format!("Balance API error: {}", e).into());
        }
    }

    Ok(())
}
