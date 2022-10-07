fn main() {
    let x: i32 = 5;
    let x: i32 = x + 1;
    {
        let x: i32 = x * 2;
        println!("x inside the curly braces:{x}");
    }
    println!("x outside the curly braces:{x}");
}
