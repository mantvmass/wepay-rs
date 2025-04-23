use wepay::Wepay;

#[tokio::main]
async fn main() {
    let we = Wepay::builder().build().unwrap();
    let result = we.product().await.unwrap();
    print!("{:?}", result)
}
