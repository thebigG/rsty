use std::ops::Deref;

fn main() {
    let x = 5;
    let y = MyBox(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); // "*y" invokes the deref methos like this: *(y.deref())
    box_ptr();

    let m = MyBox::new(String::from("Rust"));
    //This works because of deref coercion
    hello(&m);

    let c = CustomerSmartPointer{
        data: String::from(" a stuff")
    };

    //drop(c); a forced drop before c goes out of scope, if needed.

    let d = CustomerSmartPointer{
        data: String::from(" b stuff")
    };

}

fn box_ptr(){
    //This gets deallocated when it goes out of scope; very much like smart pointers
    let b = Box::new(10);
    println!("box = {}", b);
}

fn hello(name: &str){
    println!("Hello, {name}!");
}

struct MyBox<T>(T);

impl<T>  MyBox<T>{
    fn new(x: T) -> MyBox<T>{
    MyBox(x)
    }
}

impl<T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomerSmartPointer{
    data: String,
}

impl Drop for CustomerSmartPointer{
    //gets called when self goes out of scope or is "dropped". This is essentially a destructor
    fn drop(&mut self) {
        println!("Dropping data('freeing') '{}'", self.data);
    }
}