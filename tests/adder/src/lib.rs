extern crate core;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// Note that this function is "private" and our tests can still access it.
fn divide(a: u32, b:u32) -> u32{
    if b == 0{
        panic!("Cannot divide by zero");
    }

    a/b
}

#[cfg(test)]
mod tests {
    use super::Rectangle;
    use super::add;
    use super::divide;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller(){
        let larger = Rectangle{
            width: 8,
            height: 7,
        };

        let smaller = Rectangle{
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    #[ignore]
    fn smaller_cannot_larger(){
        let larger = Rectangle{
            width: 10,
            height: 2,
        };
        let smaller = Rectangle{
            width: 4,
            height: 2
        };

        assert!(!smaller.can_hold(&larger),
                "This is printed when a test fails");
    }

    #[test]
    #[should_panic(expected = "Cannot divide by zero")]
    fn divide_by_zero(){
        divide(1,0);
    }
}
