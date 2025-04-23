use tokio;
use wepay::Wepay;

#[tokio::test]
async fn test_product_api() {
    let wepay = Wepay::builder()
        .build()
        .expect("Failed to build Wepay client");

    let result = wepay.product().await;

    match result {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[tokio::test]
async fn test_balance_api() {
    let mock_server = mockito::mock("POST", "/client_api.json.php")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"code": "00000", "ledger_balance": "299987.74", "available_balance": 299987.74}"#)
        .create();
        
    let wepay = Wepay::builder()
        .base_url(&mockito::server_url())
        .username("test_username")
        .password("test_password")
        .build()
        .unwrap();
        
    let result = wepay.balance().await.unwrap();
    
    assert_eq!(result.code, "00000");
    assert_eq!(result.available_balance, 299987.74);
    
    mock_server.assert();
}