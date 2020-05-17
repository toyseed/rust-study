pub struct A {}
pub struct B {}

pub trait Draw {
    fn draw(&self);
}

impl Draw for A {
    fn draw(&self) {
        println!("draw from a");
    }
}

impl Draw for B {
    fn draw(&self) {
        println!("draw from b");
    }
}

struct Screen<T: Draw> {
    pub components: Vec<T>,
}


impl<T> Screen<T>
where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}


#[test]
fn test_generic() {
    let mut r = Some(String::from("b"));
    match r {
        Some(ref s) => {}
        None => {}
    }

    // let v = Vec::new() as Vec<Box<dyn Draw>>;
    // let mut s = Screen {
    //     components: v,
    // };
    //
    // s.components.push(Box::new(A {}));
    // s.components.push(Box::new(B {}));
    //
    // s.run();
}

