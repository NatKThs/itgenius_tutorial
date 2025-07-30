
fn main() {
    println!("Hello, world!");
    another_function(12,24);
    let result = subtract(10, 5);
    println!("The result of subtraction is: {}", result);
 
    let y = {
        let x = 3;
        x + 1
        };
    println!("The value of y is: {}", y);
}

fn another_function(x: i32,y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);   
}   

fn subtract(x: i32, y: i32) -> i32 {
    x- y
}

