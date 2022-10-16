fn main() {
    let mut s = String::new();

    let s = String::from("initial contents");
    // let s = "initial contents".to_string();

    // let hello = String::from("السلام عليكم");
    // let hello = String::from("Dobrý den");
    // let hello = String::from("Hello");
    // let hello = String::from("שָׁלוֹם");
    // let hello = String::from("नमस्ते");
    // let hello = String::from("こんにちは");
    // let hello = String::from("안녕하세요");
    // let hello = String::from("你好");
    // let hello = String::from("Olá");
    // let hello = String::from("Здравствуйте");
    // let hello = String::from("Hola");

    let mut str = String::from("lo");

    str.push('l');

    println!("str after move:{str}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let formatted_str = format!("{}{}", s1, s2);

    println!("formatted str:{formatted_str}");
    let s3 = s1 + &s2; //s1 is moved into s3 in this case so it is no longer valid

    println!("added strings:{}", s3);

    string_indexing();
}

fn string_indexing() {
    let s = String::from("hello");
    //let h = s[0]; won't compile because of multi-byte characters in some languages

    for c in "Зд".chars() {
        println!("{}", c);
    }

    //The following is illegal; will not compile. Use  .chars() instead.
    // for c in s{
    //     println!("c in s:{c}")
    // }
}
