use std::env;

use dotenvy::dotenv;
use serde_json::to_string_pretty;
use wepay::Wepay;

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

    // ทดสอบ topup mobile API
    match wepay
        .topup_mobile(
            "TEST0000000000000001",
            "TRMV",
            &5f32, // 5 บาท
            "08XXXXXXXX",
            "https://www.mywebsite.com/wepay_result.php",
        )
        .await
    {
        Ok(p) => {
            let json = to_string_pretty(&p)?;
            println!("API response:\n{}", json);
        }
        Err(e) => {
            println!("API failed: {:?}", e);
            return Err(format!("API error: {}", e).into());
        }
    }

    Ok(())
}
