use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

/// Trait สำหรับ response types ทั้งหมด
pub trait ApiResponse {
    fn code(&self) -> &str;
}

/// โครงสร้างสำหรับ error response
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: String,
    pub desc: String,
}

impl ApiResponse for ErrorResponse {
    fn code(&self) -> &str {
        &self.code
    }
}

/// โครงสร้างสำหรับการตอบกลับของ API ตรวจสอบยอดคงเหลือ
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    pub code: String,
    #[serde_as(as = "DisplayFromStr")]
    pub ledger_balance: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub available_balance: f64,
}

impl ApiResponse for Balance {
    fn code(&self) -> &str {
        &self.code
    }
}

/// โครงสร้างสำหรับการตอบกลับของ API [billpay, mtopup, cashcard]
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct BillCommon {
    pub code: String,
    pub bill_id: u32,
    pub transaction_id: String,
    pub queue_id: u32,
    pub total_amount: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub balance: f64,
}

impl ApiResponse for BillCommon {
    fn code(&self) -> &str {
        &self.code
    }
}

/// Generic enum สำหรับ API response
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenericApiResponse<T: ApiResponse> {
    Success(T),
    Error(ErrorResponse),
}

/// โครงสร้างสำหรับรายการสินค้าจาก wepay
#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub mtopup: Vec<Mtopup>,
    pub cashcard: Vec<Cashcard>,
    pub gtopup: Vec<Gtopup>,
    pub billpay: Vec<Billpay>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Billpay {
    pub company_id: CompanyId,
    pub company_name: String,
    pub fee: i64,
    pub minimum_amount: f64,
    pub maximum_amount: i64,
    pub barcode_only: bool,
    pub refs: Vec<Ref>,
    pub additional_info: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompanyId {
    Integer(i64),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ref {
    pub key: Key,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Key {
    Ref1,
    Ref2,
    Ref3,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cashcard {
    pub company_id: String,
    pub company_name: String,
    pub fee: i64,
    pub denomination: Vec<Denomination>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Denomination {
    pub price: f64,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gtopup {
    pub company_id: String,
    pub company_name: String,
    pub fee: i64,
    pub denomination: Vec<Denomination>,
    pub congestion: Vec<Option<serde_json::Value>>,
    pub gameservers: Vec<Gameserver>,
    pub refs_format: RefsFormat,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gameserver {
    pub value: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefsFormat {
    pub ref1: Option<String>,
    pub ref2: Option<String>,
    pub ref3: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mtopup {
    pub company_id: String,
    pub company_name: String,
    pub fee: i64,
    pub minimum_amount: f64,
    pub maximum_amount: i64,
    pub refundable: bool,
    pub denomination: Vec<Denomination>,
}
