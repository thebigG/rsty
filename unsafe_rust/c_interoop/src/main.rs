extern "C" {
    fn abs(input: i32) -> i32;
    fn sqrt(input: f64) -> f64;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
        println!("Square root value of 25 according to C: {}", sqrt(25.0));
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

