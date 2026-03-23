enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

// Note: Structs that don't inherit Copy trait are moved on other
// variables. While you can copy, you don't always necessarily
// want to copy because you want to make references to a particular
// variable. Hence, std::rc::Rc solves this


// Rc::clone doesn't make deep copies but only increments the 
// reference count
fn main() {
    
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("counter after a = {} ", Rc::strong_count(&a));
    
    let b = Cons(3, Rc::clone(&a));
    println!("counter after b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("counter after c = {}", Rc::strong_count(&a));
    }
    println!("counter after c goes out of scope = {}", Rc::strong_count(&a));
    
}
