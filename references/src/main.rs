fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}'is {}.", s1, len);

    //If the reference were not mutable(mut) the compiler will throw an error
    change_str(&mut s1);

    println!("Changed str:{s1}");

    {
        //r1 is borrowing s1 for its lifetime
        let r1 = &mut s1;
        r1.push_str(", r1 borrower");
        println!("{r1}");
    }

    {
        //r1 is borrowing s1 for its lifetime
        let p1 = &s1;
        // s1.push_str(", p1 borrower"); This will throw an error as well as we are mutating the
        // variable we are borrowing, since as immutable reference we don't expect that to change.
        println!("p1:{p1}");
    }

    {
        // Transfer of ownership(move). s1 is no longer valid. As a new owner we are allowed to
        // set the mutability of r2. We could have made it non-mutable if we chose to.
        let mut r2 = s1;
        r2.push_str(", r2 new owner");
        println!("{r2}");
    }

    //s1.push_str(", invalid"); invalid as s1 is no longer the owner of the string
    //r2.push_str(", invalid"); invalid as r2 is outside of its scope(lifetime)

    let mut s2 = String::from("hello from s2");

    {
        let x2 = &s2;
        println!("x2:{x2}");
        let x1 = &mut s2;
        // let x2 = &mut s2; This will throw an error because of multiple borrowers at the same time
        x1.push_str(", x1");
        s2.push_str(", s2");
        // println!("x2:{x2}"); This will throw an error as well as we are mutating the
        //variable we are borrowing, since as immutable reference we don't expect that to change.
    }

    println!("s2:{s2}");
}

//The following functions are examples of borrowing in Rust.
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_str(some_string: &mut String) {
    some_string.push_str(", world");
}

//This is bad, but the rust compiler won't allow it :)
// fn dangle() -> &String {
//     let s = String::from("hello");
//
//     &s
// }
