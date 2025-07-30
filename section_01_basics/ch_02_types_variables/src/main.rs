fn main() {
    let mut a = -14;    
    println!("types and variables");
    println!("a is: {}", a);
    a = 24;
    println!("a is now: {}", a);
    let teststr = "test string".to_string();
    println!("teststr is: {}", teststr);
    const TEST_CONST:i32 = 777;
    println!("TEST_CONST is: {}", TEST_CONST);
    #[allow(unused_variables)]
    let no_warning = 12;
}
