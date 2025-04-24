use dotenvy::dotenv;
use std::env;
use wepay::Wepay;

#[tokio::test]
async fn test_product_api() {
    let wepay = Wepay::builder()
        .build()
        .expect("Failed to build Wepay client");

    wepay.product().await.expect("Failed to get product");
}

#[tokio::test]
async fn test_balance_mock_api() {
    let mock_server = mockito::mock("POST", "/client_api.json.php")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{"code": "00000", "ledger_balance": "299987.74", "available_balance": "299987.74"}"#,
        )
        .create();

    let wepay = Wepay::builder()
        .base_url(mockito::server_url())
        .username("test_username")
        .password("test_password")
        .build()
        .expect("Failed to build Wepay client.");

    let result = wepay.balance().await.expect("Failed to get user balance.");

    assert_eq!(result.code, "00000");
    assert_eq!(result.available_balance, 299987.74);

    mock_server.assert();
}

#[tokio::test]
async fn test_valid_user_and_balance_api() {
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
