use serde_json::to_string_pretty;
use wepay_rs::Wepay;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // สร้าง Wepay instance
    let wepay = Wepay::builder()
        .build()
        .map_err(|e| format!("Failed to build Wepay client: {}", e))?;

    // ทดสอบ product API
    match wepay.product().await {
        Ok(p) => {
            let json = to_string_pretty(&p)?;
            println!("Product API response:\n{}", json);
        }
        Err(e) => {
            eprintln!("Product API failed: {:?}", e);
            return Err(format!("Product API error: {}", e).into());
        }
    }

    Ok(())
}
