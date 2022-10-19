struct Point<T> {
    x: T,
    y: T,
}
//Just to illustrate we can do the same with generic enums
enum Result<T, E> {
    Ok(T),
    Err(E),
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

//You can mix generics as well
// struct Point<X1, Y1> {
//     x: X1,
//     y: Y1,
// }
//
// impl<X1, Y1> Point<X1, Y1> {
//     fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

//The "&[]" notation refers to a slice just like &str in the case of String objects
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    //item is a "&i32" type in this case
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

//Same as functions above, except this is generic
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest_i32(&number_list);
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest_char(&char_list);
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let p1 = Point { x: 1, y: 10 };
    // let p2 = Point {x: 1, y: 10};
}
