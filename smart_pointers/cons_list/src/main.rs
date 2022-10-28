//Some lisp-y code in Rust

use crate::List::{Cons, Nil};
use std::rc::Rc; //Reference-counted immutable smart pointer

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    //"clone" in this case makes a shallow copy since it is reference-counted
    let b = Rc::new(Cons(3, Rc::clone(&a)));

    println!("Reference count for a:{}", Rc::strong_count(&a));
    println!("Reference count for b:{}", Rc::strong_count(&b));
}
