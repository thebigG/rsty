enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let immut_v = new_empty_immutable_i32_vec();
    // immut_v.push(100); compiler error as v is immutable

    let mut mut_v = new_empty_mutable_i32_vec();
    mut_v.push(12);

    let v = vec![1, 2, 3, 4, 5];

    let fourth = &v[3]; //accessing an out-of-bounds with [] will cause a panic!
    println!("The fourth element is {fourth}");

    let fourth: Option<&i32> = v.get(3);

    match fourth {
        Some(element) => println!("The fourth element is {element}"),
        None => println!("The fourth element is none"),
    }

    access_vec_elements_with_ptrs();

    //The borrow checker will not let the following compile
    // let mut v = vec![1, 2, 3, 4, 5];
    //
    // let first = &v[0];
    //
    // v.push(6); //this is not legal as at the moment we are trying to mutate a vector while having
    //an immutable reference to one of its elements
    //
    // println!("The first element is: {}", first);
}

fn new_empty_immutable_i32_vec() -> Vec<i32> {
    let v: Vec<i32> = Vec::new();
    //same as above
    // let v = vec![1,2,3];
    v
}

fn new_empty_mutable_i32_vec() -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    //same as above
    // let v = vec![1,2,3];
    v
}

fn access_vec_elements_with_ptrs() {
    let mut v = vec![100, 32, 57];

    for i in &mut v {
        *i += 50;
    }

    for i in 0..v.len() {
        println!("i:{i}");
    }

    println!("{:?}", v);
}

fn vec_enums() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
