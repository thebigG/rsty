fn main() {
    mutate_string();
    move_string();
    let s1: String = String::from("Who owns me");

    takes_ownership(s1);

     // println!(s1); // Does not compile since takes_ownership function

    let num: i32 = 18;

    makes_copy(num);

    println!("Original:{}", num);

    let s4 = gives_ownership();

    println!("{}", s4);

    let s5:String = String::from("hello");
    let s6 = takes_and_gives_back(s5);
}

fn mutate_string() {
    let mut s: String = String::from("hello");
    println!("Before mutation:{s}");
    s.push_str(", world");
    println!("After mutation:{s}");

}

fn move_string() -> String{
    let s1: String = String::from("hello");
    let s2 = s1;
    return s2;
}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32){
    println!("Copy:{}", some_integer);
}

fn gives_ownership() -> String{
    let some_string = String::from("yours");

    return some_string
}

fn takes_and_gives_back(a_string: String) -> String{
    a_string
}