pub mod error;
pub mod response;

use error::{StatusCode, WepayError};
use reqwest::{Client, ClientBuilder, Proxy};

// Struct สำหรับสร้างอินสแตนซ์ของ Wepay ด้วยตัวเลือกที่กำหนดเอง
pub struct WepayBuilder {
    username: Option<String>,
    password: Option<String>,
    base_url: String,
    proxy: Option<Proxy>,
}

// Struct หลักที่ใช้เรียกใช้งาน API ต่าง ๆ ของ Wepay
pub struct Wepay {
    client: Client,
    username: Option<String>,
    password: Option<String>,
    base_url: String,
}

// กำหนดค่าเริ่มต้นของ WepayBuilder
impl Default for WepayBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl WepayBuilder {
    /// สร้าง instance ใหม่ของ WepayBuilder พร้อมค่าเริ่มต้น
    pub fn new() -> Self {
        WepayBuilder {
            username: None,
            password: None,
            base_url: "https://www.wepay.in.th".to_string(),
            proxy: None,
        }
    }

    /// กำหนด username ที่จะใช้เรียก API
    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    /// กำหนด password ที่จะใช้เรียก API
    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    /// กำหนด URL หลักสำหรับเรียก API (สามารถปรับเปลี่ยนได้จากค่าเริ่มต้น)
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// กำหนด proxy สำหรับ client ในกรณีที่ต้องการ
    pub fn proxy(mut self, proxy_url: impl Into<String>) -> Self {
        let proxy = Proxy::all(proxy_url.into()).expect("Invalid proxy URL");
        self.proxy = Some(proxy);
        self
    }

    /// สร้าง instance ของ Wepay จากค่า configuration ทั้งหมดที่ตั้งไว้
    pub fn build(self) -> Result<Wepay, WepayError> {
        let mut builder = ClientBuilder::new();

        // ถ้ามี proxy ให้กำหนดลงใน client builder
        if let Some(proxy) = self.proxy {
            builder = builder.proxy(proxy);
        }

        // สร้าง reqwest client
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
    /// เรียกใช้งาน builder เพื่อสร้าง Wepay instance
    pub fn builder() -> WepayBuilder {
        WepayBuilder::new()
    }

    /// ฟังก์ชันภายในเพื่อจัดการกับ response ของ API และตรวจสอบว่า response สำเร็จหรือไม่
    async fn handle_response<T: response::ApiResponse + serde::de::DeserializeOwned>(
        response: reqwest::Response,
    ) -> Result<T, WepayError> {
        let status = response.status();
        if !status.is_success() {
            // ถ้า HTTP status เป็น error ให้อ่านข้อมูล error และส่งกลับเป็น WepayError
            let error = response.json::<response::ErrorResponse>().await?;
            return Err(WepayError::ApiError {
                code: StatusCode::from_str(&error.code),
                desc: error.desc,
            });
        }

        // แปลงข้อมูล response เป็น GenericApiResponse
        let res = response.json::<response::GenericApiResponse<T>>().await?;
        match res {
            response::GenericApiResponse::Success(data) => {
                if data.code() == StatusCode::Success.as_str() {
                    Ok(data)
                } else {
                    Err(WepayError::ApiError {
                        code: StatusCode::from_str(data.code()),
                        desc: String::from(""),
                    })
                }
            }
            response::GenericApiResponse::Error(error) => Err(WepayError::ApiError {
                code: StatusCode::from_str(&error.code),
                desc: error.desc,
            }),
        }
    }

    /// ดึงข้อมูลสินค้าจาก API
    pub async fn product(&self) -> Result<response::Product, reqwest::Error> {
        let url = format!("{}/comp_export.php?json", self.base_url);

        // ส่ง GET request และแปลงผลลัพธ์เป็น response::Product
        let response = self.client.get(&url).send().await?;
        let result = response.json::<response::Product>().await?;
        Ok(result)
    }

    /// ดึงยอดเงินคงเหลือของบัญชีผ่าน API
    pub async fn balance(&self) -> Result<response::Balance, WepayError> {
        // ตรวจสอบว่า username/password ถูกกำหนดไว้หรือไม่
        let username = self
            .username
            .as_ref()
            .ok_or(WepayError::MissingCredentials("username"))?;
        let password = self
            .password
            .as_ref()
            .ok_or(WepayError::MissingCredentials("password"))?;

        let url = format!("{}/client_api.json.php", self.base_url);

        // เตรียม parameters ที่จะส่งใน form request
        let params = [
            ("username", username.as_str()),
            ("password", password.as_str()),
            ("type", "balance_inquiry"),
        ];

        // ส่ง POST request และเรียก handle_response เพื่อแปลง response
        let response = self.client.post(&url).form(&params).send().await?;
        Self::handle_response(response).await
    }
}
