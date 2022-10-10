#[derive(Debug)] //tells rust to make this printable with "{:?}" and "{:#?}" syntax
struct Rectangle {
    width: u32,
    height: u32,
}

//These could be separated into multiple impl blocks, if we chose to do so.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("res is {:#?}", rect1);

    dbg!(&rect1); // print line numbers
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    let sq1 = Rectangle::square(10);

    println!("Square: {:?}", sq1);
}
