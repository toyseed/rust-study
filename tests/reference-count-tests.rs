use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

#[test]
fn test_rc() {
    let a: Rc<List> = Rc::new(Cons(3, Rc::new(Cons(2, Rc::new(Cons(1, Rc::new(Nil)))))));

    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Rc::new(Cons(4, Rc::clone(&a)));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Rc::new(Cons(5, Rc::clone(&a)));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
