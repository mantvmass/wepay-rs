use wepay_rs::Wepay;

#[tokio::test]
async fn test_balance_mock() {
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
async fn test_topup_mobile_mock() {
    let mock_server = mockito::mock("POST", "/client_api.json.php")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{"bill_id": 311057859, "code": "00000", "transaction_id": "394209456", "queue_id": 353183625, "total_amount": 4.84, "balance": "162.24"}"#,
        )
        .create();

    let wepay = Wepay::builder()
        .base_url(mockito::server_url())
        .username("test_username")
        .password("test_password")
        .build()
        .expect("Failed to build Wepay client.");

    let result = wepay
        .topup_mobile(
            "TEST0000000000000001",
            "TRMV",
            &5f32, // 5 บาท
            "0987654321",
            "https://www.mywebsite.com/wepay_result.php",
        )
        .await
        .expect("Failed to topup mobile.");

    assert_eq!(result.code, "00000");
    assert_eq!(result.balance, 162.24);

    mock_server.assert();
}
