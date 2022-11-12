//declarative macro

#[macro_export]
macro_rules! add_iter {
    (  $x:expr ) => {{
        let mut sum = 0;
        for item in $x {
            sum += item;
        }

        sum
        // let mut temp_vec = Vec::new();
        // $(
        //     temp_vec.push($x);
        // )*
        // temp_vec
    }};
}

fn main() {
    let list = vec![1, 2, 3];
    let result = add_iter![list];
    println!("Declarative add_iter macro:{result}");
}
