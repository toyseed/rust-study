use std::ops::Add;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
struct Millimeters(u32);

#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output =Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + rhs.0 * 1000)
    }
}
impl Add<&Meters> for &Millimeters {
    type Output =Millimeters;

    fn add(self, rhs: &Meters) -> Self::Output {
        Millimeters(self.0 + rhs.0 * 1000)
    }
}

impl Add for Millimeters {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Millimeters(self.0 + rhs.0)
    }
}


// #![allow(unused_variables)]
// fn main() {
//     trait Add<RHS=Self> {    // default generic
//         type Output;
//
//         fn add(self, rhs: RHS) -> Self::Output;
//     }
// }

#[test]
fn test_default_generic() {
    let millimeter = Millimeters(1000);
    let meter = Meters(1);

    let added = &millimeter + &meter;
    assert_eq!(&2000, &added.0);

    println!("{:?}", millimeter);
    let added = millimeter + meter + added;
    assert_eq!(4000, added.0);
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captin speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn fn_takes_trait<T: Wizard>(p: T) {
    p.fly();
}

fn fn_takes_trait_ref<T: Pilot>(p: &T) {
    p.fly();
}

#[test]
fn test_same_method_name() {
    let person = Human;
    person.fly();

    Pilot::fly(&person);
    Wizard::fly(&person);

    fn_takes_trait_ref(&person);
    fn_takes_trait(person);

    let person = Human;

    let b: Box<dyn Wizard> = Box::new(person);
    b.fly();


}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string(); // from Display
        let len = output.len();

        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[test]
fn test_trait_dep() {
    Point {
        x: 5, y: 9,
    }.outline_print();
}