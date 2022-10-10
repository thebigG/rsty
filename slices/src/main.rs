fn main() {
    let mut s = String::from("Hello world!");

    let  f_word =first_word(&s);

    // s.clear(); compiler error as there is an immutable borrow in f_word

    println!("first word slice: '{f_word}'");

    s.clear();
}

fn first_word(s: &str) -> &str{
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }

    &s[..]
}
