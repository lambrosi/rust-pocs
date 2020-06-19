// Use Rc<T> type when we want to allocate some data on the heap
// for multiple parts of our program to read and we can’t determine
// at compile time which part will finish using the data last
// NOTE: use ONLY in single-threaded cases

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Cons(5, Rc::new(Cons(10, Rc::new(Nil))));
    let b = Cons(3, Rc::new(a));
    let c = Cons(4, Rc::new(a));
}