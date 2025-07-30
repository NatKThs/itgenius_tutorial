// mod greeting;
mod greetings;

// use nkgreetings::morning;
// use nkgreetings::evening;
use greetings::morning::*;
use greetings::evening::*;

fn main() {
    println!("Hello, world!");
    // greeting::hello();
    good_morning();
    good_evening();
}
