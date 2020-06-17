use crate::List::{Cons, Nil};

fn main() {
    // Boxes allow us to store data on the heap rather than the stack
    // What remains on the stack is the pointer to the heap data
    // Use in this situations:
    // --> When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
    // --> When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
    // --> When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

    let b = Box::new(5);
    println!("b = {}", b);
    // -------------------------------

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
