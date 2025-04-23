use wepay::Wepay;
use tokio;

// สำหรับ async tests
#[tokio::test]
async fn test_product_api_integration() {
    // สร้าง instance ของ Wepay โดยใช้ builder pattern
    let wepay = Wepay::builder()
        .base_url("https://www.wepay.in.th")
        .build()
        .expect("Failed to build Wepay client");

    // เรียกใช้ API และแสดงผล
    let product_result = wepay.product().await;
    
    match product_result {
        Ok(products) => {
            println!("Products received: {:?}", products);
            // ทำการ assert ตามต้องการ
            assert!(true); // เปลี่ยนเป็น assertion ที่เหมาะสมตามข้อมูลจริงที่ได้รับ
        },
        Err(err) => {
            println!("Error fetching products: {:?}", err);
            // คุณอาจต้องการให้เทสต์ fail หรือไม่ ขึ้นอยู่กับว่าคุณกำลังทดสอบอะไร
            // ถ้าเป็นการทดสอบว่า API สามารถเรียกได้จริง:
            panic!("Failed to fetch products: {:?}", err);
            
            // ถ้าเป็นเพียงการตรวจสอบการทำงานของโค้ด โดยไม่สนใจว่า API จะตอบกลับอย่างไร:
            // println!("Error as expected in testing environment: {:?}", err);
        }
    }
}

#[test]
fn test_builder_functionality() {
    let wepay_builder = Wepay::builder()
        .password("test_password")
        .base_url("https://test.example.com");
        
    let wepay = wepay_builder.build().expect("Failed to build Wepay client");
    
    // ตรวจสอบว่า builder ทำงานถูกต้อง
    // แต่เนื่องจาก fields เป็น private เราอาจต้องเพิ่ม getter methods หรือใช้ assertions จากการเรียกใช้งาน methods อื่นๆ
}