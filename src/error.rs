use std::fmt;
use thiserror::Error;

/// ข้อผิดพลาดที่อาจเกิดขึ้นในระบบ
#[derive(Error, Debug)]
pub enum WepayError {
    #[error("ไม่มีข้อมูลรับรอง: {0}")]
    MissingCredentials(&'static str), // ขาดข้อมูลรับรองที่จำเป็น

    #[error("รูปแบบข้อมูลไม่ถูกต้อง: {0}")]
    InvalidFormat(&'static str), // รูปแบบข้อมูลไม่ถูกต้อง (ดูตามเอกสารของ wepay)

    #[error("ข้อผิดพลาดในการร้องขอ: {0}")]
    RequestError(#[from] reqwest::Error), // ข้อผิดพลาดจาก reqwest

    #[error("ข้อผิดพลาดจาก API: code={code}, desc={desc:?}")]
    ApiError {
        // ข้อผิดพลาดจาก API ตามรหัสสถานะ
        code: StatusCode,
        desc: String,
    },
}

/// รหัสสถานะจาก API
#[derive(Debug, PartialEq)]
pub enum StatusCode {
    Success,                      // รายการสำเร็จ
    InternalErrorDb,              // Internal Error (ไม่สามารถเชื่อมต่อระบบฐานข้อมูลได้)
    InternalErrorIncomplete,      // Internal Error (รายการไม่สมบูรณ์)
    InternalErrorTooManyRequests, // Internal Error (Too Many Requests)
    InternalErrorCreate,          // Internal Error (เกิดข้อผิดพลาดในการสร้างรายการใหม่)
    InvalidCredentialsFormat1,    // Username หรือ Password มีรูปแบบไม่ถูกต้อง (20001)
    InvalidCredentialsFormat2,    // Username หรือ Password มีรูปแบบไม่ถูกต้อง (20003)
    InvalidCredentialsApi,        // Username ไม่สามารถเรียกใช้ API ได้
    UnauthorizedIp,               // ไม่อนุญาตให้เข้าถึงระบบ (IP address นี้ไม่ได้รับอนุญาต)
    CompanyNotFound,              // ไม่พบ Company ที่ระบุ
    InvalidTransactionId,         // Transaction ID ไม่ถูกต้อง
    TransactionNotFound,          // ไม่พบ Transaction ID ที่ระบุ
    TransactionInProgress,        // Tranasction ID ที่ระบุอยู่ในระหว่างการทำรายการ
    InvalidCallbackUrl,           // Callback URL ไม่ถูกต้อง
    InvalidCallbackRefId,         // Callback Reference ID ไม่ถูกต้อง
    InvalidPaymentAmount,         // ระบุจำนวนเงินที่ชำระไม่ถูกต้อง
    InvalidCompany,               // ระบุบริษัทไม่ถูกต้อง
    InvalidRef1,                  // ระบุ Ref.1 ไม่ถูกต้อง
    InvalidRef2,                  // ระบุ Ref.2 ไม่ถูกต้อง
    InvalidRef3,                  // ระบุ Ref.3 ไม่ถูกต้อง
    InvalidRef4,                  // ระบุ Ref.4 ไม่ถูกต้อง
    InvalidBarcode,               // บาร์โค้ดไม่ถูกต้อง
    DuplicateCallbackRefId,       // มีการทำรายการซ้ำ (พบ Callback Reference ID ซ้ำกัน)
    IncorrectCompanySelection, // ตรวจพบการเลือกบริษัทผิดพลาด (เช่น กรณีที่พบว่าหมายเลขบัตรเครดิตเป็นของธนาคารอื่น)
    InvalidDebtAmount, // ตรวจพบความไม่ถูกต้องของยอดหนี้ (เช่น กรณีที่เกินกำหนดชำระเงิน หรือ ระบุยอดหนี้ไม่ถูกต้อง)
    InsufficientFunds, // ยอดเงินในระบบไม่เพียงพอ
    PaymentNotAllowed, // ไม่อนุญาตให้ชำระเงินให้กับบริษัทที่ระบุ
    Unknown(String),   // รหัสที่ไม่รู้จัก
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl StatusCode {
    /// แปลงรหัสสถานะเป็นสตริง
    pub fn as_str(&self) -> &str {
        match self {
            StatusCode::Success => "00000",                      // รายการสำเร็จ
            StatusCode::InternalErrorDb => "10001", // Internal Error (ไม่สามารถเชื่อมต่อระบบฐานข้อมูลได้)
            StatusCode::InternalErrorIncomplete => "10002", // Internal Error (รายการไม่สมบูรณ์)
            StatusCode::InternalErrorTooManyRequests => "10003", // Internal Error (Too Many Requests)
            StatusCode::InternalErrorCreate => "10004", // Internal Error (เกิดข้อผิดพลาดในการสร้างรายการใหม่)
            StatusCode::InvalidCredentialsFormat1 => "20001", // Username หรือ Password มีรูปแบบไม่ถูกต้อง
            StatusCode::InvalidCredentialsFormat2 => "20003", // Username หรือ Password มีรูปแบบไม่ถูกต้อง
            StatusCode::InvalidCredentialsApi => "20004", // Username ไม่สามารถเรียกใช้ API ได้
            StatusCode::UnauthorizedIp => "20005", // ไม่อนุญาตให้เข้าถึงระบบ (IP address นี้ไม่ได้รับอนุญาต)
            StatusCode::CompanyNotFound => "30001", // ไม่พบ Company ที่ระบุ
            StatusCode::InvalidTransactionId => "30002", // Transaction ID ไม่ถูกต้อง
            StatusCode::TransactionNotFound => "30003", // ไม่พบ Transaction ID ที่ระบุ
            StatusCode::TransactionInProgress => "30004", // Tranasction ID ที่ระบุอยู่ในระหว่างการทำรายการ
            StatusCode::InvalidCallbackUrl => "30005",    // Callback URL ไม่ถูกต้อง
            StatusCode::InvalidCallbackRefId => "30006",  // Callback Reference ID ไม่ถูกต้อง
            StatusCode::InvalidPaymentAmount => "30007",  // ระบุจำนวนเงินที่ชำระไม่ถูกต้อง
            StatusCode::InvalidCompany => "30008",        // ระบุบริษัทไม่ถูกต้อง
            StatusCode::InvalidRef1 => "30009",           // ระบุ Ref.1 ไม่ถูกต้อง
            StatusCode::InvalidRef2 => "30010",           // ระบุ Ref.2 ไม่ถูกต้อง
            StatusCode::InvalidRef3 => "30011",           // ระบุ Ref.3 ไม่ถูกต้อง
            StatusCode::InvalidRef4 => "30012",           // ระบุ Ref.4 ไม่ถูกต้อง
            StatusCode::InvalidBarcode => "30013",        // บาร์โค้ดไม่ถูกต้อง
            StatusCode::DuplicateCallbackRefId => "30016", // มีการทำรายการซ้ำ (พบ Callback Reference ID ซ้ำกัน)
            StatusCode::IncorrectCompanySelection => "30017", // ตรวจพบการเลือกบริษัทผิดพลาด
            StatusCode::InvalidDebtAmount => "30018",      // ตรวจพบความไม่ถูกต้องของยอดหนี้
            StatusCode::InsufficientFunds => "30019",      // ยอดเงินในระบบไม่เพียงพอ
            StatusCode::PaymentNotAllowed => "30020",      // ไม่อนุญาตให้ชำระเงินให้กับบริษัทที่ระบุ
            StatusCode::Unknown(code) => code.as_str(),    // รหัสที่ไม่รู้จัก
        }
    }

    /// แปลงสตริงเป็นรหัสสถานะ
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(code: &str) -> Self {
        match code {
            "00000" => StatusCode::Success,                      // รายการสำเร็จ
            "10001" => StatusCode::InternalErrorDb, // Internal Error (ไม่สามารถเชื่อมต่อระบบฐานข้อมูลได้)
            "10002" => StatusCode::InternalErrorIncomplete, // Internal Error (รายการไม่สมบูรณ์)
            "10003" => StatusCode::InternalErrorTooManyRequests, // Internal Error (Too Many Requests)
            "10004" => StatusCode::InternalErrorCreate, // Internal Error (เกิดข้อผิดพลาดในการสร้างรายการใหม่)
            "20001" => StatusCode::InvalidCredentialsFormat1, // Username หรือ Password มีรูปแบบไม่ถูกต้อง
            "20003" => StatusCode::InvalidCredentialsFormat2, // Username หรือ Password มีรูปแบบไม่ถูกต้อง
            "20004" => StatusCode::InvalidCredentialsApi, // Username ไม่สามารถเรียกใช้ API ได้
            "20005" => StatusCode::UnauthorizedIp, // ไม่อนุญาตให้เข้าถึงระบบ (IP address นี้ไม่ได้รับอนุญาต)
            "30001" => StatusCode::CompanyNotFound, // ไม่พบ Company ที่ระบุ
            "30002" => StatusCode::InvalidTransactionId, // Transaction ID ไม่ถูกต้อง
            "30003" => StatusCode::TransactionNotFound, // ไม่พบ Transaction ID ที่ระบุ
            "30004" => StatusCode::TransactionInProgress, // Tranasction ID ที่ระบุอยู่ในระหว่างการทำรายการ
            "30005" => StatusCode::InvalidCallbackUrl,    // Callback URL ไม่ถูกต้อง
            "30006" => StatusCode::InvalidCallbackRefId,  // Callback Reference ID ไม่ถูกต้อง
            "30007" => StatusCode::InvalidPaymentAmount,  // ระบุจำนวนเงินที่ชำระไม่ถูกต้อง
            "30008" => StatusCode::InvalidCompany,        // ระบุบริษัทไม่ถูกต้อง
            "30009" => StatusCode::InvalidRef1,           // ระบุ Ref.1 ไม่ถูกต้อง
            "30010" => StatusCode::InvalidRef2,           // ระบุ Ref.2 ไม่ถูกต้อง
            "30011" => StatusCode::InvalidRef3,           // ระบุ Ref.3 ไม่ถูกต้อง
            "30012" => StatusCode::InvalidRef4,           // ระบุ Ref.4 ไม่ถูกต้อง
            "30013" => StatusCode::InvalidBarcode,        // บาร์โค้ดไม่ถูกต้อง
            "30016" => StatusCode::DuplicateCallbackRefId, // มีการทำรายการซ้ำ (พบ Callback Reference ID ซ้ำกัน)
            "30017" => StatusCode::IncorrectCompanySelection, // ตรวจพบการเลือกบริษัทผิดพลาด
            "30018" => StatusCode::InvalidDebtAmount,      // ตรวจพบความไม่ถูกต้องของยอดหนี้
            "30019" => StatusCode::InsufficientFunds,      // ยอดเงินในระบบไม่เพียงพอ
            "30020" => StatusCode::PaymentNotAllowed,      // ไม่อนุญาตให้ชำระเงินให้กับบริษัทที่ระบุ
            _ => StatusCode::Unknown(code.to_string()),    // รหัสที่ไม่รู้จัก
        }
    }
}
