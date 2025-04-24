use dotenvy::dotenv;
use std::env;
use wepay::Wepay;

#[tokio::test]
async fn test_product() {
    let wepay = Wepay::builder()
        .build()
        .expect("Failed to build Wepay client");

    wepay.product().await.expect("Failed to get product");
}

#[tokio::test]
async fn test_balance() {
    dotenv().ok();

    let endpoint = env::var("WEPAY_ENDPOINT").expect("WEPAY_ENDPOINT must be set");
    let username = env::var("WEPAY_USERNAME").expect("WEPAY_USERNAME must be set");
    let password = env::var("WEPAY_PASSWORD").expect("WEPAY_PASSWORD must be set");

    let wepay = Wepay::builder()
        .base_url(endpoint)
        .username(username)
        .password(password)
        .build()
        .expect("Failed to build Wepay client.");

    let result = wepay.balance().await.expect("Failed to get user balance.");

    assert_eq!(result.code, "00000");
    assert!(result.available_balance >= 0.0);
}
