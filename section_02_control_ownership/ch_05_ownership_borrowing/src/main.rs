// Ownership: การเป็นเจ้าของ

fn main() {
    let s: String = String::from("Hello, World!");

    // ตัวชี้ไปยังโครงสร้างสตริง
    let p1: &String = &s;

    // ตัวชี้ไปยังไบต์ที่สตริงเป็นเจ้าของ
    // (จริงๆ แล้ว p2 เป็นคู่ pointer-length)
    let p2: &str = s.as_str();

    println!("string struct address: {p1:p}");
    println!("string data address: {p2:p}");
}
