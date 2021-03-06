use std::ops::Deref;

use crate::List::{Cons, Nil};

#[test]
fn test_use_box() {
    let b = Box::new(5);
    println!("b = {}", b);
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[test]
fn test_cons() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let mut r_list = &list;

    loop {
        match r_list {
            Cons(i, b) => {
                println!("print {}", i);

                match **b {
                    Nil => {
                        break;
                    },
                    Cons(_, _) => {
                        r_list = &**b;
                    }
                }
            },
            _ => {
                eprint!("!!!");
                panic!();
            }
        }
    }
}

#[test]
fn test_use_box_2() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

#[test]
fn test_use_mybox() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello("static Rust");
    hello(String::from("dynamic Rust").as_str());
    hello(&String::from("dynamic Rust"));
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'", self.data);
    }
}

#[test]
fn test_drop() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("oter stuff"),
    };

    // error
    // d.drop();

    drop(d);
    println!("CustomSmartPointers created.");
}