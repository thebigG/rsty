fn main() {
    println!("Hello, world!");
    unsafe_ptrs();
}

fn unsafe_ptrs(){
    let mut num = 5;

    //declaring pointers is fine in safe rust
    let r1 = &num as *const i32;
    let r2 = &mut num as *const i32;

    //Will not compile unless wrapped in unsafe
    //println!("r1 is: {:?}", *r1);
    unsafe{
        println!("r1 is: {:?}", *r1);
    }
}