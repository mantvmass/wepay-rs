use serde::{Deserialize, Serialize};

// โครงสร้างสำหรับการตอบกลับของ API ตรวจสอบยอดคงเหลือ
#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    pub code: String,
    pub ledger_balance: String,
    pub available_balance: String,
}

#[derive(Serialize, Deserialize)]
pub struct Product {
    data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    mtopup: Vec<Mtopup>,
    cashcard: Vec<Cashcard>,
    gtopup: Vec<Gtopup>,
    billpay: Vec<Billpay>,
}

#[derive(Serialize, Deserialize)]
pub struct Billpay {
    company_id: CompanyId,
    company_name: String,
    fee: i64,
    minimum_amount: f64,
    maximum_amount: i64,
    barcode_only: bool,
    refs: Vec<Ref>,
    additional_info: String,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompanyId {
    Integer(i64),
    String(String),
}

#[derive(Serialize, Deserialize)]
pub struct Ref {
    key: Key,
    name: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Key {
    Ref1,
    Ref2,
    Ref3,
}

#[derive(Serialize, Deserialize)]
pub struct Cashcard {
    company_id: String,
    company_name: String,
    fee: i64,
    denomination: Vec<Denomination>,
}

#[derive(Serialize, Deserialize)]
pub struct Denomination {
    price: f64,
    description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Gtopup {
    company_id: String,
    company_name: String,
    fee: i64,
    denomination: Vec<Denomination>,
    congestion: Vec<Option<serde_json::Value>>,
    gameservers: Vec<Gameserver>,
    refs_format: RefsFormat,
}

#[derive(Serialize, Deserialize)]
pub struct Gameserver {
    value: String,
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct RefsFormat {
    pub ref1: Option<String>,
    pub ref2: Option<String>,
    pub ref3: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Mtopup {
    company_id: String,
    company_name: String,
    fee: i64,
    minimum_amount: f64,
    maximum_amount: i64,
    refundable: bool,
    denomination: Vec<Denomination>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceResponse {
    pub code: String,
    pub ledger_balance: Option<String>,
    pub available_balance: Option<f64>,
}