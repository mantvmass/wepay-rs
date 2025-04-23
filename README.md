# Wepay SDK (Unofficial)

Welcome to the unofficial Wepay SDK for Rust! This library provides an easy-to-use interface to interact with the Wepay API. Designed with modern Rust practices, it supports asynchronous operations, proxy configurations, and a builder pattern for flexible and intuitive API usage.

## Features

- **Asynchronous Support**: Built on Tokio, enabling high-performance, non-blocking API calls.
- **Proxy Support**: Seamlessly integrate with proxies for enhanced connectivity.
- **Builder Pattern**: Construct API requests in a clean, readable, and flexible manner.

## Installation

Add `wepay` to your `Cargo.toml`:

```toml
[dependencies]
wepay = "0.1.0"
```

Then run:

```bash
cargo build
```

## Usage

Here is a quick example of how to use the Wepay SDK:

```rust
use wepay::Wepay;

#[tokio::main]
async fn main() {
    let we = Wepay::builder().build().unwrap();
    let result = we.product().await.unwrap();
    print!("{:?}", result)
}
```

## Documentation

Comprehensive documentation is available at [docs.rs/wepay](https://docs.rs/wepay).

## Contributing

Contributions are welcome! Please submit a pull request or open an issue for bug reports, feature requests, or suggestions.

## Disclaimer

This SDK is unofficial and is not affiliated with or endorsed by Wepay. Use it at your own risk.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

