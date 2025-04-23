mod error;
mod response;

use error::{StatusCode, WepayError};
use reqwest::{Client, ClientBuilder, Proxy};

pub struct WepayBuilder {
    username: Option<String>,
    password: Option<String>,
    base_url: String,
    proxy: Option<Proxy>,
}

pub struct Wepay {
    client: Client,
    username: Option<String>,
    password: Option<String>,
    base_url: String,
}

impl Default for WepayBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl WepayBuilder {
    pub fn new() -> Self {
        WepayBuilder {
            username: None,
            password: None,
            base_url: "https://www.wepay.in.th".to_string(),
            proxy: None,
        }
    }

    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub fn proxy(mut self, proxy_url: impl Into<String>) -> Self {
        let proxy = Proxy::all(proxy_url.into()).expect("Invalid proxy URL");
        self.proxy = Some(proxy);
        self
    }

    pub fn build(self) -> Result<Wepay, WepayError> {
        let mut builder = ClientBuilder::new();

        if let Some(proxy) = self.proxy {
            builder = builder.proxy(proxy);
        }

        let client = builder.build()?;

        Ok(Wepay {
            client,
            username: self.username,
            password: self.password,
            base_url: self.base_url,
        })
    }
}

impl Wepay {
    pub fn builder() -> WepayBuilder {
        WepayBuilder::new()
    }

    pub async fn product(&self) -> Result<response::Product, reqwest::Error> {
        let url = format!("{}/comp_export.php?json", self.base_url);

        let response = self.client.get(&url).send().await?;
        let result = response.json::<response::Product>().await?;
        Ok(result)
    }

    pub async fn balance(&self) -> Result<response::Balance, WepayError> {
        let username = self
            .username
            .as_ref()
            .ok_or(WepayError::MissingCredentials("username"))?;
        let password = self
            .password
            .as_ref()
            .ok_or(WepayError::MissingCredentials("password"))?;

        let url = format!("{}/client_api.json.php", self.base_url);

        let params = [
            ("username", username.as_str()),
            ("password", password.as_str()),
            ("type", "balance_inquiry"),
        ];

        let response = self.client.post(&url).form(&params).send().await?;

        let result = response.json::<response::Balance>().await?;

        // ตรวจสอบรหัสสถานะจาก API
        if result.code != StatusCode::Success.as_str() {
            return Err(WepayError::ApiError(StatusCode::from_str(&result.code)));
        }

        Ok(result)
    }
}
