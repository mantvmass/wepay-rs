use url::Url;

/// ตรวจสอบความถูกต้องของ dest_ref (อ้างอิงจาก doc ของ wepay)
pub fn is_valid_ref(s: &str) -> bool {
    // ตรวจสอบความยาว (max 20 characters)
    if s.len() > 20 {
        return false;
    }
    // ตรวจสอบอักขระแต่ละตัว
    s.chars().all(|c| {
        c.is_ascii_lowercase() ||  // a-z
        c.is_ascii_uppercase() ||  // A-Z
        c.is_ascii_digit() // 0-9
    })
}

/// ตรวจสอบความถูกต้องของเบอร์มือถือไทย (06, 08 and 09 Only)
pub fn is_thai_mobile_number(number: &str) -> bool {
    // ทำความสะอาดตัวเลขโดยลบตัวอักษรที่ไม่ใช่ตัวเลขทั้งหมด
    let cleaned: String = number.chars().filter(|c| c.is_ascii_digit()).collect();

    // ตรวจสอบว่าเบอร์ที่ทำความสะอาดแล้วเป็นเบอร์มือถือไทยที่ถูกต้องหรือไม่
    cleaned.len() == 10
        && cleaned.starts_with('0')
        && matches!(cleaned.chars().nth(1), Some('6') | Some('8') | Some('9'))
}

/// ตรวจสอบความถูกต้องของลิงค์
pub fn is_valid_url(url: &str) -> bool {
    Url::parse(url).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ref() {
        assert!(is_valid_ref("abcABC123"));
        assert!(is_valid_ref("ValidString123"));
        assert!(is_valid_ref("1234567890"));
        assert!(is_valid_ref("a"));
        assert!(is_valid_ref("A"));
        assert!(is_valid_ref("0"));
        assert!(is_valid_ref("")); // empty string
        assert!(is_valid_ref("aBcD1234eFgH5678")); // exactly 16 chars
    }

    #[test]
    fn test_is_valid_ref_max_length() {
        assert!(is_valid_ref("12345678901234567890")); // exactly 20 chars
        assert!(!is_valid_ref("123456789012345678901")); // 21 chars
        assert!(!is_valid_ref("ThisStringIsWayTooLongForTheLimit")); // 32 chars
    }

    #[test]
    fn test_is_valid_ref_invalid_characters() {
        assert!(!is_valid_ref("invalid!char"));
        assert!(!is_valid_ref("space in string"));
        assert!(!is_valid_ref("under_score"));
        assert!(!is_valid_ref("dash-included"));
        assert!(!is_valid_ref("日本語")); // non-ASCII
        assert!(!is_valid_ref("emoji😊")); // emoji
        assert!(!is_valid_ref("new\nline")); // control character
    }

    #[test]
    fn test_is_valid_ref_edge_cases() {
        assert!(is_valid_ref(&"a".repeat(20))); // max length with single char
        assert!(is_valid_ref(&"A".repeat(20))); // max length with single char
        assert!(is_valid_ref(&"0".repeat(20))); // max length with single char
        assert!(!is_valid_ref(&"a".repeat(21))); // just over limit
        assert!(is_valid_ref(&("aB1".repeat(6) + "aB"))); // 20 chars mixed
    }

    #[test]
    fn test_valid_thai_numbers() {
        // AIS numbers
        assert!(is_thai_mobile_number("0812345678"));
        assert!(is_thai_mobile_number("0898765432"));

        // DTAC numbers
        assert!(is_thai_mobile_number("0612345678"));
        assert!(is_thai_mobile_number("0667890123"));

        // TrueMove numbers
        assert!(is_thai_mobile_number("0912345678"));
        assert!(is_thai_mobile_number("0998765432"));

        // With formatting
        assert!(is_thai_mobile_number("081-234-5678"));
        assert!(is_thai_mobile_number("081 234 5678"));
    }

    #[test]
    fn test_invalid_thai_numbers() {
        // Wrong length
        assert!(!is_thai_mobile_number("081234567")); // too short
        assert!(!is_thai_mobile_number("08123456789")); // too long

        // Wrong prefix
        assert!(!is_thai_mobile_number("0212345678")); // Bangkok landline
        assert!(!is_thai_mobile_number("0312345678")); // Nonthaburi landline
        assert!(!is_thai_mobile_number("0712345678")); // Invalid mobile prefix

        // International numbers
        assert!(!is_thai_mobile_number("+1212345678")); // US number
        assert!(!is_thai_mobile_number("+441234567890")); // UK number

        // Non-numeric
        assert!(!is_thai_mobile_number("08ABCDEFGH"));
        assert!(!is_thai_mobile_number("phone number"));
    }

    #[test]
    fn test_thai_numbers_edge_cases() {
        assert!(!is_thai_mobile_number("")); // empty string
        assert!(!is_thai_mobile_number("0000000000")); // all zeros
        assert!(!is_thai_mobile_number("9999999999")); // all nines
        assert!(!is_thai_mobile_number("          ")); // spaces
    }

    #[test]
    fn test_valid_urls() {
        // URLs ที่คาดว่าจะเป็น valid
        assert!(is_valid_url("https://example.com"));
        assert!(is_valid_url("http://localhost:8080"));
        assert!(is_valid_url("ftp://ftp.example.com"));
        assert!(is_valid_url("https://sub.example.com/path?query=value"));
    }

    #[test]
    fn test_invalid_urls() {
        // URLs ที่คาดว่าจะเป็น invalid
        assert!(!is_valid_url("invalid-url"));
        assert!(!is_valid_url("https://"));
        assert!(!is_valid_url("ftp://"));
    }

    #[test]
    fn test_urls_edge_cases() {
        // ข้อความที่คาดว่าจะยังไม่ผ่านการตรวจสอบ
        assert!(!is_valid_url("http://:8080")); // ไม่มี host
        assert!(!is_valid_url("://example.com")); // ไม่มี schema
    }
}
