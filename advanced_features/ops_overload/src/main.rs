use std::ops::Add;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point{
    x: i32,
    y: i32
}

impl Add for Point{
    type Output = Point;

    fn add(self, other: Point) -> Point{
        Point{
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

//This is a super trait
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
fn main() {
    assert_eq!(Point{ x: 2, y: 5 } + Point{ x: 1, y: 2 },
                Point{ x: 3, y: 7 })
}
