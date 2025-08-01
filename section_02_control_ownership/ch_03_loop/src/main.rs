fn main() {

    
    // การใช้ loop เพื่อทำงานซ้ำ
    let mut counter = 0; // ประกาศตัวแปรให้สามารถเปลี่ยนแปลงค่าได้

    loop {
        counter += 1;

        if counter == 3 {
            println!("ข้ามรอบนี้ไป!");
            continue; // ข้ามไปรอบถัดไป
        }

        if counter == 5 {
            println!("จบการทำงานที่รอบที่: {}", counter);
            break; // ออกจาก loop เมื่อถึงรอบที่ 5
        }

        println!("รอบที่: {}", counter);
    }

    println!("--------------------");

    // การใช้ while loop
    let mut number = 3;
    let max_number = 10;

    while number < 10 && number < max_number {
        // println!("หมายเลขปัจจุบัน: {}", number);

        if number == 5 {
            println!("ข้ามค่าปัจจุบัน!");
            number += 1; // เพิ่มค่าขึ้น 1 เพื่อหลีกเลี่ยงการวนลูปไม่สิ้นสุด
            continue; // ข้ามค่าปัจจุบัน
        } else if number == 8 {
            println!("ถึงค่าที่ 8 แล้ว! หยุดการทำงาน");
            break; // หยุดการทำงานเมื่อถึงค่าที่ 8
        } else {
            // ทำงานเมื่อไม่ตรงกับเงื่อนไขใดๆ ก่อนหน้า
            println!("ทำงานกับค่าปัจจุบัน: {}", number);
        }

        number += 1; // เพิ่มค่า number ทีละ 1
    }

    println!("--------------------------");

    // การใช้ for loop

    // การใช้ for + range พื้นฐาน
    for i in 1..5 {
        println!("ค่าปัจจุบัน: {}", i);
    }

    println!("--------------------------");

    // ใช้ ..= เพื่อรวมค่าปลายทาง
    for i in 1..=5 {
        println!("ค่าปัจจุบัน: {}", i);
    }

    println!("--------------------------");

    // วนซ้ำแบบถอยหลัง
    for i in (1..=5).rev() {
        println!("ค่าปัจจุบัน: {}", i);
    }
    println!("จบการทำงานของ loop");

    println!("--------------------------");

    // ใช้ .step_by()
    // เริ่มจาก 2 และวนไปจนถึง (แต่ไม่รวม) 9 โดยข้ามทีละ 2
    for i in (2..=9).step_by(2) {
        println!("ค่าปัจจุบัน: {}", i);
    }
}
